use anchor_lang::prelude::*;
use crate::state::type_size::{BOOL_SIZE, CHAR_SIZE, DISCRIMINATOR_SIZE, I64_SIZE, option_size, PUB_KEY_SIZE, U64_SIZE, vec_size};

#[account]
pub struct Event {
    pub category: Pubkey,
    pub event_group: Pubkey,
    pub active: bool,

    pub authority: Pubkey,
    pub payer: Pubkey,

    pub slug: String, // event identifier e.g. LAFCvLAG@2021-08-28
    pub name: String, // for display purposes e.g. Los Angeles Football Club vs. LA Galaxy

    pub participants: Vec<u16>,

    pub expected_start_timestamp: i64,
    pub actual_start_timestamp: Option<i64>,
    pub actual_end_timestamp: Option<i64>,
}

impl Event {
    const MAX_EVENT_SLUG_LENGTH: usize = 25;
    const MAX_EVENT_NAME_LENGTH: usize = 50;
    pub const MAX_PARTICIPANTS: usize = 300;

    pub const SIZE: usize =
        DISCRIMINATOR_SIZE
            + (PUB_KEY_SIZE * 4)
            + BOOL_SIZE
            + vec_size(CHAR_SIZE, Event::MAX_EVENT_SLUG_LENGTH)
            + vec_size(CHAR_SIZE, Event::MAX_EVENT_NAME_LENGTH)
            + U64_SIZE
            + vec_size(U64_SIZE, Event::MAX_PARTICIPANTS)
            + option_size(I64_SIZE) * 2;
}
