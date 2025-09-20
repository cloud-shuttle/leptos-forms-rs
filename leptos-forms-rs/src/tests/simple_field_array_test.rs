//! Simple field array test
//!
//! This module contains a simple test for the field array functionality.

#[cfg(test)]
mod tests {
    use crate::core::types::{ArrayState, FieldValue};
    use crate::validation::ArrayValidators;

    #[test]
    fn test_array_state_basic() {
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
    fn test_array_validators_basic() {
        let array = FieldValue::Array(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("item2".to_string()),
        ]);

        assert!(ArrayValidators::min_length(&array, 1).is_ok());
        assert!(ArrayValidators::min_length(&array, 2).is_ok());
        assert!(ArrayValidators::min_length(&array, 3).is_err());
    }
}
