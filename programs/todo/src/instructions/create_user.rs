use anchor_lang::prelude::*;

use crate::{errors::TodoErrorCode, state::UserProfile};

pub fn create_user(
    ctx: Context<CreateUser>,
    name: String,
    username: String,
    password: String,
) -> Result<()> {
    if name.chars().count() > 20 {
        return Err(TodoErrorCode::TextTooLong.into());
    }
    if username.chars().count() > 20 {
        return Err(TodoErrorCode::TextTooLong.into());
    }
    if password.chars().count() > 20 {
        return Err(TodoErrorCode::TextTooLong.into());
    }

    let user_profile = &mut ctx.accounts.user_profile;

    user_profile.authority = ctx.accounts.authority.key();
    user_profile.name = name;
    user_profile.username = username;
    user_profile.password = password;
    user_profile.list_idx = 1;

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
        space = UserProfile::LEN,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}
