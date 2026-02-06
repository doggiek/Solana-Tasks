# Task 2: Anchor 金库 [Task2](https://learn.blueshift.gg/zh-CN/challenges/anchor-vault)

## 🎯 任务目标

使用 Anchor 框架创建一个简单的 lamport 保险库，展示基本账户、程序派生地址（PDA）和跨程序调用（CPI）的使用。

## 📋 任务概述

保险库允许用户安全地存储他们的资产。保险库是去中心化金融（DeFi）的一个基本构建模块，其核心功能是允许用户安全地存储他们的资产（在本例中是 lamports），并且只有该用户本人可以在之后提取这些资产。

在本次挑战中，我们将构建一个简单的 lamport 保险库，展示如何使用基本账户、程序派生地址（PDA）和跨程序调用（CPI）。

## 🛠️ 环境准备

### 安装依赖

在开始之前，请确保已安装 Rust 和 Anchor（如果需要复习，请参阅 [官方文档](https://book.anchor.com/)）。

然后在终端中运行：

```bash
anchor init blueshift_anchor_vault
```

本次挑战不需要额外的 crate，因此您现在可以打开新生成的文件夹，准备开始编码！

## 📝 程序结构

### 基本模板

让我们从基本的程序结构开始。由于这是一个简单的程序，我们将把所有内容实现到 `lib.rs` 中：

```rust
declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        // deposit logic
        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAction>) -> Result<()> {
        // withdraw logic
        Ok(())
    }
}

#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    // error enum
}
```

**注意：** 请记得将程序 ID 更改为 `22222222222222222222222222222222222222222222`，因为我们会在底层使用它来测试您的程序。

## 🏗️ 账户结构

由于两个指令使用相同的账户，为了更简洁和易读，我们可以创建一个名为 `VaultAction` 的上下文，并将其用于 `deposit` 和 `withdraw`。

`VaultAction` 账户结构需要包含以下内容：

- **signer**：这是保险库的所有者，也是创建保险库后唯一可以提取 lamports 的人。
- **vault**：一个由以下种子派生的 PDA：`[b"vault", signer.key().as_ref()]`，用于为签名者存储 lamports。
- **system_program**：系统程序账户，需要包含它，因为我们将使用系统程序的转账指令 CPI。

以下是我们定义账户结构的方法：

```rust
#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}
```

### 账户约束解析

让我们逐一解析每个账户约束：

- **signer**：需要使用 `mut` 约束，因为我们将在转账过程中修改其 lamports。
- **vault**：
  - `mut`，因为我们将在转账过程中修改其 lamports。
  - `seeds` 和 `bumps` 定义了如何从种子派生出有效的 PDA。
- **system_program**：检查账户是否设置为可执行，并且地址是否为系统程序地址。

## ⚠️ 错误处理

对于这个小程序，我们不需要太多错误处理，因此我们只创建两个枚举：

- **VaultAlreadyExists**：用于判断账户中是否已经有 lamports，因为这意味着金库已经存在。
- **InvalidAmount**：我们不能存入少于基本账户最低租金的金额，因此我们检查金额是否大于该值。

```rust
#[error_code]
pub enum VaultError {
    #[msg("Vault already exists")]
    VaultAlreadyExists,
    #[msg("Invalid amount")]
    InvalidAmount,
}
```

## 💰 存款功能 (Deposit)

存款指令执行以下步骤：

1. 验证金库为空（lamports 为零），以防止重复存款
2. 确保存款金额超过 SystemAccount 的免租金最低限额
3. 使用 CPI 调用系统程序，将 lamports 从签名者转移到金库

让我们先实现这些检查：

```rust
// Check if vault is empty
require_eq!(ctx.accounts.vault.lamports(), 0, VaultError::VaultAlreadyExists);

// Ensure amount exceeds rent-exempt minimum
require_gt!(amount, Rent::get()?.minimum_balance(0), VaultError::InvalidAmount);
```

两个 `require` 宏充当自定义保护子句：

- `require_eq!` 确认金库为空（防止重复存款）。
- `require_gt!` 检查金额是否超过免租金阈值。

一旦检查通过，Anchor 的系统程序助手会像这样调用 Transfer CPI：

```rust
use anchor_lang::system_program::{transfer, Transfer};

transfer(
    CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        },
    ),
    amount,
)?;
```

## 🏦 取款功能 (Withdraw)

取款指令执行以下步骤：

1. 验证保险库中是否有 lamports（不为空）
2. 使用保险库的 PDA 以其自身名义签署转账
3. 将保险库中的所有 lamports 转回到签署者

首先，让我们检查保险库中是否有可取出的 lamports：

```rust
// Check if vault has any lamports
require_neq!(ctx.accounts.vault.lamports(), 0, VaultError::InvalidAmount);
```

然后，我们需要创建 PDA 签名者种子并执行转账：

```rust
// Create PDA signer seeds
let signer_key = ctx.accounts.signer.key();
let signer_seeds = &[b"vault", signer_key.as_ref(), &[ctx.bumps.vault]];

// Transfer all lamports from vault to signer
transfer(
    CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.signer.to_account_info(),
        },
        &[&signer_seeds[..]]
    ),
    ctx.accounts.vault.lamports()
)?;
```

此次取款的安全性由以下两个因素保证：

1. 保险库的 PDA 是使用签署者的公钥派生的，确保只有原始存款人可以取款
2. PDA 签署转账的能力通过我们提供给 `CpiContext::new_with_signer` 的种子进行验证

## 🧪 测试与部署

现在，您可以通过我们的单元测试测试您的程序并领取您的 NFT！

### 构建程序

首先，在终端中使用以下命令构建您的程序：

```bash
anchor build
```

这将在您的 `target/deploy` 文件夹中直接生成一个 `.so` 文件。

### 提交挑战

现在点击 "take challenge" 按钮并将文件拖放到那里！

## 🎯 学习要点

通过这个任务，你将学到：

- **Anchor 框架基础** - 程序结构、账户管理
- **程序派生地址 (PDA)** - 种子派生、bump 种子
- **跨程序调用 (CPI)** - 系统程序转账
- **账户约束** - mut、seeds、bump 等约束
- **错误处理** - 自定义错误枚举
- **安全性设计** - PDA 签名、权限控制

## 📚 相关资源

- [Anchor 官方文档](https://book.anchor.com/)
- [Solana 开发者文档](https://docs.solana.com/)
- [PDA 指南](https://docs.solana.com/developing/programming-model/calling-between-programs#program-derived-addresses)
