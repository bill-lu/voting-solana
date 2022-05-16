use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum VoteInstruction {
    Approve,
    Reject
}

pub fn generate_vote_instruction(
    program_id: Pubkey,
    votes: Pubkey,
    authority: Pubkey,
    instruction: VoteInstruction,
) -> Result<Instruction, ProgramError> {
    Ok(Instruction {
        accounts: vec![
            AccountMeta::new(votes, false),
            AccountMeta::new_readonly(authority, true),
        ],
        data: instruction.try_to_vec()?,
        program_id,
    })
}