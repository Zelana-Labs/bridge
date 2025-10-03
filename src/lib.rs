#![allow(unexpected_cfgs)]
// #![no_std]

#[cfg(feature = "std")]
extern crate std;

use pinocchio::{account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::{Pubkey},  ProgramResult};

pub mod state;
pub mod helpers;
pub mod instruction;
use instruction::*;

pinocchio_pubkey::declare_id!("95sWqtU9fdm19cvQYu94iKijRuYAv3wLqod1pcsSfYth");

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    
    assert_eq!(program_id,&ID);

    let ( discriminator , data ) = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;

    match BridgeIx::try_from(discriminator)? {
        BridgeIx::INIT=>{
            instruction::init::process_initialize(program_id, accounts, data)
        }
        BridgeIx::DEPOSIT=>{
            instruction::deposit::process_deposit(accounts, data)
        }
        BridgeIx::WITHDRAWATTESTED=>{
            instruction::withdraw::process_withdraw_attested(program_id, accounts, data)
        }
    }   
}