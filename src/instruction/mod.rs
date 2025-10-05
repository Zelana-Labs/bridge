use pinocchio::{program_error::ProgramError, pubkey::Pubkey};
use bytemuck::{Pod,Zeroable};

use crate::helpers::DataLen;

pub mod deposit;
pub mod init;
pub mod withdraw;

#[repr(u8)]
pub enum BridgeIx{
    INIT = 0,
    DEPOSIT = 1,
    WITHDRAWATTESTED= 2
}

impl TryFrom<&u8> for BridgeIx  {
    type Error = ProgramError;
     fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(BridgeIx::INIT),
            1=>Ok(BridgeIx::DEPOSIT),
            2=>Ok(BridgeIx::WITHDRAWATTESTED),
            _=>Err(ProgramError::InvalidInstructionData)
        }
    }
}
#[derive(Pod,Zeroable,Clone, Copy, Debug, PartialEq, shank::ShankType)]
#[repr(C)]
pub struct InitParams{
    pub sequencer_authority:Pubkey,
    pub domain:[u8;32]
}

impl DataLen for InitParams {
    const LEN: usize = core::mem::size_of::<InitParams>();
}

#[derive(Pod,Zeroable,Clone, Copy)]
#[repr(C)]
pub struct DepositParams{
    pub amount:u64,
    pub nonce:u64
}

impl DataLen for DepositParams{
    const LEN: usize = core::mem::size_of::<DepositParams>();
}

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct WithdrawAttestedParams {
    pub recipient: Pubkey,
    pub amount: u64,
    pub nullifier: [u8; 32],
}

impl DataLen for WithdrawAttestedParams{
    const LEN: usize = core::mem::size_of::<WithdrawAttestedParams>();
}