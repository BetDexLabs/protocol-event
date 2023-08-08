use crate::error::EventError;
use crate::state::classification::Classification;
use crate::state::event_group::EventGroup;
use crate::state::subcategory::Subcategory;
use anchor_lang::prelude::*;

pub fn create_classification(
    classification: &mut Classification,
    payer: Pubkey,
    code: String,
    name: String,
) -> Result<()> {
    validate_classification(&code, &name)?;

    classification.payer = payer;
    classification.authority = payer;
    classification.code = code;
    classification.name = name;

    Ok(())
}

fn validate_classification(code: &String, name: &String) -> Result<()> {
    require!(
        code.len() <= Subcategory::MAX_CODE_LENGTH,
        EventError::MaxStringLengthExceeded,
    );
    require!(
        name.len() <= Subcategory::MAX_NAME_LENGTH,
        EventError::MaxStringLengthExceeded,
    );
    Ok(())
}

pub fn create_subcategory(
    subcategory: &mut Subcategory,
    classification: Pubkey,
    payer: Pubkey,
    code: String,
    name: String,
) -> Result<()> {
    validate_subcategory(&code, &name)?;

    subcategory.classification = classification;
    subcategory.payer = payer;
    subcategory.authority = payer;
    subcategory.code = code;
    subcategory.name = name;
    subcategory.participant_count = 0;

    Ok(())
}

fn validate_subcategory(code: &String, name: &String) -> Result<()> {
    require!(
        code.len() <= Subcategory::MAX_CODE_LENGTH,
        EventError::MaxStringLengthExceeded,
    );
    require!(
        name.len() <= Subcategory::MAX_NAME_LENGTH,
        EventError::MaxStringLengthExceeded,
    );
    Ok(())
}

pub fn create_event_group(
    event_group: &mut EventGroup,
    category: Pubkey,
    payer: Pubkey,
    code: String,
    name: String,
) -> Result<()> {
    validate_event_group(&code, &name)?;

    event_group.subcategory = category;
    event_group.payer = payer;
    event_group.authority = payer;
    event_group.code = code;
    event_group.name = name;

    Ok(())
}

fn validate_event_group(code: &String, name: &String) -> Result<()> {
    require!(
        code.len() <= EventGroup::MAX_CODE_LENGTH,
        EventError::MaxStringLengthExceeded,
    );
    require!(
        name.len() <= EventGroup::MAX_NAME_LENGTH,
        EventError::MaxStringLengthExceeded,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::error::EventError;
    use crate::instructions::create_grouping::{validate_event_group, validate_subcategory};
    use crate::instructions::{create_event_group, create_subcategory};
    use crate::state::event_group::EventGroup;
    use crate::state::subcategory::Subcategory;
    use anchor_lang::error;
    use solana_program::pubkey::Pubkey;

    #[test]
    fn test_create_subcategory() {
        let mut new_subcategory = Subcategory {
            code: "".to_string(),
            name: "".to_string(),
            participant_count: 0,
            authority: Default::default(),
            payer: Default::default(),
            classification: Default::default(),
        };

        let classification = Pubkey::new_unique();
        let code = "FOOTBALL".to_string();
        let name = "Football".to_string();
        let payer = Pubkey::new_unique();

        let result = create_subcategory(
            &mut new_subcategory,
            classification,
            payer,
            code.clone(),
            name.clone(),
        );

        assert!(result.is_ok());
        assert_eq!(new_subcategory.classification, classification);
        assert_eq!(new_subcategory.payer, payer);
        assert_eq!(new_subcategory.authority, payer);
        assert_eq!(new_subcategory.code, code);
        assert_eq!(new_subcategory.name, name);
        assert_eq!(new_subcategory.participant_count, 0);
    }

    #[test]
    fn test_validate_subcategory_code_exceeds_limit() {
        let result = validate_subcategory(&"012345678".to_string(), &"Football".to_string());

        assert_eq!(result, Err(error!(EventError::MaxStringLengthExceeded)));
    }

    #[test]
    fn test_validate_subcategory_name_exceeds_limit() {
        let result = validate_subcategory(
            &"FOOTBALL".to_string(),
            &"012345678901234567890123456789012345678901234567890".to_string(),
        );

        assert_eq!(result, Err(error!(EventError::MaxStringLengthExceeded)));
    }

    #[test]
    fn test_create_event_group() {
        let mut new_event_group = EventGroup {
            subcategory: Default::default(),
            code: "".to_string(),
            name: "".to_string(),
            authority: Default::default(),
            payer: Default::default(),
        };

        let code = "MLS".to_string();
        let name = "Major League Soccer".to_string();
        let category = Pubkey::new_unique();
        let payer = Pubkey::new_unique();

        let result = create_event_group(
            &mut new_event_group,
            category,
            payer,
            code.clone(),
            name.clone(),
        );

        assert!(result.is_ok());
        assert_eq!(new_event_group.subcategory, category);
        assert_eq!(new_event_group.payer, payer);
        assert_eq!(new_event_group.authority, payer);
        assert_eq!(new_event_group.code, code);
        assert_eq!(new_event_group.name, name);
    }

    #[test]
    fn test_create_event_group_code_exceeds_limit() {
        let result =
            validate_event_group(&"012345678".to_string(), &"Major League Soccer".to_string());

        assert_eq!(result, Err(error!(EventError::MaxStringLengthExceeded)));
    }

    #[test]
    fn test_create_event_group_name_exceeds_limit() {
        let result = validate_event_group(
            &"MLS".to_string(),
            &"012345678901234567890123456789012345678901234567890".to_string(),
        );

        assert_eq!(result, Err(error!(EventError::MaxStringLengthExceeded)));
    }
}
