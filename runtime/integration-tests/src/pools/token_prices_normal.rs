// Copyright 2021 Centrifuge Foundation (centrifuge.io).
//
// This file is part of the Centrifuge chain project.
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! Tests for token-price behaviour in a normal and healthy scenario
//! I.e. no defaults. But we test weird tranche investment structures
use crate::chain::centrifuge::{Amount, Call, Event, Runtime, Timestamp, PARA_ID};
use crate::pools::utils::*;
use crate::pools::utils::{
	accounts::Keyring,
	env::{ChainState, EventRange},
	loans::NftManager,
	loans::{borrow_call, init_loans_for_pool, issue_default_loan, update_nav},
	pools::{close_epoch, default_pool_calls, invest_order_call, permission_call},
	time::secs::SECONDS_PER_DAY,
	tokens::DECIMAL_BASE_12,
};
use common_types::PoolRole;
use fudge::primitives::Chain;
use pallet_loans::types::Asset;
use runtime_common::{AccountId, Address, Balance, InstanceId};
use sp_runtime::{traits::AccountIdConversion, DispatchError, Storage, TokenError};
use tokio::runtime::Handle;

#[tokio::test]
async fn tranche_prices_with_single_tranche_investor() {
	// The block time we use for this test (in seconds)
	// -> two blocks per day
	pub const BLOCK_TIME: u64 = 86400 / 2u64;

	// THE MANAGER MUST NOT BE DROPPED! It is the receiver of a lot of channels
	let manager = env::task_manager(Handle::current());
	let mut env = {
		let mut genesis = Storage::default();
		genesis::default_balances::<Runtime>(&mut genesis);
		env::test_env_with_centrifuge_storage::<BLOCK_TIME>(&manager, genesis)
	};

	let mut nft_manager = NftManager::new();
	let pool_id = 0u64;
	let loan_amount = 10_000 * DECIMAL_BASE_12;
	let loan_id = InstanceId(1);
	let borrow_amount = Amount::from_inner(9_000 * DECIMAL_BASE_12);
	let investment = 5_000 * DECIMAL_BASE_12;
	let maturity = time::date(90 * SECONDS_PER_DAY);

	env::run!(
		env,
		Chain::Para(PARA_ID),
		Call,
		ChainState::PoolEmpty,
		Keyring::Admin => default_pool_calls(Keyring::Admin.into(), pool_id, &mut nft_manager),
			issue_default_loan(
				Keyring::Admin.into(),
				pool_id,
				loan_amount,
				maturity,
				&mut nft_manager,
			)
	);

	env::assert_events!(
		env,
		Chain::Para(PARA_ID),
		Event,
		EventRange::Latest,
		Event::System(frame_system::Event::ExtrinsicFailed{..}) if [count 0],
		Event::Pools(pallet_pools::Event::Created(id, ..)) if [id == pool_id],
		Event::Loans(pallet_loans::Event::PoolInitialised(id)) if [id == pool_id],
		Event::Loans(pallet_loans::Event::Created(id, loan, asset))
			if [id == pool_id && loan == loan_id && asset == Asset(4294967296, InstanceId(1))],
		Event::Loans(pallet_loans::Event::Priced(id, loan)) if [id == pool_id && loan == loan_id],
	);

	env::run!(
		env,
		Chain::Para(PARA_ID),
		Call,
		ChainState::PoolEmpty,
		Keyring::TrancheInvestor(0) => invest_order_call(pool_id, 0, investment);
		Keyring::TrancheInvestor(1) => invest_order_call(pool_id, 0, investment);
	);

	env::assert_events!(
		env,
		Chain::Para(PARA_ID),
		Event,
		EventRange::Latest,
		Event::System(frame_system::Event::ExtrinsicFailed{..}) if [count 0],
		Event::Pools(pallet_pools::Event::InvestOrderUpdated(id, _tranche, who))
			if [id == pool_id && who == Keyring::TrancheInvestor(0).to_account_id()],
		Event::Pools(pallet_pools::Event::InvestOrderUpdated(id, _tranche, who))
			if [id == pool_id && who == Keyring::TrancheInvestor(1).to_account_id()],
	);

	env::pass_n(&mut env, (1 * time::blocks_per_day::<BLOCK_TIME>()).into());

	env::run!(
		env,
		Chain::Para(PARA_ID),
		Call,
		ChainState::PoolEmpty,
		Keyring::Admin =>
			update_nav(pool_id),
			close_epoch(pool_id),
			borrow_call(pool_id, loan_id, borrow_amount)
	);

	env::assert_events!(
		env,
		Chain::Para(PARA_ID),
		Event,
		EventRange::Latest,
		Event::System(frame_system::Event::ExtrinsicFailed{..}) if [count 0],
		Event::Pools(pallet_pools::Event::EpochExecuted(id, ..)) if [id == pool_id],
		Event::Pools(pallet_pools::Event::EpochClosed(id, ..)) if [id == pool_id],
		Event::Loans(pallet_loans::Event::Borrowed(id, loan_id, amount))
			if [id == pool_id && loan_id == loan_id && amount == borrow_amount],
	);

	env::pass_n(&mut env, (30 * time::blocks_per_day::<BLOCK_TIME>()).into());

	let token_prices = env
		.with_state(Chain::Para(PARA_ID), || {
			(
				pools::with_ext::get_tranche_prices(pool_id),
				Timestamp::now(),
			)
		})
		.expect("ESSENTIAL: Chain state is available.");

	tracing::event!(
		tracing::Level::INFO,
		?token_prices,
		"Token prices after borrow and 10 minutes passed"
	);
}