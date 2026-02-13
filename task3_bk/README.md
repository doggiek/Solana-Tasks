# Task 3 - Anchor 托管

## 📋 任务概述

使用 Anchor 创建一个托管应用，实现安全的代币交换功能。

## 🎯 学习目标

- 理解托管服务的核心概念
- 掌握 Anchor 框架的高级用法
- 实现复杂的链上程序逻辑
- 学习模块化代码组织

## 📚 任务链接

- **任务页面**: [Anchor 托管](https://learn.blueshift.gg/zh-CN/challenges/anchor-escrow)
- **创建指令**: [Make](https://learn.blueshift.gg/zh-CN/challenges/anchor-escrow/make)
- **接受指令**: [Take](https://learn.blueshift.gg/zh-CN/challenges/anchor-escrow/take)
- **退款指令**: [Refund](https://learn.blueshift.gg/zh-CN/challenges/anchor-escrow/refund)

## 🔧 核心功能

托管服务包含三个主要指令：

### 1. 创建（Make）

- 创建者定义交易条款
- 将约定数量的代币 A 存入保险库
- 设定交换条件

### 2. 接受（Take）

- 接受者通过转移代币 B 给创建者来接受报价
- 获得锁定的代币 A
- 完成双方交易

### 3. 退款（Refund）

- 创建者可以取消报价
- 取回存入的代币 A
- 适用于交易失败的情况

## 🏗️ 项目结构

```
src/
├── instructions/
│   ├── make.rs      # 创建托管指令
│   ├── mod.rs       # 指令模块声明
│   ├── refund.rs    # 退款指令
│   └── take.rs      # 接受指令
├── errors.rs        # 自定义错误定义
├── lib.rs          # 主程序文件
└── state.rs        # 托管状态结构
```

## 📊 状态结构

托管账户包含以下字段：

- `seed`: 随机种子，用于派生 PDA
- `maker`: 创建者钱包地址
- `mint_a`: 给出的代币铸币地址
- `mint_b`: 期望获得的代币铸币地址
- `receive`: 期望获得的代币 B 数量
- `bump`: 缓存的 bump 字节

## 🛠️ 技术要求

- Anchor 0.31.0 或更高版本
- 启用 `init-if-needed` 功能
- 使用 `anchor-spl` 进行 SPL 代币操作
- 自定义指令 discriminator

## ⚠️ 注意事项

- 建议先完成 [Anchor for Dummies](https://learn.blueshift.gg/zh-CN/courses/anchor-for-dummies) 课程
- 确保理解 PDA（程序派生地址）的概念
- 注意代币转账的安全性和权限控制

## 📝 提交要求

- 在 [Blueshift](https://learn.blueshift.gg/zh-CN) 平台提交并通过测试
- 确保所有三个指令功能正常
- 代码结构清晰，注释完整
