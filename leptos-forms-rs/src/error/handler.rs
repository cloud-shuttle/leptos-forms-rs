//! Error handler module - Error handling strategies and implementations
//!
//! This module provides the ErrorHandler trait and DefaultErrorHandler implementation,
//! error handling strategies, logging and reporting decisions.

use super::{ErrorContext, FormError};

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

/// Silent error handler that doesn't log or report errors
pub struct SilentErrorHandler;

impl ErrorHandler for SilentErrorHandler {
    fn handle_error(&self, _error: &FormError, _context: &ErrorContext) {
        // Do nothing - silent handler
    }

    fn should_log_error(&self, _error: &FormError) -> bool {
        false
    }

    fn should_report_error(&self, _error: &FormError) -> bool {
        false
    }
}

/// Verbose error handler that logs and reports all errors
pub struct VerboseErrorHandler;

impl ErrorHandler for VerboseErrorHandler {
    fn handle_error(&self, error: &FormError, context: &ErrorContext) {
        log::error!("Form error: {} (context: {:?})", error, context);

        // Also log to debug level for more details
        log::debug!("Error details: {:?}", error);
        log::debug!("Context details: {:?}", context);
    }

    fn should_log_error(&self, _error: &FormError) -> bool {
        true
    }

    fn should_report_error(&self, _error: &FormError) -> bool {
        true
    }
}

/// Selective error handler that only handles certain types of errors
pub struct SelectiveErrorHandler {
    log_field_errors: bool,
    log_validation_errors: bool,
    log_submission_errors: bool,
    log_state_errors: bool,
    log_persistence_errors: bool,
    log_configuration_errors: bool,
    log_unknown_errors: bool,
    report_critical_errors: bool,
}

impl SelectiveErrorHandler {
    /// Create a new selective error handler
    pub fn new() -> Self {
        Self {
            log_field_errors: false,
            log_validation_errors: true,
            log_submission_errors: true,
            log_state_errors: true,
            log_persistence_errors: true,
            log_configuration_errors: true,
            log_unknown_errors: true,
            report_critical_errors: true,
        }
    }

    /// Enable logging for field errors
    pub fn with_field_error_logging(mut self) -> Self {
        self.log_field_errors = true;
        self
    }

    /// Disable logging for validation errors
    pub fn without_validation_error_logging(mut self) -> Self {
        self.log_validation_errors = false;
        self
    }

    /// Disable reporting for critical errors
    pub fn without_critical_error_reporting(mut self) -> Self {
        self.report_critical_errors = false;
        self
    }
}

impl Default for SelectiveErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorHandler for SelectiveErrorHandler {
    fn handle_error(&self, error: &FormError, context: &ErrorContext) {
        let should_log = match error {
            FormError::FieldError { .. } => self.log_field_errors,
            FormError::ValidationError { .. } => self.log_validation_errors,
            FormError::SubmissionError { .. } => self.log_submission_errors,
            FormError::StateError { .. } => self.log_state_errors,
            FormError::PersistenceError { .. } => self.log_persistence_errors,
            FormError::ConfigurationError { .. } => self.log_configuration_errors,
            FormError::Unknown { .. } => self.log_unknown_errors,
            FormError::SerializationError { .. } => true, // Always log serialization errors
        };

        if should_log {
            log::error!("Form error: {} (context: {:?})", error, context);
        }
    }

    fn should_log_error(&self, error: &FormError) -> bool {
        match error {
            FormError::FieldError { .. } => self.log_field_errors,
            FormError::ValidationError { .. } => self.log_validation_errors,
            FormError::SubmissionError { .. } => self.log_submission_errors,
            FormError::StateError { .. } => self.log_state_errors,
            FormError::PersistenceError { .. } => self.log_persistence_errors,
            FormError::ConfigurationError { .. } => self.log_configuration_errors,
            FormError::Unknown { .. } => self.log_unknown_errors,
            FormError::SerializationError { .. } => true, // Always log serialization errors
        }
    }

    fn should_report_error(&self, error: &FormError) -> bool {
        if !self.report_critical_errors {
            return false;
        }

        match error {
            FormError::FieldError { .. } => false, // Don't report field errors
            FormError::ValidationError { .. } => false, // Don't report validation errors
            FormError::SubmissionError { .. } => true, // Report submission errors
            FormError::StateError { .. } => true,  // Report state errors
            FormError::PersistenceError { .. } => true, // Report persistence errors
            FormError::ConfigurationError { .. } => true, // Report configuration errors
            FormError::Unknown { .. } => true,     // Report unknown errors
            FormError::SerializationError { .. } => true, // Report serialization errors
        }
    }
}
