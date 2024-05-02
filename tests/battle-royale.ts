import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BattleRoyale } from "../target/types/battle_royale";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";

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
});
