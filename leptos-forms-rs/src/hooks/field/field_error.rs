//! Field error hook
//!
//! This module provides the `use_field_error` hook for getting field errors.

use crate::core::traits::Form;
use crate::core::FormHandle;
use leptos::prelude::*;

/// Hook for getting field errors
pub fn use_field_error<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
    field_name: &str,
) -> Memo<Vec<String>> {
    let form_handle = form_handle.clone();
    let field_name = field_name.to_string();
    Memo::new(move |_| {
        form_handle
            .errors()
            .get()
            .get_field_error(&field_name)
            .cloned()
            .unwrap_or_default()
    })
}
