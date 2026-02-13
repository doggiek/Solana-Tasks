#!/bin/zsh

# 1️⃣ 回到工作目录
cd ~/code/doggiek/Solana-Tasks/task3 || exit

# 2️⃣ 删除旧项目目录（谨慎，会清空 task3/blueshift_anchor_escrow）
rm -rf blueshift_anchor_escrow

# 3️⃣ 安装 Anchor 推荐 nightly（避免 lockfile v4）
rustup install nightly-2025-12-01
rustup override set nightly-2025-12-01

# 4️⃣ 设置环境变量，避免新特性 lockfile 报错
export RUSTC_BOOTSTRAP=1

# 5️⃣ 初始化 Anchor 项目
anchor init blueshift_anchor_escrow --javascript

# 6️⃣ 进入项目目录
cd blueshift_anchor_escrow || exit

# 7️⃣ 删除旧 lockfile / target（保险起见）
rm -f Cargo.lock
rm -rf target
rm -rf programs/*/Cargo.lock

# 8️⃣ 生成兼容 lockfile v3
cargo generate-lockfile

# 9️⃣ 构建 Anchor 程序
anchor build

echo "✅ Anchor build 完成，如果没报错就成功了！"
