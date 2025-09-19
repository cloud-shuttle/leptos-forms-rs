//! Validation module - Type-safe form validation for Leptos Forms
//!
//! This module provides comprehensive validation capabilities including:
//! - Built-in validators (email, URL, patterns, etc.)
//! - Custom validation rules
//! - Conditional validation
//! - Validation error handling

pub mod conditional;
pub mod engine;
pub mod errors;
pub mod rules;
pub mod validators;

// Re-export public API
pub use conditional::*;
pub use engine::*;
pub use errors::*;
pub use rules::*;
pub use validators::*;
