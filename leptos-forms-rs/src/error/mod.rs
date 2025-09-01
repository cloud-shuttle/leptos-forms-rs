use std::fmt;
use std::error::Error as StdError;

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
        field_errors: Vec<FieldError>,
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
    StateError {
        message: String,
        operation: String,
    },
    /// Persistence/storage error
    PersistenceError {
        message: String,
        storage_type: String,
    },
    /// Configuration error
    ConfigurationError {
        message: String,
        component: String,
    },
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
    pub fn field_error_with_code(field: impl Into<String>, message: impl Into<String>, code: impl Into<String>) -> Self {
        Self::FieldError {
            field: field.into(),
            message: message.into(),
            code: Some(code.into()),
        }
    }
    
    /// Create a new validation error
    pub fn validation_error(message: impl Into<String>, field_errors: Vec<FieldError>) -> Self {
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
    pub fn submission_error(message: impl Into<String>, status_code: Option<u16>, response: Option<String>) -> Self {
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
    pub fn as_field_error(&self) -> Option<FieldError> {
        match self {
            Self::FieldError { field, message, code } => Some(FieldError {
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
            Self::ValidationError { message, field_errors } => {
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
            Self::SubmissionError { message, status_code, .. } => {
                write!(f, "Submission error: {}", message)?;
                if let Some(code) = status_code {
                    write!(f, " (status: {})", code)?;
                }
                Ok(())
            }
            Self::StateError { message, operation } => {
                write!(f, "State error during {}: {}", operation, message)
            }
            Self::PersistenceError { message, storage_type } => {
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
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

impl StdError for FieldError {}

/// Error context for additional debugging information
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub form_name: Option<String>,
    pub field_name: Option<String>,
    pub operation: Option<String>,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub user_agent: Option<String>,
    pub additional_data: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    /// Create a new error context
    pub fn new() -> Self {
        Self {
            form_name: None,
            field_name: None,
            operation: None,
            timestamp: Some(chrono::Utc::now()),
            user_agent: None,
            additional_data: std::collections::HashMap::new(),
        }
    }
    
    /// Set the form name
    pub fn with_form_name(mut self, form_name: impl Into<String>) -> Self {
        self.form_name = Some(form_name.into());
        self
    }
    
    /// Set the field name
    pub fn with_field_name(mut self, field_name: impl Into<String>) -> Self {
        self.field_name = Some(field_name.into());
        self
    }
    
    /// Set the operation
    pub fn with_operation(mut self, operation: impl Into<String>) -> Self {
        self.operation = Some(operation.into());
        self
    }
    
    /// Set the user agent
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
    
    /// Add additional data
    pub fn with_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.additional_data.insert(key.into(), value.into());
        self
    }
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Error result type for form operations
pub type FormResult<T> = Result<T, FormError>;

/// Error handler trait for custom error handling
pub trait ErrorHandler {
    /// Handle a form error
    fn handle_error(&self, error: &FormError, context: &ErrorContext);
    
    /// Check if an error should be logged
    fn should_log_error(&self, error: &FormError) -> bool;
    
    /// Check if an error should be reported to external services
    fn should_report_error(&self, error: &FormError) -> bool;
}

/// Default error handler implementation
pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
    fn handle_error(&self, error: &FormError, context: &ErrorContext) {
        log::error!("Form error: {} (context: {:?})", error, context);
    }
    
    fn should_log_error(&self, _error: &FormError) -> bool {
        true
    }
    
    fn should_report_error(&self, error: &FormError) -> bool {
        // Report all errors except field validation errors
        !error.is_field_error()
    }
}

/// Error reporting service trait
pub trait ErrorReporter {
    /// Report an error to external service
    fn report_error(&self, error: &FormError, context: &ErrorContext);
}

/// Console error reporter (for development)
pub struct ConsoleErrorReporter;

impl ErrorReporter for ConsoleErrorReporter {
    fn report_error(&self, error: &FormError, context: &ErrorContext) {
        log::error!("Error reported: {} (context: {:?})", error, context);
    }
}

/// Error utilities
pub mod utils {
    use super::*;
    
    /// Create a field error from a validation error
    pub fn field_error_from_validation(field: &str, message: &str) -> FormError {
        FormError::field_error(field, message)
    }
    
    /// Create a validation error from multiple field errors
    pub fn validation_error_from_fields(message: &str, field_errors: Vec<FieldError>) -> FormError {
        FormError::validation_error(message, field_errors)
    }
    
    /// Extract field errors from a form error
    pub fn extract_field_errors(error: &FormError) -> Vec<FieldError> {
        match error {
            FormError::ValidationError { field_errors, .. } => field_errors.clone(),
            FormError::FieldError { field, message, code } => {
                vec![FieldError {
                    field: field.clone(),
                    message: message.clone(),
                    code: code.clone(),
                }]
            }
            _ => Vec::new(),
        }
    }
    
    /// Check if an error is recoverable
    pub fn is_recoverable_error(error: &FormError) -> bool {
        matches!(error, 
            FormError::FieldError { .. } | 
            FormError::ValidationError { .. } |
            FormError::SerializationError { .. }
        )
    }
    
    /// Check if an error is critical
    pub fn is_critical_error(error: &FormError) -> bool {
        matches!(error,
            FormError::StateError { .. } |
            FormError::ConfigurationError { .. }
        )
    }
}
