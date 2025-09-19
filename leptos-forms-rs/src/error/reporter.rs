//! Error reporter module - Error reporting to external services
//!
//! This module provides the ErrorReporter trait and implementations,
//! error reporting strategies, and external service integration points.

use super::{ErrorContext, FormError};

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

/// File error reporter that writes errors to a file
pub struct FileErrorReporter {
    file_path: String,
}

impl FileErrorReporter {
    /// Create a new file error reporter
    pub fn new(file_path: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }

    /// Get the file path
    pub fn file_path(&self) -> &str {
        &self.file_path
    }
}

impl ErrorReporter for FileErrorReporter {
    fn report_error(&self, error: &FormError, context: &ErrorContext) {
        // In a real implementation, this would write to a file
        // For now, we'll just log it
        log::error!(
            "Error reported to file {}: {} (context: {:?})",
            self.file_path,
            error,
            context
        );
    }
}

/// HTTP error reporter that sends errors to an HTTP endpoint
pub struct HttpErrorReporter {
    endpoint: String,
    api_key: Option<String>,
}

impl HttpErrorReporter {
    /// Create a new HTTP error reporter
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            api_key: None,
        }
    }

    /// Create a new HTTP error reporter with API key
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Get the endpoint
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Check if API key is set
    pub fn has_api_key(&self) -> bool {
        self.api_key.is_some()
    }
}

impl ErrorReporter for HttpErrorReporter {
    fn report_error(&self, error: &FormError, context: &ErrorContext) {
        // In a real implementation, this would make an HTTP request
        // For now, we'll just log it
        log::error!(
            "Error reported to HTTP endpoint {}: {} (context: {:?})",
            self.endpoint,
            error,
            context
        );
    }
}

/// Composite error reporter that reports to multiple services
pub struct CompositeErrorReporter {
    reporters: Vec<Box<dyn ErrorReporter + Send + Sync>>,
}

impl CompositeErrorReporter {
    /// Create a new composite error reporter
    pub fn new() -> Self {
        Self {
            reporters: Vec::new(),
        }
    }

    /// Add a reporter to the composite
    pub fn add_reporter(mut self, reporter: Box<dyn ErrorReporter + Send + Sync>) -> Self {
        self.reporters.push(reporter);
        self
    }

    /// Get the number of reporters
    pub fn reporter_count(&self) -> usize {
        self.reporters.len()
    }
}

impl Default for CompositeErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorReporter for CompositeErrorReporter {
    fn report_error(&self, error: &FormError, context: &ErrorContext) {
        for reporter in &self.reporters {
            reporter.report_error(error, context);
        }
    }
}

/// Utility functions for error handling
pub mod utils {
    use super::*;
    use crate::error::field_error;

    /// Create a field error from a validation error
    pub fn field_error_from_validation(field: &str, message: &str) -> FormError {
        FormError::field_error(field, message)
    }

    /// Create a validation error from multiple field errors
    pub fn validation_error_from_fields(
        message: &str,
        field_errors: Vec<field_error::FieldError>,
    ) -> FormError {
        FormError::validation_error(message, field_errors)
    }

    /// Extract field errors from a form error
    pub fn extract_field_errors(error: &FormError) -> Vec<field_error::FieldError> {
        match error {
            FormError::ValidationError { field_errors, .. } => field_errors.clone(),
            FormError::FieldError {
                field,
                message,
                code,
            } => {
                vec![field_error::FieldError {
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
        matches!(
            error,
            FormError::FieldError { .. }
                | FormError::ValidationError { .. }
                | FormError::SerializationError { .. }
        )
    }

    /// Check if an error is critical
    pub fn is_critical_error(error: &FormError) -> bool {
        matches!(
            error,
            FormError::StateError { .. } | FormError::ConfigurationError { .. }
        )
    }

    /// Get error severity level
    pub fn get_error_severity(error: &FormError) -> ErrorSeverity {
        match error {
            FormError::FieldError { .. } => ErrorSeverity::Low,
            FormError::ValidationError { .. } => ErrorSeverity::Medium,
            FormError::SerializationError { .. } => ErrorSeverity::Medium,
            FormError::SubmissionError { .. } => ErrorSeverity::High,
            FormError::StateError { .. } => ErrorSeverity::Critical,
            FormError::PersistenceError { .. } => ErrorSeverity::High,
            FormError::ConfigurationError { .. } => ErrorSeverity::Critical,
            FormError::Unknown { .. } => ErrorSeverity::High,
        }
    }

    /// Format error for logging
    pub fn format_error_for_logging(error: &FormError, context: &ErrorContext) -> String {
        format!("Error: {} | Context: {}", error, context)
    }

    /// Format error for reporting
    pub fn format_error_for_reporting(error: &FormError, context: &ErrorContext) -> String {
        let severity = get_error_severity(error);
        format!("[{}] {} | Context: {}", severity, error, context)
    }
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSeverity::Low => write!(f, "LOW"),
            ErrorSeverity::Medium => write!(f, "MEDIUM"),
            ErrorSeverity::High => write!(f, "HIGH"),
            ErrorSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}
