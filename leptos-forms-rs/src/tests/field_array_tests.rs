//! Field array tests
//!
//! This module contains tests for the field array functionality.

#[cfg(test)]
mod tests {
    use crate::core::types::{ArrayState, FieldValue};
    use crate::validation::ArrayValidators;

    #[test]
    fn test_array_state_creation() {
        let items = vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("item2".to_string()),
        ];
        let state = ArrayState::new(items.clone());

        assert_eq!(state.items, items);
        assert_eq!(state.len(), 2);
        assert!(!state.is_dirty);
        assert!(!state.has_errors());
    }

    #[test]
    fn test_array_state_add_item() {
        let mut state = ArrayState::new(vec![FieldValue::String("item1".to_string())]);
        state.add_item(FieldValue::String("item2".to_string()));

        assert_eq!(state.len(), 2);
        assert!(state.is_dirty);
        assert_eq!(state.items[1], FieldValue::String("item2".to_string()));
    }

    #[test]
    fn test_array_state_remove_item() {
        let mut state = ArrayState::new(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("item2".to_string()),
        ]);
        let removed = state.remove_item(0);

        assert_eq!(removed, Some(FieldValue::String("item1".to_string())));
        assert_eq!(state.len(), 1);
        assert!(state.is_dirty);
        assert_eq!(state.items[0], FieldValue::String("item2".to_string()));
    }

    #[test]
    fn test_array_state_move_item() {
        let mut state = ArrayState::new(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("item2".to_string()),
            FieldValue::String("item3".to_string()),
        ]);
        state.move_item(0, 2).unwrap();

        assert_eq!(
            state.items,
            vec![
                FieldValue::String("item2".to_string()),
                FieldValue::String("item3".to_string()),
                FieldValue::String("item1".to_string()),
            ]
        );
        assert!(state.is_dirty);
    }

    #[test]
    fn test_array_state_clear() {
        let mut state = ArrayState::new(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("item2".to_string()),
        ]);
        state.clear();

        assert_eq!(state.len(), 0);
        assert!(state.is_dirty);
        assert!(state.is_empty());
    }

    #[test]
    fn test_array_validators_min_length() {
        let array = FieldValue::Array(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("item2".to_string()),
        ]);

        assert!(ArrayValidators::min_length(&array, 1).is_ok());
        assert!(ArrayValidators::min_length(&array, 2).is_ok());
        assert!(ArrayValidators::min_length(&array, 3).is_err());
    }

    #[test]
    fn test_array_validators_max_length() {
        let array = FieldValue::Array(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("item2".to_string()),
        ]);

        assert!(ArrayValidators::max_length(&array, 3).is_ok());
        assert!(ArrayValidators::max_length(&array, 2).is_ok());
        assert!(ArrayValidators::max_length(&array, 1).is_err());
    }

    #[test]
    fn test_array_validators_not_empty() {
        let empty_array = FieldValue::Array(vec![]);
        let non_empty_array = FieldValue::Array(vec![FieldValue::String("item".to_string())]);

        assert!(ArrayValidators::not_empty(&empty_array).is_err());
        assert!(ArrayValidators::not_empty(&non_empty_array).is_ok());
    }

    #[test]
    fn test_array_validators_all_items_not_empty() {
        let array_with_empty = FieldValue::Array(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("".to_string()),
        ]);
        let array_without_empty = FieldValue::Array(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("item2".to_string()),
        ]);

        assert!(ArrayValidators::all_items_not_empty(&array_with_empty).is_err());
        assert!(ArrayValidators::all_items_not_empty(&array_without_empty).is_ok());
    }
}
