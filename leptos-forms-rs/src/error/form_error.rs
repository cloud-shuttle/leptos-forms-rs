//! Form error module - Core form error types and handling
//!
//! This module provides the main FormError enum with all variants,
//! error creation methods, error classification methods, and error conversion utilities.

use std::error::Error as StdError;
use std::fmt;

/// Main error type for the Leptos Forms library
#[derive(Debug, Clone)]
pub enum FormError {
    /// Field-specific validation error
    FieldError {
        field: String,
        message: String,
        code: Option<String>,
    },
    /// Form-level validation error
    ValidationError {
        message: String,
        field_errors: Vec<super::field_error::FieldError>,
    },
    /// Serialization/deserialization error
    SerializationError {
        message: String,
        field: Option<String>,
    },
    /// Form submission error
    SubmissionError {
        message: String,
        status_code: Option<u16>,
        response: Option<String>,
    },
    /// Form state management error
    StateError { message: String, operation: String },
    /// Persistence/storage error
    PersistenceError {
        message: String,
        storage_type: String,
    },
    /// Configuration error
    ConfigurationError { message: String, component: String },
    /// Unknown or unexpected error
    Unknown {
        message: String,
        source: Option<String>,
    },
}

impl FormError {
    /// Create a new field error
    pub fn field_error(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::FieldError {
            field: field.into(),
            message: message.into(),
            code: None,
        }
    }

    /// Convert from types::FieldError to error::FormError
    pub fn from_field_error(field_error: crate::core::types::FieldError) -> Self {
        Self::FieldError {
            field: field_error.field,
            message: field_error.message,
            code: field_error.code,
        }
    }

    /// Create a new field error with error code
    pub fn field_error_with_code(
        field: impl Into<String>,
        message: impl Into<String>,
        code: impl Into<String>,
    ) -> Self {
        Self::FieldError {
            field: field.into(),
            message: message.into(),
            code: Some(code.into()),
        }
    }

    /// Create a new validation error
    pub fn validation_error(
        message: impl Into<String>,
        field_errors: Vec<super::field_error::FieldError>,
    ) -> Self {
        Self::ValidationError {
            message: message.into(),
            field_errors,
        }
    }

    /// Create a new serialization error
    pub fn serialization_error(message: impl Into<String>, field: Option<String>) -> Self {
        Self::SerializationError {
            message: message.into(),
            field,
        }
    }

    /// Create a new submission error
    pub fn submission_error(
        message: impl Into<String>,
        status_code: Option<u16>,
        response: Option<String>,
    ) -> Self {
        Self::SubmissionError {
            message: message.into(),
            status_code,
            response,
        }
    }

    /// Create a new state error
    pub fn state_error(message: impl Into<String>, operation: impl Into<String>) -> Self {
        Self::StateError {
            message: message.into(),
            operation: operation.into(),
        }
    }

    /// Create a new persistence error
    pub fn persistence_error(message: impl Into<String>, storage_type: impl Into<String>) -> Self {
        Self::PersistenceError {
            message: message.into(),
            storage_type: storage_type.into(),
        }
    }

    /// Create a new configuration error
    pub fn configuration_error(message: impl Into<String>, component: impl Into<String>) -> Self {
        Self::ConfigurationError {
            message: message.into(),
            component: component.into(),
        }
    }

    /// Create a new unknown error
    pub fn unknown(message: impl Into<String>, source: Option<String>) -> Self {
        Self::Unknown {
            message: message.into(),
            source,
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        match self {
            Self::FieldError { message, .. } => message,
            Self::ValidationError { message, .. } => message,
            Self::SerializationError { message, .. } => message,
            Self::SubmissionError { message, .. } => message,
            Self::StateError { message, .. } => message,
            Self::PersistenceError { message, .. } => message,
            Self::ConfigurationError { message, .. } => message,
            Self::Unknown { message, .. } => message,
        }
    }

    /// Check if this is a field error
    pub fn is_field_error(&self) -> bool {
        matches!(self, Self::FieldError { .. })
    }

    /// Check if this is a validation error
    pub fn is_validation_error(&self) -> bool {
        matches!(self, Self::ValidationError { .. })
    }

    /// Check if this is a submission error
    pub fn is_submission_error(&self) -> bool {
        matches!(self, Self::SubmissionError { .. })
    }

    /// Get the field name if this is a field error
    pub fn field_name(&self) -> Option<&str> {
        match self {
            Self::FieldError { field, .. } => Some(field),
            Self::SerializationError { field, .. } => field.as_deref(),
            _ => None,
        }
    }

    /// Get the error code if available
    pub fn error_code(&self) -> Option<&str> {
        match self {
            Self::FieldError { code, .. } => code.as_deref(),
            _ => None,
        }
    }

    /// Convert to a field error if possible
    pub fn as_field_error(&self) -> Option<super::field_error::FieldError> {
        match self {
            Self::FieldError {
                field,
                message,
                code,
            } => Some(super::field_error::FieldError {
                field: field.clone(),
                message: message.clone(),
                code: code.clone(),
            }),
            _ => None,
        }
    }
}

impl fmt::Display for FormError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FieldError { field, message, .. } => {
                write!(f, "Field '{}': {}", field, message)
            }
            Self::ValidationError {
                message,
                field_errors,
            } => {
                write!(f, "Validation error: {}", message)?;
                if !field_errors.is_empty() {
                    write!(f, " ({} field errors)", field_errors.len())?;
                }
                Ok(())
            }
            Self::SerializationError { message, field } => {
                write!(f, "Serialization error: {}", message)?;
                if let Some(field) = field {
                    write!(f, " (field: {})", field)?;
                }
                Ok(())
            }
            Self::SubmissionError {
                message,
                status_code,
                ..
            } => {
                write!(f, "Submission error: {}", message)?;
                if let Some(code) = status_code {
                    write!(f, " (status: {})", code)?;
                }
                Ok(())
            }
            Self::StateError { message, operation } => {
                write!(f, "State error during {}: {}", operation, message)
            }
            Self::PersistenceError {
                message,
                storage_type,
            } => {
                write!(f, "Persistence error ({}): {}", storage_type, message)
            }
            Self::ConfigurationError { message, component } => {
                write!(f, "Configuration error in {}: {}", component, message)
            }
            Self::Unknown { message, source } => {
                write!(f, "Unknown error: {}", message)?;
                if let Some(source) = source {
                    write!(f, " (source: {})", source)?;
                }
                Ok(())
            }
        }
    }
}

impl StdError for FormError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

/// Error result type for form operations
pub type FormResult<T> = Result<T, FormError>;
