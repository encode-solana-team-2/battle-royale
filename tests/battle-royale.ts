import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BattleRoyale } from "../target/types/battle_royale";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import assert = require("assert");
import * as spl from '@solana/spl-token';

describe("battle-royale", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.BattleRoyale as Program<BattleRoyale>;
  const systemProgram = anchor.web3.SystemProgram.programId
  let tokenVaultPDA;

  const LAMPORTS_PER_SOL = 1000000000;

  const BONK_MINT_ADDRESS = new PublicKey("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263")
  const WIF_MINT_ADDRESS = new PublicKey("EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm")

  it("Is initialized!", async () => {
    [tokenVaultPDA] = await anchor.web3.PublicKey.findProgramAddressSync(
      [provider.publicKey.toBuffer()],
      program.programId
    );
    const tx = await program.methods.initializeTokenVault()
      .accounts({
        tokenVault: tokenVaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([])
      .rpc()

    let vaultState = await program.account.tokenVault.fetch(tokenVaultPDA);
    expect(vaultState.bonkPrice).to.equal(0.00002308);
    expect(vaultState.wifPrice).to.equal(2.58);
  });

  it("transferSplTokens", async () => {
    // Generate keypairs for the new accounts
    const wallet = new anchor.web3.Keypair;
    const connection = provider.connection;
    const fromKp = wallet;

    // Fund the wallet with SOL
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        wallet.publicKey,
        2 * LAMPORTS_PER_SOL
      )
    );

    [tokenVaultPDA] = await anchor.web3.PublicKey.findProgramAddressSync(
      [provider.publicKey.toBuffer()],
      program.programId
    );

    // Create a new mint and initialize it
    const mintKp = new anchor.web3.Keypair();
    const mint = await spl.createMint(
      connection,
      wallet,
      fromKp.publicKey,
      null,
      0
    );

    // Create associated token accounts for the new accounts
    const fromAta = await spl.getOrCreateAssociatedTokenAccount(
      connection,
      wallet,
      mint,
      fromKp.publicKey
    );
    const toAta = await spl.getOrCreateAssociatedTokenAccount(
      connection,
      wallet,
      mint,
      tokenVaultPDA,
      true
    );

    // Mint tokens to the 'from' associated token account
    const mintAmount = 1000;
    await spl.mintTo(
      connection,
      wallet,
      mint,
      fromAta.address,
      wallet.publicKey,
      mintAmount
    );

    // Send transaction
    const transferAmount = new anchor.BN(500);
    const txHash = await program.methods
      .transferSplTokens(transferAmount)
      .accounts({
        from: fromKp.publicKey,
        fromAta: fromAta.address,
        tokenVault: toAta.address,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
      })
      .signers([wallet])
      .rpc();

    console.log(`https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
    await connection.confirmTransaction(txHash, "finalized");
    const toTokenAccount = await connection.getTokenAccountBalance(toAta.address);
    assert.strictEqual(
      toTokenAccount.value.uiAmount,
      transferAmount.toNumber(),
      "The 'to' token account should have the transferred tokens"
    );
  });
});
