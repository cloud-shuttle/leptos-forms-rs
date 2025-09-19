//! Form State Inspector implementation
//!
//! This module provides the FormStateInspector for monitoring form state changes.

use crate::core::{Form, FormHandle};
use crate::validation::ValidationErrors;
use leptos::prelude::GetUntracked;
use std::collections::HashMap;
use std::time::SystemTime;

use super::super::types::{ChangeListener, FieldState, FormStateSnapshot};

/// Form State Inspector for DevTools
pub struct FormStateInspector<T: Form + Send + Sync + PartialEq> {
    form_handle: FormHandle<T>,
    change_listeners: Vec<ChangeListener>,
}

impl<T: Form + Send + Sync + PartialEq> FormStateInspector<T> {
    pub fn new(form_handle: &FormHandle<T>) -> Result<Self, String> {
        Ok(Self {
            form_handle: form_handle.clone(),
            change_listeners: Vec::new(),
        })
    }

    pub fn get_current_state(&self) -> FormStateSnapshot {
        let state = self.form_handle.state().get_untracked();
        FormStateSnapshot {
            form_name: T::schema().name,
            field_count: T::schema().field_metadata.len(),
            is_dirty: Some(state.is_dirty),
            is_submitting: Some(state.is_submitting),
            has_errors: Some(!state.errors.is_empty()),
            timestamp: SystemTime::now(),
        }
    }

    pub fn get_field_states(&self) -> HashMap<String, FieldState> {
        let mut field_states = HashMap::new();
        let form_data = self.form_handle.values().get_untracked();
        let state = self.form_handle.state().get_untracked();

        for field_metadata in &T::schema().field_metadata {
            let value = form_data.get_field_value(&field_metadata.name);
            let has_error = state.errors.get_field_error(&field_metadata.name).is_some();
            let error_message = state
                .errors
                .get_field_error(&field_metadata.name)
                .map(|errors| errors.join(", "));

            field_states.insert(
                field_metadata.name.clone(),
                FieldState {
                    name: field_metadata.name.clone(),
                    field_type: format!("{:?}", field_metadata.field_type),
                    is_required: field_metadata.is_required,
                    value: Some(value),
                    has_error: Some(has_error),
                    error_message,
                },
            );
        }

        field_states
    }

    pub fn get_validation_errors(&self) -> ValidationErrors {
        let state = self.form_handle.state().get_untracked();
        state.errors
    }

    pub fn subscribe_to_changes<F>(&mut self, callback: F) -> impl Fn()
    where
        F: Fn(&FormStateSnapshot) + Send + Sync + 'static,
    {
        let callback = Box::new(callback);
        self.change_listeners.push(callback);

        // Return unsubscribe function
        move || {
            // In a real implementation, this would remove the listener
        }
    }
}
