use pinocchio::{
    account::AccountView,
    address::Address,
    error::ProgramError,
};

// 辅助函数：从账户迭代器中获取下一个账户
pub fn next_account_info(accounts: &mut std::slice::Iter<&AccountView>) -> Result<&AccountView, ProgramError> {
    accounts.next().ok_or(ProgramError::NotEnoughAccountKeys)
}

// 辅助函数：生成 PDA 种子
pub fn create_escrow_seed(seed: u64, maker: &Address) -> [u8; 40] {
    let mut seed_bytes = [0u8; 40];
    seed_bytes[0..8].copy_from_slice(&seed.to_le_bytes());
    seed_bytes[8..40].copy_from_slice(maker.as_ref());
    seed_bytes
}

// 辅助函数：生成金库种子
pub fn create_vault_seed(escrow: &Address, mint: &Address) -> [u8; 64] {
    let mut seed_bytes = [0u8; 64];
    seed_bytes[0..32].copy_from_slice(escrow.as_ref());
    seed_bytes[32..64].copy_from_slice(mint.as_ref());
    seed_bytes
}

// 辅助函数：验证账户所有权
pub fn assert_owned_by(account: &AccountView, owner: &Address) -> Result<(), ProgramError> {
    if account.owner() != owner {
        Err(ProgramError::InvalidAccountOwner)
    } else {
        Ok(())
    }
}

// 辅助函数：验证签名者
pub fn assert_signer(account: &AccountView) -> Result<(), ProgramError> {
    if !account.is_signer() {
        Err(ProgramError::MissingRequiredSignature)
    } else {
        Ok(())
    }
}