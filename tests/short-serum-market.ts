import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import { createMint, mintTo } from "@solana/spl-token";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { expect } from "chai";
import { ShortSerumMarket } from '../target/types/short_serum_market';
const { genesis, sleep } = require("./utils");


const CORE_STATE_SEED: string = "core-state";
const EXPIRY_PERIOD_DAYS: number = 30;

describe('short-serum-market', () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ShortSerumMarket as Program<ShortSerumMarket>;
  const admin = Keypair.generate();
  let marketProxy, tokenAccount, usdcAccount;
  let usdcClient;

  it('Is initialized!', async () => {
    // airdrop to admin account
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        admin.publicKey,
        10000000000
      ),
      "confirmed"
    );

    // get coreState from PDA
    const [coreState, coreStateNonce] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode(CORE_STATE_SEED)),
        admin.publicKey.toBuffer()
      ],
      program.programId
    );

    // initialize
    const tx = await program.rpc.initialize({
      coreStateNonce,
      expiryPeriodDays: new anchor.BN(EXPIRY_PERIOD_DAYS)
    }, {
      accounts: {
        admin: admin.publicKey,
        coreState,
        systemProgram: SystemProgram.programId
      },
      signers: [admin]
    });
    
    console.log("Your transaction signature", tx);
  });

  it("Get marketProxyClient", async () => {
    const { marketProxyClient, godA, godUsdc, usdc } = await genesis({
      provider,
      proxyProgramId: program.programId,
    });
    marketProxy = marketProxyClient;
    usdcAccount = godUsdc;
    tokenAccount = godA;

    console.log(usdc);

    // usdcClient = new Token(
    //   provider.connection,
    //   usdc,
    //   TOKEN_PROGRAM_ID,
    //   provider.wallet.payer
    // );

    // xtokenMint = await createMint(
    //   provider.connection,
    //   admin,
    //   tokenMintAuthority.publicKey,
    //   tokenMintAuthority.publicKey,
    //   9
    // );

    // referral = await usdcClient.createAccount(REFERRAL_AUTHORITY);
  });
});
