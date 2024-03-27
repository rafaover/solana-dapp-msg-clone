use anchor_lang::prelude::*;

/* Every tweet will have an account, which facilitates charging as every
tweet from a account owner will be charged it's storage in it's limit of chars \
*/
#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
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
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH
        + TIMESTAMP_SIZE
        + STRING_LENGTH_PREFIX
        + TOPIC_SIZE
        + CONTENT_SIZE;
}
