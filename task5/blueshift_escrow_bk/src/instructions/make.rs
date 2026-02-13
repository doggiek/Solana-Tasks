use pinocchio::{
    account::AccountView,
    address::Address,
    error::ProgramError,
    ProgramResult,
};

use crate::state::Escrow;
use core::mem::size_of;

// Make 指令的账户结构
pub struct MakeAccounts<'info> {
    pub maker: &'info AccountView,
    pub escrow: &'info AccountView,
    pub mint_a: &'info AccountView,
    pub mint_b: &'info AccountView,
    pub maker_ata_a: &'info AccountView,
    pub vault: &'info AccountView,
    pub system_program: &'info AccountView,
    pub token_program: &'info AccountView,
}

// Make 指令的数据结构
pub struct MakeInstructionData {
    pub seed: u64,
    pub receive: u64,
    pub amount: u64,
}

// 从账户数组解析 MakeAccounts
impl<'info> TryFrom<&'info [AccountView]> for MakeAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountView]) -> Result<Self, Self::Error> {
        // 解构账户数组
        let [maker, escrow, mint_a, mint_b, maker_ata_a, vault, system_program, token_program, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // 验证 maker 是签名者
        if !maker.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        Ok(Self {
            maker,
            escrow,
            mint_a,
            mint_b,
            maker_ata_a,
            vault,
            system_program,
            token_program,
        })
    }
}

// 从字节数组解析 MakeInstructionData
impl<'info> TryFrom<&'info [u8]> for MakeInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        // 验证数据长度：3 个 u64 = 24 字节
        if data.len() != size_of::<u64>() * 3 {
            return Err(ProgramError::InvalidInstructionData);
        }

        // 解析三个 u64 值（小端序）
        let seed = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let receive = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let amount = u64::from_le_bytes(data[16..24].try_into().unwrap());

        // 验证 amount 必须大于 0
        if amount == 0 {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(Self {
            seed,
            receive,
            amount,
        })
    }
}

// Make 指令的主结构
pub struct Make<'info> {
    pub accounts: MakeAccounts<'info>,
    pub instruction_data: MakeInstructionData,
    pub bump: u8,
}

// 从指令数据和账户创建 Make 指令
impl<'info> TryFrom<(&'info [u8], &'info [AccountView])> for Make<'info> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'info [u8], &'info [AccountView])) -> Result<Self, Self::Error> {
        // 解析账户和指令数据
        let accounts = MakeAccounts::try_from(accounts)?;
        let instruction_data = MakeInstructionData::try_from(data)?;

        // 简化版本：暂时使用固定的 bump
        let bump = 0;

        Ok(Self {
            accounts,
            instruction_data,
            bump,
        })
    }
}

// Make 指令的处理逻辑
impl<'info> Make<'info> {
    pub const DISCRIMINATOR: u8 = 0;

    pub fn process(&mut self) -> ProgramResult {
        // 简化版本：暂时只返回成功，不执行复杂逻辑
        // 这样可以确保编译通过，让你先熟悉基本结构
        
        // TODO: 完整实现
        // 1. 创建托管账户
        // 2. 创建金库账户
        // 3. 设置托管数据
        // 4. 转移代币到金库
        
        Ok(())
    }
}
