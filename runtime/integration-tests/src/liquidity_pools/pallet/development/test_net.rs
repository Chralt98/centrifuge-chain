// Copyright 2021 Centrifuge GmbH (centrifuge.io).
// This file is part of Centrifuge chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! Relay chain and parachains emulation.

use cfg_primitives::{parachains, AccountId};
use cfg_types::tokens::CurrencyId;
use cumulus_primitives_core::ParaId;
use frame_support::{traits::GenesisBuild, weights::Weight};
use polkadot_parachain::primitives::Id;
use polkadot_primitives::{BlockNumber, MAX_CODE_SIZE, MAX_POV_SIZE};
use polkadot_runtime_parachains::{
	configuration::HostConfiguration,
	paras::{ParaGenesisArgs, ParaKind},
};
use sp_runtime::traits::AccountIdConversion;
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain, TestExt};

use super::setup::{cfg, ExtBuilder, ALICE, BOB, PARA_ID_MOONBEAM};
use crate::utils::{
	AUSD_CURRENCY_ID, AUSD_ED, GLMR_CURRENCY_ID, GLMR_ED, USDT_CURRENCY_ID, USDT_ED,
};

decl_test_relay_chain! {
	pub struct RelayChain {
		Runtime = polkadot_runtime::Runtime,
		RuntimeCall = polkadot_runtime::RuntimeCall,
		RuntimeEvent = polkadot_runtime::RuntimeEvent,
		XcmConfig = polkadot_runtime::xcm_config::XcmConfig,
		MessageQueue = polkadot_runtime::MessageQueue,
		System = polkadot_runtime::System,
		new_ext = relay_ext(),
	}
}

decl_test_parachain! {
	pub struct Development {
		Runtime = development_runtime::Runtime,
		XcmpMessageHandler = development_runtime::XcmpQueue,
		DmpMessageHandler = development_runtime::DmpQueue,
		new_ext = para_ext(parachains::polkadot::centrifuge::ID),
	}
}

decl_test_parachain! {
	pub struct Moonbeam {
		Runtime = development_runtime::Runtime,
		XcmpMessageHandler = development_runtime::XcmpQueue,
		DmpMessageHandler = development_runtime::DmpQueue,
		new_ext = para_ext(PARA_ID_MOONBEAM),
	}
}

decl_test_network! {
	pub struct TestNet {
		relay_chain = RelayChain,
		parachains = vec![
			// N.B: Ideally, we could use the defined para id constants but doing so
			// fails with: "error: arbitrary expressions aren't allowed in patterns"

			// Be sure to use `parachains::polkadot::centrifuge::ID`
			(2031, Development),
			// Be sure to use `PARA_ID_MOONBEAM`
			(2023, Moonbeam),
		],
	}
}

pub fn relay_ext() -> sp_io::TestExternalities {
	use polkadot_runtime::{Runtime, System};

	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Runtime>()
		.unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![
			(AccountId::from(ALICE), cfg(2002)),
			(
				ParaId::from(parachains::polkadot::centrifuge::ID).into_account_truncating(),
				cfg(7),
			),
			(
				ParaId::from(PARA_ID_MOONBEAM).into_account_truncating(),
				cfg(7),
			),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	polkadot_runtime_parachains::configuration::GenesisConfig::<Runtime> {
		config: default_parachains_host_configuration(),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	<polkadot_runtime_parachains::paras::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(&polkadot_runtime_parachains::paras::GenesisConfig {
		paras: vec![
			(
				Id::from(parachains::polkadot::centrifuge::ID),
				ParaGenesisArgs {
					genesis_head: Default::default(),
					validation_code: vec![1].into(),
					para_kind: ParaKind::Parachain,
				},
			),
			(
				Id::from(PARA_ID_MOONBEAM),
				ParaGenesisArgs {
					genesis_head: Default::default(),
					validation_code: vec![2].into(),
					para_kind: ParaKind::Parachain,
				},
			),
		],
	}, &mut t).unwrap();

	<pallet_xcm::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
		&pallet_xcm::GenesisConfig {
			safe_xcm_version: Some(2),
		},
		&mut t,
	)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);

	ext.execute_with(|| {
		polkadot_runtime_parachains::hrmp::Pallet::<Runtime>::force_open_hrmp_channel(
			polkadot_runtime::RuntimeOrigin::root(),
			Id::from(parachains::polkadot::centrifuge::ID),
			Id::from(PARA_ID_MOONBEAM),
			10,
			1024,
		)
		.unwrap();

		polkadot_runtime_parachains::hrmp::Pallet::<Runtime>::force_process_hrmp_open(
			polkadot_runtime::RuntimeOrigin::root(),
			0,
		)
		.unwrap();

		System::set_block_number(1);
	});
	ext
}

pub fn para_ext(parachain_id: u32) -> sp_io::TestExternalities {
	ExtBuilder::default()
		.balances(vec![
			(AccountId::from(ALICE), CurrencyId::Native, cfg(10_000)),
			(AccountId::from(BOB), CurrencyId::Native, cfg(10_000)),
			(AccountId::from(ALICE), AUSD_CURRENCY_ID, AUSD_ED),
			(AccountId::from(BOB), AUSD_CURRENCY_ID, AUSD_ED),
			(AccountId::from(ALICE), USDT_CURRENCY_ID, USDT_ED),
			(AccountId::from(BOB), USDT_CURRENCY_ID, USDT_ED),
			(AccountId::from(ALICE), GLMR_CURRENCY_ID, GLMR_ED),
			(AccountId::from(BOB), GLMR_CURRENCY_ID, GLMR_ED),
		])
		.parachain_id(parachain_id)
		.build()
}

fn default_parachains_host_configuration() -> HostConfiguration<BlockNumber> {
	HostConfiguration {
		hrmp_channel_max_capacity: u32::MAX,
		hrmp_channel_max_total_size: u32::MAX,
		hrmp_max_parachain_inbound_channels: 10,
		hrmp_max_parachain_outbound_channels: 10,
		hrmp_channel_max_message_size: u32::MAX,
		// Changed to avoid aritmetic errors within hrmp_close
		max_downward_message_size: 100_000u32,
		..Default::default()
	}
}
