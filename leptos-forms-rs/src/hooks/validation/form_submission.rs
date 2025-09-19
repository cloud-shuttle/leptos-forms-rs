//! Form submission hook
//!
//! This module provides the `use_form_submission` hook for
//! managing form submission state and operations.

use crate::core::traits::Form;
use crate::core::FormHandle;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Hook for form submission
pub fn use_form_submission<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
) -> (Memo<bool>, Callback<()>) {
    let is_submitting = form_handle.is_submitting();

    let form_clone = form_handle.clone();
    let submit = Callback::new(move |_| {
        let form_clone = form_clone.clone();
        spawn_local(async move {
            if let Err(error) = form_clone.submit() {
                log::error!("Form submission failed: {:?}", error);
            }
        });
    });

    (is_submitting, submit)
}
