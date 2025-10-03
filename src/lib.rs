#![allow(unexpected_cfgs)]
// #![no_std]

#[cfg(feature = "std")]
extern crate std;

use pinocchio::{account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::{self, Pubkey}, sysvars::instructions, ProgramResult};

pub mod state;
pub mod helpers;
pub mod instruction;
use instruction::*;

//BrdgVaulT1111111111111111111111111111111111
pub const ID: Pubkey = [
    0x1d, 0x07, 0x3d, 0x2e, 0x89, 0x19, 0xe6, 0xd6,
    0x3c, 0x36, 0x65, 0x27, 0x3a, 0x7d, 0x64, 0x9a,
    0x1b, 0xd5, 0x3b, 0x36, 0x8c, 0x9d, 0x92, 0x37,
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
];


fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    
    assert_eq!(program_id,&ID);

    let ( discriminator , data ) = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;

    match BridgeIx::try_from(discriminator)? {
        BridgeIx::INIT=>{
            todo!()
        }
        BridgeIx::DEPOSIT=>{
            todo!()
        }
        BridgeIx::WITHDRAWATTESTED=>{
            todo!()
        }
        _=>{
            todo!()
        }
    }   
}