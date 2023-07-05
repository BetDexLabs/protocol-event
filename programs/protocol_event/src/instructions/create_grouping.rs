use crate::error::EventError;
use crate::state::category::Category;
use crate::state::event_group::EventGroup;
use anchor_lang::prelude::*;

pub fn create_category(
    category: &mut Category,
    code: String,
    name: String,
    payer: Pubkey,
) -> Result<()> {
    require!(
        code.len() <= Category::MAX_CODE_LENGTH,
        EventError::MaxStringLengthExceeded,
    );
    require!(
        name.len() <= Category::MAX_NAME_LENGTH,
        EventError::MaxStringLengthExceeded,
    );

    category.payer = payer;
    category.code = code;
    category.name = name;
    category.participant_count = 0;

    Ok(())
}

pub fn create_event_group(
    event_group: &mut EventGroup,
    category: Pubkey,
    payer: Pubkey,
    code: String,
    name: String,
) -> Result<()> {
    require!(
        code.len() <= EventGroup::MAX_CODE_LENGTH,
        EventError::MaxStringLengthExceeded,
    );
    require!(
        name.len() <= EventGroup::MAX_NAME_LENGTH,
        EventError::MaxStringLengthExceeded,
    );

    event_group.category = category;
    event_group.payer = payer;
    event_group.code = code;
    event_group.name = name;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::error::EventError;
    use crate::instructions::{create_category, create_event_group};
    use crate::state::category::Category;
    use crate::state::event_group::EventGroup;
    use anchor_lang::error;
    use solana_program::pubkey::Pubkey;

    #[test]
    fn test_create_category() {
        let mut new_category = category();

        let code = "FOOTBALL".to_string();
        let name = "Football".to_string();
        let payer = Pubkey::new_unique();

        let result = create_category(&mut new_category, code.clone(), name.clone(), payer);

        assert!(result.is_ok());
        assert_eq!(new_category.payer, payer);
        assert_eq!(new_category.code, code);
        assert_eq!(new_category.name, name);
        assert_eq!(new_category.participant_count, 0);
    }

    #[test]
    fn test_create_category_code_exceeds_limit() {
        let result = create_category(
            &mut category(),
            "012345678".to_string(),
            "Football".to_string(),
            Pubkey::new_unique(),
        );

        assert_eq!(result, Err(error!(EventError::MaxStringLengthExceeded)));
    }

    #[test]
    fn test_create_category_name_exceeds_limit() {
        let result = create_category(
            &mut category(),
            "FOOTBALL".to_string(),
            "012345678901234567890123456789012345678901234567890".to_string(),
            Pubkey::new_unique(),
        );

        assert_eq!(result, Err(error!(EventError::MaxStringLengthExceeded)));
    }

    #[test]
    fn test_create_event_group() {
        let mut new_event_group = event_group();

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
        assert_eq!(new_event_group.category, category);
        assert_eq!(new_event_group.payer, payer);
        assert_eq!(new_event_group.code, code);
        assert_eq!(new_event_group.name, name);
    }

    #[test]
    fn test_create_event_group_code_exceeds_limit() {
        let result = create_event_group(
            &mut event_group(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            "012345678".to_string(),
            "Major League Soccer".to_string(),
        );

        assert_eq!(result, Err(error!(EventError::MaxStringLengthExceeded)));
    }

    #[test]
    fn test_create_event_group_name_exceeds_limit() {
        let result = create_event_group(
            &mut event_group(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            "MLS".to_string(),
            "012345678901234567890123456789012345678901234567890".to_string(),
        );

        assert_eq!(result, Err(error!(EventError::MaxStringLengthExceeded)));
    }

    fn category() -> Category {
        Category {
            code: "".to_string(),
            name: "".to_string(),
            participant_count: 0,
            payer: Default::default(),
        }
    }

    fn event_group() -> EventGroup {
        EventGroup {
            category: Default::default(),
            code: "".to_string(),
            name: "".to_string(),
            payer: Default::default(),
        }
    }
}
