use crate::state::event::Event;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, PartialEq)]
pub struct CreateEventInfo {
    pub slug: String,
    pub name: String,
    pub participants: Vec<u16>,
    pub expected_start_timestamp: i64,
    pub actual_start_timestamp: Option<i64>,
    pub actual_end_timestamp: Option<i64>,
}

pub fn create(
    event: &mut Event,
    event_info: CreateEventInfo,
    authority: Pubkey,
    payer: Pubkey,
    category: Pubkey,
    event_group: Pubkey,
) -> Result<()> {
    event.authority = authority;
    event.payer = payer;

    event.category = category;
    event.event_group = event_group;

    event.active = false;

    event.slug = event_info.slug;
    event.name = event_info.name;

    event.participants = event_info.participants;

    event.expected_start_timestamp = event_info.expected_start_timestamp;
    event.actual_start_timestamp = event_info.actual_start_timestamp;
    event.actual_end_timestamp = event_info.actual_end_timestamp;

    Ok(())
}
