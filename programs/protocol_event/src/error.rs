use anchor_lang::prelude::*;

#[error_code]
pub enum EventError {
    #[msg("Event participants list is full.")]
    EventParticipantsListFull,
}