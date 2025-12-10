#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::{DispatchResult},
        pallet_prelude::*,
    };
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::storage]
    #[pallet::getter(fn balances)]
    pub type Balances<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, u128, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn total_supply)]
    pub type TotalSupply<T: Config> = StorageValue<_, u128, ValueQuery>;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000)]
        pub fn mint(origin: OriginFor<T>, to: T::AccountId, amount: u128) -> DispatchResult {
            let _sender = ensure_signed(origin)?;

            TotalSupply::<T>::mutate(|s| *s += amount);
            Balances::<T>::mutate(&to, |b| *b += amount);

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: u128
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            Balances::<T>::try_mutate(&sender, |bal| -> DispatchResult {
                ensure!(*bal >= amount, "Insufficient balance");
                *bal -= amount;
                Ok(())
            })?;

            Balances::<T>::mutate(&to, |bal| *bal += amount);

            Ok(())
        }
    }
}
