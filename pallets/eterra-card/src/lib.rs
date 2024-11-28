#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::Weight,
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency},
		Blake2_128Concat,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_runtime::traits::Hash;

	#[pallet::pallet]
	#[pallet::without_storage_info] // Recommended to avoid unnecessary storage info generation
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
	pub struct Card {
		pub level: u32,
		pub hp: u32,
		pub mp: u32,
	}

	#[pallet::storage]
	#[pallet::getter(fn cards)]
	pub type Cards<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Card, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner_of)]
	pub type OwnerOf<T: Config> =
		StorageMap<_, Blake2_128Concat, T::Hash, T::AccountId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CardCreated { card_id: T::Hash, owner: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		CardAlreadyExists,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new card.
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(10_000, 0))] // Explicit weight type
		pub fn create_card(origin: OriginFor<T>, stats: Card) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let card_id = T::Hashing::hash_of(&stats);

			ensure!(!Cards::<T>::contains_key(&card_id), Error::<T>::CardAlreadyExists);

			Cards::<T>::insert(&card_id, stats);
			OwnerOf::<T>::insert(&card_id, &who);

			Self::deposit_event(Event::CardCreated { card_id, owner: who });

			Ok(())
		}
	}
}
