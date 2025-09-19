//! Form analytics hook
//!
//! This module provides the `use_form_analytics` hook for
//! tracking form interactions and analytics events.

use crate::core::traits::Form;
use crate::core::FormHandle;
use leptos::prelude::*;

/// Hook for form analytics
pub fn use_form_analytics<T: Form + PartialEq + Clone + Send + Sync>(
    _form_handle: &FormHandle<T>,
) -> Callback<&'static str> {
    Callback::new(move |event: &'static str| {
        log::info!("Form analytics event: {}", event);
    })
}
