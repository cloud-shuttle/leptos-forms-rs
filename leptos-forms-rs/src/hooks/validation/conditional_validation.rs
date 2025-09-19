//! Conditional validation hook
//!
//! This module provides the `use_conditional_validation` hook for
//! conditional field validation based on form state.

use crate::core::traits::Form;
use crate::core::FormHandle;
use leptos::prelude::*;

/// Hook for conditional validation
pub fn use_conditional_validation<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
    field_name: &str,
    condition: impl Fn(&T) -> bool + Send + Sync + 'static,
) -> Memo<bool> {
    let form_handle = form_handle.clone();
    let _field_name = field_name.to_string();
    Memo::new(move |_| {
        let values = form_handle.values().get();
        condition(&values)
    })
}
