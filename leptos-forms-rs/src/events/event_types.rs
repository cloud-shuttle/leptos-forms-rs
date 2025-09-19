//! Event types and event data structures
//!
//! This module defines the core event types and data structures used throughout
//! the event handling system.

use serde::{Deserialize, Serialize};

/// Form event types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FormEventType {
    Submit,
    Reset,
    Validation,
    Error,
    Success,
}

/// Field event types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldEventType {
    Change,
    Focus,
    Blur,
    Input,
    KeyDown,
    KeyUp,
}

/// Event data structure for form events
#[derive(Debug, Clone)]
pub struct FormEvent<T> {
    pub event_type: FormEventType,
    pub form_data: T,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source: String,
}

impl<T> FormEvent<T> {
    /// Create a new form event
    pub fn new(event_type: FormEventType, form_data: T, source: String) -> Self {
        Self {
            event_type,
            form_data,
            timestamp: chrono::Utc::now(),
            source,
        }
    }

    /// Get the event type
    pub fn event_type(&self) -> &FormEventType {
        &self.event_type
    }

    /// Get the form data
    pub fn form_data(&self) -> &T {
        &self.form_data
    }

    /// Get the timestamp
    pub fn timestamp(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.timestamp
    }

    /// Get the source
    pub fn source(&self) -> &str {
        &self.source
    }
}

/// Field event data structure
#[derive(Debug, Clone)]
pub struct FieldEvent {
    pub event_type: FieldEventType,
    pub field_name: String,
    pub field_value: crate::core::types::FieldValue,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source: String,
}

impl FieldEvent {
    /// Create a new field event
    pub fn new(
        event_type: FieldEventType,
        field_name: String,
        field_value: crate::core::types::FieldValue,
        source: String,
    ) -> Self {
        Self {
            event_type,
            field_name,
            field_value,
            timestamp: chrono::Utc::now(),
            source,
        }
    }

    /// Get the event type
    pub fn event_type(&self) -> &FieldEventType {
        &self.event_type
    }

    /// Get the field name
    pub fn field_name(&self) -> &str {
        &self.field_name
    }

    /// Get the field value
    pub fn field_value(&self) -> &crate::core::types::FieldValue {
        &self.field_value
    }

    /// Get the timestamp
    pub fn timestamp(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.timestamp
    }

    /// Get the source
    pub fn source(&self) -> &str {
        &self.source
    }
}

/// Event handler result
#[derive(Debug, Clone)]
pub enum EventResult {
    Success,
    Error(String),
    PreventDefault,
    StopPropagation,
}

impl EventResult {
    /// Check if the event was successful
    pub fn is_success(&self) -> bool {
        matches!(self, EventResult::Success)
    }

    /// Check if the event had an error
    pub fn is_error(&self) -> bool {
        matches!(self, EventResult::Error(_))
    }

    /// Check if default should be prevented
    pub fn should_prevent_default(&self) -> bool {
        matches!(self, EventResult::PreventDefault)
    }

    /// Check if propagation should be stopped
    pub fn should_stop_propagation(&self) -> bool {
        matches!(self, EventResult::StopPropagation)
    }

    /// Get the error message if there was an error
    pub fn error_message(&self) -> Option<&String> {
        match self {
            EventResult::Error(msg) => Some(msg),
            _ => None,
        }
    }
}

/// Event handler trait for custom event handling
pub trait EventHandler: Send + Sync {
    /// Handle an event
    fn handle(&self, event: &dyn std::any::Any) -> EventResult;

    /// Get the event type this handler handles
    fn event_type(&self) -> &'static str;
}

/// Event listener configuration
#[derive(Debug, Clone)]
pub struct EventListenerConfig {
    pub capture: bool,
    pub once: bool,
    pub passive: bool,
}

impl Default for EventListenerConfig {
    fn default() -> Self {
        Self {
            capture: false,
            once: false,
            passive: false,
        }
    }
}

impl EventListenerConfig {
    /// Create a new event listener configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set capture mode
    pub fn capture(mut self, capture: bool) -> Self {
        self.capture = capture;
        self
    }

    /// Set once mode
    pub fn once(mut self, once: bool) -> Self {
        self.once = once;
        self
    }

    /// Set passive mode
    pub fn passive(mut self, passive: bool) -> Self {
        self.passive = passive;
        self
    }
}
