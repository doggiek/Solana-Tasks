use pinocchio::{error::ProgramError, address::Address};
use core::mem::size_of;

#[repr(C)]
pub struct Escrow {
    pub seed: u64,        // Random seed for PDA derivation
    pub maker: Address,    // Creator of the escrow
    pub mint_a: Address,   // Token being deposited
    pub mint_b: Address,   // Token being requested
    pub receive: u64,     // Amount of token B wanted
    pub bump: [u8;1],      // PDA bump seed
}

impl Escrow {
    pub const LEN: usize = size_of::<u64>() 
    + size_of::<Address>() 
    + size_of::<Address>() 
    + size_of::<Address>() 
    + size_of::<u64>()
    + size_of::<[u8;1]>();
    
    // 初始化托管账户
    pub fn init(&mut self, seed: u64, maker: Address, mint_a: Address, mint_b: Address, receive: u64, bump: u8) {
        self.seed = seed;
        self.maker = maker;
        self.mint_a = mint_a;
        self.mint_b = mint_b;
        self.receive = receive;
        self.bump = [bump];
    }

    // 从字节数组加载 Escrow 结构
    pub fn try_from(data: &[u8]) -> Result<&Self, ProgramError> {
        if data.len() < Escrow::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        
        // 安全地转换字节数据为结构体引用
        let escrow = unsafe { &*(data.as_ptr() as *const Escrow) };
        Ok(escrow)
    }

    // 从可变字节数组加载 Escrow 结构
    pub fn try_from_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if data.len() < Escrow::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        
        // 安全地转换字节数据为可变结构体引用
        let escrow = unsafe { &mut *(data.as_mut_ptr() as *mut Escrow) };
        Ok(escrow)
    }
}