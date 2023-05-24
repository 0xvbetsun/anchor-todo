use anchor_lang::prelude::*;

use crate::{constants::USER_TAG, state::UserProfile};

pub fn initialize(ctx: Context<InitializeUser>) -> Result<()> {
    // Initialize user profile with default data
    let user_profile = &mut ctx.accounts.user_profile;
    user_profile.authority = ctx.accounts.authority.key();
    user_profile.last_todo = 0;
    user_profile.todo_count = 0;

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}
