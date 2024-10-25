import * as anchor from "@project-serum/anchor";
import {Program} from "@project-serum/anchor";
import {CpiSwapProgram} from "../target/types/cpi_swap_program";
import {PublicKey,Keypair,SystemProgram} from "@solana/web3.js";
import {TOKEN_PROGRAM_ID,createMint,createAccount,mintTo,getAccount} from "@solana/spl-token";
import {assert} from "chai";

describe("cpi-swap-program",() => {
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
  let whitelistAccount: Keypair;

  before(async () => {
    wallet = anchor.Wallet.local();
    whitelistAccount = Keypair.generate();

    // Airdrop SOL to wallet
    await provider.connection.requestAirdrop(wallet.publicKey,10 * anchor.web3.LAMPORTS_PER_SOL);

    // Create test tokens
    mintA = await createMint(provider.connection,wallet.payer,wallet.publicKey,null,9);
    mintB = await createMint(provider.connection,wallet.payer,wallet.publicKey,null,9);

    // Create token accounts for wallet
    sourceTokenAccount = await createAccount(provider.connection,wallet.payer,mintA,wallet.publicKey);
    destinationTokenAccount = await createAccount(provider.connection,wallet.payer,mintB,wallet.publicKey);

    // Mint some tokens to source account
    await mintTo(provider.connection,wallet.payer,mintA,sourceTokenAccount,wallet.payer,1000000000);
  });

  it("Initializes the program",async () => {
    try {
      const tx = await program.methods.initialize()
        .accounts({
          whitelist: whitelistAccount.publicKey,
          authority: wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([whitelistAccount])
        .rpc();

      console.log("Initialization Transaction Signature:",tx);
      assert.ok(tx,"Failed to initialize the program");

      const whitelistState = await program.account.whitelist.fetch(whitelistAccount.publicKey);
      assert.equal(whitelistState.authority.toBase58(),wallet.publicKey.toBase58(),"Authority not set correctly");
      assert.equal(whitelistState.users.length,0,"Whitelist should be empty initially");
    } catch(err) {
      console.error("Error during initialization:",err);
      throw err;
    }
  });

  it("Manages whitelist",async () => {
    const addressToManage = Keypair.generate().publicKey;

    try {
      // Add to whitelist
      let tx = await program.methods.manageWhitelist(true)
        .accounts({
          authority: wallet.publicKey,
          whitelist: whitelistAccount.publicKey,
          addressToManage: addressToManage,
        }).signers([wallet.payer]).rpc();

      console.log("Add to Whitelist Transaction Signature:",tx);
      assert.ok(tx,"Failed to add to whitelist");

      let whitelistState = await program.account.whitelist.fetch(whitelistAccount.publicKey);
      assert.include(whitelistState.users.map(pub => pub.toBase58()),addressToManage.toBase58(),"Address not added to whitelist");

      // Remove from whitelist
      tx = await program.methods.manageWhitelist(false)
        .accounts({
          authority: wallet.publicKey,
          whitelist: whitelistAccount.publicKey,
          addressToManage: addressToManage,
        }).signers([wallet.payer]).rpc();

      console.log("Remove from Whitelist Transaction Signature:",tx);
      assert.ok(tx,"Failed to remove from whitelist");

      whitelistState = await program.account.whitelist.fetch(whitelistAccount.publicKey);
      assert.notInclude(whitelistState.users.map(pub => pub.toBase58()),addressToManage.toBase58(),"Address not removed from whitelist");
    } catch(err) {
      console.error("Error managing whitelist:",err);
      throw err;
    }
  });

  it("Swaps on Raydium",async () => {
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
      // Add wallet to whitelist first
      await program.methods.manageWhitelist(true)
        .accounts({
          authority: wallet.publicKey,
          whitelist: whitelistAccount.publicKey,
          addressToManage: wallet.publicKey,
        }).signers([wallet.payer]).rpc();

      const balanceBefore = await getAccount(provider.connection,sourceTokenAccount);

      const tx = await program.methods.swapOnRaydium(
        new anchor.BN(1000000),
        new anchor.BN(500000)
      ).accounts({
        user: wallet.publicKey,
        whitelist: whitelistAccount.publicKey,
        sourceTokenAccount: sourceTokenAccount,
        destinationTokenAccount: destinationTokenAccount,
        poolAccounts: raydiumPoolAccounts.ammId,
        raydiumProgram: RAYDIUM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      }).signers([wallet.payer]).rpc();

      console.log("Raydium Swap Transaction Signature:",tx);
      assert.ok(tx,"Failed to swap on Raydium");

      const balanceAfter = await getAccount(provider.connection,sourceTokenAccount);
      assert(balanceAfter.amount < balanceBefore.amount,"Token balance did not decrease after swap");
    } catch(err) {
      console.error("Error during Raydium swap:",err);
      throw err;
    }
  });

  it("Swaps on Whirlpools",async () => {
    const whirlpoolsPoolAccounts = {
      whirlpool: new PublicKey("11111111111111111111111111111111"),
      tickArrayLower: new PublicKey("11111111111111111111111111111111"),
      tickArrayUpper: new PublicKey("11111111111111111111111111111111"),
      oracle: new PublicKey("11111111111111111111111111111111"),
    };

    try {
      const balanceBefore = await getAccount(provider.connection,sourceTokenAccount);

      const tx = await program.methods.swapOnWhirlpools(
        new anchor.BN(1000000),
        new anchor.BN(500000),
        new anchor.BN(0), // sqrt_price_limit
        true, // amount_specified_is_input
        true // a_to_b
      ).accounts({
        user: wallet.publicKey,
        whitelist: whitelistAccount.publicKey,
        tokenOwnerAccountA: sourceTokenAccount,
        tokenOwnerAccountB: destinationTokenAccount,
        tokenVaultA: whirlpoolsPoolAccounts.whirlpool,
        tokenVaultB: whirlpoolsPoolAccounts.whirlpool,
        tickArray0: whirlpoolsPoolAccounts.tickArrayLower,
        tickArray1: whirlpoolsPoolAccounts.tickArrayUpper,
        tickArray2: whirlpoolsPoolAccounts.tickArrayUpper,
        oracle: whirlpoolsPoolAccounts.oracle,
        whirlpoolsProgram: WHIRLPOOLS_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
      }).signers([wallet.payer]).rpc();

      console.log("Whirlpools Swap Transaction Signature:",tx);
      assert.ok(tx,"Failed to swap on Whirlpools");

      const balanceAfter = await getAccount(provider.connection,sourceTokenAccount);
      assert(balanceAfter.amount < balanceBefore.amount,"Token balance did not decrease after swap");
    } catch(err) {
      console.error("Error during Whirlpools swap:",err);
      throw err;
    }
  });
});