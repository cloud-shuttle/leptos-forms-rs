use crate::core::traits::Form;
use crate::core::types::FieldValue;
use crate::validation::ValidationErrors;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Utility functions for form operations
/// Convert a form to a HashMap of field values
pub fn form_to_map<T: Form>(form: &T) -> HashMap<String, FieldValue> {
    form.get_form_data()
}

/// Convert a HashMap of field values to a form
pub fn map_to_form<T: Form>(map: &HashMap<String, FieldValue>) -> Result<T, String> {
    let form = T::default_values();

    for (field_name, value) in map {
        // For now, we'll just log the operation
        // In a real implementation, you'd need to mutate the form
        log::info!("Setting field {} to {:?}", field_name, value);
    }

    Ok(form)
}

/// Merge validation errors from multiple sources
pub fn merge_validation_errors(errors: Vec<ValidationErrors>) -> ValidationErrors {
    let mut merged = ValidationErrors::new();

    for error_set in errors {
        merged.merge(error_set);
    }

    merged
}

/// Check if a form has any validation errors
pub fn has_validation_errors<T: Form>(form: &T) -> bool {
    match form.validate() {
        Ok(_) => false,
        Err(errors) => !errors.is_empty(),
    }
}

/// Get all field names from a form
pub fn get_form_field_names<T: Form>() -> Vec<String> {
    T::field_metadata()
        .into_iter()
        .map(|meta| meta.name)
        .collect()
}

/// Get required field names from a form
pub fn get_required_field_names<T: Form>() -> Vec<String> {
    T::field_metadata()
        .into_iter()
        .filter(|meta| meta.is_required)
        .map(|meta| meta.name)
        .collect()
}

/// Check if a field is required
pub fn is_field_required<T: Form>(field_name: &str) -> bool {
    T::field_metadata()
        .into_iter()
        .any(|meta| meta.name == field_name && meta.is_required)
}

/// Get field type for a specific field
pub fn get_field_type<T: Form>(field_name: &str) -> Option<crate::core::types::FieldType> {
    T::field_metadata()
        .into_iter()
        .find(|meta| meta.name == field_name)
        .map(|meta| meta.field_type)
}

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
fn validate_value_against_validator(
    value: &FieldValue,
    validator: &crate::validation::Validator,
) -> Result<(), String> {
    use crate::validation::Validator;

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

/// Serialize a form to JSON
pub fn serialize_form<T: Form>(form: &T) -> Result<String, String> {
    serde_json::to_string(form).map_err(|e| format!("Failed to serialize form: {}", e))
}

/// Deserialize a form from JSON
pub fn deserialize_form<T: Form>(json: &str) -> Result<T, String> {
    serde_json::from_str(json).map_err(|e| format!("Failed to deserialize form: {}", e))
}

/// Create a form from a JSON object
pub fn form_from_json<T: Form>(json: serde_json::Value) -> Result<T, String> {
    serde_json::from_value(json).map_err(|e| format!("Failed to create form from JSON: {}", e))
}

/// Convert a form to a JSON object
pub fn form_to_json<T: Form>(form: &T) -> Result<serde_json::Value, String> {
    serde_json::to_value(form).map_err(|e| format!("Failed to convert form to JSON: {}", e))
}

/// Check if two forms are equal
pub fn forms_are_equal<T: Form + PartialEq>(form1: &T, form2: &T) -> bool {
    form1 == form2
}

/// Get form statistics
pub fn get_form_stats<T: Form>(_form: &T) -> FormStats {
    let metadata = T::field_metadata();
    let total_fields = metadata.len();
    let required_fields = metadata.iter().filter(|meta| meta.is_required).count();
    let optional_fields = total_fields - required_fields;

    FormStats {
        total_fields,
        required_fields,
        optional_fields,
    }
}

/// Form statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormStats {
    pub total_fields: usize,
    pub required_fields: usize,
    pub optional_fields: usize,
}

/// Form validation result
#[derive(Debug, Clone)]
pub struct FormValidationResult {
    pub is_valid: bool,
    pub errors: ValidationErrors,
    pub field_count: usize,
    pub error_count: usize,
}

impl Default for FormValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

impl FormValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: ValidationErrors::new(),
            field_count: 0,
            error_count: 0,
        }
    }

    pub fn with_errors(errors: ValidationErrors) -> Self {
        let error_count = errors.field_errors.len() + errors.form_errors.len();
        Self {
            is_valid: error_count == 0,
            errors,
            field_count: 0,
            error_count,
        }
    }

    pub fn with_field_count(field_count: usize) -> Self {
        Self {
            is_valid: true,
            errors: ValidationErrors::new(),
            field_count,
            error_count: 0,
        }
    }
}

/// Validate a form and return detailed results
pub fn validate_form_detailed<T: Form>(form: &T) -> FormValidationResult {
    let field_count = T::field_metadata().len();
    let validation_result = form.validate();

    match validation_result {
        Ok(_) => FormValidationResult::with_field_count(field_count),
        Err(errors) => {
            let mut result = FormValidationResult::with_errors(errors);
            result.field_count = field_count;
            result
        }
    }
}
