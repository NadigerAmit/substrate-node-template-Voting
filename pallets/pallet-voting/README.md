# Voting Pallet

This is a custom Substrate pallet that implements a decentralized voting system. The pallet allows users to create, vote on, and manage proposals on the blockchain..

## Requiremnts
#### The pallet must satisfy the following requirements:
1. Allow users to create new proposals by submitting a description and a duration for the voting period.
2. Store proposals on-chain with a unique identifier, the creator's address, the description, and the voting period.
3. Allow users to cast votes on active proposals, with each user able to vote only once per proposal.
Support "Yes" or "No" votes.
4. Finalize the voting results once the voting period ends, marking the proposal as approved if the majority of votes are "Yes," otherwise, it is rejected.

## Features

### Create Proposals: 
Users can create new proposals by submitting a description and a duration for the voting period. Proposals are stored on-chain with a unique identifier i.e hash of the description, the creator's address(account ID), the description, and the voting period(BlockNumberFor).

### Vote on Proposals: 
Users can cast votes on active proposals. Each user can only vote once per proposal, and votes can be either "Yes" or "No".
If there are multiple proposals, then each user can vote on each proposal only once.

### End Voting Period: 
Once the voting period for a proposal ends, voters will no longer be able to vote on the proposal.
Can get the result of the proposal. Whether the proposal is accepted*(True) or rejected(False)

### Events
`ProposalCreated`: Emitted when a proposal is successfully created.
`Voted`:` Emitted when a user votes on a proposal.
`ProposalResults`: Emitted when the results of a proposal are finalized.

### Errors
`ProposalNotFound`: The proposal was not found.This error is returned when user gives the wrong or unavilable hash of the proposal i.e wrong unique id of proposal in `vote` or `get_proposal_results` api.

`ProposalDescriptionTooLong`: The description of the proposal is too long.

`VotingEnded`: The voting period has already ended.This error is returned when user `vote` after the voting end period for the proposal is reached.

`VotingNotEnded`: The voting period is still ongoing.This error is returned when user try to get the voting results before the voting end period is reached.

`AlreadyVoted`: The user has already voted on this proposal and tries to again on same proposal.

`MaxVotersReached`: The maximum number of voters for the proposal has been reached.

Use the following command to build the node without launching it:

## Dispatchable Functions or Extrinsics
### create_proposal: 
Allows a user to create a new proposal.
#### Parameters:
description: A description of the proposal.
duration: The duration of the voting period in blocks.
#### Emits: 
`ProposalCreated` event

### vote: 
Allows a user to vote on an active proposal.
#### Parameters:
`proposal_hash`: The hash of the proposal.
`vote`: A boolean indicating the vote (true for "Yes", false for "No").
#### Emits: 
`Voted` event 

### get_proposal_results: 
Allows to retrieve the results of a proposal after the voting period has ended.
#### Parameters:
`proposal_hash`: The hash of the proposal.
#### Emits: 
`ProposalResults` Accepted(True) or Rejected(False)

