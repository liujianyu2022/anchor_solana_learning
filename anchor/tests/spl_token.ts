import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { SplTokenProgram } from "../target/types/spl_token_program";
import { PublicKey, SystemProgram, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount, mintTo, transfer, AccountLayout } from "@solana/spl-token";
import { expect } from "chai"

describe("spl_token_program", () => {
  // 配置 Anchor 运行环境
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const program = anchor.workspace.SplTokenProgram as Program<SplTokenProgram>;

  let mint: PublicKey;
  let mintAuthority: Keypair;
  let payer: Keypair;
  let user1: Keypair;
  let user2: Keypair;
  let tokenAccount1: PublicKey;
  let tokenAccount2: PublicKey;
  let decimals = 6;
  let initialSupply = 1000;

  before(async () => {
    // 生成密钥对
    mintAuthority = Keypair.generate();
    payer = Keypair.generate();
    user1 = Keypair.generate();
    user2 = Keypair.generate();

    // 资金账户
    const airdropSignature = await provider.connection.requestAirdrop(
      payer.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSignature);

    // 创建 Mint
    mint = await createMint(
      provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      decimals
    );

    // 获取用户的 Token 账户
    tokenAccount1 = (await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mint,
      user1.publicKey
    )).address;

    tokenAccount2 = (await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mint,
      user2.publicKey
    )).address;

    // 铸造初始供应量到 user1
    await mintTo(
      provider.connection,
      payer,
      mint,
      tokenAccount1,
      mintAuthority,
      initialSupply * 10 ** decimals
    );
  });

  it("checks initial balance", async () => {
    const accountInfo = await provider.connection.getParsedAccountInfo(tokenAccount1);
    const balance = (accountInfo.value?.data as any).parsed.info.tokenAmount.uiAmount;

    console.log("Initial Supply Balance = ", balance)

    expect(balance).to.equal(initialSupply)
  });

  it("transfers tokens", async () => {
    const amount = 100 * 10 ** decimals;
    await transfer(
      provider.connection,
      payer,
      tokenAccount1,
      tokenAccount2,
      user1,
      amount
    );

    const accountInfo1 = await provider.connection.getParsedAccountInfo(tokenAccount1);
    const balance1 = (accountInfo1.value?.data as any).parsed.info.tokenAmount.uiAmount;
    const accountInfo2 = await provider.connection.getParsedAccountInfo(tokenAccount2);
    const balance2 = (accountInfo2.value?.data as any).parsed.info.tokenAmount.uiAmount;

    console.log("Balance after transfer: User1:", balance1, "User2:", balance2);

    expect(balance1).to.equal(initialSupply - 100)
    expect(balance2).to.equal(100)

  });

  // it("approves another account", async () => {
  //   const tx = await program.methods.approve(0) // MintTokens 权限
  //     .accounts({
  //       account: tokenAccount1,
  //       // account: mint,
  //       owner: user1.publicKey,
  //       delegate: user2.publicKey,
  //     })
  //     .signers([user1, user2])
  //     .rpc();

  //   console.log("Approve transaction = ", tx);
  // });

  // it("freezes an account", async () => {
  //   const tx = await program.methods.freezeAccount()
  //     .accounts({
  //       account: tokenAccount2,
  //       mint,
  //       authority: user1.publicKey,
  //     })
  //     .signers([user1])
  //     .rpc();

  //   console.log("Freeze account transaction:", tx);
  // });

  // it("closes an account", async () => {
  //   const tx = await program.methods.closeAccount()
  //     .accounts({
  //       account: tokenAccount2,
  //       destination: user1.publicKey,
  //       authority: user1.publicKey,
  //     })
  //     .signers([user1])
  //     .rpc();

  //   console.log("Close account transaction:", tx);
  // });
});