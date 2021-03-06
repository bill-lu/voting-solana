use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke_signed, invoke},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    system_program::ID as SYSTEM_PROGRAM_ID,
    sysvar::{rent::Rent, Sysvar},
};
use voting::instruction::VoteInstruction;

use crate::instruction::TrackerInstruction;
use crate::state::VoteTracker;

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
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = TrackerInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let accounts_iter = &mut accounts.iter();
        match instruction {
            TrackerInstruction::Initialize => {
                msg!("Instruction: Initialize");
                let tracker_ai = next_account_info(accounts_iter)?;
                let user = next_account_info(accounts_iter)?;
                let authority = next_account_info(accounts_iter)?;
                let votes = next_account_info(accounts_iter)?;
                let system_program = next_account_info(accounts_iter)?;

                let (authority_key, auth_bump) =
                    Pubkey::find_program_address(&[votes.key.as_ref()], program_id);

                let (tracker_key, bump) = Pubkey::find_program_address(
                    &[user.key.as_ref(), votes.key.as_ref()],
                    program_id,
                );
                // invoke: use when no PDAs
                // invoke_signed: use when PDAs sign
                invoke_signed(
                    &system_instruction::create_account(
                        user.key,
                        tracker_ai.key,
                        Rent::get()?.minimum_balance(42),
                        42,
                        program_id,
                    ),
                    &[user.clone(), tracker_ai.clone(), system_program.clone()],
                    &[&[user.key.as_ref(), votes.key.as_ref(), &[bump]]],
                )?;
                assert_with_msg(
                    *system_program.key == SYSTEM_PROGRAM_ID,
                    ProgramError::InvalidArgument,
                    "Invalid passed in for system program",
                )?;
                assert_with_msg(
                    tracker_key == *tracker_ai.key,
                    ProgramError::InvalidArgument,
                    "Invalid PDA seeds for tracker",
                )?;
                assert_with_msg(
                    authority_key == *authority.key,
                    ProgramError::InvalidArgument,
                    "Invalid PDA seeds for authority",
                )?;

                let mut tracker = VoteTracker::try_from_slice(&tracker_ai.data.borrow())?;
                tracker.bump = bump;
                tracker.auth_bump = auth_bump;
                // Not necessary but potentially useful for client side queries
                tracker.proposalToVote = *votes.key;
                tracker.approve_count = 0;
                tracker.reject_count = 0;
                tracker.serialize(&mut *tracker_ai.data.borrow_mut())?;
            }
            TrackerInstruction::Approve => {
                msg!("Instruction: Increment");
                // Decode AccountInfo's
                let tracker_ai = next_account_info(accounts_iter)?;
                let user = next_account_info(accounts_iter)?;
                let counter_program = next_account_info(accounts_iter)?;
                let counter = next_account_info(accounts_iter)?;
                let authority = next_account_info(accounts_iter)?;

                // Deserialize account data
                let mut tracker = VoteTracker::try_from_slice(&tracker_ai.data.borrow())?;

                // Validate auth seeds
                let authority_seeds = &[counter.key.as_ref(), &[tracker.auth_bump]];
                let auth_key = Pubkey::create_program_address(authority_seeds, program_id)?;
                assert_with_msg(
                    auth_key == *authority.key,
                    ProgramError::InvalidArgument,
                    "Invalid PDA seeds for authority",
                )?;

                // Validate tracker seeds
                let tracker_seeds = &[user.key.as_ref(), counter.key.as_ref(), &[tracker.bump]];
                let tracker_key = Pubkey::create_program_address(tracker_seeds, program_id)?;
                assert_with_msg(
                    tracker_key == *tracker_ai.key,
                    ProgramError::InvalidArgument,
                    "Invalid PDA seeds for tracker",
                )?;

                // TODO CPI to authorized_counter
                invoke_signed(
                    &voting::instruction::generate_vote_instruction(
                        *counter_program.key,
                        *counter.key,
                        *authority.key,
                        VoteInstruction::Approve)?,
                    &[
                        counter_program.clone(),
                        counter.clone(),
                        authority.clone(),
                    ],
                    &[&[counter.key.as_ref(), &[tracker.auth_bump]]],
                )?;
                tracker.approve_count += 1;
                msg!("User Count {}", tracker.approve_count);
                tracker.serialize(&mut *tracker_ai.data.borrow_mut())?;
            }

            TrackerInstruction::Reject => {
                msg!("Instruction: Increment");
                // Decode AccountInfo's
                let tracker_ai = next_account_info(accounts_iter)?;
                let user = next_account_info(accounts_iter)?;
                let counter_program = next_account_info(accounts_iter)?;
                let counter = next_account_info(accounts_iter)?;
                let authority = next_account_info(accounts_iter)?;

                // Deserialize account data
                let mut tracker = VoteTracker::try_from_slice(&tracker_ai.data.borrow())?;

                // Validate auth seeds
                let authority_seeds = &[counter.key.as_ref(), &[tracker.auth_bump]];
                let auth_key = Pubkey::create_program_address(authority_seeds, program_id)?;
                assert_with_msg(
                    auth_key == *authority.key,
                    ProgramError::InvalidArgument,
                    "Invalid PDA seeds for authority",
                )?;

                // Validate tracker seeds
                let tracker_seeds = &[user.key.as_ref(), counter.key.as_ref(), &[tracker.bump]];
                let tracker_key = Pubkey::create_program_address(tracker_seeds, program_id)?;
                assert_with_msg(
                    tracker_key == *tracker_ai.key,
                    ProgramError::InvalidArgument,
                    "Invalid PDA seeds for tracker",
                )?;

                // TODO CPI to authorized_counter
                invoke_signed(
                    &voting::instruction::generate_vote_instruction(
                        *counter_program.key,
                        *counter.key,
                        *authority.key,
                        VoteInstruction::Reject)?,
                    &[
                        counter_program.clone(),
                        counter.clone(),
                        authority.clone(),
                    ],
                    &[&[counter.key.as_ref(), &[tracker.auth_bump]]],
                )?;
                tracker.reject_count += 1;
                msg!("User Count {}", tracker.reject_count);
                tracker.serialize(&mut *tracker_ai.data.borrow_mut())?;
            }
        }
        Ok(())
    }
}
