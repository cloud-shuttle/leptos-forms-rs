//! Event handling system for Leptos Forms
//!
//! This module provides a comprehensive event handling system for form management,
//! including form submission, reset, field changes, and validation events.

pub mod event_handlers;
pub mod event_system;
pub mod event_types;
pub mod field_event_handlers;

// Re-export public API
pub use event_handlers::*;
pub use event_system::*;
pub use event_types::*;
pub use field_event_handlers::*;
