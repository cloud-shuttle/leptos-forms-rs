//! Form options module - Form configuration and options
//!
//! This module provides form configuration types including ValidationMode,
//! SubmissionMode, PersistenceOptions, AnalyticsOptions, and FormSubmissionResult.

use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::form_error::FormError;

/// Type alias for custom tracking functions
pub type TrackingFunction = Box<dyn Fn(&str, &str, &str) + Send + Sync>;

/// Form validation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationMode {
    /// Validate on field blur
    OnBlur,
    /// Validate on field change
    OnChange,
    /// Validate on form submission only
    OnSubmit,
    /// Validate on both blur and change
    OnBlurAndChange,
}

impl ValidationMode {
    /// Get the display name for the validation mode
    pub fn display_name(&self) -> &'static str {
        match self {
            ValidationMode::OnBlur => "On Blur",
            ValidationMode::OnChange => "On Change",
            ValidationMode::OnSubmit => "On Submit",
            ValidationMode::OnBlurAndChange => "On Blur and Change",
        }
    }

    /// Check if validation should occur on blur
    pub fn validates_on_blur(&self) -> bool {
        matches!(
            self,
            ValidationMode::OnBlur | ValidationMode::OnBlurAndChange
        )
    }

    /// Check if validation should occur on change
    pub fn validates_on_change(&self) -> bool {
        matches!(
            self,
            ValidationMode::OnChange | ValidationMode::OnBlurAndChange
        )
    }

    /// Check if validation should occur on submit
    pub fn validates_on_submit(&self) -> bool {
        matches!(self, ValidationMode::OnSubmit)
    }
}

impl Default for ValidationMode {
    fn default() -> Self {
        ValidationMode::OnBlur
    }
}

/// Form submission mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubmissionMode {
    /// Submit immediately when form is valid
    Immediate,
    /// Submit only when explicitly triggered
    Manual,
    /// Submit with debouncing
    Debounced(Duration),
}

impl SubmissionMode {
    /// Get the display name for the submission mode
    pub fn display_name(&self) -> &'static str {
        match self {
            SubmissionMode::Immediate => "Immediate",
            SubmissionMode::Manual => "Manual",
            SubmissionMode::Debounced(_) => "Debounced",
        }
    }

    /// Check if submission is immediate
    pub fn is_immediate(&self) -> bool {
        matches!(self, SubmissionMode::Immediate)
    }

    /// Check if submission is manual
    pub fn is_manual(&self) -> bool {
        matches!(self, SubmissionMode::Manual)
    }

    /// Check if submission is debounced
    pub fn is_debounced(&self) -> bool {
        matches!(self, SubmissionMode::Debounced(_))
    }

    /// Get the debounce duration if applicable
    pub fn debounce_duration(&self) -> Option<Duration> {
        match self {
            SubmissionMode::Debounced(duration) => Some(*duration),
            _ => None,
        }
    }
}

impl Default for SubmissionMode {
    fn default() -> Self {
        SubmissionMode::Manual
    }
}

/// Form state persistence options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceOptions {
    pub enabled: bool,
    pub storage_key: Option<String>,
    pub auto_save: bool,
    pub auto_save_interval: Option<Duration>,
    pub clear_on_submit: bool,
}

impl PersistenceOptions {
    /// Create new persistence options
    pub fn new() -> Self {
        Self {
            enabled: false,
            storage_key: None,
            auto_save: false,
            auto_save_interval: None,
            clear_on_submit: true,
        }
    }

    /// Enable persistence
    pub fn enabled(mut self) -> Self {
        self.enabled = true;
        self
    }

    /// Set the storage key
    pub fn with_storage_key(mut self, key: String) -> Self {
        self.storage_key = Some(key);
        self
    }

    /// Enable auto-save
    pub fn with_auto_save(mut self) -> Self {
        self.auto_save = true;
        self
    }

    /// Set the auto-save interval
    pub fn with_auto_save_interval(mut self, interval: Duration) -> Self {
        self.auto_save_interval = Some(interval);
        self
    }

    /// Set whether to clear on submit
    pub fn with_clear_on_submit(mut self, clear: bool) -> Self {
        self.clear_on_submit = clear;
        self
    }

    /// Check if persistence is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if auto-save is enabled
    pub fn is_auto_save_enabled(&self) -> bool {
        self.auto_save && self.enabled
    }

    /// Get the storage key
    pub fn storage_key(&self) -> Option<&str> {
        self.storage_key.as_deref()
    }

    /// Get the auto-save interval
    pub fn auto_save_interval(&self) -> Option<Duration> {
        self.auto_save_interval
    }

    /// Check if data should be cleared on submit
    pub fn should_clear_on_submit(&self) -> bool {
        self.clear_on_submit
    }
}

impl Default for PersistenceOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Form analytics options
pub struct AnalyticsOptions {
    pub enabled: bool,
    pub track_field_interactions: bool,
    pub track_validation_errors: bool,
    pub track_submission_attempts: bool,
    pub custom_tracking: Option<TrackingFunction>,
}

impl AnalyticsOptions {
    /// Create new analytics options
    pub fn new() -> Self {
        Self {
            enabled: false,
            track_field_interactions: true,
            track_validation_errors: true,
            track_submission_attempts: true,
            custom_tracking: None,
        }
    }

    /// Enable analytics
    pub fn enabled(mut self) -> Self {
        self.enabled = true;
        self
    }

    /// Set field interaction tracking
    pub fn with_field_interaction_tracking(mut self, enabled: bool) -> Self {
        self.track_field_interactions = enabled;
        self
    }

    /// Set validation error tracking
    pub fn with_validation_error_tracking(mut self, enabled: bool) -> Self {
        self.track_validation_errors = enabled;
        self
    }

    /// Set submission attempt tracking
    pub fn with_submission_attempt_tracking(mut self, enabled: bool) -> Self {
        self.track_submission_attempts = enabled;
        self
    }

    /// Set custom tracking function
    pub fn with_custom_tracking(mut self, tracking: TrackingFunction) -> Self {
        self.custom_tracking = Some(tracking);
        self
    }

    /// Check if analytics is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if field interactions are tracked
    pub fn tracks_field_interactions(&self) -> bool {
        self.track_field_interactions && self.enabled
    }

    /// Check if validation errors are tracked
    pub fn tracks_validation_errors(&self) -> bool {
        self.track_validation_errors && self.enabled
    }

    /// Check if submission attempts are tracked
    pub fn tracks_submission_attempts(&self) -> bool {
        self.track_submission_attempts && self.enabled
    }

    /// Check if custom tracking is available
    pub fn has_custom_tracking(&self) -> bool {
        self.custom_tracking.is_some() && self.enabled
    }
}

impl Default for AnalyticsOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for AnalyticsOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnalyticsOptions")
            .field("enabled", &self.enabled)
            .field("track_field_interactions", &self.track_field_interactions)
            .field("track_validation_errors", &self.track_validation_errors)
            .field("track_submission_attempts", &self.track_submission_attempts)
            .field(
                "custom_tracking",
                &if self.custom_tracking.is_some() {
                    "Some(Fn)"
                } else {
                    "None"
                },
            )
            .finish()
    }
}

impl Clone for AnalyticsOptions {
    fn clone(&self) -> Self {
        Self {
            enabled: self.enabled,
            track_field_interactions: self.track_field_interactions,
            track_validation_errors: self.track_validation_errors,
            track_submission_attempts: self.track_submission_attempts,
            custom_tracking: None, // Cannot clone the function, so we set it to None
        }
    }
}

/// Form submission result
#[derive(Debug, Clone)]
pub struct FormSubmissionResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub errors: Vec<FormError>,
    pub warnings: Vec<String>,
}

impl<T> FormSubmissionResult<T> {
    /// Create a successful submission result
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Create a failed submission result
    pub fn failure(errors: Vec<FormError>) -> Self {
        Self {
            success: false,
            data: None,
            errors,
            warnings: Vec::new(),
        }
    }

    /// Add warnings to the result
    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings = warnings;
        self
    }

    /// Check if the submission was successful
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Check if the submission failed
    pub fn is_failure(&self) -> bool {
        !self.success
    }

    /// Get the submitted data
    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    /// Get the errors
    pub fn errors(&self) -> &[FormError] {
        &self.errors
    }

    /// Get the warnings
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Get the number of errors
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Get the number of warnings
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }
}
