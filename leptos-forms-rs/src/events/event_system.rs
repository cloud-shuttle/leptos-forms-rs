//! Central event system coordination
//!
//! This module provides the central event system for coordinating all form events
//! and managing event handlers.

use crate::core::*;
use crate::events::event_handlers::{
    FormResetHandler, FormSubmitHandler as EventFormSubmitHandler,
};
use crate::events::event_types::EventHandler;
use crate::events::field_event_handlers::{
    FieldBlurHandler, FieldChangeHandler, FieldFocusHandler, FieldInputHandler,
};
use leptos::prelude::*;
use std::collections::HashMap;

/// Central event system for form management
pub struct EventSystem<T: Form + Send + Sync> {
    form_handle: FormHandle<T>,
    event_handlers: HashMap<String, Box<dyn EventHandler>>,
}

impl<T: Form + PartialEq + Clone + Send + Sync> EventSystem<T> {
    /// Create a new event system
    pub fn new(form_handle: FormHandle<T>) -> Self {
        Self {
            form_handle,
            event_handlers: HashMap::new(),
        }
    }

    /// Register an event handler
    pub fn register_handler(&mut self, name: String, handler: Box<dyn EventHandler>) {
        self.event_handlers.insert(name, handler);
    }

    /// Get form submit handler
    pub fn get_submit_handler(&self) -> EventFormSubmitHandler<T> {
        EventFormSubmitHandler::new(self.form_handle.clone())
    }

    /// Get form reset handler
    pub fn get_reset_handler(&self) -> FormResetHandler<T> {
        FormResetHandler::new(self.form_handle.clone())
    }

    /// Get field change handler
    pub fn get_field_change_handler(&self, field_name: String) -> FieldChangeHandler<T> {
        FieldChangeHandler::new(self.form_handle.clone(), field_name)
    }

    /// Get field focus handler
    pub fn get_field_focus_handler(&self, field_name: String) -> FieldFocusHandler<T> {
        FieldFocusHandler::new(self.form_handle.clone(), field_name)
    }

    /// Get field blur handler
    pub fn get_field_blur_handler(&self, field_name: String) -> FieldBlurHandler<T> {
        FieldBlurHandler::new(self.form_handle.clone(), field_name)
    }

    /// Get field input handler
    pub fn get_field_input_handler(&self, field_name: String) -> FieldInputHandler<T> {
        FieldInputHandler::new(self.form_handle.clone(), field_name)
    }

    /// Get the form handle
    pub fn form_handle(&self) -> &FormHandle<T> {
        &self.form_handle
    }

    /// Get all registered event handlers
    pub fn event_handlers(&self) -> &HashMap<String, Box<dyn EventHandler>> {
        &self.event_handlers
    }

    /// Check if an event handler is registered
    pub fn has_handler(&self, name: &str) -> bool {
        self.event_handlers.contains_key(name)
    }

    /// Remove an event handler
    pub fn remove_handler(&mut self, name: &str) -> Option<Box<dyn EventHandler>> {
        self.event_handlers.remove(name)
    }

    /// Clear all event handlers
    pub fn clear_handlers(&mut self) {
        self.event_handlers.clear();
    }

    /// Get the number of registered handlers
    pub fn handler_count(&self) -> usize {
        self.event_handlers.len()
    }
}

/// Event system builder for fluent configuration
pub struct EventSystemBuilder<T: Form + Send + Sync> {
    form_handle: FormHandle<T>,
    event_handlers: HashMap<String, Box<dyn EventHandler>>,
}

impl<T: Form + PartialEq + Clone + Send + Sync> EventSystemBuilder<T> {
    /// Create a new event system builder
    pub fn new(form_handle: FormHandle<T>) -> Self {
        Self {
            form_handle,
            event_handlers: HashMap::new(),
        }
    }

    /// Add an event handler
    pub fn with_handler(mut self, name: String, handler: Box<dyn EventHandler>) -> Self {
        self.event_handlers.insert(name, handler);
        self
    }

    /// Build the event system
    pub fn build(self) -> EventSystem<T> {
        EventSystem {
            form_handle: self.form_handle,
            event_handlers: self.event_handlers,
        }
    }
}

/// Event system utilities
pub struct EventSystemUtils;

impl EventSystemUtils {
    /// Create a submit handler
    pub fn create_submit_handler<T: Form + PartialEq + Clone + Send + Sync>(
        form_handle: FormHandle<T>,
    ) -> EventFormSubmitHandler<T> {
        EventFormSubmitHandler::new(form_handle)
    }

    /// Create a reset handler with confirmation
    pub fn create_reset_handler<T: Form + PartialEq + Clone + Send + Sync>(
        form_handle: FormHandle<T>,
        confirm_message: Option<String>,
    ) -> FormResetHandler<T> {
        let mut handler = FormResetHandler::new(form_handle);

        if let Some(message) = confirm_message {
            handler = handler.with_confirmation(message);
        }

        handler
    }

    /// Create a field change handler
    pub fn create_field_change_handler<T: Form + PartialEq + Clone + Send + Sync>(
        form_handle: FormHandle<T>,
        field_name: String,
    ) -> FieldChangeHandler<T> {
        FieldChangeHandler::new(form_handle, field_name)
    }
}
