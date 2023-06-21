use anchor_lang::prelude::*;
use crate::context::CreateParticipant;
use crate::state::participant::ParticipantType;

pub fn create_individual_participant(
    ctx: Context<CreateParticipant>,
    code: String,
    name: String,
) -> Result<()> {
    initialize_participant(ctx, code, name, ParticipantType::Individual);

    Ok(())
}

pub fn create_team_participant(
    ctx: Context<CreateParticipant>,
    code: String,
    name: String,
) -> Result<()> {
    initialize_participant(ctx, code, name, ParticipantType::Team);

    Ok(())
}

fn initialize_participant(
    ctx: Context<CreateParticipant>,
    code: String,
    name: String,
    participant_type: ParticipantType
) {
    let participant = &mut ctx.accounts.participant;

    participant.category = ctx.accounts.category.key();
    participant.participant_type = participant_type;
    participant.code = code;
    participant.name = name;
    participant.id = ctx.accounts.category.participant_count;
    participant.payer = ctx.accounts.payer.key();

    let category = &mut ctx.accounts.category;

    category.participant_count = category.participant_count.checked_add(1).unwrap();
}