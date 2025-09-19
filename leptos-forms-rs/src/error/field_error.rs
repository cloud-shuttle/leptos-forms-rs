//! Field error module - Field-specific error handling
//!
//! This module provides the FieldError struct for field-specific errors,
//! including error creation and manipulation, error code management,
//! and display and Debug implementations.

use std::error::Error as StdError;
use std::fmt;

/// Field-specific error
#[derive(Debug, Clone)]
pub struct FieldError {
    pub field: String,
    pub message: String,
    pub code: Option<String>,
}

impl FieldError {
    /// Create a new field error
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            code: None,
        }
    }

    /// Create a new field error with code
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Get the field name
    pub fn field_name(&self) -> &str {
        &self.field
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the error code
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Check if this error has a code
    pub fn has_code(&self) -> bool {
        self.code.is_some()
    }

    /// Set the error code
    pub fn set_code(&mut self, code: impl Into<String>) {
        self.code = Some(code.into());
    }

    /// Clear the error code
    pub fn clear_code(&mut self) {
        self.code = None;
    }

    /// Update the error message
    pub fn update_message(&mut self, message: impl Into<String>) {
        self.message = message.into();
    }

    /// Update the field name
    pub fn update_field(&mut self, field: impl Into<String>) {
        self.field = field.into();
    }

    /// Clone with a new field name
    pub fn with_field(&self, field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: self.message.clone(),
            code: self.code.clone(),
        }
    }

    /// Clone with a new message
    pub fn with_message(&self, message: impl Into<String>) -> Self {
        Self {
            field: self.field.clone(),
            message: message.into(),
            code: self.code.clone(),
        }
    }

    /// Clone with a new code
    pub fn with_error_code(&self, code: impl Into<String>) -> Self {
        Self {
            field: self.field.clone(),
            message: self.message.clone(),
            code: Some(code.into()),
        }
    }
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

impl StdError for FieldError {}

impl PartialEq for FieldError {
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field && self.message == other.message && self.code == other.code
    }
}

impl Eq for FieldError {}
