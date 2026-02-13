# Solana Foundation Bootcamp

## ✅ Challenge 任务

| 任务         | 名称                                                                                       | 状态      | 说明                                                                                   |
| ------------ | ------------------------------------------------------------------------------------------ | --------- | -------------------------------------------------------------------------------------- |
| **Task 1**   | [铸造 SPL Token](https://learn.blueshift.gg/zh-CN/challenges/typescript-mint-an-spl-token) | ✅ 已完成 | 使用 web3.js 铸造一个 SPL Token                                                        |
| **Task 2**   | [Anchor 金库](https://learn.blueshift.gg/zh-CN/challenges/anchor-vault)                    | ✅ 已完成 | 使用 Anchor 创建用户金库                                                               |
| **Task 3**   | [Anchor 托管](https://learn.blueshift.gg/zh-CN/challenges/anchor-escrow)                   | ⏳ 进行中 | 使用 Anchor 创建托管应用                                                               |
| **Task 4**   | [Pinocchio 金库](https://learn.blueshift.gg/zh-CN/challenges/pinocchio-vault)              | ✅ 已完成 | 使用 Pinocchio 创建用户金库并提交                                                      |
| **Task 5**   | [Pinocchio 托管](https://learn.blueshift.gg/zh-CN/challenges/pinocchio-escrow)             | ✅ 已完成 | 使用 Pinocchio 创建用户托管并提交                                                      |
| **Task 6**   | [Pinocchio AMM (可选)](https://learn.blueshift.gg/zh-CN/challenges/pinocchio-amm)          | ⏸️ 未开始 | 使用 Pinocchio 开发一个 AMM Swap                                                       |
| **毕业设计** | Solana 应用开发                                                                            | ⏸️ 未开始 | 使用 create-solana-dapp 结合课上所学做一个 Solana 小应用并提交 Solana 黑客松，主题自选 |

## 📝 任务提交要求

### Task 1 - Task 6

- 直接在 [Blueshift](https://learn.blueshift.gg/zh-CN) 提交并测试通过
- Task 6 为可选项目，非必须

### 毕业设计

1. 在你的 GitHub 账户下新建 GitHub repo 完成项目
2. Fork 本 bootcamp repo
3. 参考 [finalProject/demo.md](./finalProject/demo.md) 填写：
   - 项目简介
   - 技术介绍
   - Demo/视频
     ...
4. 提交 Pull Request 合并到本 repo finalProject 文件夹

**在 task 1-4 或 完成毕业设计并提交 中完成一个即算满足毕业条件**
**参加 Solana 华语 Vibe Coding 黑客松并提交可直接毕业 https://x.com/trendsdotfun/status/2013570113430265869**

## 📊 完成进度总结

### ✅ 已完成任务 (3/6)

- **Task 1**: SPL Token 铸造 - 使用 web3.js 完成
- **Task 2**: Anchor 金库 - 使用 Anchor 框架完成
- **Task 4**: Pinocchio 金库 - 使用 Pinocchio 框架完成，已编译生成 `blueshift_vault.so`

### ⏳ 进行中任务 (1/6)

- **Task 3**: Anchor 托管 - 使用 Anchor 框架开发中

### ⏸️ 未开始任务 (1/6)

- **Task 6**: Pinocchio AMM (可选)
- **毕业设计**: Solana 应用开发

### 📝 已记录任务 (1/6)

- **Task 5**: Pinocchio 托管 - 已创建完整的技术文档和实现指南

### 🎯 下一步建议

1. 完成 **Task 3** (Anchor 托管) - 已有基础框架，继续开发
2. 考虑完成 **毕业设计** 以满足毕业条件
3. 可选完成 **Task 5** 或 **Task 6** 获得更多实践经验

### 📁 项目文件结构

```markdown
Solana-Tasks/
├── task1/ # ✅ SPL Token (web3.js)
├── task2/ # ✅ Anchor Vault  
├── task3/ # ⏳ Anchor Escrow
├── task4/ # ✅ Pinocchio Vault
│ ├── README.md # 详细文档
│ └── blueshift_vault/ # Rust 实现
│ ├── src/
│ │ ├── lib.rs
│ │ └── instructions/
│ ├── Cargo.toml
│ └── target/deploy/
│ └── blueshift_vault.so # 可部署文件
├── task5/ # 📝 Pinocchio Escrow
│ └── README.md # 技术文档和实现指南
└── README.md # 本文件
```
