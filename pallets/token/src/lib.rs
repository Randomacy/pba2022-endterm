#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use frame_support::traits::{Currency, Randomness};
    use codec::{Decode, Encode};
    use sp_std::prelude::Vec;
    use sp_runtime::{traits::{Bounded, Hash}};
    use frame_system::ensure_signed;

    use pallet_balances as balances;
    use frame_system as system;
    use pallet_randomness_collective_flip as randomness_collective_flip;

    // The struct on which we build all of our Pallet logic.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /* Placeholder for defining custom types. */
    #[derive(Encode, Decode, Default, Clone, PartialEq)]
    #[cfg_attr(feature = "std", derive(Debug))]
    pub struct Token<T:Config> {
        pub hash: T::Hash,
        pub symbol: Vec<u8>,
    }
    
    /* Placeholder for defining custom storage items. */

    // Your Pallet's configuration trait, representing custom external types and interfaces.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Hash: sp_runtime::traits::Hash;
    }

    // Your Pallet's events.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {}

    // Your Pallet's error messages.
    #[pallet::error]
    pub enum Error<T> {
        /// There is no match token
		NoMatchingToken,
		/// The balance is not enough
		BalanceNotEnough,
		/// Amount overflow
		AmountOverflow,
		/// Sender does not have token
		SenderHaveNoToken,
		/// Memo length exceed limitation
		MemoLengthExceedLimitation,
    }

    // Your Pallet's callable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    // Your Pallet's internal functions.
    impl<T: Config> Pallet<T> {}
}