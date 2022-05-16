use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum TrackerInstruction {
    Initialize,
    Approve,
    Reject
}