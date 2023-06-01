use anchor_lang::prelude::*;
use instructions::*;

declare_id!("8uvpDc9tZxwYspeqX37HtDTBQPzJZU7Gp4GaKU8qz4Us");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

#[program]
pub mod todo {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, name: String, username: String, password: String) -> Result<()> {
        instructions::create_user(ctx, name, username, password)
    }

    pub fn create_list(ctx: Context<CreateList>, title: String, description: String) -> Result<()> {
        instructions::create_list(ctx, title, description)
    }

    pub fn update_list(ctx: Context<UpdateList>, id: u8, title: String, description: String) -> Result<()> {
        instructions::update_list(ctx, id, title, description)
    }

    pub fn remove_list(ctx: Context<RemoveList>) -> Result<()> {
        instructions::remove_list(ctx)
    }
}
