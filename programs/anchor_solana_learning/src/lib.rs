use anchor_lang::prelude::*;

declare_id!("DJtPN7Rjgt3gdwhtmKwAQzPWzeTamT8rPZoKJQ1nPrBK");

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
// 这个宏大大简化了程序的开发过程，使开发者可以更专注于业务逻辑而不是底层的账户处理
#[account]
pub struct Counter {
    count: u64,
}
