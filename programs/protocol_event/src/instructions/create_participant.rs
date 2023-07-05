use crate::error::EventError;
use crate::state::participant::{Participant, ParticipantType};
use anchor_lang::prelude::*;

pub fn create_individual_participant(
    participant: &mut Participant,
    category: &Pubkey,
    payer: &Pubkey,
    code: String,
    name: String,
    participant_id: u16,
) -> Result<()> {
    validate_participant(&code, &name)?;
    initialize_participant(
        participant,
        category,
        payer,
        code,
        name,
        ParticipantType::Individual,
        participant_id,
    );

    Ok(())
}

pub fn create_team_participant(
    participant: &mut Participant,
    category: &Pubkey,
    payer: &Pubkey,
    code: String,
    name: String,
    participant_id: u16,
) -> Result<()> {
    validate_participant(&code, &name)?;
    initialize_participant(
        participant,
        category,
        payer,
        code,
        name,
        ParticipantType::Team,
        participant_id,
    );

    Ok(())
}

fn initialize_participant(
    participant: &mut Participant,
    category: &Pubkey,
    payer: &Pubkey,
    code: String,
    name: String,
    participant_type: ParticipantType,
    participant_id: u16,
) {
    participant.category = *category;
    participant.payer = *payer;

    participant.participant_type = participant_type;
    participant.code = code;
    participant.name = name;
    participant.id = participant_id;
}

fn validate_participant(code: &String, name: &String) -> Result<()> {
    require!(
        code.len() <= Participant::MAX_CODE_LENGTH,
        EventError::MaxStringLengthExceeded,
    );
    require!(
        name.len() <= Participant::MAX_NAME_LENGTH,
        EventError::MaxStringLengthExceeded,
    );
    Ok(())
}
