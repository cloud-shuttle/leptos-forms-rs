//! Form validation hooks
//!
//! This module provides hooks for form validation operations.

use crate::core::traits::Form;
use crate::core::FormHandle;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Hook for form validation
pub fn use_form_validation<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
) -> (Memo<bool>, Callback<()>) {
    let is_valid = form_handle.is_valid();

    let form_clone = form_handle.clone();
    let validate = Callback::new(move |_| {
        let form_clone = form_clone.clone();
        spawn_local(async move {
            if let Err(error) = form_clone.validate() {
                log::error!("Form validation failed: {:?}", error);
            }
        });
    });

    (is_valid, validate)
}
