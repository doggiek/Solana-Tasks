# Pinocchio 托管挑战

## 概述

托管是一种强大的金融工具，可以在两方之间实现安全的代币交换。可以将其视为一个数字保险箱，其中一位用户可以锁定代币 A，等待另一位用户存入代币 B，然后完成交换。这创造了一个无需信任的环境，双方都不需要担心对方会退出交易。

在本次挑战中，我们将通过三个简单但强大的指令来实现这一概念：

- **创建（Make）**：创建者（第一位用户）定义交易条款，并将约定数量的代币 A 存入一个安全的保险库
- **接受（Take）**：接受者（第二位用户）通过将承诺的代币 B 转移给创建者来接受报价，并作为回报，获得锁定的代币 A
- **退款（Refund）**：如果创建者改变主意或未找到合适的接受者，他们可以取消报价并取回代币 A

## 前置条件

如果您不熟悉匹诺曹，建议先阅读[匹诺曹简介](https://learn.blueshift.gg/zh-CN/courses/introduction-to-pinocchio)，以熟悉我们将在本程序中使用的核心概念。

## 安装

让我们从创建一个全新的 Rust 环境开始：

```bash
# create workspace
cargo new blueshift_escrow --lib --edition 2021
cd blueshift_escrow
```

添加必要的依赖：

```bash
cargo add pinocchio pinocchio-system pinocchio-token pinocchio-associated-token-account
```

在 Cargo.toml 中声明 crate 类型：

```toml
[lib]
crate-type = ["lib", "cdylib"]
```

## 程序结构

### 目录结构

```
src/
├── lib.rs              # 程序入口点
├── instructions/
│   ├── mod.rs          # 模块导出
│   ├── make.rs         # 创建指令
│   ├── take.rs         # 接受指令
│   ├── refund.rs       # 退款指令
│   └── helpers.rs      # 辅助函数
├── state.rs            # 托管状态定义
└── errors.rs           # 错误定义
```

### 主要组件

#### 1. 程序入口点 (lib.rs)

```rust
#![no_std]

use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult
};

entrypoint!(process_instruction);

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

// 程序 ID
pub const ID: Pubkey = [
    0x0f, 0x1e, 0x6b, 0x14, 0x21, 0xc0, 0x4a, 0x07,
    0x04, 0x31, 0x26, 0x5c, 0x19, 0xc5, 0xbb, 0xee,
    0x19, 0x92, 0xba, 0xe8, 0xaf, 0xd1, 0xcd, 0x07,
    0x8e, 0xf8, 0xaf, 0x70, 0x47, 0xdc, 0x11, 0xf7,
];

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Make::DISCRIMINATOR, data)) => Make::try_from((data, accounts))?.process(),
        Some((Take::DISCRIMINATOR, _)) => Take::try_from(accounts)?.process(),
        Some((Refund::DISCRIMINATOR, _)) => Refund::try_from(accounts)?.process(),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
```

#### 2. 托管状态 (state.rs)

`#[repr(C)]` 属性确保我们的结构体具有可预测的内存布局，这对于链上数据至关重要。

```rust
use pinocchio::{program_error::ProgramError, pubkey::Pubkey};
use core::mem::size_of;

#[repr(C)]
pub struct Escrow {
    pub seed: u64,        // 随机种子，用于 PDA 派生
    pub maker: Pubkey,      // 创建托管的钱包地址
    pub mint_a: Pubkey,    // 存入代币的 SPL 代币铸造地址
    pub mint_b: Pubkey,    // 请求代币的 SPL 代币铸造地址
    pub receive: u64,      // 创建者希望接收的代币 B 的确切数量
    pub bump: [u8; 1],    // PDA 推导中使用的单字节
}
```

**字段说明：**

- `seed`: 一个随机数，允许一个创建者使用相同的代币对创建多个托管
- `maker`: 创建托管并将接收代币的钱包地址
- `mint_a`: 存入代币的 SPL 代币铸造地址
- `mint_b`: 请求代币的 SPL 代币铸造地址
- `receive`: 创建者希望接收的代币 B 的确切数量
- `bump`: 在 PDA 推导中使用的单字节，用于确保地址不在 Ed25519 曲线上

#### 3. 核心指令

##### 创建指令 (Make)

创建托管并存入代币 A，设置交换条款。

##### 接受指令 (Take)

接受者通过存入代币 B 来接受报价，获得代币 A。

##### 退款指令 (Refund)

创建者取消托管并取回代币 A。

## 关键特性

### 内存安全

- 精确的大小计算：`LEN` 通过汇总每个字段的大小，精确计算账户大小
- 安全加载：`load` 提供了一种安全的方式来加载和验证托管数据
- 性能优化：在 getter 上使用 `#[inline(always)]` 以实现最大性能

### PDA 使用

- 使用程序派生地址确保托管账户的唯一性和安全性
- bump 种子确保地址不在 Ed25519 曲线上
- 种子包括：`"escrow"`, maker 地址, 和随机 seed

### SPL 代币集成

- 使用 `pinocchio-token` 和 `pinocchio-associated-token-account`
- 支持代币转账和关联代币账户操作
- 完整的代币铸造和转账逻辑

## 构建和测试

```bash
# 构建程序
cargo build-sbf

# 这会在 target/deploy 中生成 .so 文件
```

## 部署

1. 点击 Blueshift 平台上的 "参加挑战" 按钮
2. 将生成的 `.so` 文件拖拽到上传区域
3. 等待验证结果

## 相关链接

- [参加挑战](https://learn.blueshift.gg/challenges/pinocchio-escrow/verify)
- [查看源码](https://github.com/blueshift-gg/blueshift-dashboard/tree/master/src/app/content/challenges/pinocchio-escrow/zh-CN/challenge.mdx)
- [匹诺曹简介](https://learn.blueshift.gg/zh-CN/courses/introduction-to-pinocchio)
