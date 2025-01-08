use anchor_lang::prelude::*;

// declare_id 中的地址是由 anchor 在 init 项目的时候生成的，这个将作为部署到链上的地址
declare_id!("DJtPN7Rjgt3gdwhtmKwAQzPWzeTamT8rPZoKJQ1nPrBK");

/*
    pub struct Context<'a, 'b, 'c, 'info, T: Bumps> {
        pub program_id: &'a Pubkey,                         // 当前正在执行的程序ID
        pub accounts: &'b mut T,                            // 反序列化的账户
        pub remaining_accounts: &'c [AccountInfo<'info>],   // 剩下的账户信息，但是为被反序列化或验证
        pub bumps: T::Bumps,                                // 在约束验证期间找到的Bump种子
    }
*/


// #[program] 定义了包含所有指令的模块
#[program]
pub mod anchor_solana_learning {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, instruction_data: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.counter.count = instruction_data;
        Ok(())
    }

    pub fn increment(ctx: Context<UpdateAccount>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        msg!("previous count is: {}", counter.count);

        counter.count = counter.count.checked_add(1).unwrap();

        msg!("now count is: {}", counter.count);

        Ok(())
    }
}


// 解析和验证账户
#[derive(Accounts)]
pub struct Initialize<'info> {
    
    #[account(
        init, 
        seeds = [b"my_seed"], 
        bump,
        payer = user, 
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
