/** Challenge: Mint an SPL Token
 *
 * In this challenge, you will create an SPL token!
 *
 * Goal:
 *   Mint an SPL token in a single transaction using Web3.js and the SPL Token library.
 *
 * Objectives:
 *   1. Create an SPL mint account. 创建代币铸造账户
 *   2. Initialize the mint with 6 decimals and your public key (feePayer) as the mint and freeze authorities. 初始化代币铸造账户
 *   3. Create an associated token account for your public key (feePayer) to hold the minted tokens. 创建代币账户
 *   4. Mint 21,000,000 tokens to your associated token account. 发行代币
 *   5. Sign and send the transaction. 签名并发送交易
 */

import dotenv from "dotenv";
dotenv.config();

import {
  Keypair,
  Connection,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";

import {
  createAssociatedTokenAccountInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  createMintToCheckedInstruction,
  MINT_SIZE,
  getMinimumBalanceForRentExemptMint,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

import bs58 from "bs58";

// Import our keypair from the wallet file
const feePayer = Keypair.fromSecretKey(
  // ⚠️ INSECURE KEY. DO NOT USE OUTSIDE OF THIS CHALLENGE
  bs58.decode(process.env.SECRET!),
);

//Create a connection to the RPC endpoint
const connection = new Connection(process.env.RPC_ENDPOINT!, "confirmed");

// Entry point of your TypeScript code (we will call this)
async function main() {
  try {
    // Generate a new keypair for the mint account
    const mint = Keypair.generate();

    const mintRent = await getMinimumBalanceForRentExemptMint(connection);

    // START HERE

    // 1. Create the mint account -- 创建空账户
    const createAccountIx = SystemProgram.createAccount({
      fromPubkey: feePayer.publicKey,
      newAccountPubkey: mint.publicKey,
      lamports: mintRent,
      space: MINT_SIZE,
      programId: TOKEN_PROGRAM_ID,
    });

    // 2. Initialize the mint account -- 初始化为mint账户
    // Set decimals to 6, and the mint and freeze authorities to the fee payer (you).
    const initializeMintIx = createInitializeMint2Instruction(
      mint.publicKey, // mint pubkey
      6, // decimals
      feePayer.publicKey, // mint authority
      feePayer.publicKey, // freeze authority  -- 这个不对，要可以冻结
      TOKEN_PROGRAM_ID,
    );

    // 3. Create the associated token account -- 创建代币账户(持有者)
    const associatedTokenAccount = getAssociatedTokenAddressSync(
      mint.publicKey, // 代币mint地址（中央银行）
      feePayer.publicKey, // 账户所有者（你的钱包）
      false, // 不允许非椭圆曲线所有者
      TOKEN_PROGRAM_ID, // 标准代币程序ID
      ASSOCIATED_TOKEN_PROGRAM_ID, // 关联代币程序ID
    );
    const createAssociatedTokenAccountIx =
      createAssociatedTokenAccountInstruction(
        feePayer.publicKey, // 付钱创建账户的人
        associatedTokenAccount, // 要创建的代币账户地址
        feePayer.publicKey, // 账户所有者
        mint.publicKey, // 代币mint地址
        TOKEN_PROGRAM_ID, // 代币程序ID
        ASSOCIATED_TOKEN_PROGRAM_ID, // 关联代币程序ID
      );

    // 4. Mint 21,000,000 tokens to the associated token account -- 铸造代币，并分配代币全给我
    const mintAmount = 21_000_000 * Math.pow(10, 6); // 21 million tokens with 6 decimals
    const mintToCheckedIx = createMintToCheckedInstruction(
      mint.publicKey, // 代币mint地址（中央银行）
      associatedTokenAccount, // 接收代币的账户地址
      feePayer.publicKey, // 有铸造权限的账户
      mintAmount, // 要铸造的数量（考虑小数位）
      6, // 代币的小数位数
    );

    // Get the latest blockhash -- 获取最新区块哈希
    const recentBlockhash = await connection.getLatestBlockhash();

    // 构建交易
    const transaction = new Transaction({
      feePayer: feePayer.publicKey,
      blockhash: recentBlockhash.blockhash,
      lastValidBlockHeight: recentBlockhash.lastValidBlockHeight,
    }).add(
      createAccountIx,
      initializeMintIx,
      createAssociatedTokenAccountIx,
      mintToCheckedIx,
    );

    // 发送交易
    const transactionSignature = await sendAndConfirmTransaction(
      connection,
      transaction,
      [feePayer, mint], // feePayer 付手续费，mint 账户需要签名授权创建
    );

    console.log("Mint Address:", mint.publicKey.toBase58());
    console.log("Transaction Signature:", transactionSignature);
  } catch (error) {
    console.error(`Oops, something went wrong: ${error}`);
  }
}

// 调用主函数
main();
