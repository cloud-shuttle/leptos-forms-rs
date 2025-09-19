//! Form lifecycle operations module
//!
//! This module provides form lifecycle management for the FormHandle including
//! form submission, reset, and state management operations.

use crate::core::traits::{Form, FormState};
use crate::error::FormError;
use leptos::prelude::{GetUntracked, Set};

use super::core::FormHandle;

impl<T: Form + Send + Sync + PartialEq> FormHandle<T> {
    /// Submit the form
    pub fn submit(&self) -> Result<T, FormError> {
        // Validate first
        self.validate()?;

        let state = self.state.get_untracked();
        let new_state = state.mark_submitting();
        self.state.set(new_state.clone());

        // In a real implementation, you would send the data here
        // For now, just return the values
        Ok(new_state.values)
    }

    /// Reset the form to initial values
    pub fn reset(&self) {
        let _state = self.state.get_untracked();
        let initial_values = T::default_values();
        let new_state = FormState::new(initial_values);
        self.state.set(new_state);
    }

    /// Reset the form to a specific state
    pub fn reset_to(&self, form: T) {
        let new_state = FormState::new(form);
        self.state.set(new_state);
    }

    /// Reset the form to default values but keep errors
    pub fn reset_values_keep_errors(&self) {
        let state = self.state.get_untracked();
        let initial_values = T::default_values();
        let new_state = FormState {
            values: initial_values,
            is_dirty: false,
            is_submitting: false,
            errors: state.errors,
        };
        self.state.set(new_state);
    }

    /// Reset the form to default values and clear errors
    pub fn reset_values_clear_errors(&self) {
        let initial_values = T::default_values();
        let new_state = FormState::new(initial_values);
        self.state.set(new_state);
    }

    /// Mark the form as submitting
    pub fn mark_submitting(&self) {
        let state = self.state.get_untracked();
        let new_state = state.mark_submitting();
        self.state.set(new_state);
    }

    /// Mark the form as not submitting
    pub fn mark_not_submitting(&self) {
        let state = self.state.get_untracked();
        let new_state = state.mark_not_submitting();
        self.state.set(new_state);
    }

    /// Mark the form as dirty
    pub fn mark_dirty(&self) {
        let state = self.state.get_untracked();
        let new_state = state.mark_dirty();
        self.state.set(new_state);
    }

    /// Mark the form as clean (not dirty)
    pub fn mark_clean(&self) {
        let state = self.state.get_untracked();
        let new_state = FormState {
            values: state.values,
            is_dirty: false,
            is_submitting: state.is_submitting,
            errors: state.errors,
        };
        self.state.set(new_state);
    }

    /// Save the current form state
    pub fn save_state(&self) -> FormState<T> {
        self.state.get_untracked()
    }

    /// Restore a previously saved form state
    pub fn restore_state(&self, saved_state: FormState<T>) {
        self.state.set(saved_state);
    }

    /// Get form state snapshot
    pub fn get_state_snapshot(&self) -> FormStateSnapshot<T> {
        let state = self.state.get_untracked();
        FormStateSnapshot {
            values: state.values.clone(),
            is_dirty: state.is_dirty,
            is_submitting: state.is_submitting,
            has_errors: !state.errors.is_empty(),
            error_count: state.errors.error_count(),
        }
    }

    /// Restore form state from snapshot
    pub fn restore_from_snapshot(&self, snapshot: FormStateSnapshot<T>) {
        let current_state = self.state.get_untracked();
        let new_state = FormState {
            values: snapshot.values,
            is_dirty: snapshot.is_dirty,
            is_submitting: snapshot.is_submitting,
            errors: if snapshot.has_errors {
                current_state.errors
            } else {
                crate::validation::ValidationErrors::new()
            },
        };
        self.state.set(new_state);
    }

    /// Check if the form can be submitted
    pub fn can_submit(&self) -> bool {
        let state = self.state.get_untracked();
        !state.is_submitting && state.is_valid()
    }

    /// Check if the form can be reset
    pub fn can_reset(&self) -> bool {
        let state = self.state.get_untracked();
        state.is_dirty || !state.errors.is_empty()
    }

    /// Get form submission status
    pub fn get_submission_status(&self) -> SubmissionStatus {
        let state = self.state.get_untracked();

        if state.is_submitting {
            SubmissionStatus::Submitting
        } else if state.is_valid() {
            SubmissionStatus::Ready
        } else {
            SubmissionStatus::HasErrors
        }
    }

    /// Submit form with custom handler
    pub fn submit_with_handler<F>(&self, handler: F) -> Result<T, FormError>
    where
        F: FnOnce(&T) -> Result<T, FormError>,
    {
        // Validate first
        self.validate()?;

        let state = self.state.get_untracked();
        let new_state = state.mark_submitting();
        self.state.set(new_state.clone());

        // Call the custom handler
        match handler(&new_state.values) {
            Ok(result) => {
                let final_state = new_state.mark_not_submitting();
                self.state.set(final_state);
                Ok(result)
            }
            Err(error) => {
                let final_state = new_state.mark_not_submitting();
                self.state.set(final_state);
                Err(error)
            }
        }
    }

    /// Reset form with confirmation
    pub fn reset_with_confirmation(&self, confirm: bool) -> bool {
        if confirm {
            self.reset();
            true
        } else {
            false
        }
    }

    /// Reset form to default values with callback
    pub fn reset_with_callback<F>(&self, callback: F)
    where
        F: FnOnce(&T),
    {
        let initial_values = T::default_values();
        callback(&initial_values);
        self.reset_to(initial_values);
    }

    /// Get form lifecycle state
    pub fn get_lifecycle_state(&self) -> FormLifecycleState {
        let state = self.state.get_untracked();

        FormLifecycleState {
            is_dirty: state.is_dirty,
            is_submitting: state.is_submitting,
            has_errors: !state.errors.is_empty(),
            is_valid: state.is_valid(),
            can_submit: self.can_submit(),
            can_reset: self.can_reset(),
        }
    }
}

/// Form state snapshot for saving/restoring form state
#[derive(Debug, Clone)]
pub struct FormStateSnapshot<T: Form> {
    pub values: T,
    pub is_dirty: bool,
    pub is_submitting: bool,
    pub has_errors: bool,
    pub error_count: usize,
}

/// Form submission status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubmissionStatus {
    Ready,
    Submitting,
    HasErrors,
}

impl SubmissionStatus {
    /// Get the display name for the submission status
    pub fn display_name(&self) -> &'static str {
        match self {
            SubmissionStatus::Ready => "Ready",
            SubmissionStatus::Submitting => "Submitting",
            SubmissionStatus::HasErrors => "Has Errors",
        }
    }

    /// Check if the form is ready to submit
    pub fn is_ready(&self) -> bool {
        matches!(self, SubmissionStatus::Ready)
    }

    /// Check if the form is currently submitting
    pub fn is_submitting(&self) -> bool {
        matches!(self, SubmissionStatus::Submitting)
    }

    /// Check if the form has errors
    pub fn has_errors(&self) -> bool {
        matches!(self, SubmissionStatus::HasErrors)
    }
}

/// Form lifecycle state information
#[derive(Debug, Clone)]
pub struct FormLifecycleState {
    pub is_dirty: bool,
    pub is_submitting: bool,
    pub has_errors: bool,
    pub is_valid: bool,
    pub can_submit: bool,
    pub can_reset: bool,
}

impl FormLifecycleState {
    /// Get a summary of the form state
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();

        if self.is_dirty {
            parts.push("dirty");
        }
        if self.is_submitting {
            parts.push("submitting");
        }
        if self.has_errors {
            parts.push("has errors");
        }
        if self.is_valid {
            parts.push("valid");
        }

        if parts.is_empty() {
            "clean".to_string()
        } else {
            parts.join(", ")
        }
    }

    /// Check if the form is in a clean state
    pub fn is_clean(&self) -> bool {
        !self.is_dirty && !self.has_errors && !self.is_submitting
    }

    /// Check if the form needs attention
    pub fn needs_attention(&self) -> bool {
        self.has_errors || self.is_dirty
    }
}
