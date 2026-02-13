#![no_std]

use pinocchio::{
    account::AccountView,
    address::Address,
    entrypoint,
    error::{ProgramError, ProgramResult},
};

entrypoint!(process_instruction);

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

pub mod errors;
pub use errors::*;

// 程序 ID
pub const ID: Address = [
    0x0f, 0x1e, 0x6b, 0x14, 0x21, 0xc0, 0x4a, 0x07,
    0x04, 0x31, 0x26, 0x5c, 0x19, 0xc5, 0xbb, 0xee,
    0x19, 0x92, 0xba, 0xe8, 0xaf, 0x70, 0x47, 0xdc, 0x11, 0xf7,
];

// 指令路由机制：
// 程序使用"判别器（Discriminator）"模式来路由不同的指令
// 每个指令都有一个唯一的字节（DISCRIMINATOR）作为标识
// Solana 运行时会将 instruction_data 的第一个字节与判别器匹配
// 来决定调用哪个指令处理器
fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    // split_first() 将 instruction_data 分成第一个字节（判别器）和剩余数据
    // 使用模式匹配来路由到对应的指令处理器
    match instruction_data.split_first() {
        // Make 指令：创建托管交易
        // - 解析: 传入指令数据和账户列表
        // - Make::try_from: 验证账户并解析指令数据
        // - .process(): 执行业务逻辑
        Some((instructions::make::Make::DISCRIMINATOR, data)) => {
            instructions::make::Make::try_from((data, accounts))?.process()
        }

        // Take 指令：接受托管交易
        // - 无额外数据，只需要账户列表
        Some((instructions::take::Take::DISCRIMINATOR, _)) => {
            instructions::take::Take::try_from(accounts)?.process()
        }

        // Refund 指令：取消托管交易并退款
        // - 无额外数据，只需要账户列表
        Some((instructions::refund::Refund::DISCRIMINATOR, _)) => {
            instructions::refund::Refund::try_from(accounts)?.process()
        }

        // 如果判别器不匹配任何已知指令，返回错误
        _ => Err(ProgramError::InvalidInstructionData)
    }
}