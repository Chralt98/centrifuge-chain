//! Autogenerated weights for pallet_migration_manager
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("development-local"), DB CACHE: 1024

// Executed Command:
// ./target/release/centrifuge-chain
// benchmark
// --chain=development-local
// --steps=50
// --repeat=20
// --pallet=pallet_migration_manager
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/development/src/weights/pallet_migration_manager.rs
// --template=./scripts/runtime-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weights for pallet_migration_manager using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_migration_manager::weights::WeightInfo for WeightInfo<T> {
	fn finalize() -> Weight {
		Weight::from_ref_time(26_615_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn migrate_system_account(n: u32) -> Weight {
		Weight::from_ref_time(27_663_000) // Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(1_582_000).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
	}

	fn migrate_balances_issuance() -> Weight {
		Weight::from_ref_time(34_731_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn migrate_vesting_vesting(n: u32) -> Weight {
		Weight::from_ref_time(120_260_000) // Standard Error: 359_000
			.saturating_add(Weight::from_ref_time(54_192_000).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(n as u64)))
	}

	fn migrate_proxy_proxies(n: u32) -> Weight {
		Weight::from_ref_time(99_970_000) // Standard Error: 172_000
			.saturating_add(Weight::from_ref_time(10_752_000).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
	}
}
