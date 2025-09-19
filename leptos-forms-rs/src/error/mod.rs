//! Error handling module - Comprehensive error management for Leptos Forms
//!
//! This module provides:
//! - Form and field error types
//! - Error context and debugging information
//! - Error handling and reporting
//! - Error recovery strategies

pub mod context;
pub mod field_error;
pub mod form_error;
pub mod handler;
pub mod reporter;

// Re-export public API
pub use context::*;
pub use field_error::*;
pub use form_error::*;
pub use handler::*;
pub use reporter::*;
