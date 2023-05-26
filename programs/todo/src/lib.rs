use anchor_lang::prelude::*;
use instructions::*;

declare_id!("5kZtVwH69P8uUH6fZ1Dd4Fh55H4254vNnigWZ8VAZirp");

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

    pub fn delete_list(ctx: Context<RemoveList>, idx: u8) -> Result<()> {
        instructions::remove_list(ctx, idx)
    }

    pub fn update_list(ctx: Context<UpdateList>, idx: u8, _title: String, _description: String) -> Result<()> {
        instructions::update_list(ctx, idx, _title, _description)
    }
}
