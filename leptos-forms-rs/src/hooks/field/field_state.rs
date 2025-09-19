//! Field state hooks
//!
//! This module provides hooks for field state management including
//! dirty state and touched state.

use crate::core::traits::Form;
use crate::core::FormHandle;
use leptos::prelude::*;

/// Hook for checking if a field is dirty
pub fn use_field_dirty<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
) -> Memo<bool> {
    form_handle.is_dirty()
}

/// Hook for checking if a field has been touched
pub fn use_field_touched<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
    field_name: &str,
) -> Memo<bool> {
    let form_handle = form_handle.clone();
    let field_name = field_name.to_string();

    Memo::new(move |_| {
        // For now, we'll consider a field touched if it has a value
        // This is a simplified implementation - in a full implementation,
        // we would track touch state in the FormState
        let current_values = form_handle.get_current_values();
        let field_metadata = form_handle.get_field_metadata(&field_name);

        if let Some(metadata) = field_metadata {
            // Check if the field has a non-default value
            match metadata.field_type {
                crate::core::types::FieldType::Text
                | crate::core::types::FieldType::Email
                | crate::core::types::FieldType::Password => {
                    // For text fields, check if they have content
                    let value = current_values.get_field_value(&field_name);
                    match value {
                        crate::core::types::FieldValue::String(s) => !s.is_empty(),
                        _ => false,
                    }
                }
                crate::core::types::FieldType::Boolean => {
                    // For boolean fields, check if they've been set
                    let value = current_values.get_field_value(&field_name);
                    match value {
                        crate::core::types::FieldValue::Boolean(b) => {
                            let default_bool = metadata
                                .default_value
                                .as_ref()
                                .and_then(|v| v.as_boolean())
                                .unwrap_or(false);
                            b != default_bool
                        }
                        _ => false,
                    }
                }
                _ => {
                    // For other field types, consider touched if they have any value
                    let value = current_values.get_field_value(&field_name);
                    !matches!(value, crate::core::types::FieldValue::String(s) if s.is_empty())
                }
            }
        } else {
            false
        }
    })
}
