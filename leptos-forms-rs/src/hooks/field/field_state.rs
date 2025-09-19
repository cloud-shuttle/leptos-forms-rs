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
    _form_handle: &FormHandle<T>,
) -> Memo<bool> {
    // For now, return false - this would need to be tracked in FormState
    Memo::new(move |_| false)
}
