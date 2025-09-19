//! Event handler implementations
//!
//! This module provides the core event handlers for form submission, reset,
//! and other form-level events.

use crate::core::*;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Form submission handler
pub struct FormSubmitHandler<T: Form + Send + Sync> {
    form_handle: FormHandle<T>,
}

impl<T: Form + PartialEq + Clone + Send + Sync> FormSubmitHandler<T> {
    /// Create a new form submit handler
    pub fn new(form_handle: FormHandle<T>) -> Self {
        Self { form_handle }
    }

    /// Get the submit handler function
    pub fn handle_submit(&self) -> impl Fn(leptos::ev::MouseEvent) + Clone {
        let form_handle = self.form_handle.clone();

        move |event: leptos::ev::MouseEvent| {
            event.prevent_default();

            let form_handle = form_handle.clone();

            spawn_local(async move {
                match form_handle.submit() {
                    Ok(_form_data) => {
                        log::info!("Form submitted successfully");
                    }
                    Err(error) => {
                        log::error!("Form submission error: {}", error);
                    }
                }
            });
        }
    }

    /// Check if the form can be submitted
    pub fn can_submit(&self) -> bool {
        self.form_handle.can_submit()
    }

    /// Get the form handle
    pub fn form_handle(&self) -> &FormHandle<T> {
        &self.form_handle
    }
}

/// Form reset handler
pub struct FormResetHandler<T: Form + Send + Sync> {
    form_handle: FormHandle<T>,
    confirm_message: Option<String>,
}

impl<T: Form + PartialEq + Clone + Send + Sync> FormResetHandler<T> {
    /// Create a new form reset handler
    pub fn new(form_handle: FormHandle<T>) -> Self {
        Self {
            form_handle,
            confirm_message: None,
        }
    }

    /// Set the confirmation message
    pub fn with_confirmation(mut self, message: String) -> Self {
        self.confirm_message = Some(message);
        self
    }

    /// Get the reset handler function
    pub fn handle_reset(&self) -> impl Fn(leptos::ev::MouseEvent) + Clone {
        let form_handle = self.form_handle.clone();
        let confirm_message = self.confirm_message.clone();

        move |event: leptos::ev::MouseEvent| {
            event.prevent_default();

            let form_handle = form_handle.clone();

            if let Some(message) = &confirm_message {
                if let Some(window) = web_sys::window() {
                    if !window.confirm_with_message(message).unwrap_or(false) {
                        return;
                    }
                }
            }

            form_handle.reset();
            log::info!("Form reset successfully");
        }
    }

    /// Check if the form can be reset
    pub fn can_reset(&self) -> bool {
        self.form_handle.can_reset()
    }

    /// Get the form handle
    pub fn form_handle(&self) -> &FormHandle<T> {
        &self.form_handle
    }
}
