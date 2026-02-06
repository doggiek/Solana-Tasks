import dotenv from "dotenv";
dotenv.config();

import { Connection, PublicKey } from "@solana/web3.js";
import {
  getAccount,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import bs58 from "bs58";
import { Keypair } from "@solana/web3.js";

const connection = new Connection(process.env.RPC_ENDPOINT!, "confirmed");
const mintAddress = new PublicKey(
  "7jRnvaCPV8qFwHgMejW1VUoAZhhYCyxFCnGpUVFB7wVw",
);

async function checkBalance() {
  try {
    const feePayerKeypair = Keypair.fromSecretKey(
      bs58.decode(process.env.SECRET!),
    );

    const tokenAccount = getAssociatedTokenAddressSync(
      mintAddress,
      feePayerKeypair.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID,
    );

    console.log("Your Token Account:", tokenAccount.toBase58());

    // 检查余额
    const accountInfo = await getAccount(connection, tokenAccount);
    console.log("Token Balance:", accountInfo.amount.toString());
    console.log(
      "Formatted Balance:",
      (Number(accountInfo.amount) / Math.pow(10, 6)).toLocaleString(),
      "tokens",
    );
  } catch (error) {
    console.error("Error:", error);
  }
}

checkBalance();
