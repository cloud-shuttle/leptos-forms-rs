//! Form error module - Form-level error types
//!
//! This module provides FormError and FieldError types for form-level error handling.

/// Field-specific error
#[derive(Debug, Clone)]
pub struct FieldError {
    pub field: String,
    pub message: String,
    pub code: Option<String>,
}

impl FieldError {
    /// Create a new field error
    pub fn new(field: String, message: String) -> Self {
        Self {
            field,
            message,
            code: None,
        }
    }

    /// Create a new field error with code
    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
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
}

impl std::fmt::Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

impl std::error::Error for FieldError {}

/// Form-level error
#[derive(Debug, Clone)]
pub struct FormError {
    pub message: String,
    pub code: Option<String>,
    pub field_errors: Vec<FieldError>,
}

impl FormError {
    /// Create a new form error
    pub fn new(message: String) -> Self {
        Self {
            message,
            code: None,
            field_errors: Vec::new(),
        }
    }

    /// Create a new form error with code
    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    /// Add field errors to the form error
    pub fn with_field_errors(mut self, field_errors: Vec<FieldError>) -> Self {
        self.field_errors = field_errors;
        self
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the error code
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Get the field errors
    pub fn field_errors(&self) -> &[FieldError] {
        &self.field_errors
    }

    /// Check if there are field errors
    pub fn has_field_errors(&self) -> bool {
        !self.field_errors.is_empty()
    }
}

impl std::fmt::Display for FormError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FormError {}
