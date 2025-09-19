//! Core form handle functionality
//!
//! This module provides the main FormHandle struct and core state management
//! functionality including signal management and basic form operations.

use crate::core::traits::{Form, FormState};
use leptos::prelude::*;

/// Form handle for managing form state and operations
pub struct FormHandle<T: Form>
where
    T: Send,
    T: std::marker::Sync,
{
    pub(crate) state: RwSignal<FormState<T>>,
}

impl<T: Form + Send + Sync + PartialEq> FormHandle<T> {
    /// Create a new form handle
    pub fn new(form: T) -> Self {
        let initial_state = FormState::new(form);
        let state = RwSignal::new(initial_state);
        Self { state }
    }

    /// Get the form state signal
    pub fn state(&self) -> ReadSignal<FormState<T>> {
        self.state.read_only()
    }

    /// Get the form values signal
    pub fn values(&self) -> Memo<T> {
        let state = self.state;
        Memo::new(move |_| state.get().values.clone())
    }

    /// Get the form errors signal
    pub fn errors(&self) -> Memo<crate::validation::ValidationErrors> {
        let state = self.state;
        Memo::new(move |_| state.get().errors.clone())
    }

    /// Get the form validity signal
    pub fn is_valid(&self) -> Memo<bool> {
        let state = self.state;
        Memo::new(move |_| state.get().is_valid())
    }

    /// Get the form dirty state signal
    pub fn is_dirty(&self) -> Memo<bool> {
        let state = self.state;
        Memo::new(move |_| state.get().is_dirty)
    }

    /// Get the form submitting state signal
    pub fn is_submitting(&self) -> Memo<bool> {
        let state = self.state;
        Memo::new(move |_| state.get().is_submitting)
    }

    /// Get the form schema
    pub fn schema(&self) -> crate::core::traits::FormSchema {
        T::schema()
    }

    /// Get field metadata
    pub fn get_field_metadata(
        &self,
        field_name: &str,
    ) -> Option<crate::core::traits::FieldMetadata> {
        let binding = T::schema();
        binding.get_field(field_name).cloned()
    }

    /// Check if a field is required
    pub fn is_field_required(&self, field_name: &str) -> bool {
        self.get_field_metadata(field_name)
            .map(|meta| meta.is_required)
            .unwrap_or(false)
    }

    /// Get the field type
    pub fn get_field_type(&self, field_name: &str) -> Option<crate::core::types::FieldType> {
        self.get_field_metadata(field_name)
            .map(|meta| meta.field_type.clone())
    }

    /// Get the current form state (untracked)
    pub fn get_current_state(&self) -> FormState<T> {
        self.state.get_untracked()
    }

    /// Update the form state
    pub fn update_state<F>(&self, updater: F)
    where
        F: FnOnce(FormState<T>) -> FormState<T>,
    {
        let current_state = self.state.get_untracked();
        let new_state = updater(current_state);
        self.state.set(new_state);
    }

    /// Check if the form has any errors
    pub fn has_errors(&self) -> bool {
        let state = self.state.get_untracked();
        !state.errors.is_empty()
    }

    /// Check if the form is in a valid state
    pub fn is_form_valid(&self) -> bool {
        let state = self.state.get_untracked();
        state.is_valid()
    }

    /// Check if the form has been modified
    pub fn is_form_dirty(&self) -> bool {
        let state = self.state.get_untracked();
        state.is_dirty
    }

    /// Check if the form is currently being submitted
    pub fn is_form_submitting(&self) -> bool {
        let state = self.state.get_untracked();
        state.is_submitting
    }

    /// Get the current form values
    pub fn get_current_values(&self) -> T {
        let state = self.state.get_untracked();
        state.values
    }

    /// Get the current validation errors
    pub fn get_current_errors(&self) -> crate::validation::ValidationErrors {
        let state = self.state.get_untracked();
        state.errors
    }

    /// Internal method to validate a field against its validators
    pub(crate) fn validate_field_internal(
        &self,
        form: &T,
        field_name: &str,
    ) -> Result<(), crate::validation::ValidationErrors> {
        let schema = T::schema();
        let field_meta = schema.get_field(field_name);

        if let Some(field_meta) = field_meta {
            let field_value = form.get_field_value(field_name);
            let mut errors = crate::validation::ValidationErrors::new();

            // Validate against field validators
            for validator in &field_meta.validators {
                if let Err(error) =
                    self.validate_field_value_internal(field_value.clone(), validator)
                {
                    errors.add_field_error(field_name, error);
                }
            }

            if errors.has_errors() {
                Err(errors)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    /// Internal method to validate a field value against a validator
    pub(crate) fn validate_field_value_internal(
        &self,
        value: crate::core::types::FieldValue,
        validator: &crate::validation::Validator,
    ) -> Result<(), String> {
        use crate::validation::ValidationRuleEngine;

        let mut engine = ValidationRuleEngine::new();
        engine.add_validator(validator.clone());

        match engine.validate_value(value) {
            Ok(_) => Ok(()),
            Err(errors) => {
                if let Some(error) = errors.get_field_error("").and_then(|v| v.first()) {
                    Err(error.clone())
                } else {
                    Err("Validation failed".to_string())
                }
            }
        }
    }
}

impl<T: Form + Send + Sync> Clone for FormHandle<T> {
    fn clone(&self) -> Self {
        Self { state: self.state }
    }
}

impl<T: Form + Send + Sync + PartialEq> PartialEq for FormHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.state.get_untracked() == other.state.get_untracked()
    }
}
