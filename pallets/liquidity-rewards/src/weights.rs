//! Autogenerated weights for pallet_liquidity_rewards
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-26, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("development"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=development
// --steps=50
// --repeat=20
// --pallet=pallet-liquidity-rewards
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/liquidity-rewards/src/weights.rs
// --template=./scripts/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_liquidity_rewards.
pub trait WeightInfo {
	fn on_initialize(x: u32, y: u32, z: u32) -> Weight;
	fn stake() -> Weight;
	fn unstake() -> Weight;
	fn claim_reward() -> Weight;
	fn set_distributed_reward() -> Weight;
	fn set_epoch_duration() -> Weight;
	fn set_group_weight() -> Weight;
	fn set_currency_group() -> Weight;
}

/// Weights for pallet_liquidity_rewards using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn on_initialize(x: u32, y: u32, z: u32) -> Weight {
		Weight::from_ref_time(21_368_000) // Standard Error: 19_000
			.saturating_add(Weight::from_ref_time(1_146_000).saturating_mul(x as u64)) // Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(115_000).saturating_mul(y as u64)) // Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(8_907_000).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(z as u64)))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(z as u64)))
	}

	fn stake() -> Weight {
		Weight::from_ref_time(26_000_000)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}

	fn unstake() -> Weight {
		Weight::from_ref_time(25_000_000)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}

	fn claim_reward() -> Weight {
		Weight::from_ref_time(24_000_000)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn set_distributed_reward() -> Weight {
		Weight::from_ref_time(6_000_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn set_epoch_duration() -> Weight {
		Weight::from_ref_time(6_000_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn set_group_weight() -> Weight {
		Weight::from_ref_time(6_000_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn set_currency_group() -> Weight {
		Weight::from_ref_time(7_000_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn on_initialize(x: u32, y: u32, z: u32) -> Weight {
		Weight::from_ref_time(21_368_000) // Standard Error: 19_000
			.saturating_add(Weight::from_ref_time(1_146_000).saturating_mul(x as u64)) // Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(115_000).saturating_mul(y as u64)) // Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(8_907_000).saturating_mul(z as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(x as u64)))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(z as u64)))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(z as u64)))
	}

	fn stake() -> Weight {
		Weight::from_ref_time(26_000_000)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}

	fn unstake() -> Weight {
		Weight::from_ref_time(25_000_000)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}

	fn claim_reward() -> Weight {
		Weight::from_ref_time(24_000_000)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}

	fn set_distributed_reward() -> Weight {
		Weight::from_ref_time(6_000_000)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}

	fn set_epoch_duration() -> Weight {
		Weight::from_ref_time(6_000_000)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}

	fn set_group_weight() -> Weight {
		Weight::from_ref_time(6_000_000)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}

	fn set_currency_group() -> Weight {
		Weight::from_ref_time(7_000_000)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
}
