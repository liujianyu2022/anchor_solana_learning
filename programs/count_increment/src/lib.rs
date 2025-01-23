use anchor_lang::prelude::*;

// declare_id 中的地址是由 anchor 在 init 项目的时候生成的，这个将作为部署到链上的地址
declare_id!("3QwVsMDRrF9hmQrgjFVDumXs5AHUtyy9VvQbwaJ9PF1t");

/*
    Context是 Anchor 框架中定义的一个结构体，用于封装与 Solana 程序执行相关的上下文信息，
    包含了 instruction 指令元数据以及逻辑中所需要的所有账户信息
    
    pub struct Context<'a, 'b, 'c, 'info, T: Bumps> {
        pub program_id: &'a Pubkey,                         // 当前正在执行的程序ID
        pub accounts: &'b mut T,                            // 反序列化的账户集合accounts, 指令函数的账户集合
        pub remaining_accounts: &'c [AccountInfo<'info>],   // 剩下的账户信息，但是未被反序列化或验证   包含了当前指令中未被 #[derive(Accounts)] 明确声明的账户
        pub bumps: T::Bumps,                                // 在约束验证期间找到的Bump种子
    }
*/


// #[program] 定义了包含所有指令的模块, 程序的业务逻辑代码实现都将在 #[program] 模块下完成
// #[program] 包含了所有的指令函数 在程序模块中，开发者可以定义处理不同指令的函数, 这些函数包含了具体的指令处理逻辑
#[program]
pub mod anchor_solana_learning {
    use super::*;

    // 实现了2个指令函数：initialize和increment
    // 初始化账户，并以传入的 instruction_data 作为计数器的初始值
    pub fn initialize(ctx: Context<Initialize>, instruction_data: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.counter.count = instruction_data;
        Ok(())
    }

    // 在初始值的基础上实现累加 1 操作
    // 指令函数的第一个参数ctx是必须的，而第二个参数是指令函数执行时传递的额外数据，是可选的
    pub fn increment(ctx: Context<UpdateAccount>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        msg!("previous count is: {}", counter.count);

        counter.count = counter.count.checked_add(1).unwrap();

        msg!("now count is: {}", counter.count);

        Ok(())
    }
}


// 解析和验证账户
// 由于Solana 账户模型的特点，大部分的参数将以账户集合的形式传入程序，在该宏修饰的结构体中定义了程序所需要的账户集合。
// #[derive(Accounts)] 宏应用于指令所要求的账户列表，实现了给定 struct 结构体数据的反序列化功能，
// 因此在获取账户时不再需要手动迭代账户以及反序列化操作，并且实现了账户满足程序安全运行所需要的安全检查，当然，需要#[account]宏配合使用。
#[derive(Accounts)]
pub struct Initialize<'info> {
    
    // #[account] 宏用来修饰程序所需要的自定义账户
    // 提供了一种声明式的方式来指定账户的初始化、权限、空间（占用字节数）、是否可变等属性，从而简化了与 Solana 程序交互的代码
    #[account(
        init, 
        seeds = [b"my_seed"], 
        bump,
        payer = user,                                   // 指定了支付账户，即进行账户初始化时，使用user这个账户支付交易费用

        // 指定账户的空间大小为16个字节，前 8 个字节存储 Anchor 自动添加的鉴别器，用于识别帐户类型
        // 接下来的 8 个字节为存储在Counter帐户类型中的数据分配空间（count为 u64 类型，占用 8 字节
        space = 8 + 8       
        
    )]
    pub counter: Account<'info, Counter>,               // 数据账户

    #[account(mut)]
    pub user: Signer<'info>,                            // 调用者

    pub system_program: Program<'info, System>          // 程序账户
}

#[derive(Accounts)]
pub struct UpdateAccount<'info> {

    #[account(mut)]
    pub counter: Account<'info, Counter>,

    pub user: Signer<'info>,                            // 调用者
}



// Anchor 利用 Rust 宏提供了简洁的方式来定义账户结构，它用于处理账户的**（反）序列化**、账户识别器、所有权验证。
// 给结构体实现了如下的trait：AccountSerialize, AccountDesrialize, AnchorSerialize, AnchorDeSerialize, Clone, Discriminator, Owner指定次账户数据归此program所有
// 这个宏大大简化了程序的开发过程，使开发者可以更专注于业务逻辑而不是底层的账户处理
#[account]
pub struct Counter {
    count: u64,
}