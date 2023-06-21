use anchor_lang::prelude::*;
use crate::state::grouping::{MAX_CODE_LENGTH, MAX_NAME_STRING_LENGTH};
use crate::state::type_size::{CHAR_SIZE, DISCRIMINATOR_SIZE, ENUM_SIZE, PUB_KEY_SIZE, U16_SIZE, vec_size};

#[account]
pub struct Participant {
    pub category: Pubkey,
    pub participant_type: ParticipantType,

    pub name: String,
    pub code: String,

    pub id: u16,
    pub payer: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum ParticipantType {
    Individual,
    Team,
}

impl Participant {
    pub const SIZE: usize = DISCRIMINATOR_SIZE
        + PUB_KEY_SIZE * 3
        + ENUM_SIZE
        + vec_size(CHAR_SIZE, MAX_NAME_STRING_LENGTH)
        + vec_size(CHAR_SIZE, MAX_CODE_LENGTH)
        + U16_SIZE;
}
