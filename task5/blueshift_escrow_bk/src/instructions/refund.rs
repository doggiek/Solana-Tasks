use pinocchio::{
    account::AccountView,
    address::Address,
    error::ProgramError,
    ProgramResult,
};

use crate::state::Escrow;

pub struct RefundAccounts<'info> {
    pub maker: &'info AccountView,
    pub escrow: &'info AccountView,
    pub mint_a: &'info AccountView,
    pub vault: &'info AccountView,
    pub maker_ata_a: &'info AccountView,
    pub system_program: &'info AccountView,
    pub token_program: &'info AccountView,
}

impl<'info> TryFrom<&'info [AccountView]> for RefundAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountView]) -> Result<Self, Self::Error> {
        let [maker, escrow, mint_a, vault, maker_ata_a, system_program, token_program, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !maker.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        Ok(Self {
            maker,
            escrow,
            mint_a,
            vault,
            maker_ata_a,
            system_program,
            token_program,
        })
    }
}

pub struct Refund<'info> {
    pub accounts: RefundAccounts<'info>,
}

impl<'info> TryFrom<&'info [AccountView]> for Refund<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountView]) -> Result<Self, Self::Error> {
        let accounts = RefundAccounts::try_from(accounts)?;

        Ok(Self {
            accounts,
        })
    }
}

impl<'info> Refund<'info> {
    pub const DISCRIMINATOR: u8 = 2;

    pub fn process(&mut self) -> ProgramResult {
        Ok(())
    }
}
