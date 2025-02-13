use anchor_lang::prelude::*;
use anchor_spl::token::spl_token::instruction::AuthorityType;
use anchor_spl::token::{
    self, CloseAccount as TokenCloseAccount, FreezeAccount as TokenFreezeAccount, InitializeMint,
    Mint, MintTo, SetAuthority, Token, TokenAccount, Transfer,
};

declare_id!("7VE1KT8XXssnwBcYA3eZbmi4ZCkVPWzQZam63ZDQfQQn");

#[program]
pub mod spl_token_program {
    use super::*;

    // 创建新的代币
    pub fn create_token(
        ctx: Context<CreateToken>,          // CreateToken 是一个结构体，包含与创建代币相关的账户信息
        decimals: u8, 
        initial_supply: u64
    ) -> Result<()> {

        // 代表该代币的 mint 账户，代币的所有元数据（例如符号、名字、权限等）都保存在这个账户中。
        let mint = &ctx.accounts.mint;    

        // 一个用户或钱包账户，用于接收代币的初始供应量
        let initial_supply_account = &ctx.accounts.initial_supply_account;


        let rent = &ctx.accounts.rent;

        // 支付交易费用的账户，也是铸造和冻结代币的权限持有者
        let payer = &ctx.accounts.payer;                               
        
        // 表示 SPL Token 程序的账户，用于执行与代币相关的操作
        let token_program = &ctx.accounts.token_program;       

        
        let cpi_context1 = CpiContext::new(
            token_program.to_account_info(),
            InitializeMint {                                               // 包含 mint 账户和 rent 账户的结构体
                mint: mint.to_account_info(),
                rent: rent.to_account_info(),
            },
        );

        // 这个操作会将代币的 mint 账户初始化并设置为可以铸造和冻结代币
        token::initialize_mint(
            cpi_context1,                                                        // 上下文对象，用于提供程序间调用的账户信息
            decimals,
            &payer.key(),                                                 // 指定代币的 mint_authority 和 freeze_authority（即 payer）的公钥
            Some(&payer.key()),
        )?;

        // 计算初始供应量
        // initial_supply * 10^decimals
        // checked_mul 用于进行乘法运算并检查溢出，如果发生溢出，则返回 None，并触发 ProgramError::InvalidArgument 错误。
        let initial_supply_tokens = initial_supply
            .checked_mul(10u64.pow(decimals as u32))
            .ok_or(ProgramError::InvalidArgument)?;


        let cpi_content2 = CpiContext::new(
            token_program.to_account_info(),
            MintTo {
                mint: mint.to_account_info(),                                       // mint 账户
                to: initial_supply_account.to_account_info(),                       // 目标账户
                authority: payer.to_account_info(),
            },
        );

        // 铸造初始代币
        // 这个操作会把 initial_supply_tokens 代币铸造到 initial_supply_account 账户中
        token::mint_to(
            cpi_content2,
            initial_supply_tokens,
        )?;

        Ok(())
    }

    // 转账代币
    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );

        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }

    // 查询余额（仅示例，Solana 账户状态本身是公开的）
    pub fn get_balance(ctx: Context<GetBalance>) -> Result<u64> {
        Ok(ctx.accounts.account.amount)
    }

    // 授权其他账户
    pub fn approve(ctx: Context<Approve>, authority_type: u8) -> Result<()> {
        // 将 u8 转换为 AuthorityType
        let authority_type = match authority_type {
            0 => AuthorityType::MintTokens,
            1 => AuthorityType::FreezeAccount,
            2 => AuthorityType::AccountOwner,
            3 => AuthorityType::CloseAccount,
            _ => return Err(ProgramError::InvalidArgument.into()),
        };
    
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SetAuthority {
                account_or_mint: ctx.accounts.account.to_account_info(),
                current_authority: ctx.accounts.owner.to_account_info(),
            },
        );
    
        // 这里正确传递了 `new_authority`
        token::set_authority(
            cpi_ctx, 
            authority_type, 
            Some(ctx.accounts.delegate.key())
        )?;
    
        Ok(())
    }

    // 冻结账户
    pub fn freeze_account(ctx: Context<FreezeAccountCtx>) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TokenFreezeAccount {
                account: ctx.accounts.account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );
        token::freeze_account(cpi_ctx)?;
        Ok(())
    }

    // 关闭账户
    pub fn close_account(ctx: Context<CloseAccountCtx>) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TokenCloseAccount {
                account: ctx.accounts.account.to_account_info(),
                destination: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );
        token::close_account(cpi_ctx)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    // mint 账户是代币的元数据账户，它存储了代币的基本信息，如名称、符号、mint_authority（铸币权限）以及 freeze_authority（冻结权限）。
    // min 账户并不持有任何代币的余额，而是负责管理代币的元数据和管理代币的铸造和冻结操作。
    #[account(init, payer = payer, space = 82)]
    pub mint: Account<'info, Mint>,

    // initial_supply_account 是一个用户或钱包账户，用于接收代币的初始供应量。
    // 在代币创建过程中，初始供应量需要铸造并发送到一个实际的持有账户，这样用户才能看到他们拥有的代币数量
    #[account(mut)]
    pub initial_supply_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,

    #[account(mut)]
    pub to: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct GetBalance<'info> {
    #[account(mut)]
    pub account: Account<'info, TokenAccount>,
}

// #[derive(Accounts)]
// pub struct Approve<'info> {
//     #[account(mut)]
//     pub account: Account<'info, TokenAccount>,
//     pub owner: Signer<'info>,
//     pub delegate: AccountInfo<'info>,
//     pub token_program: Program<'info, Token>,
// }

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(mut)]
    pub account: Account<'info, TokenAccount>,          // 需要修改权限的账户
    pub owner: Signer<'info>,                           // 当前账户的拥有者
    pub delegate: Signer<'info>,                        // 被授权的新账户
    pub token_program: Program<'info, Token>,           // SPL Token 程序
}

#[derive(Accounts)]
pub struct FreezeAccountCtx<'info> {
    #[account(mut)]
    pub account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

// Account<'info, TokenAccount> 类型的字段会自动进行类型检查，而 AccountInfo<'info> 类型的字段不会，因此 Anchor 需要额外的安全检查。
// destination 是 close_account 指令的目标账户，在关闭账户时接收资金，它通常是一个外部账户，并不会由 Anchor 自动检查其类型。
// 通过 /// CHECK: 注释，开发者向 Anchor 明确声明这个字段的用途，并表明在逻辑上是安全的。

#[derive(Accounts)]
pub struct CloseAccountCtx<'info> {
    #[account(mut)]
    pub account: Account<'info, TokenAccount>,

    /// CHECK: 目标账户用于接收关闭账户的资金，调用者应确保该账户的正确性
    #[account(mut)]
    pub destination: AccountInfo<'info>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}


// anchor deploy -p spl_token
// anchor deploy -p spl_token --provider.cluster https://young-restless-bridge.solana-devnet.quiknode.pro/64df14141046ed00f4320d627db7e1119aef0b52  强制指定 RPC