use anchor_lang::prelude::*;

declare_id!("6vnHJQpQL1G8x9DUTjAfsegeqwf2uPd56eNisEvnuoRn");

#[program]
pub mod curd_app {

    use super::*;

    pub fn create(context: Context<CreateEntry>, title: String, message: String) -> Result<()> {
        msg!("create title = {}", title);  
        msg!("create message = {}", message);

        let entry = &mut context.accounts.entry;
        entry.owner = context.accounts.owner.key();
        entry.title = title;
        entry.message = message;
        Ok(())
    }

    pub fn update(context: Context<UpdateEntry>, title: String, message: String) -> Result<()> {
        msg!("update title = {}", title);  
        msg!("update message = {}", message);

        let entry = &mut context.accounts.entry;
        entry.message = message;

        Ok(())
    }

    pub fn delete(_context: Context<DeleteEntry>, title: String, message: String) -> Result<()> {
        msg!("delete title = {}", title);  
        msg!("delete message = {}", message);
        Ok(())
    }
}

#[account]                  // 用于定义账户的结构和属性
#[derive(InitSpace)]
pub struct EntryState {
    pub owner: Pubkey,

    #[max_len(50)]
    pub title: String,

    #[max_len(1000)]
    pub message: String
}

#[derive(Accounts)]         // 用于派生一个结构体，该结构体包含与 Solana 程序交互所需的所有账户
#[instruction(title: String, message: String)]
pub struct CreateEntry<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + EntryState::INIT_SPACE,

        // 使用 title 和 owner 作为 seeds 生成唯一的 PDA
        seeds = [title.as_bytes(), owner.key().as_ref()],

        // 自动计算一个 bump 值，确保 PDA 地址不会与普通公钥冲突
        bump
    )]
    pub entry: Account<'info, EntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>
}


#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateEntry<'info> {
    #[account(
        mut,

        // 在更新账户时调整账户的大小, 重新分配账户空间
        // 8 代表 Solana 账户的元数据所占的空间（即账户的基本存储需求）
        // EntryState::INIT_SPACE 是 EntryState 结构体所需的存储空间大小
        realloc = 8 + EntryState::INIT_SPACE, 

        // 指定谁来支付账户重新分配空间的费用
        // Solana 上的账户创建和存储空间调整都需要支付租金（Lamports）
        // 由 owner 账户支付重新分配存储空间所需的费用
        realloc::payer = owner,

        // 决定新分配的存储空间是否初始化为 0, 确保不会含有垃圾数据。
        // 免账户中存在未初始化的数据，防止数据读取时出现未定义行为
        realloc::zero = true,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
    )]
    pub entry: Account<'info, EntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {
    #[account(
        mut,

        // 当 entry 账户被删除时，剩余的 SOL 余额会转移到 owner 账户, 关闭账户时，资金转移到 owner
        // 在 Solana 中，账户不能直接删除，但可以通过 close 将账户的剩余 SOL 退还给指定的接收者，并释放链上的存储空间。
        // close = owner 表示 entry 账户被关闭后，账户中的剩余 lamports（Solana 的最小单位）将转移到 owner 账户
        close = owner,

        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub entry: Account<'info, EntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>
}
