mod error_codes;
mod state;

use state::*;
use anchor_lang::prelude::*;
use error_codes::ErrorCode;
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
        content: String,
        // Also I could use Result<()>, that would manage ErrorCode directly without conversion.
        // If used, I could use the err! macro to return the error message.
        // Ex: return err!(ErrorCode::ContentTooLong)
    ) -> ProgramResult {
        // & is used to access account by reference, it's like borrowing in Rust
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;
        // Signer is used to guarantee that the account is owned by the author
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        if topic.chars().count() > 50 {
            return Err(ErrorCode::TopicTooLong.into());
        }
        if content.chars().count() > 280 {
            return Err(ErrorCode::ContentTooLong.into());
        }

        tweet.author = *author.key;
        tweet.timestamp = clock.unix_timestamp;
        tweet.topic = topic;
        tweet.content = content;

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
