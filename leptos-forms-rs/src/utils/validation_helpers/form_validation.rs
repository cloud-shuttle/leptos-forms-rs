//! Form validation utilities
//!
//! This module provides utilities for form-level validation operations.

use crate::core::traits::Form;
use crate::validation::ValidationErrors;

/// Check if a form has any validation errors
pub fn has_validation_errors<T: Form>(form: &T) -> bool {
    match form.validate() {
        Ok(_) => false,
        Err(errors) => !errors.is_empty(),
    }
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
