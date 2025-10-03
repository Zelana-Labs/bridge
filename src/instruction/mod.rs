use pinocchio::program_error::ProgramError;



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