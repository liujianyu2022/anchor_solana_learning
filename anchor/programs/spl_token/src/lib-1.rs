// use anchor_lang::prelude::*;
// use anchor_spl::token::{
//     self, CloseAccount, FreezeAccount, Mint, SetAuthority, Token, TokenAccount,
//     Transfer,
// };
// use spl_token::instruction::AuthorityType;

// declare_id!("7VE1KT8XXssnwBcYA3eZbmi4ZCkVPWzQZam63ZDQfQQn");


// #[program]
// pub mod spl_token_program {
//     use super::*;

//     // 创建新的代币
//     pub fn create_token(
//         ctx: Context<CreateToken>,
//         decimals: u8,
//         initial_supply: u64,
//     ) -> Result<()> {
//         let mint = &ctx.accounts.mint;
//         let token_program = &ctx.accounts.token_program;
//         let payer = &ctx.accounts.payer;
//         let rent_account = ctx.accounts.rent.to_account_info();

//         token::initialize_mint(
//             CpiContext::new(
//                 token_program.to_account_info(), 
//                 anchor_spl::token::InitializeMint {
//                     mint: mint.to_account_info(),
//                     rent: rent_account,
//                 },
//             ),
//             decimals,
//             &payer.key(),
//             Some(&payer.key()),
//         )?;

//         // 初始化 Mint 账户
//         // token::initialize_mint(
//         //     token_program,
//         //     decimals,
//         //     &payer.key(),
//         //     Some(&payer.key()),
//         // )?;

//         // 创建初始供应量
//         let initial_supply_tokens = initial_supply * 10u64.pow(decimals as u32);

//         token::mint_to(
//             token_program,
//             &initial_supply_tokens
//         )?;


//         Ok(())
//     }

//     // 转账代币
//     pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {

//         let transfer_instruction = Transfer {
//             from: ctx.accounts.from.to_account_info(),
//             to: ctx.accounts.to.to_account_info(),
//             authority: ctx.accounts.authority.to_account_info(),
//         };

//         let cpi_ctx = CpiContext::new(
//             ctx.accounts.token_program.to_account_info(),
//             transfer_instruction,
//         );

//         token::transfer(cpi_ctx, amount)?;
//         Ok(())
//     }

//     // 查询余额
//     pub fn get_balance(ctx: Context<GetBalance>) -> Result<()> {
//         let account = &ctx.accounts.account;
//         let balance = account.amount;
//         // 这里可以将余额记录到日志或返回给客户端
//         Ok(())
//     }

//     // 授权其他账户
//     pub fn approve(ctx: Context<Approve>, amount: u64) -> Result<()> {
//         let approve_instruction = SetAuthority {
//             account_or_mint: ctx.accounts.account.to_account_info(),
//             current_authority: ctx.accounts.owner.to_account_info(),
//             new_authority: Some(ctx.accounts.delegate.to_account_info()),
//             authority_type: AuthorityType::AccountOwner,
//         };

//         let cpi_ctx = CpiContext::new(
//             ctx.accounts.token_program.to_account_info(),
//             approve_instruction,
//         );

//         token::set_authority(cpi_ctx, amount)?;
//         Ok(())
//     }

//     // 冻结账户
//     pub fn freeze_account(ctx: Context<FreezeAccount>) -> Result<()> {
//         let freeze_instruction = FreezeAccount {
//             account: ctx.accounts.account.to_account_info(),
//             mint: ctx.accounts.mint.to_account_info(),
//             authority: ctx.accounts.authority.to_account_info(),
//         };

//         let cpi_ctx = CpiContext::new(
//             ctx.accounts.token_program.to_account_info(),
//             freeze_instruction,
//         );

//         token::freeze_account(cpi_ctx)?;
//         Ok(())
//     }

//     // 关闭账户
//     pub fn close_account(ctx: Context<CloseAccount>) -> Result<()> {
//         let close_instruction = CloseAccount {
//             account: ctx.accounts.account.to_account_info(),
//             destination: ctx.accounts.destination.to_account_info(),
//             authority: ctx.accounts.authority.to_account_info(),
//         };

//         let cpi_ctx = CpiContext::new(
//             ctx.accounts.token_program.to_account_info(),
//             close_instruction,
//         );

//         token::close_account(cpi_ctx)?;
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct CreateToken<'info> {
//     #[account(init, payer = payer, space = 82)]
//     pub mint: Account<'info, Mint>,
//     #[account(mut)]
//     pub initial_supply_account: Account<'info, TokenAccount>,
//     pub token_program: Program<'info, Token>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct TransferTokens<'info> {
//     #[account(mut)]
//     pub from: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub to: Account<'info, TokenAccount>,
//     pub authority: Signer<'info>,
//     pub token_program: Program<'info, Token>,
// }

// #[derive(Accounts)]
// pub struct GetBalance<'info> {
//     #[account(mut)]
//     pub account: Account<'info, TokenAccount>,
//     pub token_program: Program<'info, Token>,
// }

// #[derive(Accounts)]
// pub struct Approve<'info> {
//     #[account(mut)]
//     pub account: Account<'info, TokenAccount>,
//     pub owner: Signer<'info>,
//     pub delegate: AccountInfo<'info>,
//     pub token_program: Program<'info, Token>,
// }

// #[derive(Accounts)]
// pub struct FreezeAccount<'info> {
//     #[account(mut)]
//     pub account: Account<'info, TokenAccount>,
//     pub mint: Account<'info, Mint>,
//     pub authority: Signer<'info>,
//     pub token_program: Program<'info, Token>,
// }

// #[derive(Accounts)]
// pub struct CloseAccount<'info> {
//     #[account(mut)]
//     pub account: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub destination: AccountInfo<'info>,
//     pub authority: Signer<'info>,
//     pub token_program: Program<'info, Token>,
// }

// #[derive(Accounts)]
// pub struct MyInstruction<'info> {
//     #[account(address = sysvar::rent::ID)]
//     pub rent: AccountInfo<'info>,
// }

