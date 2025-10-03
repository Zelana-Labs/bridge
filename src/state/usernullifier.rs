use bytemuck::{Pod,Zeroable};
use pinocchio::pubkey::Pubkey;

use crate::helpers::StateDefinition;

#[derive(Pod, Zeroable, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct UsedNullifier{
    pub nullifier:[u8;32],
    pub used:u8,
    pub bump:u8,
    pub _paddinig: [u8; 6],
}


impl StateDefinition for UsedNullifier{
    const LEN: usize = core::mem::size_of::<UsedNullifier>();
    const SEED: &'static str = "usernullified";
}