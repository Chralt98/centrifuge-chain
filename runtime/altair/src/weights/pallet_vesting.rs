
//! Autogenerated weights for `pallet_vesting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner`, CPU: `Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("altair-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-dev
// --steps=50
// --repeat=20
// --pallet=pallet_vesting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_vesting.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_vesting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::WeightInfo for WeightInfo<T> {
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 70_002 nanoseconds.
		Weight::from_ref_time(73_148_294 as u64)
			// Standard Error: 3_376
			.saturating_add(Weight::from_ref_time(104_831 as u64).saturating_mul(l as u64))
			// Standard Error: 6_006
			.saturating_add(Weight::from_ref_time(143_955 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 68_001 nanoseconds.
		Weight::from_ref_time(73_811_676 as u64)
			// Standard Error: 3_910
			.saturating_add(Weight::from_ref_time(78_301 as u64).saturating_mul(l as u64))
			// Standard Error: 6_958
			.saturating_add(Weight::from_ref_time(56_165 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 69_601 nanoseconds.
		Weight::from_ref_time(73_111_484 as u64)
			// Standard Error: 3_124
			.saturating_add(Weight::from_ref_time(90_194 as u64).saturating_mul(l as u64))
			// Standard Error: 5_558
			.saturating_add(Weight::from_ref_time(121_007 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 69_101 nanoseconds.
		Weight::from_ref_time(72_731_407 as u64)
			// Standard Error: 2_968
			.saturating_add(Weight::from_ref_time(77_466 as u64).saturating_mul(l as u64))
			// Standard Error: 5_281
			.saturating_add(Weight::from_ref_time(71_742 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn vested_transfer(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 96_201 nanoseconds.
		Weight::from_ref_time(106_121_673 as u64)
			// Standard Error: 6_336
			.saturating_add(Weight::from_ref_time(86_143 as u64).saturating_mul(l as u64))
			// Standard Error: 11_274
			.saturating_add(Weight::from_ref_time(41_741 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 96_002 nanoseconds.
		Weight::from_ref_time(104_211_765 as u64)
			// Standard Error: 13_747
			.saturating_add(Weight::from_ref_time(108_418 as u64).saturating_mul(l as u64))
			// Standard Error: 24_459
			.saturating_add(Weight::from_ref_time(68_043 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 72_201 nanoseconds.
		Weight::from_ref_time(76_321_122 as u64)
			// Standard Error: 2_965
			.saturating_add(Weight::from_ref_time(85_708 as u64).saturating_mul(l as u64))
			// Standard Error: 5_475
			.saturating_add(Weight::from_ref_time(94_325 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 70_601 nanoseconds.
		Weight::from_ref_time(75_329_870 as u64)
			// Standard Error: 2_816
			.saturating_add(Weight::from_ref_time(92_803 as u64).saturating_mul(l as u64))
			// Standard Error: 5_200
			.saturating_add(Weight::from_ref_time(122_179 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}
