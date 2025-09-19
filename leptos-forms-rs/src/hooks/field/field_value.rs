//! Field value hook
//!
//! This module provides the `use_field_value` hook for getting field values.

use crate::core::traits::Form;
use crate::core::types::FieldValue;
use crate::core::FormHandle;
use leptos::prelude::*;

/// Hook for getting a field value
pub fn use_field_value<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
    field_name: &str,
) -> Memo<FieldValue> {
    let form_handle = form_handle.clone();
    let field_name = field_name.to_string();
    Memo::new(move |_| {
        form_handle
            .get_field_value(&field_name)
            .unwrap_or(FieldValue::String(String::new()))
    })
}
