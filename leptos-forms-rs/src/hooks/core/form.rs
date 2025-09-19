//! Core form hook
//!
//! This module provides the main `use_form` hook for managing form state.

use crate::core::traits::Form;
use crate::core::FormHandle;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Hook for managing form state
pub fn use_form<T: Form + PartialEq + Clone + Send + Sync>(
    initial_values: T,
) -> (FormHandle<T>, Callback<()>, Callback<()>) {
    let form_handle = FormHandle::new(initial_values);

    let form_clone1 = form_handle.clone();
    let submit = Callback::new(move |_| {
        let form_clone = form_clone1.clone();
        spawn_local(async move {
            if let Err(error) = form_clone.submit() {
                log::error!("Form submission failed: {:?}", error);
            }
        });
    });

    let form_clone2 = form_handle.clone();
    let reset = Callback::new(move |_| {
        let form_clone = form_clone2.clone();
        form_clone.reset();
    });

    (form_handle, submit, reset)
}
