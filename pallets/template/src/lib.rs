#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, ensure, StorageMap
};
use frame_system::ensure_signed;
use sp_std::vec::Vec;


/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        /// The storage item for our proofs.
        /// It maps a proof to the user who made the claim and when they made it.
        Proofs: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
    }
}

// Pallets use events to inform users when important changes are made.
// Event documentation should end with an array that provides descriptive names for parameters.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event! {
    pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
        /// Event emitted when a proof has been claimed. [who, claim]
        ClaimCreated(AccountId, Vec<u8>),
        /// Event emitted when a claim is revoked by the owner. [who, claim]
		ClaimRevoked(AccountId, Vec<u8>),
		/// Event evmitted when a claim is transferred from one owner to another
		ClaimTransfered(AccountId, Vec<u8>, AccountId),
    }
}

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// The proof has already been claimed.
        ProofAlreadyClaimed,
        /// The proof does not exist, so it cannot be revoked.
        NoSuchProof,
        /// The proof is claimed by another account, so caller can't revoke it.
        NotProofOwner,
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        #[weight = 10_000]
        fn create_claim(origin, proof: Vec<u8>) {
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;

            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            let current_block = <frame_system::Module<T>>::block_number();

            Proofs::<T>::insert(&proof, (&sender, current_block));

            Self::deposit_event(RawEvent::ClaimCreated(sender, proof));
        }


        /// revoke their claim.
        #[weight = 10_000]
        fn revoke_claim(origin, proof: Vec<u8>) {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            let (owner, _) = Proofs::<T>::get(&proof);

            ensure!(sender == owner, Error::<T>::NotProofOwner);

            Proofs::<T>::remove(&proof);

            Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));
		}
		

        /// transfer ownership of a proof.
        #[weight = 10_000]
        fn transfer_claim(origin, proof: Vec<u8>, target: <T as frame_system::Trait>::AccountId) {
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

			let (owner, _) = Proofs::<T>::get(&proof);
			
            ensure!(sender == owner, Error::<T>::NotProofOwner);

            let current_block = <frame_system::Module<T>>::block_number();

			let target_copy = target.clone();
            Proofs::<T>::mutate(&proof, |v| * v = (target_copy, current_block));

            Self::deposit_event(RawEvent::ClaimTransfered(sender, proof, target));
        }

    }
}
