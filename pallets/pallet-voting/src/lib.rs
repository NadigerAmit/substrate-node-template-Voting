
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	pallet_prelude::*,
	sp_runtime::traits::{Hash},
};

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
//#[cfg(test)]
//mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::{dispatch::DispatchResult, BoundedVec};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use scale_info::prelude::vec::Vec;

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// The pallet's configuration trait.
	///
	/// All our types and constants a pallet depends on must be declared here.
	/// These types are defined generically and made concrete when the pallet is declared in the
	/// `runtime/src/lib.rs` file of your chain.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
		//type BlockNumber: Parameter + Member + Default + Copy + MaxEncodedLen;
	}

	/// A storage item for this pallet.

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
    pub struct Proposal<T: Config> {
        pub creator: T::AccountId,
		pub description: BoundedVec<u8, ConstU32<256>>,
        pub end: BlockNumberFor<T>,
        pub yes_votes: u64,
        pub no_votes: u64,
        //pub finalized: bool,
		pub voters: BoundedBTreeSet<T::AccountId, ConstU32<256>>,
    }

	/*
	#[pallet::storage]
    #[pallet::getter(fn proposal_count)]
    pub type ProposalCount<T> = StorageValue<_, u64, ValueQuery>;
	*/

	#[pallet::storage]
    #[pallet::getter(fn proposals)]
    pub type Proposals<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Proposal<T>>;

	/// Events that functions in this pallet can emit.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A user has successfully proposal.
		ProposalCreated(T::AccountId, T::Hash),

		/// A user has successfully proposal.
        Voted(T::AccountId, T::Hash, bool),

       // ProposalFinalized(T::Hash, bool),
		ProposalResults(T::Hash, bool),
	}

	/// Errors that can be returned by this pallet.
	#[pallet::error]
	pub enum Error<T> {
		
		/// The proposal was not found.
		ProposalNotFound,

		/// Description of proposal is too long .
		ProposalDescriptionTooLong,

		/// The voting duration is already ended.
        VotingEnded,

		/// The voting still on.
        VotingNotEnded,	

		///Voter is already voted  .
		AlreadyVoted,

		/// Max ovetr is reached.
		MaxVotersReached
	}

	/// The pallet's dispatchable functions ([`Call`]s).
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_proposal())]
		pub fn create_proposal(
            origin: OriginFor<T>,
            description: Vec<u8>,
            duration: BlockNumberFor<T>,
        ) -> DispatchResult {
			let who = ensure_signed(origin)?;
			log::info!("create_proposal called {:?}",who);
            let current_block = <frame_system::Pallet<T>>::block_number();
            let end_block = current_block + duration;

			// Convert Vec<u8> to BoundedVec<u8, ConstU32<256>>
			let bounded_description = BoundedVec::<u8, ConstU32<256>>::
				try_from(description)
				.map_err(|_| Error::<T>::ProposalDescriptionTooLong)?;

			let proposal = Proposal {
				creator: who.clone(),
				description: bounded_description,
				end: end_block,
				yes_votes: 0,
				no_votes: 0,
				voters: BoundedBTreeSet::default(),
				//finalized: false,
			};
		
			let proposal_hash = T::Hashing::hash_of(&proposal);
			<Proposals<T>>::insert(proposal_hash, proposal);
		
			Self::deposit_event(Event::ProposalCreated(who, proposal_hash));
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::vote())]
        pub fn vote(
            origin: OriginFor<T>,
            proposal_hash: T::Hash,
            vote: bool,
        ) -> DispatchResult {
			let who = ensure_signed(origin)?;
			log::info!("vote called {:?}",who);
            Proposals::<T>::try_mutate(proposal_hash, |proposal_opt| {
                let proposal = proposal_opt.as_mut().ok_or(Error::<T>::ProposalNotFound)?;
				let current_block = <frame_system::Pallet<T>>::block_number();
				ensure!(current_block <= proposal.end, Error::<T>::VotingEnded);
				
				ensure!(!proposal.voters.contains(&who), Error::<T>::AlreadyVoted);
				proposal.voters.try_insert(who.clone()).map_err(|_| Error::<T>::MaxVotersReached)?;

				if vote {
					proposal.yes_votes = proposal.yes_votes.saturating_add(1);
				} else {
					proposal.no_votes = proposal.no_votes.saturating_add(1);
				}

				Self::deposit_event(Event::Voted(who, proposal_hash, vote));
				Ok(())
            })
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::get_proposal_results())]
		pub fn get_proposal_results(
			origin: OriginFor<T>,
			proposal_hash: T::Hash,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			let proposal = Proposals::<T>::get(proposal_hash).ok_or(Error::<T>::ProposalNotFound)?;
			let current_block = <frame_system::Pallet<T>>::block_number();
			ensure!(current_block > proposal.end, Error::<T>::VotingNotEnded);
			
			let approved = proposal.yes_votes > proposal.no_votes;

			Self::deposit_event(Event::ProposalResults(proposal_hash, approved));
            Ok(())
		}
	}

	/*
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::finalize_proposal())]
        pub fn finalize_proposal(
            origin: OriginFor<T>,
            proposal_hash: T::Hash,
        ) -> DispatchResult {
			let who = ensure_signed(origin)?;
			log::info!("finalize_proposal called {:?}",who);
            Proposals::<T>::try_mutate(proposal_hash, |proposal_opt| {
                let proposal = proposal_opt.as_mut().ok_or(Error::<T>::ProposalNotFound)?;
                ensure!(proposal.end <= <frame_system::Pallet<T>>::block_number(), Error::<T>::VotingNotEnded);

                proposal.finalized = true;
				let approved = proposal.yes_votes > proposal.no_votes;

                Self::deposit_event(Event::ProposalFinalized(proposal_hash, approved));
                Ok(())
            })
		}
*/
}
