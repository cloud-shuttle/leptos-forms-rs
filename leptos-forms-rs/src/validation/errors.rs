//! Validation errors module - Error handling and validation error types
//!
//! This module provides comprehensive error handling for form validation,
//! including error aggregation, field error management, and error display formatting.

use std::collections::HashMap;

/// Validation errors for a form
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationErrors {
    pub field_errors: HashMap<String, Vec<String>>,
    pub form_errors: Vec<String>,
}

impl Default for ValidationErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationErrors {
    /// Create a new empty ValidationErrors instance
    pub fn new() -> Self {
        Self {
            field_errors: HashMap::new(),
            form_errors: Vec::new(),
        }
    }

    /// Add a field error
    pub fn add_field_error(&mut self, field_name: &str, error: String) {
        self.field_errors
            .entry(field_name.to_string())
            .or_default()
            .push(error);
    }

    /// Add a form-level error
    pub fn add_form_error(&mut self, error: String) {
        self.form_errors.push(error);
    }

    /// Check if there are no errors
    pub fn is_empty(&self) -> bool {
        self.field_errors.is_empty() && self.form_errors.is_empty()
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.is_empty()
    }

    /// Check if a specific field has errors
    pub fn has_field_error(&self, field: &str) -> bool {
        self.field_errors.contains_key(field)
    }

    /// Get errors for a specific field
    pub fn get_field_error(&self, field: &str) -> Option<&Vec<String>> {
        self.field_errors.get(field)
    }

    /// Clear errors for a specific field
    pub fn clear_field(&mut self, field: &str) {
        self.field_errors.remove(field);
    }

    /// Remove field errors (alias for clear_field)
    pub fn remove_field_error(&mut self, field: &str) {
        self.field_errors.remove(field);
    }

    /// Convert to field errors for compatibility
    pub fn to_field_errors(&self) -> Vec<crate::error::FieldError> {
        let mut errors = Vec::new();
        for (field_name, field_errors) in &self.field_errors {
            for error_msg in field_errors {
                errors.push(crate::error::FieldError::new(
                    field_name.clone(),
                    error_msg.clone(),
                ));
            }
        }
        errors
    }

    /// Merge another ValidationErrors into this one
    pub fn merge(&mut self, other: ValidationErrors) {
        for (field, errors) in other.field_errors {
            self.field_errors.entry(field).or_default().extend(errors);
        }
        self.form_errors.extend(other.form_errors);
    }

    /// Get all field names that have errors
    pub fn get_error_fields(&self) -> Vec<String> {
        self.field_errors.keys().cloned().collect()
    }

    /// Get total error count
    pub fn error_count(&self) -> usize {
        let field_error_count: usize = self.field_errors.values().map(|v| v.len()).sum();
        field_error_count + self.form_errors.len()
    }

    /// Clear all errors
    pub fn clear_all(&mut self) {
        self.field_errors.clear();
        self.form_errors.clear();
    }
}

impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.form_errors.is_empty() {
            writeln!(f, "Form errors:")?;
            for error in &self.form_errors {
                writeln!(f, "  - {}", error)?;
            }
        }

        if !self.field_errors.is_empty() {
            writeln!(f, "Field errors:")?;
            for (field, errors) in &self.field_errors {
                writeln!(f, "  {}:", field)?;
                for error in errors {
                    writeln!(f, "    - {}", error)?;
                }
            }
        }

        Ok(())
    }
}

impl std::error::Error for ValidationErrors {}
