use bytemuck::{Pod,Zeroable};
use pinocchio::pubkey::Pubkey;

use crate::helpers::StateDefinition;

#[derive(Pod, Zeroable, Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Config{
    pub sequencer_authority: Pubkey,
    pub domain: [u8;32],
    pub bump: u8
}


impl StateDefinition for Config{
    const LEN: usize = core::mem::size_of::<Config>();
    const SEED: &'static str = "config";
}