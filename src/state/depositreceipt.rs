use bytemuck::{Pod,Zeroable};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::helpers::StateDefinition;

#[derive(Pod, Zeroable, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Depositreceipt{
    pub depositor: Pubkey,
    pub amount: u64,
    pub nonce: u64,
    pub ts: i64,
    pub bump: u8,
    pub _padding: [u8; 7],
}


impl StateDefinition for Depositreceipt{
    const LEN: usize = core::mem::size_of::<Depositreceipt>();
    const SEED: &'static str = "depositreceipt";
}

impl Depositreceipt{
    pub fn from_account_info_unchecked(account_info: &AccountInfo)-> &mut Self{
        unsafe { &mut *(account_info.borrow_mut_data_unchecked().as_ptr() as *mut Self) }
    }

    pub fn from_account_info(
        account_info: &AccountInfo
    )->Result<&mut Self,ProgramError>{
        if account_info.data_len()<Depositreceipt::LEN{
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(Self::from_account_info_unchecked(account_info))
    }

    pub fn new(
        &mut self,
        bump: u8,
        amount:u64,

    )
}