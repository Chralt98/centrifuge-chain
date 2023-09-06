
//! Autogenerated weights for `frame_system`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("altair-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-dev
// --steps=50
// --repeat=20
// --pallet=frame_system
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/frame_system.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `frame_system`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::WeightInfo for WeightInfo<T> {
	/// The range of component `b` is `[0, 3932160]`.
	fn remark(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_096 nanoseconds.
		Weight::from_ref_time(4_025_510)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(310).saturating_mul(b.into()))
	}
	/// The range of component `b` is `[0, 3932160]`.
	fn remark_with_event(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_021 nanoseconds.
		Weight::from_ref_time(79_690_232)
			// Standard Error: 8
			.saturating_add(Weight::from_ref_time(1_779).saturating_mul(b.into()))
	}
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: unknown `0x3a686561707061676573` (r:0 w:1)
	/// Proof Skipped: unknown `0x3a686561707061676573` (r:0 w:1)
	fn set_heap_pages() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `495`
		// Minimum execution time: 6_031 nanoseconds.
		Weight::from_parts(6_403_000, 495)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Skipped Metadata (r:0 w:0)
	/// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
	/// The range of component `i` is `[0, 1000]`.
	fn set_storage(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_156 nanoseconds.
		Weight::from_ref_time(3_296_000)
			// Standard Error: 2_320
			.saturating_add(Weight::from_ref_time(885_296).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
	}
	/// Storage: Skipped Metadata (r:0 w:0)
	/// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
	/// The range of component `i` is `[0, 1000]`.
	fn kill_storage(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_226 nanoseconds.
		Weight::from_ref_time(3_356_000)
			// Standard Error: 851
			.saturating_add(Weight::from_ref_time(634_566).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
	}
	/// Storage: Skipped Metadata (r:0 w:0)
	/// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
	/// The range of component `p` is `[0, 1000]`.
	fn kill_prefix(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `149 + p * (69 ±0)`
		//  Estimated: `139 + p * (70 ±0)`
		// Minimum execution time: 6_212 nanoseconds.
		Weight::from_parts(6_392_000, 139)
			// Standard Error: 1_225
			.saturating_add(Weight::from_ref_time(1_283_115).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_proof_size(70).saturating_mul(p.into()))
	}
}
