//! Field-specific event handlers
//!
//! This module provides event handlers for field-level events including
//! change, focus, blur, and input events.

use crate::core::*;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

/// Field change handler
pub struct FieldChangeHandler<T: Form + Send + Sync> {
    form_handle: FormHandle<T>,
    field_name: String,
}

impl<T: Form + PartialEq + Clone + Send + Sync> FieldChangeHandler<T> {
    /// Create a new field change handler
    pub fn new(form_handle: FormHandle<T>, field_name: String) -> Self {
        Self {
            form_handle,
            field_name,
        }
    }

    /// Get the change handler function
    pub fn handle_change(&self) -> impl Fn(leptos::ev::Event) + Clone {
        let form_handle = self.form_handle.clone();
        let field_name = self.field_name.clone();

        move |event: leptos::ev::Event| {
            if let Some(target) = event.target() {
                if let Some(input) = target.clone().dyn_into::<web_sys::HtmlInputElement>().ok() {
                    let value = crate::core::types::FieldValue::String(input.value());
                    form_handle.set_field_value(&field_name, value);
                } else if let Some(select) =
                    target.clone().dyn_into::<web_sys::HtmlSelectElement>().ok()
                {
                    let value = crate::core::types::FieldValue::String(select.value());
                    form_handle.set_field_value(&field_name, value);
                } else if let Some(textarea) =
                    target.dyn_into::<web_sys::HtmlTextAreaElement>().ok()
                {
                    let value = crate::core::types::FieldValue::String(textarea.value());
                    form_handle.set_field_value(&field_name, value);
                }
            }
        }
    }

    /// Get the field name
    pub fn field_name(&self) -> &str {
        &self.field_name
    }

    /// Get the form handle
    pub fn form_handle(&self) -> &FormHandle<T> {
        &self.form_handle
    }
}

/// Field focus handler
pub struct FieldFocusHandler<T: Form + Send + Sync> {
    form_handle: FormHandle<T>,
    field_name: String,
}

impl<T: Form + PartialEq + Clone + Send + Sync> FieldFocusHandler<T> {
    /// Create a new field focus handler
    pub fn new(form_handle: FormHandle<T>, field_name: String) -> Self {
        Self {
            form_handle,
            field_name,
        }
    }

    /// Get the focus handler function
    pub fn handle_focus(&self) -> impl Fn(leptos::ev::FocusEvent) + Clone {
        move |_event: leptos::ev::FocusEvent| {
            // Field focused - could add focus tracking here
        }
    }

    /// Get the field name
    pub fn field_name(&self) -> &str {
        &self.field_name
    }

    /// Get the form handle
    pub fn form_handle(&self) -> &FormHandle<T> {
        &self.form_handle
    }
}

/// Field blur handler
pub struct FieldBlurHandler<T: Form + Send + Sync> {
    form_handle: FormHandle<T>,
    field_name: String,
}

impl<T: Form + PartialEq + Clone + Send + Sync> FieldBlurHandler<T> {
    /// Create a new field blur handler
    pub fn new(form_handle: FormHandle<T>, field_name: String) -> Self {
        Self {
            form_handle,
            field_name,
        }
    }

    /// Get the blur handler function
    pub fn handle_blur(&self) -> impl Fn(leptos::ev::FocusEvent) + Clone {
        let form_handle = self.form_handle.clone();
        let field_name = self.field_name.clone();

        move |_event: leptos::ev::FocusEvent| {
            // Trigger field validation on blur
            let _ = form_handle.validate_field_only(&field_name);
        }
    }

    /// Get the field name
    pub fn field_name(&self) -> &str {
        &self.field_name
    }

    /// Get the form handle
    pub fn form_handle(&self) -> &FormHandle<T> {
        &self.form_handle
    }
}

/// Field input handler for real-time updates
pub struct FieldInputHandler<T: Form + Send + Sync> {
    form_handle: FormHandle<T>,
    field_name: String,
    debounce_ms: u64,
}

impl<T: Form + PartialEq + Clone + Send + Sync> FieldInputHandler<T> {
    /// Create a new field input handler
    pub fn new(form_handle: FormHandle<T>, field_name: String) -> Self {
        Self {
            form_handle,
            field_name,
            debounce_ms: 300, // Default 300ms debounce
        }
    }

    /// Set the debounce delay
    pub fn debounce(mut self, ms: u64) -> Self {
        self.debounce_ms = ms;
        self
    }

    /// Get the input handler function
    pub fn handle_input(&self) -> impl Fn(leptos::ev::Event) + Clone {
        let form_handle = self.form_handle.clone();
        let field_name = self.field_name.clone();

        move |event: leptos::ev::Event| {
            if let Some(target) = event.target() {
                if let Some(input) = target.clone().dyn_into::<web_sys::HtmlInputElement>().ok() {
                    let value = crate::core::types::FieldValue::String(input.value());
                    form_handle.set_field_value(&field_name, value);
                } else if let Some(textarea) =
                    target.dyn_into::<web_sys::HtmlTextAreaElement>().ok()
                {
                    let value = crate::core::types::FieldValue::String(textarea.value());
                    form_handle.set_field_value(&field_name, value);
                }
            }
        }
    }

    /// Get the field name
    pub fn field_name(&self) -> &str {
        &self.field_name
    }

    /// Get the form handle
    pub fn form_handle(&self) -> &FormHandle<T> {
        &self.form_handle
    }

    /// Get the debounce delay
    pub fn debounce_ms(&self) -> u64 {
        self.debounce_ms
    }
}
