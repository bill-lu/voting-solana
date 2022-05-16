use solana_program::{pubkey::Pubkey};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Votes {
    pub authority: Pubkey,
    pub proposal_number: u32,
    pub approve_count: u64,
    pub reject_count: u64,
    pub initialized: bool
}