#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod types;
pub mod weitghts;

// Imports and Dependencies
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use crate::types::*;
	use crate::weitghts::WeightInfo;

	// Runtime Configuration Trait
    // All types and constants go here.
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// ToDo's maximum length.
		#[pallet::constant]
		type MaxToDoLength: Get<u32>;

		/// ToDo's minimum length.
		#[pallet::constant]
		type MinToDoLength: Get<u32>;

		/// The maximum number of todos that can be inserted into a storage at once.
		#[pallet::constant]
		type ToDosInsertLimit: Get<u32>;

		type WeightInfo: WeightInfo;
	}

	// Declaration of the Pallet type
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.

	#[pallet::storage]
	#[pallet::getter(fn user_todo)]
	pub(super) type UserToDo<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ToDo<T>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// ToDo was successfully stored. [owner]
		ToDoStored(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Length of the todo is too large.
		ToDoTooLarge,
		/// Length of the todo is too small.
		ToDoTooSmall,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates new user todo
		#[pallet::weight(T::WeightInfo::create_todo(data.len()))]
		pub fn create_todo(origin: OriginFor<T>, data: ToDo<T>) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			let data_len = data.len();
			ensure!(
				data_len < <T as Config>::MaxToDoLength::get() as usize,
				Error::<T>::ToDoTooLarge,
			);
			ensure!(
				data_len > <T as Config>::MinToDoLength::get() as usize,
				Error::<T>::ToDoTooSmall,
			);

			UserToDo::<T>::insert(owner.clone(), data);
			Self::deposit_event(Event::ToDoStored(owner.clone()));
			Ok(())
		}
	}
}
