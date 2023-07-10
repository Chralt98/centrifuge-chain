// Copyright 2023 Centrifuge Foundation (centrifuge.io).
//
// This file is part of the Centrifuge chain project.
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use cfg_primitives::{Balance, PoolId, TrancheId};
use cfg_traits::connectors::Codec;
use cfg_types::{
	domain_address::{Domain, DomainAddress},
	fixed_point::Rate,
	tokens::{CurrencyId, CustomMetadata},
};
use connectors_gateway_routers::{
	axelar_evm::AxelarEVMRouter, ethereum_xcm::EthereumXCMRouter, DomainRouter, EVMChain,
	EVMDomain, FeeValues, XcmDomain, XcmTransactInfo,
};
use development_runtime::xcm::CurrencyIdConvert;
use frame_support::{
	assert_ok,
	dispatch::{GetDispatchInfo, Pays},
	weights::Weight,
};
use fudge::primitives::Chain;
use orml_traits::asset_registry::AssetMetadata;
use pallet_connectors::Message;
use pallet_connectors_gateway::GatewayOrigin;
use pallet_democracy::{AccountVote, Conviction, ReferendumIndex, Vote, VoteThreshold};
use sp_core::{bounded::BoundedVec, bounded_vec, H160, H256};
use sp_runtime::{
	traits::{BlakeTwo256, Convert, Hash},
	Storage,
};
use tokio::runtime::Handle;
use xcm::{
	latest::{Junction, Junctions, MultiLocation},
	VersionedMultiLocation,
};

use crate::{
	chain::centrifuge::{
		AccountId, CouncilCollective, FastTrackVotingPeriod, MinimumDeposit, Runtime, RuntimeCall,
		RuntimeEvent, PARA_ID,
	},
	utils::{
		accounts::Keyring,
		collective::{collective_close, collective_propose, collective_vote},
		connectors_gateway::{add_connector, remove_connector, set_domain_router},
		democracy::{democracy_vote, execute_via_democracy, external_propose_majority, fast_track},
		env::{ChainState, EventRange},
		preimage::note_preimage,
		*,
	},
};

fn get_council_members() -> Vec<Keyring> {
	vec![Keyring::Alice, Keyring::Bob, Keyring::Charlie]
}

#[tokio::test]
async fn set_router() {
	let mut env = {
		let mut genesis = Storage::default();
		genesis::default_balances::<Runtime>(&mut genesis);
		genesis::council_members::<Runtime, CouncilCollective>(get_council_members(), &mut genesis);
		env::test_env_with_centrifuge_storage(Handle::current(), genesis)
	};

	let test_domain = Domain::EVM(1);

	let xcm_domain_location = MultiLocation {
		parents: 0,
		interior: Junctions::X1(Junction::Parachain(456)),
	};

	let currency_id = CurrencyId::ForeignAsset(1);
	let currency_location = MultiLocation {
		parents: 0,
		interior: Junctions::X1(Junction::Parachain(123)),
	};

	let currency_meta = AssetMetadata::<Balance, CustomMetadata> {
		decimals: 18,
		name: "Test".into(),
		symbol: "TST".into(),
		existential_deposit: 1_000_000,
		location: Some(VersionedMultiLocation::V3(currency_location)),
		additional: Default::default(),
	};

	let xcm_domain = XcmDomain {
		location: Box::new(xcm_domain_location.clone().into_versioned()),
		ethereum_xcm_transact_call_index: bounded_vec![0],
		contract_address: H160::from_slice(rand::random::<[u8; 20]>().as_slice()),
		max_gas_limit: 10,
		transact_info: XcmTransactInfo {
			transact_extra_weight: 1.into(),
			max_weight: 100_000_000_000.into(),
			transact_extra_weight_signed: None,
		},
		fee_currency: currency_id,
		fee_per_second: 1u128,
		fee_asset_location: Box::new(currency_location.clone().into_versioned()),
	};

	let ethereum_xcm_router = EthereumXCMRouter::<Runtime> {
		xcm_domain: xcm_domain,
		_marker: Default::default(),
	};

	let test_router = DomainRouter::<Runtime>::EthereumXCM(ethereum_xcm_router);

	let set_domain_router_call = set_domain_router(test_domain.clone(), test_router.clone());

	let council_threshold = 2;
	let voting_period = 3;

	execute_via_democracy(
		&mut env,
		get_council_members(),
		set_domain_router_call,
		council_threshold,
		voting_period,
		0,
		0,
	);

	env::evolve_until_event_is_found!(
		env,
		Chain::Para(PARA_ID),
		RuntimeEvent,
		voting_period + 1,
		RuntimeEvent::ConnectorsGateway(pallet_connectors_gateway::Event::DomainRouterSet {
			domain,
			router,
		}) if [*domain == test_domain && *router == test_router],
	);
}

#[tokio::test]
async fn add_remove_connectors() {
	let mut env = {
		let mut genesis = Storage::default();
		genesis::default_balances::<Runtime>(&mut genesis);
		genesis::council_members::<Runtime, CouncilCollective>(get_council_members(), &mut genesis);
		env::test_env_with_centrifuge_storage(Handle::current(), genesis)
	};

	let test_connector = DomainAddress::EVM {
		0: 1,
		1: H160::random().0,
	};

	let add_connector_call = add_connector(test_connector.clone());

	let council_threshold = 2;
	let voting_period = 3;

	let (prop_index, ref_index) = execute_via_democracy(
		&mut env,
		get_council_members(),
		add_connector_call,
		council_threshold,
		voting_period,
		0,
		0,
	);

	env::evolve_until_event_is_found!(
		env,
		Chain::Para(PARA_ID),
		RuntimeEvent,
		voting_period + 1,
		RuntimeEvent::ConnectorsGateway(pallet_connectors_gateway::Event::ConnectorAdded {
			connector,
		}) if [*connector == test_connector],
	);

	let remove_connector_call = remove_connector(test_connector.clone());

	execute_via_democracy(
		&mut env,
		get_council_members(),
		remove_connector_call,
		council_threshold,
		voting_period,
		prop_index,
		ref_index,
	);

	env::evolve_until_event_is_found!(
		env,
		Chain::Para(PARA_ID),
		RuntimeEvent,
		voting_period + 1,
		RuntimeEvent::ConnectorsGateway(pallet_connectors_gateway::Event::ConnectorRemoved {
			connector,
		}) if [*connector == test_connector],
	);
}

#[tokio::test]
async fn process_msg() {
	let mut env = {
		let mut genesis = Storage::default();
		genesis::default_balances::<Runtime>(&mut genesis);
		genesis::council_members::<Runtime, CouncilCollective>(get_council_members(), &mut genesis);
		env::test_env_with_centrifuge_storage(Handle::current(), genesis)
	};

	let test_connector = DomainAddress::EVM {
		0: 1,
		1: H160::random().0,
	};

	let add_connector_call = add_connector(test_connector.clone());

	let council_threshold = 2;
	let voting_period = 3;

	let (prop_index, ref_index) = execute_via_democracy(
		&mut env,
		get_council_members(),
		add_connector_call,
		council_threshold,
		voting_period,
		0,
		0,
	);

	env::evolve_until_event_is_found!(
		env,
		Chain::Para(PARA_ID),
		RuntimeEvent,
		voting_period + 1,
		RuntimeEvent::ConnectorsGateway(pallet_connectors_gateway::Event::ConnectorAdded {
			connector,
		}) if [*connector == test_connector],
	);

	let msg = Message::<Domain, PoolId, TrancheId, Balance, Rate>::Transfer {
		currency: 0,
		sender: Keyring::Alice.public().0,
		receiver: Keyring::Bob.public().0,
		amount: 1_000u128,
	};

	let encoded_msg = msg.serialize();

	let gateway_msg = BoundedVec::<
		u8,
		<Runtime as pallet_connectors_gateway::Config>::MaxIncomingMessageSize,
	>::try_from(encoded_msg)
	.unwrap();

	let res = env
		.with_state(Chain::Para(PARA_ID), || {
			pallet_connectors_gateway::Pallet::<Runtime>::process_msg(
				GatewayOrigin::Local(test_connector).into(),
				gateway_msg,
			)
		})
		.unwrap();
	assert_ok!(res);
}
