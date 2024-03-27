use anchor_lang::prelude::*;

// Acts as a ENUM CLASS to handle errors messages
#[error_code]
pub enum ErrorCode {
    #[msg("The provided topic should be 50 characters long maximum.")]
    TopicTooLong,
    #[msg("The provided content should be 280 characters long maximum.")]
    ContentTooLong,
}


// ProgramError conversion from ErrorCode to ProgramError
// If used Result<()> instead of ProgramResult, this conversion would not be necessary
impl From<ErrorCode> for ProgramError {
  fn from(error: ErrorCode) -> Self {
    match error {
      ErrorCode::TopicTooLong => ProgramError::Custom(1),
      ErrorCode::ContentTooLong => ProgramError::Custom(2),
    }
  }
}
