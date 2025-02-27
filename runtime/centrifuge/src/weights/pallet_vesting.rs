
//! Autogenerated weights for `pallet_vesting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("centrifuge-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=centrifuge-dev
// --steps=50
// --repeat=20
// --pallet=pallet_vesting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_vesting.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_vesting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::WeightInfo for WeightInfo<T> {
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(157), added: 2632, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 3]`.
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `331 + l * (25 ±0) + s * (37 ±0)`
		//  Estimated: `6406`
		// Minimum execution time: 36_108 nanoseconds.
		Weight::from_parts(36_866_562, 6406)
			// Standard Error: 1_531
			.saturating_add(Weight::from_ref_time(42_179).saturating_mul(l.into()))
			// Standard Error: 29_342
			.saturating_add(Weight::from_ref_time(162_742).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(157), added: 2632, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 3]`.
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `331 + l * (25 ±0) + s * (37 ±0)`
		//  Estimated: `6406`
		// Minimum execution time: 36_037 nanoseconds.
		Weight::from_parts(37_222_111, 6406)
			// Standard Error: 1_409
			.saturating_add(Weight::from_ref_time(27_486).saturating_mul(l.into()))
			// Standard Error: 26_997
			.saturating_add(Weight::from_ref_time(140_438).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(157), added: 2632, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 3]`.
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `466 + l * (25 ±0) + s * (37 ±0)`
		//  Estimated: `9009`
		// Minimum execution time: 39_023 nanoseconds.
		Weight::from_parts(40_451_564, 9009)
			// Standard Error: 4_552
			.saturating_add(Weight::from_ref_time(35_056).saturating_mul(l.into()))
			// Standard Error: 87_220
			.saturating_add(Weight::from_ref_time(319_423).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(157), added: 2632, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 3]`.
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `466 + l * (25 ±0) + s * (37 ±0)`
		//  Estimated: `9009`
		// Minimum execution time: 39_454 nanoseconds.
		Weight::from_parts(39_995_327, 9009)
			// Standard Error: 2_606
			.saturating_add(Weight::from_ref_time(44_988).saturating_mul(l.into()))
			// Standard Error: 49_946
			.saturating_add(Weight::from_ref_time(294_310).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(157), added: 2632, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 2]`.
	fn vested_transfer(_l: u32, _s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + l * (25 ±0) + s * (332 ±0)`
		//  Estimated: `9009`
		// Minimum execution time: 58_839 nanoseconds.
		Weight::from_parts(74_797_626, 9009)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(157), added: 2632, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 2]`.
	fn force_vested_transfer(_l: u32, _s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `120 + l * (25 ±0) + s * (332 ±0)`
		//  Estimated: `11612`
		// Minimum execution time: 62_657 nanoseconds.
		Weight::from_parts(78_059_563, 11612)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(157), added: 2632, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 3]`.
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `467 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `9009`
		// Minimum execution time: 39_925 nanoseconds.
		Weight::from_parts(42_334_138, 9009)
			// Standard Error: 5_252
			.saturating_add(Weight::from_ref_time(26_770).saturating_mul(l.into()))
			// Standard Error: 167_510
			.saturating_add(Weight::from_ref_time(219_387).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(157), added: 2632, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 3]`.
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `467 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `9009`
		// Minimum execution time: 40_466 nanoseconds.
		Weight::from_parts(41_481_440, 9009)
			// Standard Error: 3_722
			.saturating_add(Weight::from_ref_time(40_013).saturating_mul(l.into()))
			// Standard Error: 118_715
			.saturating_add(Weight::from_ref_time(399_091).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
