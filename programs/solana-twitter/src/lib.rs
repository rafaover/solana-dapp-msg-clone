use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;

declare_id!("DhKx558vL1z4NhcPYVXnYKe9C5BJiqYtijDpVqFTArp3");

#[program]
pub mod solana_twitter {
    use super::*;
    pub fn send_tweet(
      // SendTweet was used as generic type to link the context to this function
      ctx: Context<SendTweet>,
      // Extra arguments, only account cannot be used as argument here.
      topic: String,
      // This function returns a ProgramResult which can either be Ok or ProgramError
      content: String) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendTweet<'info> {
  /* $[account(init)] indicates we are creating an account when sending a tweet
  and howing who's paying for the transaction and how much storage it's using */
  #[account(init, payer = author, space = Tweet::LEN)]
  pub tweet: Account<'info, Tweet>,
  /* making the author account mutable, because their tweet will change how much
  money they have in their account */
  #[account(mut)]
  pub author: Signer<'info>,
  /* Type of account called Program and passing it the System type to ensure
  it is the official System program. To avoid malicious action from outside */
  pub system_program: Program<'info, System>,
}

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

const DISCRIMINATOR_LENGTH: usize = 8; // discriminator for every account created
const PUBLIC_KEY_LENGTH: usize = 32; //author property size
const TIMESTAMP_SIZE: usize = 8; //timestamp property size
const STRING_LENGTH_PREFIX: usize = 4; // Size for each char in a String
const TOPIC_SIZE: usize = 50 * 4; // 50 chars maximum, 200 bytes
const CONTENT_SIZE: usize = 280 * 4; // 280 chars maximum, 1120 bytes


// Add a constant on the Tweet account that provides its total size.
impl Tweet {
  const LEN: usize = DISCRIMINATOR_LENGTH +
  PUBLIC_KEY_LENGTH + TIMESTAMP_SIZE + STRING_LENGTH_PREFIX +
  TOPIC_SIZE + CONTENT_SIZE;
}
