//! Validation Hooks Module - Form validation operations
//!
//! This module provides hooks for form validation including
//! form validation, form submission, real-time validation, and conditional validation.

pub mod conditional_validation;
pub mod form_submission;
pub mod form_validation;
pub mod real_time_validation;

// Re-export public API
pub use conditional_validation::*;
pub use form_submission::*;
pub use form_validation::*;
pub use real_time_validation::*;
