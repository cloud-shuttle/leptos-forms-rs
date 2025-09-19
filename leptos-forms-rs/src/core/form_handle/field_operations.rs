//! Field operations module
//!
//! This module provides field-level operations for the FormHandle including
//! getting and setting field values, field validation, and field metadata access.

use crate::core::traits::{Form, FormState};
use crate::core::types::FieldValue;
use leptos::prelude::{GetUntracked, Set};

use super::core::FormHandle;

impl<T: Form + Send + Sync + PartialEq> FormHandle<T> {
    /// Get a field value
    pub fn get_field_value(&self, field_name: &str) -> Option<FieldValue> {
        let state = self.state.get_untracked();
        let schema = T::schema();

        // Check if the field exists in the schema
        if schema.get_field(field_name).is_some() {
            Some(state.values.get_field_value(field_name))
        } else {
            None
        }
    }

    /// Set a field value with automatic validation
    pub fn set_field_value(&self, field_name: &str, value: FieldValue) {
        let current_state = self.state.get_untracked();

        // Create a new form instance with the updated field
        let mut new_form = current_state.values.clone();

        // Update the field using the form's set_field_value method
        new_form.set_field_value(field_name, value);

        // Validate the field after setting the value
        let mut new_errors = current_state.errors.clone();
        if let Err(field_errors) = self.validate_field_internal(&new_form, field_name) {
            // Update errors for this field
            for (error_field, error_messages) in field_errors.field_errors {
                for error_message in error_messages {
                    new_errors.add_field_error(&error_field, error_message);
                }
            }
        } else {
            // Clear errors for this field if validation passes
            new_errors.remove_field_error(field_name);
        }

        // Create new state with updated form, mark as dirty, and update errors
        let new_state = FormState {
            values: new_form,
            is_dirty: true,
            is_submitting: current_state.is_submitting,
            errors: new_errors,
        };

        self.state.set(new_state);
    }

    /// Update a field value using a closure
    pub fn update_field_value<F>(&self, field_name: &str, updater: F)
    where
        F: FnOnce(FieldValue) -> FieldValue,
    {
        if let Some(current_value) = self.get_field_value(field_name) {
            let new_value = updater(current_value);
            self.set_field_value(field_name, new_value);
        }
    }

    /// Check if a field exists in the form schema
    pub fn has_field(&self, field_name: &str) -> bool {
        let schema = T::schema();
        schema.get_field(field_name).is_some()
    }

    /// Get all field names from the form schema
    pub fn get_field_names(&self) -> Vec<String> {
        let schema = T::schema();
        schema
            .field_metadata
            .iter()
            .map(|f| f.name.clone())
            .collect()
    }

    /// Get field values as a HashMap
    pub fn get_all_field_values(&self) -> std::collections::HashMap<String, FieldValue> {
        let state = self.state.get_untracked();
        state.values.get_form_data()
    }

    /// Set multiple field values at once with validation
    pub fn set_multiple_field_values(&self, values: std::collections::HashMap<String, FieldValue>) {
        let current_state = self.state.get_untracked();
        let mut new_form = current_state.values.clone();
        let mut new_errors = current_state.errors.clone();

        for (field_name, value) in values {
            if self.has_field(&field_name) {
                new_form.set_field_value(&field_name, value);

                // Validate each field after setting
                if let Err(field_errors) = self.validate_field_internal(&new_form, &field_name) {
                    for (error_field, error_messages) in field_errors.field_errors {
                        for error_message in error_messages {
                            new_errors.add_field_error(&error_field, error_message);
                        }
                    }
                } else {
                    new_errors.remove_field_error(&field_name);
                }
            }
        }

        let new_state = FormState {
            values: new_form,
            is_dirty: true,
            is_submitting: current_state.is_submitting,
            errors: new_errors,
        };

        self.state.set(new_state);
    }

    /// Get field value as a specific type
    pub fn get_field_value_as_string(&self, field_name: &str) -> Option<String> {
        self.get_field_value(field_name)?.as_string().cloned()
    }

    /// Get field value as a number
    pub fn get_field_value_as_number(&self, field_name: &str) -> Option<f64> {
        self.get_field_value(field_name)?.as_number()
    }

    /// Get field value as a boolean
    pub fn get_field_value_as_boolean(&self, field_name: &str) -> Option<bool> {
        self.get_field_value(field_name)?.as_boolean()
    }

    /// Get field value as an array
    pub fn get_field_value_as_array(&self, field_name: &str) -> Option<Vec<FieldValue>> {
        self.get_field_value(field_name)?.as_array().cloned()
    }

    /// Set field value from a string
    pub fn set_field_value_from_string(&self, field_name: &str, value: String) {
        self.set_field_value(field_name, FieldValue::String(value));
    }

    /// Set field value from a number
    pub fn set_field_value_from_number(&self, field_name: &str, value: f64) {
        self.set_field_value(field_name, FieldValue::Number(value));
    }

    /// Set field value from a boolean
    pub fn set_field_value_from_boolean(&self, field_name: &str, value: bool) {
        self.set_field_value(field_name, FieldValue::Boolean(value));
    }

    /// Check if a field is empty
    pub fn is_field_empty(&self, field_name: &str) -> bool {
        self.get_field_value(field_name)
            .map(|value| value.is_empty())
            .unwrap_or(true)
    }

    /// Check if a field has a value
    pub fn has_field_value(&self, field_name: &str) -> bool {
        !self.is_field_empty(field_name)
    }

    /// Get field display value (converted to string)
    pub fn get_field_display_value(&self, field_name: &str) -> String {
        self.get_field_value(field_name)
            .map(|value| value.to_string())
            .unwrap_or_default()
    }

    /// Clear a field value (set to null)
    pub fn clear_field_value(&self, field_name: &str) {
        self.set_field_value(field_name, FieldValue::Null);
    }

    /// Get field validation rules
    pub fn get_field_validators(&self, field_name: &str) -> Vec<crate::validation::Validator> {
        self.get_field_metadata(field_name)
            .map(|meta| meta.validators)
            .unwrap_or_default()
    }

    /// Check if a field has validation rules
    pub fn has_field_validators(&self, field_name: &str) -> bool {
        !self.get_field_validators(field_name).is_empty()
    }

    /// Get field default value
    pub fn get_field_default_value(&self, field_name: &str) -> Option<FieldValue> {
        self.get_field_metadata(field_name)
            .and_then(|meta| meta.default_value)
    }

    /// Reset a field to its default value
    pub fn reset_field_to_default(&self, field_name: &str) {
        if let Some(default_value) = self.get_field_default_value(field_name) {
            self.set_field_value(field_name, default_value);
        } else {
            self.clear_field_value(field_name);
        }
    }

    /// Get field dependencies
    pub fn get_field_dependencies(&self, field_name: &str) -> Vec<String> {
        self.get_field_metadata(field_name)
            .map(|meta| meta.dependencies)
            .unwrap_or_default()
    }

    /// Check if a field has dependencies
    pub fn has_field_dependencies(&self, field_name: &str) -> bool {
        !self.get_field_dependencies(field_name).is_empty()
    }

    /// Get field attributes
    pub fn get_field_attributes(
        &self,
        field_name: &str,
    ) -> std::collections::HashMap<String, String> {
        self.get_field_metadata(field_name)
            .map(|meta| meta.attributes)
            .unwrap_or_default()
    }

    /// Get a specific field attribute
    pub fn get_field_attribute(&self, field_name: &str, attribute_name: &str) -> Option<String> {
        self.get_field_attributes(field_name)
            .get(attribute_name)
            .cloned()
    }

    /// Check if a field has a specific attribute
    pub fn has_field_attribute(&self, field_name: &str, attribute_name: &str) -> bool {
        self.get_field_attributes(field_name)
            .contains_key(attribute_name)
    }

    /// Set field value without triggering validation (useful for initialization)
    pub fn set_field_value_no_validation(&self, field_name: &str, value: FieldValue) {
        let current_state = self.state.get_untracked();
        let mut new_form = current_state.values.clone();

        new_form.set_field_value(field_name, value);

        let new_state = FormState {
            values: new_form,
            is_dirty: true,
            is_submitting: current_state.is_submitting,
            errors: current_state.errors,
        };

        self.state.set(new_state);
    }

    /// Validate a specific field without changing its value
    pub fn validate_field_only(
        &self,
        field_name: &str,
    ) -> Result<(), crate::validation::ValidationErrors> {
        let current_state = self.state.get_untracked();
        self.validate_field_internal(&current_state.values, field_name)
    }

    /// Get field state information
    pub fn get_field_state(&self, field_name: &str) -> FieldState {
        let state = self.state.get_untracked();
        let has_errors = state.errors.get_field_error(field_name).is_some();
        let is_dirty = state.is_dirty;
        let has_value = self.has_field_value(field_name);
        let is_required = self.is_field_required(field_name);

        FieldState {
            field_name: field_name.to_string(),
            has_errors,
            is_dirty,
            has_value,
            is_required,
            errors: if has_errors {
                state
                    .errors
                    .get_field_error(field_name)
                    .cloned()
                    .unwrap_or_default()
            } else {
                Vec::new()
            },
        }
    }

    /// Get all field states
    pub fn get_all_field_states(&self) -> Vec<FieldState> {
        let field_names = self.get_field_names();
        field_names
            .into_iter()
            .map(|name| self.get_field_state(&name))
            .collect()
    }

    /// Check if any field has errors
    pub fn has_any_field_errors(&self) -> bool {
        let state = self.state.get_untracked();
        !state.errors.is_empty()
    }

    /// Get fields with errors
    pub fn get_fields_with_errors(&self) -> Vec<String> {
        let state = self.state.get_untracked();
        state.errors.field_errors.keys().cloned().collect()
    }

    /// Get fields without errors
    pub fn get_fields_without_errors(&self) -> Vec<String> {
        let all_fields = self.get_field_names();
        let fields_with_errors = self.get_fields_with_errors();

        all_fields
            .into_iter()
            .filter(|field| !fields_with_errors.contains(field))
            .collect()
    }
}

/// Field state information
#[derive(Debug, Clone)]
pub struct FieldState {
    pub field_name: String,
    pub has_errors: bool,
    pub is_dirty: bool,
    pub has_value: bool,
    pub is_required: bool,
    pub errors: Vec<String>,
}

impl FieldState {
    /// Check if the field is valid
    pub fn is_valid(&self) -> bool {
        !self.has_errors
    }

    /// Check if the field is empty
    pub fn is_empty(&self) -> bool {
        !self.has_value
    }

    /// Get the first error message
    pub fn first_error(&self) -> Option<&String> {
        self.errors.first()
    }

    /// Get all error messages as a single string
    pub fn errors_as_string(&self) -> String {
        self.errors.join(", ")
    }
}
