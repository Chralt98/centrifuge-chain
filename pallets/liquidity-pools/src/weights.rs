
// Copyright 2023 Centrifuge Foundation (centrifuge.io).
// This file is part of Centrifuge chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! Autogenerated weights for `pallet_liquidity_pools`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Apple Macbook Pro M1 Max`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("development-local"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=development-local
// --steps=50
// --repeat=20
// --pallet=pallet_liquidity_pools
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_liquidity_pools`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_liquidity_pools::WeightInfo for WeightInfo<T> {
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:2 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook TradingPair (r:1 w:0)
	/// Proof: OrderBook TradingPair (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: ForeignInvestments RedemptionPayoutCurrency (r:1 w:0)
	/// Proof: ForeignInvestments RedemptionPayoutCurrency (max_values: None, max_size: Some(113), added: 2588, mode: MaxEncodedLen)
	/// Storage: Investments RedeemOrders (r:1 w:1)
	/// Proof: Investments RedeemOrders (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: Investments RedeemOrderId (r:1 w:0)
	/// Proof: Investments RedeemOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Investments ClearedRedeemOrders (r:1 w:0)
	/// Proof: Investments ClearedRedeemOrders (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:3 w:3)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ForeignInvestments RedemptionState (r:1 w:1)
	/// Proof: ForeignInvestments RedemptionState (max_values: None, max_size: Some(187), added: 2662, mode: MaxEncodedLen)
	/// Storage: ForeignInvestments CollectedRedemption (r:1 w:1)
	/// Proof: ForeignInvestments CollectedRedemption (max_values: None, max_size: Some(120), added: 2595, mode: MaxEncodedLen)
	/// Storage: ForeignInvestments TokenSwapOrderIds (r:1 w:1)
	/// Proof: ForeignInvestments TokenSwapOrderIds (max_values: None, max_size: Some(96), added: 2571, mode: MaxEncodedLen)
	/// Storage: ForeignInvestments InvestmentState (r:1 w:1)
	/// Proof: ForeignInvestments InvestmentState (max_values: None, max_size: Some(187), added: 2662, mode: MaxEncodedLen)
	/// Storage: OrderBook Orders (r:1 w:2)
	/// Proof: OrderBook Orders (max_values: None, max_size: Some(186), added: 2661, mode: MaxEncodedLen)
	/// Storage: OrderBook AssetPairOrders (r:2 w:2)
	/// Proof: OrderBook AssetPairOrders (max_values: None, max_size: Some(8068), added: 10543, mode: MaxEncodedLen)
	/// Storage: OrderBook OrderIdNonceStore (r:1 w:1)
	/// Proof: OrderBook OrderIdNonceStore (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: ForeignInvestments ForeignInvestmentInfo (r:0 w:2)
	/// Proof: ForeignInvestments ForeignInvestmentInfo (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: OrderBook UserOrders (r:0 w:2)
	/// Proof: OrderBook UserOrders (max_values: None, max_size: Some(226), added: 2701, mode: MaxEncodedLen)
	fn inbound_collect_redeem() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5737`
		//  Estimated: `71940`
		// Minimum execution time: 231_000 nanoseconds.
		Weight::from_parts(236_000_000, 71940)
			.saturating_add(T::DbWeight::get().reads(20))
			.saturating_add(T::DbWeight::get().writes(17))
	}
}
