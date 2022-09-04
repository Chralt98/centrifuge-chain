#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//#[cfg(feature = "runtime-benchmarks")]
//mod benchmarking;

use frame_support::pallet_prelude::*;

#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct EpochDetails<BlockNumber, Balance> {
	ends_on: BlockNumber,
	total_reward: Balance,
}

#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub struct GroupDetails<Balance, Rate> {
	total_staked: Balance,
	reward_per_token: Rate,
}

#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub struct StakedDetails<Balance, SignedBalance> {
	amount: Balance,
	reward_tally: SignedBalance,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{traits::Currency, transactional};
	use frame_system::pallet_prelude::*;

	use sp_runtime::{ArithmeticError, FixedPointNumber, FixedPointOperand};

	use num_traits::{NumAssignOps, NumOps, Signed};

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[pallet::constant]
		type BlockPerEpoch: Get<Self::BlockNumber>;

		type Currency: Currency<Self::AccountId>;

		type SignedBalance: From<BalanceOf<Self>>
			+ TryInto<BalanceOf<Self>>
			+ codec::FullCodec
			+ Copy
			+ Default
			+ scale_info::TypeInfo
			+ MaxEncodedLen
			+ NumOps
			+ NumAssignOps
			+ Signed;

		type Rate: FixedPointNumber<Inner = BalanceOf<Self>>
			+ TypeInfo
			+ MaxEncodedLen
			+ Encode
			+ Decode;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// --------------------------
	//          Storage
	// --------------------------

	#[pallet::storage]
	pub type ActiveEpoch<T: Config> = StorageValue<_, EpochDetails<T::BlockNumber, BalanceOf<T>>>;

	#[pallet::storage]
	pub type NextTotalReward<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	pub type Group<T: Config> = StorageValue<_, GroupDetails<BalanceOf<T>, T::Rate>, ValueQuery>;

	#[pallet::storage]
	pub type Staked<T: Config> = StorageMap<
		_,
		Blake2_256,
		T::AccountId,
		StakedDetails<BalanceOf<T>, T::SignedBalance>,
		ValueQuery,
	>;

	// --------------------------

	#[pallet::event]
	//#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T> {}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T>
	where
		BalanceOf<T>: FixedPointOperand,
	{
		fn on_initialize(current_block: T::BlockNumber) -> Weight {
			let active_epoch = ActiveEpoch::<T>::get().unwrap_or(EpochDetails {
				ends_on: current_block,
				total_reward: NextTotalReward::<T>::get(),
			});

			//TODO: transfer active_epoch.total_reward to rewards account.

			if active_epoch.ends_on != current_block {
				return 0; //FIXME
			}

			Group::<T>::mutate(|group| {
				if group.total_staked > BalanceOf::<T>::default() {
					let rate_increment = T::Rate::saturating_from_rational(
						active_epoch.total_reward,
						group.total_staked,
					);
					group.reward_per_token = group.reward_per_token + rate_increment;
				}
			});

			ActiveEpoch::<T>::put(EpochDetails {
				ends_on: current_block + T::BlockPerEpoch::get(),
				total_reward: NextTotalReward::<T>::get(),
			});

			0 //FIXME
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		BalanceOf<T>: FixedPointOperand,
	{
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn stake(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let _amount = Group::<T>::mutate(|group| {
				Staked::<T>::mutate(who, |staked| {
					staked.amount += amount;
					staked.reward_tally += group.reward_per_token.saturating_mul_int(amount).into();
				});

				group.total_staked += amount;
			});

			//TODO: reserve _amount

			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn unstake(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let _amount = Group::<T>::mutate(|group| {
				Staked::<T>::mutate(who, |staked| {
					staked.amount -= amount;
					staked.reward_tally -= group.reward_per_token.saturating_mul_int(amount).into();
				});

				group.total_staked -= amount;
				amount
			});

			//TODO: unreserve _amount

			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn claim(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let group = Group::<T>::get();

			let _reward: BalanceOf<T> = Staked::<T>::try_mutate(who, |staked| {
				let reward = group
					.reward_per_token
					.saturating_mul_int(staked.amount)
					.into();

				let rectified_reward = reward - staked.reward_tally;

				staked.reward_tally = reward;
				rectified_reward
					.try_into()
					.map_err(|_| DispatchError::Arithmetic(ArithmeticError::Underflow))
			})?;

			//TODO: transfer _reward

			Ok(())
		}
	}
}
