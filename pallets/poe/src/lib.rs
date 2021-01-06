#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, };
// use frame_system::ensure_signed;
use frame_system::Module;
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as PoeModule {
    Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
    ClaimCreated(AccountId, Vec<u8>),
    ClaimRevoked(AccountId, Vec<u8>),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
    ProofAlreadyExist,
    ClaimNotExist,
    NotClaimOwner,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 0]
		pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
      let sender = ensure_signed(origin)?;
      ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);
      Proofs::<T>::insert(&claim, (sender.clone(), frame_system::Module::<T>::block_number()));

      Self::deposit_event(RawEvent::ClaimCreated(sender, claim));
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[weight = 0]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

      ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
      let(owner, _block_number) = Proofs::<T>::get(&claim);
      ensure!(owner == sender, Error::<T>::NotClaimOwner);
      Proofs::<T>::remove(&claim);

      Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));
    }
    
    #[weight = 0]
    pub fn transfer_claim(origin, claim: Vec<u8>, dest::T::AccountId) -> dispatch::DispatchResult {
      let sender = ensure_signed(origin)?;

      ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

      let(owner, _block_number) = Proofs::<T>::get(&claim);

      ensure!(owner == sender, Error::<T>::NotClaimOwner);

      Proofs::<T>::insert(&owner, (dest, frame_system::Module::<T>::block_number()));

      Ok(())

    }
	}
}
