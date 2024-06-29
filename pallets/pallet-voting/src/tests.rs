/*#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok, assert_noop, dispatch::DispatchError};
    use sp_core::H256;
    use frame_system as system;

    #[test]
    fn create_proposal_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(VotingModule::create_proposal(
                Origin::signed(1),
                b"My Proposal".to_vec(),
                10
            ));
            let count = VotingModule::proposal_count();
            assert_eq!(count, 1);
        });
    }

    #[test]
    fn vote_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(VotingModule::create_proposal(
                Origin::signed(1),
                b"My Proposal".to_vec(),
                10
            ));
            let proposal_hash = VotingModule::proposals(0).unwrap();
            assert_ok!(VotingModule::vote(
                Origin::signed(2),
                proposal_hash,
                true
            ));
            let proposal = VotingModule::proposals(proposal_hash).unwrap();
            assert_eq!(proposal.yes_votes, 1);
        });
    }

    #[test]
    fn finalize_proposal_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(VotingModule::create_proposal(
                Origin::signed(1),
                b"My Proposal".to_vec(),
                10
            ));
            let proposal_hash = VotingModule::proposals(0).unwrap();
            run_to_block(11);
            assert_ok!(VotingModule::finalize_proposal(
                Origin::signed(1),
                proposal_hash
            ));
            let proposal = VotingModule::proposals(proposal_hash).unwrap();
            assert!(proposal.finalized);
        });
    }
}
	*/
	use crate::{mock::*, Error, Event, Something};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(Something::<Test>::get(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(RuntimeOrigin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}
