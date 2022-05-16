use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::VoteInstruction;
use crate::state::Votes;

pub fn assert_with_msg(statement: bool, err: ProgramError, msg: &str) -> ProgramResult {
    if !statement {
        msg!(msg);
        Err(err)
    } else {
        Ok(())
    }
}

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = VoteInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            VoteInstruction::Approve => {
                msg!("Instruction: Increment");
                // Decode AccountInfo's
                let accounts_iter = &mut accounts.iter();
                let votes_ai = next_account_info(accounts_iter)?;
                let authority = next_account_info(accounts_iter)?;
                assert_with_msg(
                    authority.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Authority must sign",
                )?;
                // Deserialize account data
                let mut votes = Votes::try_from_slice(&votes_ai.data.borrow())?;
                if ! votes.initialized  {
                    // Set the authority if this is the first time the counter has been used. 
                    votes.authority = *authority.key;
                    votes.initialized = true;
                }
                assert_with_msg(
                    votes.authority == *authority.key,
                    ProgramError::MissingRequiredSignature,
                    "Attempted to increment with an invalid authority",
                )?;
                // Update account data
                votes.approve_count += 1;
                msg!("Global count: {}", votes.approve_count);
                // Serialize account
                votes.serialize(&mut *votes_ai.data.borrow_mut())?;
            }
            VoteInstruction::Reject => {
                msg!("Instruction: Increment");
                // Decode AccountInfo's
                let accounts_iter = &mut accounts.iter();
                let votes_ai = next_account_info(accounts_iter)?;
                let authority = next_account_info(accounts_iter)?;
                assert_with_msg(
                    authority.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Authority must sign",
                )?;
                // Deserialize account data
                let mut votes = Votes::try_from_slice(&votes_ai.data.borrow())?;
                if ! votes.initialized  {
                    // Set the authority if this is the first time the counter has been used. 
                    votes.authority = *authority.key;
                    votes.initialized = true;
                }
                assert_with_msg(
                    votes.authority == *authority.key,
                    ProgramError::MissingRequiredSignature,
                    "Attempted to increment with an invalid authority",
                )?;
                // Update account data
                votes.reject_count += 1;
                msg!("Global count: {}", votes.reject_count);
                // Serialize account
                votes.serialize(&mut *votes_ai.data.borrow_mut())?;
            }
        }
        Ok(())
    }
}
