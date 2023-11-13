
//! Autogenerated weights for `pallet_crowdloan_reward`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `kf-FG`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("altair-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-dev
// --steps=50
// --repeat=20
// --pallet=pallet_crowdloan_reward
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_crowdloan_reward.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_crowdloan_reward`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_crowdloan_reward::WeightInfo for WeightInfo<T> {
	/// Storage: CrowdloanReward VestingStart (r:0 w:1)
	/// Proof: CrowdloanReward VestingStart (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: CrowdloanReward VestingPeriod (r:0 w:1)
	/// Proof: CrowdloanReward VestingPeriod (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: CrowdloanReward DirectPayoutRatio (r:0 w:1)
	/// Proof: CrowdloanReward DirectPayoutRatio (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(8_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: CrowdloanReward VestingStart (r:0 w:1)
	/// Proof: CrowdloanReward VestingStart (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_vesting_start() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(7_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: CrowdloanReward VestingPeriod (r:0 w:1)
	/// Proof: CrowdloanReward VestingPeriod (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_vesting_period() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(7_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: CrowdloanReward DirectPayoutRatio (r:0 w:1)
	/// Proof: CrowdloanReward DirectPayoutRatio (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_direct_payout_ratio() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(7_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
