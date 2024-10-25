import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CpiSwapProgram } from "../target/types/cpi_swap_program";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo } from "@solana/spl-token";
import { assert } from "chai";

describe("cpi-swap-program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CpiSwapProgram as Program<CpiSwapProgram>;

  const RAYDIUM_PROGRAM_ID = new PublicKey("rvkNPyHNR4FZ7CsDE1ASsMXq6XAPDWRXhx6y7DFTHG5");
  const WHIRLPOOLS_PROGRAM_ID = new PublicKey("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

  let wallet: anchor.Wallet;
  let mintA: PublicKey;
  let mintB: PublicKey;
  let sourceTokenAccount: PublicKey;
  let destinationTokenAccount: PublicKey;

  before(async () => {
    wallet = anchor.Wallet.local();

    // Airdrop SOL to wallet
    await provider.connection.requestAirdrop(wallet.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);

    // Create test tokens
    mintA = await createMint(provider.connection, wallet.payer, wallet.publicKey, null, 9);
    mintB = await createMint(provider.connection, wallet.payer, wallet.publicKey, null, 9);

    // Create token accounts for wallet
    sourceTokenAccount = await createAccount(provider.connection, wallet.payer, mintA, wallet.publicKey);
    destinationTokenAccount = await createAccount(provider.connection, wallet.payer, mintB, wallet.publicKey);

    // Mint some tokens to source account
    await mintTo(provider.connection, wallet.payer, mintA, sourceTokenAccount, wallet.payer, 1000000000);
  });

  it("Swaps on Raydium", async () => {
    const raydiumPoolAccounts = {
      ammId: new PublicKey("5quB2QDEqgPCN7zWToyqMmd95nbkAHoEjey5WuwfU91m"),
      ammAuthority: new PublicKey("ByRkQHrN96bx4dJvrMMEUbKGcF95Z8LW7NzJu2pYVnn5"),
      ammOpenOrders: new PublicKey("HtNb2BFeGuUtdUXAqjrCGnykVcsDjgqRU2sa9uRfoiB"),
      ammTargetOrders: new PublicKey("5XH27KSYZdu63sqBxLqAo4krndvHqvUuAzmtix9HmWYo"),
      poolCoinTokenAccount: new PublicKey("5GGN45Umyv7Rz7oXT7q9SJxyV6puZxm73ewVbxrV4KdP"),
      poolPcTokenAccount: new PublicKey("7XSRvZGL9sycBBURyDBVD5Cuv1AFnGeWNc7Ax9MGYUi4"),
      serumProgramId: new PublicKey("9xQeWvG816bUx9EPGXSs8Bcw9EgoLkSLHkbGeas8erNZ"),
      serumMarket: new PublicKey("9wFFmyn5k7kHqo6c9GMp2pD8U8opvswEo1VNCL78YWsp"),
      serumBids: new PublicKey("6eygT75maTSdwihJZ8v3oZyms8XDcaZ6knzCMjtjGMKp"),
      serumAsks: new PublicKey("D1e52KYChzFAxVpCBxySoTeeXRYuogRLuNRMmCuNREpM"),
      serumEventQueue: new PublicKey("5KKsLVU5jp5tWBqMMKPvj5UHzE5wksyQJkxuLMM8SVLu"),
      serumCoinVault: new PublicKey("6HfnpMUnuDwSCfgC3DaVAYVeGAkWsNokzVJV48DL6jTo"),
      serumPcVault: new PublicKey("FbNZocXymnp9n1dCd4udfgCzqF9u9XKMGYwNsiTS4W5V"),
      serumVaultSigner: new PublicKey("7ehXaNmefjYExTxSH5nzYBkgp64RSyNx2dm5RTU52LpB"),
    };

    try {
      const tx = await program.methods.swapOnRaydium(
        new anchor.BN(1000000),
        new anchor.BN(500000)
      ).accounts({
        wallet: wallet.publicKey,
        sourceTokenAccount: sourceTokenAccount,
        destinationTokenAccount: destinationTokenAccount,
        poolAccounts: raydiumPoolAccounts.ammId,
        raydiumProgram: RAYDIUM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      }).signers([wallet.payer]).rpc();

      console.log("Raydium Swap Transaction Signature:", tx);
      assert.ok(tx, "Failed to swap on Raydium");
    } catch (err) {
      console.error("Error during Raydium swap:", err);
      throw err;
    }
  });

  it("Swaps on Whirlpools", async () => {
    const whirlpoolsPoolAccounts = {
      whirlpool: new PublicKey("11111111111111111111111111111111"),
      tickArrayLower: new PublicKey("11111111111111111111111111111111"),
      tickArrayUpper: new PublicKey("11111111111111111111111111111111"),
      position: new PublicKey("11111111111111111111111111111111"),
      tokenVaultA: new PublicKey("11111111111111111111111111111111"),
      tokenVaultB: new PublicKey("11111111111111111111111111111111"),
    };

    try {
      const tx = await program.methods.swapOnWhirlpools(
        new anchor.BN(1000000),
        new anchor.BN(500000)
      ).accounts({
        wallet: wallet.publicKey,
        sourceTokenAccount: sourceTokenAccount,
        destinationTokenAccount: destinationTokenAccount,
        poolAccounts: whirlpoolsPoolAccounts.whirlpool,
        whirlpoolsProgram: WHIRLPOOLS_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      }).signers([wallet.payer]).rpc();

      console.log("Whirlpools Swap Transaction Signature:", tx);
      assert.ok(tx, "Failed to swap on Whirlpools");
    } catch (err) {
      console.error("Error during Whirlpools swap:", err);
      throw err;
    }
  });

  it("Manages whitelist", async () => {
    const addressToManage = Keypair.generate().publicKey;

    try {
      // Add to whitelist
      let tx = await program.methods.manageWhitelist(true)
        .accounts({
          wallet: wallet.publicKey,
          addressToManage: addressToManage,
          systemProgram: SystemProgram.programId
        }).signers([wallet.payer]).rpc();

      console.log("Add to Whitelist Transaction Signature:", tx);
      assert.ok(tx, "Failed to add to whitelist");

      // Remove from whitelist
      tx = await program.methods.manageWhitelist(false)
        .accounts({
          wallet: wallet.publicKey,
          addressToManage: addressToManage,
          systemProgram: SystemProgram.programId
        }).signers([wallet.payer]).rpc();

      console.log("Remove from Whitelist Transaction Signature:", tx);
      assert.ok(tx, "Failed to remove from whitelist");
    } catch (err) {
      console.error("Error managing whitelist:", err);
      throw err;
    }
  });
});