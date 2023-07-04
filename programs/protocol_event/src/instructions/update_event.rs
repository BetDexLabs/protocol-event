use anchor_lang::prelude::*;
use crate::error::EventError;
use crate::UpdateEvent;
use crate::state::event::Event;

pub fn update_active_flag(ctx: Context<UpdateEvent>, active: bool) -> Result<()> {
    let event = &mut ctx.accounts.event;
    event.active = active;
    Ok(())
}

pub fn add_participants(participants: &mut Vec<u16>, participants_to_add: Vec<u16>) -> Result<()> {
    if participants_to_add.is_empty() {
        return Ok(());
    }

    participants.extend(participants_to_add.into_iter());
    participants.sort_by(|a, b| a.partial_cmp(b).unwrap());
    participants.dedup();

    require!(
        participants.len() < Event::MAX_PARTICIPANTS,
        EventError::EventParticipantsListFull
    );

    Ok(())
}

pub fn remove_participants(participants: &mut Vec<u16>, participants_to_remove: Vec<u16>) -> Result<()> {
    if participants_to_remove.is_empty() || participants.is_empty() {
        return Ok(());
    }

    participants.retain(|participant| !participants_to_remove.contains(participant));

    Ok(())
}

pub fn updated_expected_start_timestamp(ctx: Context<UpdateEvent>, timestamp: i64) -> Result<()> {
    let event = &mut ctx.accounts.event;
    event.expected_start_timestamp = timestamp;
    Ok(())
}


#[cfg(test)]
mod tests {
    use anchor_lang::error;
    use crate::instructions::{add_participants, remove_participants};
    use crate::error::EventError;
    use crate::state::event::Event;


    // add participants

    #[test]
    fn test_add_participants() {
        let existing_participants = &mut vec![1, 2, 3, 4];
        let participants_to_add = vec![5, 6, 7, 8];

        add_participants(existing_participants, participants_to_add).unwrap();

        assert_eq!(existing_participants, &vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_add_participants_empty_vec() {
        let existing_participants = &mut vec![1, 2, 3, 4];
        let participants_to_add = vec![];

        add_participants(existing_participants, participants_to_add).unwrap();

        assert_eq!(existing_participants, &vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_add_participants_dedup() {
        let existing_participants = &mut vec![1, 2, 3, 4, 5];
        let participants_to_add = vec![5, 6, 7, 8];

        add_participants(existing_participants, participants_to_add).unwrap();

        assert_eq!(existing_participants, &vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_add_participants_list_full() {
        let existing_participants: &mut Vec<u16> = &mut vec![];
        for i in 0..Event::MAX_PARTICIPANTS {
            existing_participants.push((i + 1) as u16);
        }

        let participants_to_add = vec![301];

        let result = add_participants(existing_participants, participants_to_add);

        assert_eq!(result, Err(error!(EventError::EventParticipantsListFull)));
    }

    // remove participants

    #[test]
    fn test_remove_participants() {
        let existing_participants = &mut vec![1, 2, 3, 4];
        let participants_to_remove = vec![1, 2];

        remove_participants(existing_participants, participants_to_remove).unwrap();

        assert_eq!(existing_participants, &vec![3, 4]);
    }

    #[test]
    fn test_remove_participants_empty_vecs() {
        let existing_participants = &mut vec![];
        let participants_to_remove = vec![];

        remove_participants(existing_participants, participants_to_remove).unwrap();

        assert_eq!(existing_participants, &vec![]);
    }
}
