use anchor_lang::prelude::*;

declare_id!("FsgyMvD4vw6xSMNkFD14gbgRK5kadrZYzF1xGAcj2WfR");

#[program]
pub mod todo {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world, from solana smart contract");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
