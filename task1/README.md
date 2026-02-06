# Task 1: Mint an SPL Token

## 🎯 任务目标
使用 Web3.js 和 SPL Token 库在单个交易中铸造一个 SPL Token。

## 🚀 快速开始

### 1. 安装依赖
```bash
npm install
```

### 2. 配置环境变量
复制 `.env` 文件并填入你的私钥和 RPC 端点：
```env
SECRET=你的私钥(Base58格式)
RPC_ENDPOINT=https://api.devnet.solana.com
```

### 3. 运行任务
```bash
npm run task1
```

## 📋 完成的功能

✅ **创建 SPL mint 账户** - 使用 `SystemProgram.createAccount`  
✅ **初始化 mint 账户** - 设置 6 位小数，铸造和冻结权限  
✅ **创建关联代币账户** - 为钱包创建代币持有账户  
✅ **铸造 21,000,000 个代币** - 发行到关联代币账户  
✅ **签名并发送交易** - 使用 feePayer 和 mint 账户签名  

## 🎊 结果

- **Mint 地址:** `7jRnvaCPV8qFwHgMejW1VUoAZhhYCyxFCnGpUVFB7wVw`
- **代币数量:** 21,000,000 个代币
- **交易签名:** `44BKXNbS3XVzGPxZC2ky2T1g4Tjh9iHKqc3yCFaH4NMreVHibVsPgjP19TUphgPYNLMkN53NuQwT1MkSeSdzfDCQ`

## 🔍 验证

### 在区块链浏览器查看
[Solscan 链接](https://solscan.io/tx/44BKXNbS3XVzGPxZC2ky2T1g4Tjh9iHKqc3yCFaH4NMreVHibVsPgjP19TUphgPYNLMkN53NuQwT1MkSeSdzfDCQ?cluster=devnet)

### 在 Phantom 钱包查看
1. 导入钱包（使用恢复短语）
2. 切换到 Devnet
3. 查看代币余额

## 📚 学习要点

- **Solana 账户模型** - 程序与数据分离
- **SPL Token 机制** - Mint 账户与代币账户的关系
- **交易原子性** - 多个指令打包执行
- **关联代币账户** - 确定性地址计算

## 📖 详细文档
查看 `TASK1_SUMMARY.md` 获取完整的学习总结和技术细节。
