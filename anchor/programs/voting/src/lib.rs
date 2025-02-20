use anchor_lang::prelude::*;

declare_id!("6ghmiYfqXdugFBdkMruXdhH6qD4rFoL1z7KeuxWeVEYo");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(context: Context<InitializePoll>, poll_id: u64, description: String, poll_start: u64, poll_end: u64) -> Result<()> {
        let poll = &mut context.accounts.poll;

        poll.poll_id = poll_id;
        poll.description = description;
        poll.poll_start = poll_start;
        poll.poll_end = poll_end;
        poll.candidate_amount = 0;

        Ok(())
    }

    pub fn initialize_candidate(context: Context<InitializeCandidate>, candidate_name: String, _poll_id: u64) -> Result<()> {
        let candidate = &mut context.accounts.candidate;
        let poll = &mut context.accounts.poll;

        candidate.candidate_name = candidate_name;
        candidate.candidate_votes = 0;

        poll.candidate_amount += 1;

        Ok(())
    }

    pub fn vote(context: Context<Vote>, _candidate_name: String, _poll_id: u64) -> Result<()> {
        let candidate = &mut context.accounts.candidate;
        candidate.candidate_votes += 1;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,

    #[max_len(280)]
    pub description: String, 

    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64
}


#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(32)]
    pub candidate_name: String,

    pub candidate_votes: u64
}


#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + Poll::INIT_SPACE,

        // seeds 的类型必须是字节切片（&[u8]）
        // to_le_bytes() 是 u64 类型的方法，将整数转换为小端字节序（Little Endian）的字节数组 [u8; 8]
        // as_ref() 将字节数组 [u8; 8] 转换为字节切片 &[u8]，这是 seeds 所需的类型
        seeds=[poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>
}


#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct InitializeCandidate<'info> {
    
    // We have initialized the poll at the 'InitializePoll' already, so it doesn't 'init', 'payer', 'space' characters here!
    #[account(
        mut,
        seeds=[poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        init,
        payer = signer,
        space = 8 + Candidate::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>
}


#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct Vote<'info> {

    #[account(
        seeds=[poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,

    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>
}

