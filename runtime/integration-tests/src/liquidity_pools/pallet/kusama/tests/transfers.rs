// Copyright 2021 Centrifuge GmbH (centrifuge.io).
// This file is part of Centrifuge chain project.
//
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// Copyright 2021 Centrifuge GmbH (centrifuge.io).
// This file is part of Centrifuge chain project.
//
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use altair_runtime::{Balances, OrmlAssetRegistry, OrmlTokens, RuntimeOrigin, XTokens};
use cfg_primitives::{constants::currency_decimals, parachains, Balance};
use cfg_types::{
	tokens::{CrossChainTransferability, CurrencyId, CustomMetadata},
	xcm::XcmMetadata,
};
use frame_support::assert_ok;
use orml_traits::{asset_registry::AssetMetadata, FixedConversionRateProvider, MultiCurrency};
use runtime_common::{
	xcm::general_key,
	xcm_fees::{default_per_second, ksm_per_second},
};
use sp_runtime::DispatchError::BadOrigin;
use xcm::{
	latest::{Junction, Junction::*, Junctions::*, MultiLocation, NetworkId, WeightLimit},
	VersionedMultiLocation,
};
use xcm_emulator::TestExt;

use crate::{
	liquidity_pools::pallet::{
		kusama::{
			setup::{
				air, altair_account, ausd, foreign, karura_account, ksm, sibling_account, ALICE,
				BOB, PARA_ID_SIBLING,
			},
			test_net::{Altair, Karura, KusamaNet, Sibling, TestNet},
			tests::register_ausd,
		},
		xcm_metadata,
	},
	utils::AUSD_CURRENCY_ID,
};

/*

NOTE: We hardcode the expected balances after an XCM operation given that the weights involved in
XCM execution often change slightly with each Polkadot update. We could simply test that the final
balance after some XCM operation is `initialBalance - amount - fee`, which would mean we would
never have to touch the tests again. However, by hard-coding these values we are forced to catch
an unexpectedly big change that would have a big impact on the weights and fees and thus balances,
which would go unnoticed and untreated otherwise.

 */

#[test]
fn transfer_air_to_sibling() {
	TestNet::reset();

	let alice_initial_balance = air(10);
	let bob_initial_balance = air(10);
	let transfer_amount = air(1);
	let transfer_amount = air(5);
	let air_in_sibling = CurrencyId::ForeignAsset(12);

	Altair::execute_with(|| {
		assert_eq!(Balances::free_balance(&ALICE.into()), alice_initial_balance);
		assert_eq!(Balances::free_balance(&sibling_account()), 0);

		// Register AIR as foreign asset in the sibling parachain
		let meta: AssetMetadata<Balance, CustomMetadata> = AssetMetadata {
			decimals: 18,
			name: "Altair".into(),
			symbol: "AIR".into(),
			existential_deposit: 1_000_000_000_000,
			location: Some(VersionedMultiLocation::V3(MultiLocation::new(
				1,
				X2(
					Parachain(parachains::kusama::altair::ID),
					general_key(parachains::kusama::altair::AIR_KEY),
				),
			))),
			additional: CustomMetadata {
				transferability: CrossChainTransferability::Xcm(Default::default()),
				..CustomMetadata::default()
			},
		};
		assert_ok!(OrmlAssetRegistry::register_asset(
			RuntimeOrigin::root(),
			meta,
			Some(CurrencyId::Native)
		));
	});

	Sibling::execute_with(|| {
		assert_eq!(OrmlTokens::free_balance(air_in_sibling, &BOB.into()), 0);

		// Register AIR as foreign asset in the sibling parachain
		let meta: AssetMetadata<Balance, CustomMetadata> = AssetMetadata {
			decimals: 18,
			name: "Altair".into(),
			symbol: "AIR".into(),
			existential_deposit: 1_000_000_000_000,
			location: Some(VersionedMultiLocation::V3(MultiLocation::new(
				1,
				X2(
					Parachain(parachains::kusama::altair::ID),
					general_key(parachains::kusama::altair::AIR_KEY),
				),
			))),
			additional: CustomMetadata {
				transferability: CrossChainTransferability::Xcm(Default::default()),
				..CustomMetadata::default()
			},
		};
		assert_ok!(OrmlAssetRegistry::register_asset(
			RuntimeOrigin::root(),
			meta,
			Some(air_in_sibling)
		));
	});

	Altair::execute_with(|| {
		assert_ok!(XTokens::transfer(
			RuntimeOrigin::signed(ALICE.into()),
			CurrencyId::Native,
			transfer_amount,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(PARA_ID_SIBLING),
						Junction::AccountId32 {
							network: None,
							id: BOB,
						}
					)
				)
				.into()
			),
			WeightLimit::Limited(8_000_000_000_000.into()),
		));

		// Confirm that Alice's balance is initial balance - amount transferred
		assert_eq!(
			Balances::free_balance(&ALICE.into()),
			alice_initial_balance - transfer_amount
		);

		// Verify that the amount transferred is now part of the sibling account here
		assert_eq!(Balances::free_balance(&sibling_account()), transfer_amount);
	});

	Sibling::execute_with(|| {
		let current_balance = OrmlTokens::free_balance(air_in_sibling, &BOB.into());

		// Verify that BOB now has (amount transferred - fee)
		assert_eq!(current_balance, transfer_amount - fee(18));

		// Sanity check for the actual amount BOB ends up with
		assert_eq!(current_balance, 4991987200000000000);
	});
}

#[test]
fn transfer_air_sibling_to_altair() {
	TestNet::reset();

	// In order to be able to transfer AIR from Sibling to Altair, we need to first
	// send AIR from Altair to Sibling, or else it fails since it'd be like Sibling
	// had minted AIR on their side.
	transfer_air_to_sibling();

	let alice_initial_balance = air(5);
	let bob_initial_balance = air(5) - air_fee();
	let transfer_amount = air(1);
	// Note: This asset was registered in `transfer_air_to_sibling`
	let air_in_sibling = CurrencyId::ForeignAsset(12);

	Altair::execute_with(|| {
		assert_eq!(Balances::free_balance(&ALICE.into()), alice_initial_balance);
	});

	Sibling::execute_with(|| {
		assert_eq!(Balances::free_balance(&altair_account()), 0);
		assert_eq!(
			OrmlTokens::free_balance(air_in_sibling, &BOB.into()),
			bob_initial_balance
		);
	});

	Sibling::execute_with(|| {
		assert_ok!(XTokens::transfer(
			RuntimeOrigin::signed(BOB.into()),
			air_in_sibling,
			transfer_amount,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(parachains::kusama::altair::ID),
						Junction::AccountId32 {
							network: None,
							id: ALICE,
						}
					)
				)
				.into()
			),
			WeightLimit::Limited(8_000_000_000_000.into()),
		));

		// Confirm that Bobs's balance is initial balance - amount transferred
		assert_eq!(
			OrmlTokens::free_balance(air_in_sibling, &BOB.into()),
			bob_initial_balance - transfer_amount
		);
	});

	Altair::execute_with(|| {
		// Verify that ALICE now has initial balance + amount transferred - fee
		assert_eq!(
			Balances::free_balance(&ALICE.into()),
			alice_initial_balance + transfer_amount - air_fee(),
		);
	});
}

#[test]
fn transfer_ausd_to_altair() {
	TestNet::reset();

	let alice_initial_balance = ausd(10);
	let transfer_amount = ausd(7);

	Karura::execute_with(|| {
		register_ausd();

		assert_ok!(OrmlTokens::deposit(
			AUSD_CURRENCY_ID,
			&ALICE.into(),
			alice_initial_balance
		));

		assert_eq!(
			OrmlTokens::free_balance(AUSD_CURRENCY_ID, &altair_account()),
			0
		);
	});

	Altair::execute_with(|| {
		register_ausd();

		assert_eq!(OrmlTokens::free_balance(AUSD_CURRENCY_ID, &BOB.into()), 0,);
	});

	Karura::execute_with(|| {
		assert_eq!(
			OrmlTokens::free_balance(AUSD_CURRENCY_ID, &ALICE.into()),
			ausd(10),
		);
		assert_ok!(XTokens::transfer(
			RuntimeOrigin::signed(ALICE.into()),
			AUSD_CURRENCY_ID,
			transfer_amount,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(parachains::kusama::altair::ID),
						Junction::AccountId32 {
							network: None,
							id: BOB,
						}
					)
				)
				.into()
			),
			WeightLimit::Limited(8_000_000_000_000.into()),
		));

		assert_eq!(
			OrmlTokens::free_balance(AUSD_CURRENCY_ID, &ALICE.into()),
			alice_initial_balance - transfer_amount
		);

		// Verify that the amount transferred is now part of the altair parachain
		// account here
		assert_eq!(
			OrmlTokens::free_balance(AUSD_CURRENCY_ID, &altair_account()),
			transfer_amount
		);
	});

	Altair::execute_with(|| {
		// Verify that BOB now has initial balance + amount transferred - fee
		assert_eq!(
			OrmlTokens::free_balance(AUSD_CURRENCY_ID, &BOB.into()),
			transfer_amount - ausd_fee()
		);
	});
}

#[test]
fn transfer_ksm_from_relay_chain() {
	let transfer_amount: Balance = ksm(2);
	let currency_id = CurrencyId::ForeignAsset(3001);
	let meta: AssetMetadata<Balance, CustomMetadata> = AssetMetadata {
		decimals: 12,
		name: "Kusama".into(),
		symbol: "KSM".into(),
		existential_deposit: 1_000_000_000,
		location: Some(VersionedMultiLocation::V3(MultiLocation::new(1, Here))),
		additional: CustomMetadata {
			transferability: CrossChainTransferability::Xcm(Default::default()),
			..CustomMetadata::default()
		},
	};

	Altair::execute_with(|| {
		assert_ok!(OrmlAssetRegistry::register_asset(
			RuntimeOrigin::root(),
			meta.clone(),
			Some(currency_id),
		));

		assert_eq!(OrmlTokens::free_balance(currency_id, &BOB.into()), 0);
	});

	KusamaNet::execute_with(|| {
		assert_ok!(OrmlAssetRegistry::register_asset(
			RuntimeOrigin::root(),
			AssetMetadata {
				location: Some(VersionedMultiLocation::V3(MultiLocation::new(0, Here))),
				..meta.clone()
			},
			Some(CurrencyId::Native),
		));

		assert_ok!(kusama_runtime::XcmPallet::reserve_transfer_assets(
			kusama_runtime::RuntimeOrigin::signed(ALICE.into()),
			Box::new(Parachain(parachains::kusama::altair::ID).into()),
			Box::new(
				Junction::AccountId32 {
					network: None,
					id: BOB,
				}
				.into()
			),
			Box::new((Here, transfer_amount).into()),
			0
		));
	});

	Altair::execute_with(|| {
		assert_eq!(
			OrmlTokens::free_balance(currency_id, &BOB.into()),
			transfer_amount - fee(meta.decimals)
		);
	});
}

#[test]
fn transfer_ksm_to_relay_chain() {
	// First we need some KSM on Altair
	transfer_ksm_from_relay_chain();

	let currency_id = CurrencyId::ForeignAsset(3001);
	Altair::execute_with(|| {
		assert_eq!(
			OrmlTokens::free_balance(currency_id, &BOB.into()),
			1991987200000
		);

		assert_ok!(XTokens::transfer(
			RuntimeOrigin::signed(BOB.into()),
			currency_id,
			ksm(1),
			Box::new(
				MultiLocation::new(
					1,
					X1(Junction::AccountId32 {
						id: BOB,
						network: None,
					})
				)
				.into()
			),
			WeightLimit::Limited(4_000_000_000.into())
		));
	});

	KusamaNet::execute_with(|| {
		assert_eq!(
			kusama_runtime::Balances::free_balance(&BOB.into()),
			999909712564
		);
	});
}

#[test]
fn transfer_foreign_sibling_to_altair() {
	TestNet::reset();

	let alice_initial_balance = air(10);
	let sibling_asset_id = CurrencyId::ForeignAsset(1);
	let asset_location =
		MultiLocation::new(1, X2(Parachain(PARA_ID_SIBLING), general_key(&[0, 1])));
	let meta: AssetMetadata<Balance, CustomMetadata> = AssetMetadata {
		decimals: 18,
		name: "Sibling Native Token".into(),
		symbol: "SBLNG".into(),
		existential_deposit: 1_000_000_000_000,
		location: Some(VersionedMultiLocation::V3(asset_location)),
		additional: CustomMetadata {
			transferability: CrossChainTransferability::Xcm(XcmMetadata {
				// We specify a custom fee_per_second and verify below that this value is
				// used when XCM transfer fees are charged for this token.
				fee_per_second: Some(8420000000000000000),
			}),
			..CustomMetadata::default()
		},
	};
	let transfer_amount = foreign(1, meta.decimals);

	Sibling::execute_with(|| {
		assert_eq!(OrmlTokens::free_balance(sibling_asset_id, &BOB.into()), 0);
		assert_ok!(OrmlAssetRegistry::register_asset(
			RuntimeOrigin::root(),
			meta.clone(),
			Some(CurrencyId::Native),
		));
	});

	Altair::execute_with(|| {
		assert_ok!(OrmlAssetRegistry::register_asset(
			RuntimeOrigin::root(),
			meta.clone(),
			Some(sibling_asset_id)
		));
	});

	Sibling::execute_with(|| {
		assert_ok!(XTokens::transfer(
			RuntimeOrigin::signed(ALICE.into()),
			CurrencyId::Native,
			transfer_amount,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(parachains::kusama::altair::ID),
						Junction::AccountId32 {
							network: None,
							id: BOB,
						}
					)
				)
				.into()
			),
			WeightLimit::Limited(8_000_000_000_000.into()),
		));

		// Confirm that Alice's balance is initial balance - amount transferred
		assert_eq!(
			Balances::free_balance(&ALICE.into()),
			alice_initial_balance - transfer_amount
		);
	});

	Altair::execute_with(|| {
		let bob_balance = OrmlTokens::free_balance(sibling_asset_id, &BOB.into());

		// Verify that BOB now has initial balance + amount transferred - fee
		assert_eq!(
			bob_balance,
			transfer_amount
				- calc_fee(
					xcm_metadata(meta.additional.transferability)
						.unwrap()
						.fee_per_second
						.unwrap()
				)
		);
		// Sanity check to ensure the calculated is what is expected
		assert_eq!(bob_balance, 993264000000000000);
	});
}

#[test]
fn transfer_wormhole_usdc_karura_to_altair() {
	TestNet::reset();

	let usdc_asset_id = CurrencyId::ForeignAsset(39);
	let asset_location = MultiLocation::new(
		1,
		X2(
			Parachain(parachains::kusama::karura::ID),
			general_key("0x02f3a00dd12f644daec907013b16eb6d14bf1c4cb4".as_bytes()),
		),
	);
	let meta: AssetMetadata<Balance, CustomMetadata> = AssetMetadata {
		decimals: 6,
		name: "Wormhole USDC".into(),
		symbol: "WUSDC".into(),
		existential_deposit: 1,
		location: Some(VersionedMultiLocation::V3(asset_location)),
		additional: CustomMetadata {
			transferability: CrossChainTransferability::Xcm(Default::default()),
			..CustomMetadata::default()
		},
	};
	let transfer_amount = foreign(12, meta.decimals);
	let alice_initial_balance = transfer_amount * 100;

	Karura::execute_with(|| {
		assert_ok!(OrmlAssetRegistry::register_asset(
			RuntimeOrigin::root(),
			meta.clone(),
			Some(usdc_asset_id)
		));
		assert_ok!(OrmlTokens::deposit(
			usdc_asset_id,
			&ALICE.into(),
			alice_initial_balance
		));
		assert_eq!(
			OrmlTokens::free_balance(usdc_asset_id, &ALICE.into()),
			alice_initial_balance
		);
		assert_eq!(Balances::free_balance(&ALICE.into()), air(10));
	});

	Altair::execute_with(|| {
		// First, register the asset in centrifuge
		assert_ok!(OrmlAssetRegistry::register_asset(
			RuntimeOrigin::root(),
			meta.clone(),
			Some(usdc_asset_id)
		));
	});

	Karura::execute_with(|| {
		assert_ok!(XTokens::transfer(
			RuntimeOrigin::signed(ALICE.into()),
			usdc_asset_id,
			transfer_amount,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(parachains::kusama::altair::ID),
						Junction::AccountId32 {
							network: None,
							id: BOB,
						}
					)
				)
				.into()
			),
			WeightLimit::Limited(8_000_000_000.into()),
		));

		// Confirm that Alice's balance is initial balance - amount transferred
		assert_eq!(
			OrmlTokens::free_balance(usdc_asset_id, &ALICE.into()),
			alice_initial_balance - transfer_amount
		);
	});

	Altair::execute_with(|| {
		let bob_balance = OrmlTokens::free_balance(usdc_asset_id, &BOB.into());

		// Sanity check to ensure the calculated is what is expected
		assert_eq!(bob_balance, 11991988);
	});
}

#[test]
fn test_total_fee() {
	assert_eq!(air_fee(), 8012800000000000);
}

fn air_fee() -> Balance {
	fee(currency_decimals::NATIVE)
}

fn ausd_fee() -> Balance {
	fee(currency_decimals::AUSD)
}

fn fee(decimals: u32) -> Balance {
	calc_fee(default_per_second(decimals))
}

// The fee associated with transferring KSM tokens
fn ksm_fee() -> Balance {
	calc_fee(ksm_per_second())
}

fn calc_fee(fee_per_second: Balance) -> Balance {
	// We divide the fee to align its unit and multiply by 4 as that seems to be the
	// unit of time the tests take.
	// NOTE: it is possible that in different machines this value may differ. We
	// shall see.
	fee_per_second.div_euclid(10_000) * 8
}
