//! Real-time validation hook
//!
//! This module provides the `use_real_time_validation` hook for
//! real-time field validation with debouncing.

use crate::core::traits::Form;
use crate::core::types::FieldValue;
use crate::core::FormHandle;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Hook for real-time validation
pub fn use_real_time_validation<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
    field_name: &str,
    delay_ms: u32,
) -> (ReadSignal<Option<String>>, Callback<FieldValue>) {
    let validation_error = RwSignal::new(None::<String>);

    let form_clone = form_handle.clone();
    let field_name = field_name.to_string();
    let set_error = validation_error;
    let validate_field = Callback::new(move |_value: FieldValue| {
        let form_clone = form_clone.clone();
        let field_name = field_name.clone();
        let set_error = set_error;

        spawn_local(async move {
            // Simulate validation delay
            gloo_timers::callback::Timeout::new(delay_ms, move || {
                if let Err(errors) = form_clone.validate_field(&field_name) {
                    if let Some(error) = errors.get_field_error(&field_name).and_then(|v| v.first())
                    {
                        set_error.set(Some(error.clone()));
                    }
                } else {
                    set_error.set(None);
                }
            })
            .forget();
        });
    });

    (validation_error.read_only(), validate_field)
}
