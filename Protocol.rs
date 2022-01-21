//! Bounty Chain
//! 
//! This rough protocol sketch uses the Bounty and Treasury pallets from Substrate's FRAME.
//!  
//! ### Terminology: Treasury 
//!
//! - **Proposal:** A suggestion to allocate funds from the pot to a beneficiary.
//! - **Beneficiary:** An account who will receive the funds from a proposal iff the proposal is
//!   approved.
//! - **Deposit:** Funds that a proposer must lock when making a proposal. The deposit will be
//!   returned or slashed if the proposal is approved or rejected respectively.
//! - **Pot:** Unspent funds accumulated by the treasury pallet.
//! 
//! ### Terminology: Bounty
//!
//! Bounty:
//! - **Bounty spending proposal:** A proposal to reward a predefined body of work upon completion
//!   by the Treasury.
//! - **Proposer:** An account proposing a bounty spending.
//! - **Curator:** An account managing the bounty and assigning a payout address receiving the
//!   reward for the completion of work.
//! - **Deposit:** The amount held on deposit for placing a bounty proposal plus the amount held on
//!   deposit per byte within the bounty description.
//! - **Curator deposit:** The payment from a candidate willing to curate an approved bounty. The
//!   deposit is returned when/if the bounty is completed.
//! - **Bounty value:** The total amount that should be paid to the Payout Address if the bounty is
//!   rewarded.
//! - **Payout address:** The account to which the total or part of the bounty is assigned to.
//! - **Payout Delay:** The delay period for which a bounty beneficiary needs to wait before
//!   claiming.
//! - **Curator fee:** The reserved upfront payment for a curator for work related to the bounty.
//!
//! Reviewers and Contributors have reputations and can openly participate in the network.
//! 
//! The governance system has:
//! - A reviewing committee: these are experts whose task is to review bounty submissions and approve or reject them.
//!   These members earn a reputation and are voted to retain their seat on the committee. 
//! 
//! - A creators committee: this can be any bounty hunter looking to earn rewards for the bounties and participate in the network.
//!  
//! - A council: these members have been creators or reviewers for some time. 
//!   Their job is to scout experts in specific knowledge domains. 
//! 
//! Participation protocol:
//! - `request_participation` - Get tokens to get started from Treasury, temporary to kickstart adoption.
//! - `vote_for_reviewer` - Hunters can vote on reviewers they submitted to as part of the governance.
//! 
//! ## Treasury Interface
//!
//! ### Dispatchable Functions
//!
//! General spending/proposal protocol:
//! - `propose_spend` - Make a spending proposal and stake the required deposit.
//! - `reject_proposal` - Reject a proposal, slashing the deposit.
//! - `approve_proposal` - Accept the proposal, returning the deposit.
//!
//! ## GenesisConfig
//!
//! The Treasury pallet depends on the [`GenesisConfig`].
//! 
//! Bounty protocol:
//! - `propose_bounty` - Propose a specific treasury amount to be earmarked for a predefined set of
//!   tasks and stake the required deposit.
//! - `approve_bounty` - Accept a specific treasury amount to be earmarked for a predefined body of
//!   work.
//! 
//! - `propose_curator` - Assign an account to a bounty as candidate curator.
//! - `accept_curator` - Accept a bounty assignment from the Council, setting a curator deposit.
//! 
//! - `extend_bounty_expiry` - Extend the expiry block number of the bounty and stay active.
//! - `award_bounty` - Close and pay out the specified amount for the completed work.
//! - `claim_bounty` - Claim a specific bounty amount from the Payout Address.
//! - `unassign_curator` - Unassign an accepted curator from a specific earmark.
//! - `close_bounty` - Cancel the earmark for a specific treasury amount and close the bounty.
//! 
//! ## User journey
//! 
//! 1. User can browse through existing Societies or create their own.
//! 2. A new user creating their own Society needs to stake some amount and will become the Founder.
//! 3. There can be multiple founders. Founders can invite new members and set the Society rules.
//! 4. Rules can be modified: 
//!     i. amount to maintain membership or amount of activity to maintain membership.
//!     ii. reason for member to be kicked off
//!     iii. number of bounties allowed to review per week
//!     iv. voting threshold to approve reviews
//!     v. 
//! 5. Benefits: receive payouts for participating.
//! 
//! 
//! 