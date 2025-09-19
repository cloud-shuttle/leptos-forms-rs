//! Field validation utilities
//!
//! This module provides utilities for validating individual fields.

use crate::core::traits::Form;
use crate::core::types::FieldValue;
use crate::validation::Validator;

/// Validate a single field value
pub fn validate_field_value<T: Form>(
    _form: &T,
    field_name: &str,
    value: &FieldValue,
) -> Result<(), String> {
    let metadata = T::field_metadata();

    // Find field metadata
    let field_meta = metadata
        .iter()
        .find(|meta| meta.name == field_name)
        .ok_or_else(|| "Field not found".to_string())?;

    // Validate against field validators
    for validator in &field_meta.validators {
        validate_value_against_validator(value, validator)?;
    }

    // Check if field is required
    if field_meta.is_required {
        if let FieldValue::String(s) = value {
            if s.trim().is_empty() {
                return Err("This field is required".to_string());
            }
        }
    }

    Ok(())
}

/// Validate a value against a specific validator
pub fn validate_value_against_validator(
    value: &FieldValue,
    validator: &Validator,
) -> Result<(), String> {
    match validator {
        Validator::Required => match value {
            FieldValue::String(s) if s.trim().is_empty() => {
                Err("This field is required".to_string())
            }
            FieldValue::Null => Err("This field is required".to_string()),
            FieldValue::Array(arr) if arr.is_empty() => Err("This field is required".to_string()),
            _ => Ok(()),
        },
        Validator::Email => {
            if let FieldValue::String(email) = value {
                let email_regex = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
                if email_regex.is_match(email) {
                    Ok(())
                } else {
                    Err("Invalid email format".to_string())
                }
            } else {
                Err("Email must be a string".to_string())
            }
        }
        Validator::Url => {
            if let FieldValue::String(url) = value {
                let url_regex = regex::Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap();
                if url_regex.is_match(url) {
                    Ok(())
                } else {
                    Err("Invalid URL format".to_string())
                }
            } else {
                Err("URL must be a string".to_string())
            }
        }
        Validator::MinLength(min) => {
            if let FieldValue::String(s) = value {
                if s.len() >= *min {
                    Ok(())
                } else {
                    Err(format!("Minimum length is {} characters", min))
                }
            } else {
                Err("Value must be a string".to_string())
            }
        }
        Validator::MaxLength(max) => {
            if let FieldValue::String(s) = value {
                if s.len() <= *max {
                    Ok(())
                } else {
                    Err(format!("Maximum length is {} characters", max))
                }
            } else {
                Err("Value must be a string".to_string())
            }
        }
        Validator::Pattern(pattern) => {
            if let FieldValue::String(s) = value {
                let regex =
                    regex::Regex::new(pattern).map_err(|_| "Invalid pattern".to_string())?;
                if regex.is_match(s) {
                    Ok(())
                } else {
                    Err("Value doesn't match required pattern".to_string())
                }
            } else {
                Err("Value must be a string".to_string())
            }
        }
        Validator::Range(min, max) => {
            if let Some(num) = value.as_number() {
                if num >= *min && num <= *max {
                    Ok(())
                } else {
                    Err(format!("Value must be between {} and {}", min, max))
                }
            } else {
                Err("Value must be a number".to_string())
            }
        }
        Validator::Min(min_val) => {
            if let Some(num) = value.as_number() {
                if num >= *min_val {
                    Ok(())
                } else {
                    Err(format!("Value must be at least {}", min_val))
                }
            } else {
                Err("Value must be a number".to_string())
            }
        }
        Validator::Max(max_val) => {
            if let Some(num) = value.as_number() {
                if num <= *max_val {
                    Ok(())
                } else {
                    Err(format!("Value must be at most {}", max_val))
                }
            } else {
                Err("Value must be a number".to_string())
            }
        }
        Validator::Custom(_) => {
            // Custom validators are handled by the validation engine
            Ok(())
        }
    }
}
