use anchor_lang::{
    prelude::*, 
    solana_program
};

use solana_program::clock::Clock;           // 获取到当前时间的时间戳

declare_id!("53huonbTydKqUQ6RSFgXXZGnc4HjFYAJHgWrabJpVLFj");

#[program]
pub mod guess_number {
    use super::*;
}

fn generate_random_number() -> u32 {
    let clock = Clock::get().expect("get clock failed");
    let last_digit = (clock.unix_timestamp % 10) as u8;
    let random_number = (last_digit + 1) as u32;                        // 范围在 1～10 之间
    return random_number;
}


// 定义程序账户结构体，它用于管理程序交互过程中的账户状态
// #[derive(Accounts)] 这个派生宏，在获取账户时不再需要手动迭代账户以及反序列化操作，并且实现了账户满足程序安全运行所需要的安全检查。
#[derive(Accounts)]
pub struct AccountContext<'info> {
    // 使用 #[account] 宏，用来配置了 PDA 账户的各种属性，如初始化方式、占用的空间大小和付款账户等
    #[account(

        // 通知 Anchor 在需要时自动初始化一个派生账户地址 PDA。如果账户尚未初始化，Anchor 会根据提供的其他参数（如 space 和 payer ）来初始化它
        // you need to update the following configuration anchor-lang = { version = "0.30.1", features = ["init-if-needed"] } at the Cargo.toml
        init_if_needed,   

        // 前 8 个字节为账户类型识别器，用于识别帐户类型，这样 Anchor 就能对账户进行（反）系列化
        // 接下来的 4 个字节为存储在 GuessingAccount 帐户类型中的数据分配空间（ number 为 u32 类型，占用 4 字节）
        space=8+4,
        payer=payer,
        seeds = [b"guessing pda"],
        bump
    )]
    
    pub guessing_account: Account<'info, GuessingAccount>,      // 'info：是一个生命周期参数，让这个账户引用在整个结构体生命周期内都是有效的

    #[account(mut)]                                 // 表示 payer 是一个可变的账户引用，这是因为执行合约时可能会修改账户状态（例如，扣除手续费）
    pub payer: Signer<'info>,                       // payer 是 Signer 类型，表示对该笔交易进行签名的账户

    pub system_program: Program<'info, System>,     // 表示 Solana 系统程序的引用，它提供了执行合约所需的一些基础功能
}


// Solana 作为一个分布式区块链系统，所有的信息都存储在账户中，如程序代码、状态信息、Token数据、配置信息等都是存储在一个个账户中
// 定义记录数据的结构体，也需要用 #[account] 标记为 Solana 的账户类型，这样就可以在链上存储游戏要记录的数字
// #[account] 将结构体定义为账户类型，使得结构体能够映射到区块链上的一个账户，存储所需的状态信息，并通过合约中的函数进行访问和修改，
// 同时自动处理数据的序列化、反序列化和验证
#[account]
pub struct GuessingAccount {
    pub random_number: u32
}