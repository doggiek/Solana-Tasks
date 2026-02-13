use pinocchio::error::ProgramError;

#[repr(u32)]
pub enum EscrowError {
    InvalidInstruction = 3000,
    InvalidAccount,
    InvalidAmount,
    InvalidSeed,
    AccountAlreadyInitialized,
    AccountNotInitialized,
    InvalidMint,
    InvalidOwner,
    InvalidTokenAccount,
    InsufficientFunds,
    InvalidState,
}

impl From<EscrowError> for ProgramError {
    fn from(error: EscrowError) -> Self {
        ProgramError::Custom(error as u32)
    }
}