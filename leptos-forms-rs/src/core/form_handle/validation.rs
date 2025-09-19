//! Validation operations module
//!
//! This module provides validation functionality for the FormHandle including
//! field validation, form validation, and error handling.

use crate::core::traits::Form;
use crate::core::types::FieldValue;
use crate::error::FormError;
use crate::validation::{ValidationErrors, ValidationRuleEngine, Validator};
use leptos::prelude::{Get, GetUntracked, Set};

use super::core::FormHandle;

impl<T: Form + Send + Sync + PartialEq> FormHandle<T> {
    /// Validate a specific field
    pub fn validate_field(&self, field_name: &str) -> Result<(), ValidationErrors> {
        let state = self.state.get();
        let form_data = &state.values;
        let binding = T::schema();
        let metadata = binding.get_field(field_name);

        if let Some(field_meta) = metadata {
            let _default_value = FieldValue::String(String::new());
            let field_value = form_data.get_field_value(field_name);

            // Validate field
            for validator in &field_meta.validators {
                if let Err(error) = self.validate_field_value(field_value.clone(), validator) {
                    let mut errors = ValidationErrors::new();
                    errors.add_field_error(field_name, error);
                    return Err(errors);
                }
            }
        }

        Ok(())
    }

    /// Validate the entire form
    pub fn validate(&self) -> Result<(), FormError> {
        let state = self.state.get_untracked();
        let form_data = &state.values;
        let metadata = T::schema().field_metadata.clone();
        let mut errors = ValidationErrors::new();

        for field_meta in metadata {
            let field_name = &field_meta.name;
            let field_value = form_data.get_field_value(field_name);

            // Validate field
            for validator in &field_meta.validators {
                if let Err(error) = self.validate_field_value(field_value.clone(), validator) {
                    errors.add_field_error(field_name, error);
                }
            }
        }

        // Also call the form's own validate method for custom validation logic
        if let Err(form_errors) = form_data.validate() {
            for (field_name, error_msgs) in &form_errors.field_errors {
                for error_msg in error_msgs {
                    errors.add_field_error(&field_name, error_msg.clone());
                }
            }
        }

        if errors.has_errors() {
            // Update state with errors
            let new_state = state.with_errors(errors.clone());
            self.state.set(new_state);

            Err(FormError::validation_error(
                "Form validation failed".to_string(),
                errors.to_field_errors(),
            ))
        } else {
            // Clear any existing errors
            let new_state = state.with_errors(ValidationErrors::new());
            self.state.set(new_state);
            Ok(())
        }
    }

    /// Clear all validation errors
    pub fn clear_errors(&self) {
        let state = self.state.get_untracked();
        let new_state = state.with_errors(ValidationErrors::new());
        self.state.set(new_state);
    }

    /// Clear errors for a specific field
    pub fn clear_field_errors(&self, field_name: &str) {
        let state = self.state.get_untracked();
        let mut new_errors = state.errors.clone();
        new_errors.remove_field_error(field_name);
        let new_state = state.with_errors(new_errors);
        self.state.set(new_state);
    }

    /// Get validation errors for a specific field
    pub fn get_field_errors(&self, field_name: &str) -> Vec<String> {
        let state = self.state.get_untracked();
        state
            .errors
            .get_field_error(field_name)
            .cloned()
            .unwrap_or_default()
    }

    /// Check if a field has validation errors
    pub fn has_field_errors(&self, field_name: &str) -> bool {
        !self.get_field_errors(field_name).is_empty()
    }

    /// Get all validation errors
    pub fn get_all_errors(&self) -> ValidationErrors {
        let state = self.state.get_untracked();
        state.errors
    }

    /// Check if the form has any validation errors
    pub fn has_validation_errors(&self) -> bool {
        let state = self.state.get_untracked();
        state.errors.has_errors()
    }

    /// Validate a field value against a specific validator
    pub fn validate_field_with_validator(
        &self,
        field_name: &str,
        validator: &Validator,
    ) -> Result<(), String> {
        let field_value = self.get_field_value(field_name).unwrap_or(FieldValue::Null);
        self.validate_field_value(field_value, validator)
    }

    /// Validate multiple fields at once
    pub fn validate_fields(&self, field_names: &[&str]) -> ValidationErrors {
        let mut errors = ValidationErrors::new();

        for field_name in field_names {
            if let Err(field_errors) = self.validate_field(field_name) {
                for (error_field, error_messages) in field_errors.field_errors {
                    for error_message in error_messages {
                        errors.add_field_error(&error_field, error_message);
                    }
                }
            }
        }

        errors
    }

    /// Validate all required fields
    pub fn validate_required_fields(&self) -> ValidationErrors {
        let schema = T::schema();
        let required_fields: Vec<&str> = schema
            .field_metadata
            .iter()
            .filter(|field| field.is_required)
            .map(|field| field.name.as_str())
            .collect();

        self.validate_fields(&required_fields)
    }

    /// Validate fields based on their field type
    pub fn validate_fields_by_type(
        &self,
        field_type: &crate::core::types::FieldType,
    ) -> ValidationErrors {
        let schema = T::schema();
        let fields_of_type: Vec<&str> = schema
            .field_metadata
            .iter()
            .filter(|field| &field.field_type == field_type)
            .map(|field| field.name.as_str())
            .collect();

        self.validate_fields(&fields_of_type)
    }

    /// Check if a field passes a specific validation rule
    pub fn field_passes_validation(&self, field_name: &str, validator: &Validator) -> bool {
        self.validate_field_with_validator(field_name, validator)
            .is_ok()
    }

    /// Get validation summary for the form
    pub fn get_validation_summary(&self) -> ValidationSummary {
        let state = self.state.get_untracked();
        let schema = T::schema();

        let total_fields: usize = schema.field_metadata.len();
        let fields_with_errors: usize = state.errors.field_errors.len();
        let valid_fields: usize = total_fields - fields_with_errors;

        ValidationSummary {
            total_fields,
            valid_fields,
            fields_with_errors,
            is_valid: fields_with_errors == 0,
            error_count: state.errors.error_count(),
        }
    }

    /// Private helper to validate a field value against a validator
    fn validate_field_value(&self, value: FieldValue, validator: &Validator) -> Result<(), String> {
        let mut engine = ValidationRuleEngine::new();
        engine.add_validator(validator.clone());

        match engine.validate_value(value) {
            Ok(_) => Ok(()),
            Err(errors) => {
                if let Some(error) = errors.get_field_error("").and_then(|v| v.first()) {
                    Err(error.clone())
                } else {
                    Err("Validation failed".to_string())
                }
            }
        }
    }
}

/// Validation summary for form validation results
#[derive(Debug, Clone)]
pub struct ValidationSummary {
    pub total_fields: usize,
    pub valid_fields: usize,
    pub fields_with_errors: usize,
    pub is_valid: bool,
    pub error_count: usize,
}

impl ValidationSummary {
    /// Get the validation success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_fields == 0 {
            100.0
        } else {
            (self.valid_fields as f64 / self.total_fields as f64) * 100.0
        }
    }

    /// Get the validation failure rate as a percentage
    pub fn failure_rate(&self) -> f64 {
        100.0 - self.success_rate()
    }

    /// Check if validation passed
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// Check if validation failed
    pub fn is_invalid(&self) -> bool {
        !self.is_valid
    }

    /// Get a human-readable summary
    pub fn to_string(&self) -> String {
        if self.is_valid {
            format!("All {} fields are valid", self.total_fields)
        } else {
            format!(
                "{} of {} fields have errors ({} total errors)",
                self.fields_with_errors, self.total_fields, self.error_count
            )
        }
    }
}
