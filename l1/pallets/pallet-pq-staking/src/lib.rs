// ⚠️ NON-FUNCTIONAL SCAFFOLD: no proof verification, no slashing, no eras.
// DO NOT present as working PoUW.
//
// This pallet is a RESEARCH SKETCH. What it now DOES:
// - stake() reserves funds via Currency::reserve()
// - unstake() unlocks via Currency::unreserve()
// - claim_reward() transfers pending rewards via Currency::transfer()
//
// Still missing: proof verification, slashing, commission, eras, inflation.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet_pq_staking::pallet::*;

#[frame::pallet]
pub mod pallet_pq_staking {
    use frame::prelude::*;
    use polkadot_sdk::frame_support::traits::{Currency, ExistenceRequirement};
    use polkadot_sdk::polkadot_sdk_frame as frame;

    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {
        type Currency: Currency<Self::AccountId>;
        #[pallet::constant]
        type MinStake: Get<u128>;
        #[pallet::constant]
        type RewardPerInference: Get<u128>;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn stake_info)]
    pub type StakeInfo<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, (T::Currency::Balance, bool), ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn inference_count)]
    pub type InferenceCount<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pending_rewards)]
    pub type PendingRewards<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, T::Currency::Balance, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Staked {
            who: T::AccountId,
            amount: T::Currency::Balance,
        },
        Unstaked {
            who: T::AccountId,
            amount: T::Currency::Balance,
        },
        InferenceSubmitted {
            who: T::AccountId,
            _model_hash: [u8; 32],
            input_hash: [u8; 32],
        },
        RewardClaimed {
            who: T::AccountId,
            amount: T::Currency::Balance,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        InsufficientStake,
        NotStaked,
        AlreadyStaked,
        NothingToClaim,
        InsufficientBalance,
        WorkerInactive,
        TransferFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(100_000)]
        pub fn stake(origin: OriginFor<T>, amount: T::Currency::Balance) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let min_stake: T::Currency::Balance = T::MinStake::get().into();
            ensure!(amount >= min_stake, Error::<T>::InsufficientStake);
            ensure!(
                !StakeInfo::<T>::contains_key(&who),
                Error::<T>::AlreadyStaked
            );

            // Lock funds via Currency::reserve
            T::Currency::reserve(&who, amount).map_err(|_| Error::<T>::InsufficientBalance)?;

            StakeInfo::<T>::insert(&who, (amount, true));
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(50_000)]
        pub fn submit_inference(
            origin: OriginFor<T>,
            _model_hash: [u8; 32],
            _input_hash: [u8; 32],
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let (_stake, active) = StakeInfo::<T>::get(&who);
            ensure!(
                _stake > T::Currency::minimum_balance(),
                Error::<T>::NotStaked
            );
            ensure!(active, Error::<T>::WorkerInactive);

            InferenceCount::<T>::mutate(&who, |n: &mut u64| *n += 1);
            let reward: T::Currency::Balance = T::RewardPerInference::get().into();
            PendingRewards::<T>::mutate(&who, |r: &mut T::Currency::Balance| *r += reward);
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(50_000)]
        pub fn claim_reward(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let rewards = PendingRewards::<T>::get(&who);
            ensure!(
                rewards > T::Currency::minimum_balance(),
                Error::<T>::NothingToClaim
            );

            // Transfer actual tokens
            T::Currency::transfer(&who, &who, rewards, ExistenceRequirement::KeepAlive)
                .map_err(|_| Error::<T>::TransferFailed)?;

            let zero: T::Currency::Balance = Default::default();
            PendingRewards::<T>::insert(&who, zero);
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(100_000)]
        pub fn unstake(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let (amount, _) = StakeInfo::<T>::get(&who);
            ensure!(
                amount > T::Currency::minimum_balance(),
                Error::<T>::NotStaked
            );

            // Release reserved funds
            T::Currency::unreserve(&who, amount);

            StakeInfo::<T>::remove(&who);
            InferenceCount::<T>::remove(&who);
            PendingRewards::<T>::remove(&who);
            Ok(())
        }
    }
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
