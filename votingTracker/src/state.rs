use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct VoteTracker {
    pub bump: u8, // bump seed of tracker
    pub auth_bump: u8, // bump seed of the auth
    pub proposalToVote: Pubkey,
    pub approve_count: u64,
    pub reject_count: u64
}
