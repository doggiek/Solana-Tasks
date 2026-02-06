import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BlueshiftAnchorVault } from "../target/types/blueshift_anchor_vault";
import { expect } from "chai";

describe("blueshift_anchor_vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .blueshiftAnchorVault as Program<BlueshiftAnchorVault>;

  it("Deposit and withdraw", async () => {
    const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), program.provider.publicKey.toBuffer()],
      program.programId,
    );

    // Test deposit
    const depositAmount = 1000000; // 0.001 SOL
    const depositTx = await program.methods
      .deposit(new anchor.BN(depositAmount))
      .accounts({
        vault: vaultPda,
        signer: program.provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Deposit transaction signature", depositTx);

    // Check vault balance
    const vaultBalance = await program.provider.connection.getBalance(vaultPda);
    expect(vaultBalance).to.equal(depositAmount);

    // Test withdraw
    const withdrawTx = await program.methods
      .withdraw()
      .accounts({
        vault: vaultPda,
        signer: program.provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Withdraw transaction signature", withdrawTx);

    // Check vault is empty
    const finalVaultBalance = await program.provider.connection.getBalance(
      vaultPda,
    );
    expect(finalVaultBalance).to.equal(0);
  });
});
