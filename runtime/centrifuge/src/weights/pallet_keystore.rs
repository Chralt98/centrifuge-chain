
//! Autogenerated weights for `pallet_keystore`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_keystore
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_keystore.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_keystore`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_keystore::WeightInfo for WeightInfo<T> {
	/// Storage: Keystore KeyDeposit (r:1 w:0)
	/// Proof: Keystore KeyDeposit (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Keystore Keys (r:10 w:10)
	/// Proof: Keystore Keys (max_values: None, max_size: Some(120), added: 2595, mode: MaxEncodedLen)
	/// Storage: Keystore LastKeyByPurpose (r:0 w:1)
	/// Proof: Keystore LastKeyByPurpose (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 10]`.
	fn add_keys(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `248`
		//  Estimated: `3114 + n * (2595 ±0)`
		// Minimum execution time: 37_820 nanoseconds.
		Weight::from_parts(17_122_053, 3114)
			// Standard Error: 11_215
			.saturating_add(Weight::from_ref_time(22_611_583).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(2595).saturating_mul(n.into()))
	}
	/// Storage: Keystore Keys (r:10 w:10)
	/// Proof: Keystore Keys (max_values: None, max_size: Some(120), added: 2595, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 10]`.
	fn revoke_keys(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `148 + n * (75 ±0)`
		//  Estimated: `0 + n * (2595 ±0)`
		// Minimum execution time: 21_890 nanoseconds.
		Weight::from_ref_time(10_946_013)
			// Standard Error: 12_666
			.saturating_add(Weight::from_ref_time(12_587_510).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(2595).saturating_mul(n.into()))
	}
	/// Storage: Keystore KeyDeposit (r:0 w:1)
	/// Proof: Keystore KeyDeposit (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn set_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_722 nanoseconds.
		Weight::from_ref_time(11_993_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
