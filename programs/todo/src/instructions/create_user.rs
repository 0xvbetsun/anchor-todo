use anchor_lang::prelude::*;

use crate::state::{UserProfile, USER_PROFILE_SPACE};

pub fn create_user(ctx: Context<CreateUser>, name: String, username: String, password: String) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    user_profile.authority = ctx.accounts.authority.key();
    user_profile.name = name;
    user_profile.username = username;
    user_profile.password = password;

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String, username: String, password: String)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + USER_PROFILE_SPACE,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}
