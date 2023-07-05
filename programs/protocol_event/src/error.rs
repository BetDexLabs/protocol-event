use anchor_lang::prelude::*;

#[error_code]
pub enum EventError {
    #[msg("Max string length exceeded.")]
    MaxStringLengthExceeded,
}
