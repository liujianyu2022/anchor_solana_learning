import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor"                             // Anchor提供的类型，用于表示一个Solana程序。
import { expect } from "chai";
import { PublicKey, SystemProgram } from "@solana/web3.js"              // SystemProgram：Solana的系统程序，用于创建账户和转移SOL
import { CurdApp } from "../target/types/curd_app"


describe("curd_app", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();                       // 获取当前环境的Anchor提供者（通常是本地集群或Devnet）
    anchor.setProvider(provider);                                       // 设置全局的Anchor提供者，后续的Anchor操作都会使用这个提供者

    const program = anchor.workspace.CurdApp as Program<CurdApp>        // 从Anchor工作区获取CurdApp程序的实例

    const owner = provider.wallet                                       // 当前测试的签名者（通常是测试钱包）
    const entryTitle = "Test Title";
    const entryMessage = "Test Message";
    const updatedMessage = "Updated Message";

    let entryAccount: PublicKey                                         // 用于存储生成的PDA账户地址

    before(() => {
        // Generate a PDA for the entry
        // deprecated!
        // const [entryPda, entryBump] = await PublicKey.findProgramAddress(
        //     [Buffer.from(entryTitle), owner.publicKey.toBuffer()],
        //     program.programId
        // );

        /*
        PDA（Program Derived Address，程序派生地址）是一种由 Solana 程序（智能合约）生成的特殊地址，用于存储数据或作为特定交易的身份标识。
        在 Solana 中，普通账户（如钱包地址）有一个私钥对应一个公钥，而 PDA 没有私钥，它是通过种子（seeds）和程序ID（Program ID）计算出来的，并且只能由指定的程序（合约）进行签名和管理。

        PDA 的特点
            无私钥：它无法直接被普通用户（EOA 账户）签名或操作，只能由对应的合约管理。
            基于 Seeds 生成：PDA 由一个或多个 seed（种子）和 program_id 共同计算得出。
            唯一性：同样的 seeds 和 program_id 始终生成相同的 PDA。
            可被合约签名：PDA 可以通过 invoke_signed 进行签名，用于授权特定操作。

        PDA 主要用于：
            存储合约状态数据（账户数据存储）
            作为特定用户或对象的唯一标识
            程序自动管理的账户（无须私钥的账户）
            防止恶意用户篡改数据
        */

        // 使用新的方法签名生成PDA
        // PublicKey.findProgramAddressSync(seeds: Array<Buffer | Uint8Array>, programId: PublicKey): [PublicKey, number]
        // entryPda：生成的PDA地址
        // entryBump：生成的PDA的bump值（用于确保PDA的唯一性）
        const [entryPda, entryBump] = PublicKey.findProgramAddressSync(     // 同步生成PDA（Program Derived Address）
            [
                Buffer.from(entryTitle),                                    // 将标题转换为字节数组，作为PDA的种子
                owner.publicKey.toBuffer()                                  // 将签名者的公钥转换为字节数组，作为PDA的种子
            ],
            program.programId                                               // 程序的公钥，用于生成PDA
        );

        entryAccount = entryPda
    })

    it("should create an entry", async () => {

        // Create the entry
        await program.methods
            .create(entryTitle, entryMessage)                               // 调用Anchor程序的create方法
            .accounts({                                                     // 指定方法所需的账户
                // @ts-ignore
                entry: entryAccount,                                        // 生成的PDA账户
                owner: owner.publicKey,                                     // 签名者的公钥
                systemProgram: SystemProgram.programId,                     // 系统程序的公钥
            })
            .rpc()                                                          // 发送交易并等待确认

        // Fetch the created entry
        const entry = await program.account.entryState.fetch(entryAccount)  // 从链上获取entryPda账户的数据

        // Assertions
        expect(entry.owner.toString()).to.equal(owner.publicKey.toString());
        expect(entry.title).to.equal(entryTitle);
        expect(entry.message).to.equal(entryMessage)
    });

    it("should update the entry", async () => {
        // Update the entry
        await program.methods
            .update(entryTitle, updatedMessage)
            .accounts({
                // @ts-ignore
                entry: entryAccount,
                owner: owner.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .rpc();

        // Fetch the updated entry
        const entry = await program.account.entryState.fetch(entryAccount);

        // Assertions
        expect(entry.message).to.equal(updatedMessage);
    });

    it("should delete the entry", async () => {
        // Delete the entry
        await program.methods
            .delete(entryTitle, updatedMessage)
            .accounts({
                // @ts-ignore
                entry: entryAccount,
                owner: owner.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .rpc();

        // Try to fetch the deleted entry, should throw an error
        try {
            await program.account.entryState.fetch(entryAccount);
            expect.fail("Entry should not exist after deletion");
        } catch (err) {
            expect(err).to.be.an("error");
        }
    });
});