// Copyright 2021 Centrifuge Foundation (centrifuge.io).
// This file is part of Centrifuge Chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use codec::{Decode, Encode};
use frame_support::PalletError;
use scale_info::TypeInfo;

#[derive(Encode, Decode, TypeInfo, PalletError)]
pub enum InvestError {
	/// Failed to increase the investment.
	Increase,
	/// Failed to decrease the unprocessed investment.
	Decrease,
	/// Failed to transition after fulfilled swap order.
	FulfillSwapOrder,
	/// Failed to transition a (partially) processed investment after
	/// collecting.
	Collect,
	/// The investment needs to be collected before it can be updated further.
	CollectRequired,
	/// Attempted to collect an investment which has not been processed yet.
	NothingCollected,
}

#[derive(Encode, Decode, TypeInfo, PalletError)]

pub enum RedeemError {
	/// Failed to increase the redemption.
	Increase,
	/// Failed to collect the redemption.
	Collect,
	/// Failed to retrieve the foreign payout currency for a collected
	/// redemption.
	///
	/// NOTE: This error can only occur, if a user tries to collect before
	/// having increased their redemption as this would store the payout
	/// currency.
	CollectPayoutCurrencyNotFound,
	/// Failed to decrease the unprocessed redemption.
	Decrease,
	/// Failed to transition after fulfilled swap order.
	FulfillSwapOrder,
	/// Failed to transition a (partially) processed redemption after an epoch
	/// was executed.
	EpochExecution,
	/// The redemption needs to be collected before it can be updated further.
	CollectRequired,
}