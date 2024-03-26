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


/* Every tweet will have an account, which facilitates charging as every
tweet from a account owner will be charged it's storage in it's limit of chars \
*/
#[account]
pub struct Tweet {
  pub author: Pubkey,
  pub timestamp:i64,
  pub topic: String,
  pub content: String,
}
