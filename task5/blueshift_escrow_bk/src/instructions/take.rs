use pinocchio::{
    account::AccountView,
    address::Address,
    error::ProgramError,
    ProgramResult,
};

use crate::state::Escrow;

pub struct TakeAccounts<'info> {
    pub taker: &'info AccountView,
    pub maker: &'info AccountView,
    pub escrow: &'info AccountView,
    pub mint_a: &'info AccountView,
    pub mint_b: &'info AccountView,
    pub vault: &'info AccountView,
    pub taker_ata_a: &'info AccountView,
    pub taker_ata_b: &'info AccountView,
    pub maker_ata_b: &'info AccountView,
    pub system_program: &'info AccountView,
    pub token_program: &'info AccountView,
}

impl<'info> TryFrom<&'info [AccountView]> for TakeAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountView]) -> Result<Self, Self::Error> {
        let [taker, maker, escrow, mint_a, mint_b, vault, taker_ata_a, taker_ata_b, maker_ata_b, system_program, token_program, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !taker.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        Ok(Self {
            taker,
            maker,
            escrow,
            mint_a,
            mint_b,
            vault,
            taker_ata_a,
            taker_ata_b,
            maker_ata_b,
            system_program,
            token_program,
        })
    }
}

pub struct Take<'info> {
    pub accounts: TakeAccounts<'info>,
}

impl<'info> TryFrom<&'info [AccountView]> for Take<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountView]) -> Result<Self, Self::Error> {
        let accounts = TakeAccounts::try_from(accounts)?;

        Ok(Self {
            accounts,
        })
    }
}

impl<'info> Take<'info> {
    pub const DISCRIMINATOR: u8 = 1;

    pub fn process(&mut self) -> ProgramResult {
        Ok(())
    }
}
