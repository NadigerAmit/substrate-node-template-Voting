

# substrate-node-template-Voting
The substrate-node-template-Voting is a blockchain node built on the Substrate framework, leveraging the Substrate Node Template. It has been enhanced with a custom voting pallet to enable decentralized voting functionality. This introduction will cover the core components and capabilities of this project, focusing on how the voting system has been integrated and how it operates within the Substrate ecosystem.

## Overview of Substrate
Substrate is a modular framework for building blockchain networks. It provides a highly customizable environment that allows developers to tailor their blockchain’s consensus, networking, and state transition functions. The Substrate Node Template is a pre-configured, ready-to-use Substrate node that serves as a starting point for building custom blockchain solutions.

## How to build and run the node 
As this project is built on the substrate-node-template, so all the build commands and how to run is same. Please see below link for basic info such as build, run ,etc .
[https://github.com/substrate-developer-hub/substrate-node-template/blob/main/README.md]

## Key Features of substrate-node-template-Voting
1. Modular Design: Inherits the modular design of Substrate, allowing easy integration and customization of pallets (modules) to extend functionality.

2. Voting Pallet Integration: Includes a custom-built voting pallet that facilitates proposal creation, voting, and result finalization.

3. Decentralized Voting: Enables users to create proposals, cast votes, and view results in a decentralized manner, leveraging blockchain transparency and immutability.

4. Account Management: Uses Substrate’s robust account and balance management system to handle voter accounts and ensure secure transactions.

5. Event-Driven Architecture: Utilizes Substrate’s event system to log and handle important actions such as proposal creation and vote casting.

## Voting Pallet Capabilities
The custom voting pallet added to the substrate-node-template-Voting project introduces several key functionalities:

1. Proposal Creation: Users can create new proposals by submitting a description and specifying the voting duration. Each proposal is stored on-chain with a unique identifier.

2. Voting: Users can cast votes on active proposals. Each user is limited to one vote per proposal, ensuring fair voting practices.

3. Result Finalization: After the voting period ends, the results can be finalized. A proposal is concluded as approved if it receives a majority of "Yes" votes; otherwise, it is rejected.

4. Event Emission: The pallet emits events for proposal creation, voting actions, and result finalization, allowing easy tracking and auditing of the voting process.

## Testing guide on for voting Functionality using the Polkadot.js
### Prerequisites
1. Ensure you have a local Substrate node running with your custom pallet integrated.
Refer to [https://github.com/substrate-developer-hub/substrate-node-template/blob/main/README.md]
2. Open the Polkadot.js app and connect it to your local node.


## Step-by-Step Guide
### Transfer Funds to Default Accounts
To enable accounts other than Alice, Alice-stash, Bob, Bob-stash to participate in voting, you need to transfer some balance to them.

#### Go to Accounts -> Accounts.
1. Select Alice or Bob who have default balances.

2. Click on the Transfer button next to the account name.

3. Enter the account name you want to transfer to (e.g., charlie, dave).

4. Enter the amount of balance to transfer (e.g., 1000 units).

5. Click Make Transfer and confirm the transaction.

## From here pallet-voting test begins 
### Create a Proposal
1. Go to Developer -> Extrinsics.

2. Select Alice (or any funded account) from the "using the selected account" dropdown.

3. In the "submit the following extrinsic" section:

4. Select "VotingModule" pallet .

5. Select the create_proposal function.

6. Enter the description (e.g., "Proposal 1") and duration (e.g., 10 blocks).

7. Click Submit Transaction and confirm.


### Notedown the uniqueId of proposal
1. Go to Developer -> Network -> Explorer -> Recent events section.

2. Select the event which corresponds to "votingModule.ProposalCreated"

3. Confirm the block number where this transaction is stored.

4. Notedown the corresponding H256. This is the uniqueId for proposal 

5. The above unique id will be used for Vote and getPropsalResults

Example: `0x57697c5970983b431d34f39ce6ce5b9d2561391852eba4c2ca1cf3bc0e1226d3`

### To vote on the proposal from different accounts:
1. Go to Developer -> Extrinsics.

2. Select an account from the "using the selected account" dropdown (e.g., Alice, Bob).

3. In the "submit the following extrinsic" section:

4. Select "VotingModule"  pallet.

5. Select the vote function.

6. Enter the proposal_hash (you can find this hash in the events or storage) and vote (true for "Yes", false for "No").

7. Click Submit signed Transaction and confirm.

8. Repeat the above steps for other accounts like Bob, Charlie, Dave, etc who has balance.

### Get Proposal Results
After the voting period has ended (e.g., 10 blocks):

1. Go to Developer -> Extrinsics.

2. Select an account from the "using the selected account" dropdown (e.g., Alice).

3. In the "submit the following extrinsic" section:

4. Select your custom pallet (e.g., palletVoting).

5. Select the `getPropsalResults` function.

6. Enter the proposal_hash of the proposal you want to get the results for.

7. Click Submit Transaction and confirm.

### Example Workflow
#### 1. Creating a Proposal
1. Select Alice.

2. Call create_proposal with description "Proposal 1" and duration 10.

#### 2. Voting on the Proposal

1. Select Bob.

2. Call vote with the proposal_hash obtained from the proposal creation event and vote true.

3. Select Charlie.

4. Call vote with the same proposal_hash and vote false.

#### 3. Finalizing the Proposal

1. Wait for the voting period to end (e.g., 10 blocks).

2. Select Alice.

3. Call get_proposal_results with the proposal_hash.

### Notes To test on -ve test case
1. To test each account votes only once per proposal, Try voting with same account on the same proposal 2 times 

2. Cant vote after the duration: Try to vote on proposal whos duration is already completed.

3. Cant get the proposal results before the voting period ends : Try to call the get_proposal_result before the duration has ended.
