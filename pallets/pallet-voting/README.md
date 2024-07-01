# Voting Pallet

This is a custom Substrate pallet that implements a decentralized voting system. The pallet allows users to create, vote on, and manage proposals on the blockchain..

## Requiremnts
#### The pallet must satisfy the following requirements:
1. Allow users to create new proposals by submitting a description and a duration for the voting period.
2. Store proposals on-chain with a unique identifier, the creator's address, the description, and the voting period.
3. Allow users to cast votes on active proposals, with each user able to vote only once per proposal.
4. Support "Yes" or "No" votes.
5. Finalize the voting results once the voting period ends, marking the proposal as approved if the majority of votes are "Yes," otherwise, it is rejected.

## Architecture
### Pallet Structure
The pallet structure includes the following components:

1. Config: Defines the pallet's configuration.
2. Event: Defines events emitted by the pallet.
3. Storage: Defines the storage items for the pallet.
4. Call: Defines the callable functions (extrinsics) of the pallet.
5. Error: Defines the errors that can occur within the pallet.

### Storage
The pallet includes the following storage items:

`Proposals`: Maps a proposal's hash to its corresponding proposal data.


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
1. `ProposalCreated`: Emitted when a proposal is successfully created.

2. `Voted`: Emitted when a user votes on a proposal.

3. `ProposalResults`: Emitted when the results of a proposal are finalized.

### Errors
1. `ProposalNotFound`: The proposal was not found.This error is returned when user gives the wrong or unavilable hash of the proposal i.e wrong unique id of proposal in `vote` or `get_proposal_results` api.

2. `ProposalDescriptionTooLong`: The description of the proposal is too long.

3. `VotingEnded`: The voting period has already ended.This error is returned when user `vote` after the voting end period for the proposal is reached.

4. `VotingNotEnded`: The voting period is still ongoing.This error is returned when user try to get the voting results before the voting end period is reached.

5. `AlreadyVoted`: The user has already voted on this proposal and tries to again on same proposal.

6. `MaxVotersReached`: The maximum number of voters for the proposal has been reached.


### Dispatchable Functions or Extrinsics
#### 1. create_proposal: 
Allows a user to create a new proposal.
##### Parameters:
`description`: A description of the proposal.

`duration`: The duration of the voting period in blocks(BlockNumberFor). i.e 1. BlockNumberFor is the duration to create one block which is default configured to 6 sec in substrate framework. 
###### Calculation:
If you want to have the duration as 1 minute i.e 60 sec , then below is the calculation

```sh
Duration in Blocks = Duration in sec / BlockNumberFor
Duration in Blocks = 60 sec / 6 sec
Duration in Blocks = 10 

```
Since `BlockNumberFor` is configured as `6 sec`in substrate framework.
##### Emits: 
`ProposalCreated` event

#### 2. vote: 
Allows a user to vote on an active proposal.
##### Parameters:
`proposal_hash`: The hash of the proposal.
`vote`: A boolean indicating the vote (true for "Yes", false for "No").
##### Emits: 
`Voted` event 

#### 3. get_proposal_results: 
Allows to retrieve the results of a proposal after the voting period has ended.
##### Parameters:
`proposal_hash`: The hash of the proposal.
##### Emits: 
`ProposalResults` Accepted(True) or Rejected(False)

## Integration Testing with Polkadot.js Apps
Create Multiple Accounts:

Navigate to the "Accounts" tab and create multiple accounts.
Fund the new accounts from the default account (Alice).

Interact with the Pallet:

Use the "Extrinsics" tab to call create_proposal, vote, and finalize_proposal extrinsics from different accounts.
Verify the state changes using the "Chain state" tab.
