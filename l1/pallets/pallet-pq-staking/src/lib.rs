// ⚠️ NON-FUNCTIONAL SCAFFOLD: no fund locking, no proof verification, no payouts.
// DO NOT present as working PoUW.
//
// This pallet is a RESEARCH SKETCH only. Critical missing pieces:
// - stake() does NOT transfer/lock funds (no Currency trait, no balance hold)
// - submit_inference() does NOT verify ZK/proof-of-work (model_hash/input_hash ignored)
// - claim_reward() does NOT transfer tokens (PendingRewards just cleared to 0)
// - No slashing, no commission, no era/session logic, no inflation config
// - Tests use mock runtime; NOT tested against real chain.
//
// Rewrite required for production: add Currency trait, proof verification,
// actual payout logic, and full economic model.

#![cfg_attr(not(feature = "std"), no_std)]

use frame::prelude::*;
use polkadot_sdk::polkadot_sdk_frame as frame;

pub use pallet::*;

#[frame::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {
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
    pub type StakeInfo<T: Config> = StorageMap<
        _, Blake2_128Concat, T::AccountId, (u128, bool), ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn inference_count)]
    pub type InferenceCount<T: Config> = StorageMap<
        _, Blake2_128Concat, T::AccountId, u64, ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn pending_rewards)]
    pub type PendingRewards<T: Config> = StorageMap<
        _, Blake2_128Concat, T::AccountId, u128, ValueQuery,
    >;

    #[pallet::event]
    pub enum Event<T: Config> {
        Staked { who: T::AccountId, amount: u128 },
        Unstaked { who: T::AccountId, amount: u128 },
        InferenceSubmitted { who: T::AccountId, _model_hash: [u8; 32], input_hash: [u8; 32] },
        RewardClaimed { who: T::AccountId, amount: u128 },
    }

    #[pallet::error]
    pub enum Error<T> {
        InsufficientStake,
        NotStaked,
        AlreadyStaked,
        NothingToClaim,
        InsufficientBalance,
        WorkerInactive,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(100_000)]
        pub fn stake(origin: OriginFor<T>, amount: u128) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(amount >= T::MinStake::get(), Error::<T>::InsufficientStake);
            ensure!(!StakeInfo::<T>::contains_key(&who), Error::<T>::AlreadyStaked);

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
            let (stake, active) = StakeInfo::<T>::get(&who);
            ensure!(stake > 0, Error::<T>::NotStaked);
            ensure!(active, Error::<T>::WorkerInactive);

            InferenceCount::<T>::mutate(&who, |n: &mut u64| *n += 1);
            let reward = T::RewardPerInference::get();
            PendingRewards::<T>::mutate(&who, |r: &mut u128| *r += reward);
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(50_000)]
        pub fn claim_reward(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let rewards = PendingRewards::<T>::get(&who);
            ensure!(rewards > 0, Error::<T>::NothingToClaim);

            PendingRewards::<T>::insert(&who, 0);
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(100_000)]
        pub fn unstake(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let (amount, _) = StakeInfo::<T>::get(&who);
            ensure!(amount > 0, Error::<T>::NotStaked);

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
