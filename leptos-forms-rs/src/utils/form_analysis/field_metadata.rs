//! Field metadata utilities
//!
//! This module provides utilities for working with field metadata.

use crate::core::traits::Form;

/// Get all field names from a form
pub fn get_form_field_names<T: Form>() -> Vec<String> {
    T::field_metadata()
        .into_iter()
        .map(|meta| meta.name)
        .collect()
}

/// Get required field names from a form
pub fn get_required_field_names<T: Form>() -> Vec<String> {
    T::field_metadata()
        .into_iter()
        .filter(|meta| meta.is_required)
        .map(|meta| meta.name)
        .collect()
}

/// Check if a field is required
pub fn is_field_required<T: Form>(field_name: &str) -> bool {
    T::field_metadata()
        .into_iter()
        .any(|meta| meta.name == field_name && meta.is_required)
}

/// Get field type for a specific field
pub fn get_field_type<T: Form>(field_name: &str) -> Option<crate::core::types::FieldType> {
    T::field_metadata()
        .into_iter()
        .find(|meta| meta.name == field_name)
        .map(|meta| meta.field_type)
}
