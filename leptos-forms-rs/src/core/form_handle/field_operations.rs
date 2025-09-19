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

    /// Set a field value
    pub fn set_field_value(&self, field_name: &str, value: FieldValue) {
        let current_state = self.state.get_untracked();

        // Create a new form instance with the updated field
        let mut new_form = current_state.values.clone();

        // Update the field using the form's set_field_value method
        new_form.set_field_value(field_name, value);

        // Create new state with updated form and mark as dirty
        let new_state = FormState {
            values: new_form,
            is_dirty: true,
            is_submitting: current_state.is_submitting,
            errors: current_state.errors,
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

    /// Set multiple field values at once
    pub fn set_multiple_field_values(&self, values: std::collections::HashMap<String, FieldValue>) {
        let current_state = self.state.get_untracked();
        let mut new_form = current_state.values.clone();

        for (field_name, value) in values {
            if self.has_field(&field_name) {
                new_form.set_field_value(&field_name, value);
            }
        }

        let new_state = FormState {
            values: new_form,
            is_dirty: true,
            is_submitting: current_state.is_submitting,
            errors: current_state.errors,
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
}
