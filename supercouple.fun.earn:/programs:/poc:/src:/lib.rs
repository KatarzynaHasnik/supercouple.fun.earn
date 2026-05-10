use anchor_lang::prelude::*;

declare_id!("Supr111111111111111111111111111111111111111");

#[program]
pub mod supercouple_fun_earn {
    use super::*;

    pub fn create_bounty(
        ctx: Context<CreateBounty>,
        stake: u64,
        reward: u64,
    ) -> Result<()> {

        let bounty = &mut ctx.accounts.bounty;

        bounty.creator = ctx.accounts.creator.key();
        bounty.participant = None;
        bounty.stake = stake;
        bounty.reward = reward;
        bounty.completed = false;

        Ok(())
    }

    pub fn accept_bounty(ctx: Context<AcceptBounty>) -> Result<()> {

        let bounty = &mut ctx.accounts.bounty;

        bounty.participant = Some(ctx.accounts.participant.key());

        Ok(())
    }

    pub fn complete_bounty(ctx: Context<CompleteBounty>) -> Result<()> {

        let bounty = &mut ctx.accounts.bounty;

        bounty.completed = true;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateBounty<'info> {

    #[account(init, payer = creator, space = 8 + 128)]
    pub bounty: Account<'info, Bounty>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptBounty<'info> {

    #[account(mut)]
    pub bounty: Account<'info, Bounty>,

    pub participant: Signer<'info>,
}

#[derive(Accounts)]
pub struct CompleteBounty<'info> {

    #[account(mut)]
    pub bounty: Account<'info, Bounty>,
}

#[account]
pub struct Bounty {

    pub creator: Pubkey,
    pub participant: Option<Pubkey>,

    pub stake: u64,
    pub reward: u64,

    pub completed: bool,
}