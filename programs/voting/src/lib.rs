use anchor_lang::{
    prelude::*, 
    solana_program
};

declare_id!("FhnUQ3mgYLTuLV7RZQaX4WMgnvigUoL4rKF8nH8PfqVc");


#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(context: Context<InitializePoll>, poll_id: u64, description: String, poll_start: u64, poll_end: u64) -> Result<()> {
        let poll: &mut Account<'_, Poll> = &mut context.accounts.poll;

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

    pub fn vote(context: Context<Vote>, candidate_name: String, _poll_id: u64) -> Result<()> {
        let candidate = &mut context.accounts.candidate;
        candidate.candidate_votes += 1;
        Ok(())
    }
}


#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Poll::INIT_SPACE,
        seeds=[poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>
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

#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

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

    pub system_program: Program<'info, System>
}


#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(32)]
    pub candidate_name: String,

    pub candidate_votes: u64
}



#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct Vote<'info> {

    pub signer: Signer<'info>,

    // We have initialized the poll at the 'InitializePoll' already, so it doesn't 'init', 'payer', 'space' characters here!
    #[account(
        
        seeds=[poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    // We have initialized the poll at the 'InitializeCandidate' already, so it doesn't 'init', 'payer', 'space' characters here!
    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,

    pub system_program: Program<'info, System>
}

