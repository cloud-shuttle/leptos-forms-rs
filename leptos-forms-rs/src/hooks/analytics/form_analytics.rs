//! Form analytics hook
//!
//! This module provides the `use_form_analytics` hook for
//! tracking form interactions and analytics events.

use crate::core::traits::Form;
use crate::core::FormHandle;
use leptos::prelude::*;

/// Hook for form analytics
pub fn use_form_analytics<T: Form + PartialEq + Clone + Send + Sync + std::fmt::Debug>(
    form_handle: &FormHandle<T>,
) -> Callback<&'static str> {
    let form_handle = form_handle.clone();

    Callback::new(move |event: &'static str| {
        let form_handle = form_handle.clone();

        // Track the analytics event with form context
        let form_data = form_handle.values().get();
        let form_schema = form_handle.schema();

        log::info!(
            "Form analytics event: {} for form with data: {:?}",
            event,
            form_data
        );

        // In a real implementation, this would send data to an analytics service
        // For now, we'll just log it with structured data
        match event {
            "form_submit" => {
                log::info!(
                    "Form submitted with {} fields",
                    form_schema.field_metadata.len()
                );
            }
            "form_reset" => {
                log::info!("Form reset");
            }
            "field_change" => {
                log::info!("Field changed in form");
            }
            "validation_error" => {
                log::info!("Validation error in form");
            }
            _ => {
                log::info!("Custom analytics event: {}", event);
            }
        }
    })
}
