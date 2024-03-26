use anchor_lang::prelude::*;

declare_id!("DhKx558vL1z4NhcPYVXnYKe9C5BJiqYtijDpVqFTArp3");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
