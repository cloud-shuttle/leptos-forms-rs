//! Array validators module - Validation for field arrays
//!
//! This module provides validation functions specifically for field arrays including
//! length validation, item validation, and array-specific validation rules.

use crate::core::types::FieldValue;

/// Array validators implementation
pub struct ArrayValidators;

impl ArrayValidators {
    /// Validate minimum array length
    pub fn min_length(value: &FieldValue, min: usize) -> Result<(), String> {
        if let FieldValue::Array(arr) = value {
            if arr.len() >= min {
                Ok(())
            } else {
                Err(format!("Array must have at least {} items", min))
            }
        } else {
            Err("Value must be an array".to_string())
        }
    }

    /// Validate maximum array length
    pub fn max_length(value: &FieldValue, max: usize) -> Result<(), String> {
        if let FieldValue::Array(arr) = value {
            if arr.len() <= max {
                Ok(())
            } else {
                Err(format!("Array must have at most {} items", max))
            }
        } else {
            Err("Value must be an array".to_string())
        }
    }

    /// Validate exact array length
    pub fn exact_length(value: &FieldValue, exact: usize) -> Result<(), String> {
        if let FieldValue::Array(arr) = value {
            if arr.len() == exact {
                Ok(())
            } else {
                Err(format!("Array must have exactly {} items", exact))
            }
        } else {
            Err("Value must be an array".to_string())
        }
    }

    /// Validate that array is not empty
    pub fn not_empty(value: &FieldValue) -> Result<(), String> {
        if let FieldValue::Array(arr) = value {
            if !arr.is_empty() {
                Ok(())
            } else {
                Err("Array cannot be empty".to_string())
            }
        } else {
            Err("Value must be an array".to_string())
        }
    }

    /// Validate that array is empty
    pub fn empty(value: &FieldValue) -> Result<(), String> {
        if let FieldValue::Array(arr) = value {
            if arr.is_empty() {
                Ok(())
            } else {
                Err("Array must be empty".to_string())
            }
        } else {
            Err("Value must be an array".to_string())
        }
    }

    /// Validate array length range
    pub fn length_range(value: &FieldValue, min: usize, max: usize) -> Result<(), String> {
        if let FieldValue::Array(arr) = value {
            if arr.len() >= min && arr.len() <= max {
                Ok(())
            } else {
                Err(format!("Array must have between {} and {} items", min, max))
            }
        } else {
            Err("Value must be an array".to_string())
        }
    }

    /// Validate that all array items are not empty
    pub fn all_items_not_empty(value: &FieldValue) -> Result<(), String> {
        if let FieldValue::Array(arr) = value {
            for (index, item) in arr.iter().enumerate() {
                if item.is_empty() {
                    return Err(format!("Item at index {} cannot be empty", index));
                }
            }
            Ok(())
        } else {
            Err("Value must be an array".to_string())
        }
    }

    /// Validate that all array items are unique
    pub fn all_items_unique(value: &FieldValue) -> Result<(), String> {
        if let FieldValue::Array(arr) = value {
            for (i, item1) in arr.iter().enumerate() {
                for (j, item2) in arr.iter().enumerate() {
                    if i != j && item1 == item2 {
                        return Err(format!("Duplicate items found at indices {} and {}", i, j));
                    }
                }
            }
            Ok(())
        } else {
            Err("Value must be an array".to_string())
        }
    }

    /// Validate that all array items are strings
    pub fn all_items_strings(value: &FieldValue) -> Result<(), String> {
        if let FieldValue::Array(arr) = value {
            for (index, item) in arr.iter().enumerate() {
                if !matches!(item, FieldValue::String(_)) {
                    return Err(format!("Item at index {} must be a string", index));
                }
            }
            Ok(())
        } else {
            Err("Value must be an array".to_string())
        }
    }

    /// Validate that all array items are numbers
    pub fn all_items_numbers(value: &FieldValue) -> Result<(), String> {
        if let FieldValue::Array(arr) = value {
            for (index, item) in arr.iter().enumerate() {
                if !matches!(item, FieldValue::Number(_)) {
                    return Err(format!("Item at index {} must be a number", index));
                }
            }
            Ok(())
        } else {
            Err("Value must be an array".to_string())
        }
    }

    /// Validate that all array items are booleans
    pub fn all_items_booleans(value: &FieldValue) -> Result<(), String> {
        if let FieldValue::Array(arr) = value {
            for (index, item) in arr.iter().enumerate() {
                if !matches!(item, FieldValue::Boolean(_)) {
                    return Err(format!("Item at index {} must be a boolean", index));
                }
            }
            Ok(())
        } else {
            Err("Value must be an array".to_string())
        }
    }

    /// Validate array items against a custom validator function
    pub fn validate_items_with<F>(value: &FieldValue, validator: F) -> Result<(), String>
    where
        F: Fn(&FieldValue) -> Result<(), String>,
    {
        if let FieldValue::Array(arr) = value {
            for (index, item) in arr.iter().enumerate() {
                if let Err(error) = validator(item) {
                    return Err(format!("Item at index {}: {}", index, error));
                }
            }
            Ok(())
        } else {
            Err("Value must be an array".to_string())
        }
    }

    /// Validate that array contains at least one item matching a condition
    pub fn contains<F>(value: &FieldValue, predicate: F) -> Result<(), String>
    where
        F: Fn(&FieldValue) -> bool,
    {
        if let FieldValue::Array(arr) = value {
            if arr.iter().any(&predicate) {
                Ok(())
            } else {
                Err("Array must contain at least one item matching the condition".to_string())
            }
        } else {
            Err("Value must be an array".to_string())
        }
    }

    /// Validate that array contains no items matching a condition
    pub fn contains_none<F>(value: &FieldValue, predicate: F) -> Result<(), String>
    where
        F: Fn(&FieldValue) -> bool,
    {
        if let FieldValue::Array(arr) = value {
            if arr.iter().any(&predicate) {
                Err("Array must not contain any items matching the condition".to_string())
            } else {
                Ok(())
            }
        } else {
            Err("Value must be an array".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_length() {
        let array = FieldValue::Array(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("item2".to_string()),
        ]);

        assert!(ArrayValidators::min_length(&array, 1).is_ok());
        assert!(ArrayValidators::min_length(&array, 2).is_ok());
        assert!(ArrayValidators::min_length(&array, 3).is_err());
    }

    #[test]
    fn test_max_length() {
        let array = FieldValue::Array(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("item2".to_string()),
        ]);

        assert!(ArrayValidators::max_length(&array, 3).is_ok());
        assert!(ArrayValidators::max_length(&array, 2).is_ok());
        assert!(ArrayValidators::max_length(&array, 1).is_err());
    }

    #[test]
    fn test_exact_length() {
        let array = FieldValue::Array(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("item2".to_string()),
        ]);

        assert!(ArrayValidators::exact_length(&array, 2).is_ok());
        assert!(ArrayValidators::exact_length(&array, 1).is_err());
        assert!(ArrayValidators::exact_length(&array, 3).is_err());
    }

    #[test]
    fn test_not_empty() {
        let empty_array = FieldValue::Array(vec![]);
        let non_empty_array = FieldValue::Array(vec![FieldValue::String("item".to_string())]);

        assert!(ArrayValidators::not_empty(&empty_array).is_err());
        assert!(ArrayValidators::not_empty(&non_empty_array).is_ok());
    }

    #[test]
    fn test_empty() {
        let empty_array = FieldValue::Array(vec![]);
        let non_empty_array = FieldValue::Array(vec![FieldValue::String("item".to_string())]);

        assert!(ArrayValidators::empty(&empty_array).is_ok());
        assert!(ArrayValidators::empty(&non_empty_array).is_err());
    }

    #[test]
    fn test_length_range() {
        let array = FieldValue::Array(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("item2".to_string()),
        ]);

        assert!(ArrayValidators::length_range(&array, 1, 3).is_ok());
        assert!(ArrayValidators::length_range(&array, 2, 2).is_ok());
        assert!(ArrayValidators::length_range(&array, 3, 5).is_err());
        assert!(ArrayValidators::length_range(&array, 1, 1).is_err());
    }

    #[test]
    fn test_all_items_not_empty() {
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

    #[test]
    fn test_all_items_unique() {
        let array_with_duplicates = FieldValue::Array(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("item2".to_string()),
            FieldValue::String("item1".to_string()),
        ]);
        let array_without_duplicates = FieldValue::Array(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("item2".to_string()),
            FieldValue::String("item3".to_string()),
        ]);

        assert!(ArrayValidators::all_items_unique(&array_with_duplicates).is_err());
        assert!(ArrayValidators::all_items_unique(&array_without_duplicates).is_ok());
    }

    #[test]
    fn test_all_items_strings() {
        let array_with_strings = FieldValue::Array(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("item2".to_string()),
        ]);
        let array_with_mixed = FieldValue::Array(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::Number(42.0),
        ]);

        assert!(ArrayValidators::all_items_strings(&array_with_strings).is_ok());
        assert!(ArrayValidators::all_items_strings(&array_with_mixed).is_err());
    }

    #[test]
    fn test_contains() {
        let array = FieldValue::Array(vec![
            FieldValue::String("item1".to_string()),
            FieldValue::String("item2".to_string()),
        ]);

        assert!(ArrayValidators::contains(&array, |item| {
            matches!(item, FieldValue::String(s) if s == "item1")
        })
        .is_ok());

        assert!(ArrayValidators::contains(&array, |item| {
            matches!(item, FieldValue::String(s) if s == "item3")
        })
        .is_err());
    }
}
