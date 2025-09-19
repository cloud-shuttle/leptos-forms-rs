//! Form handle module - Core form state management and operations
//!
//! This module provides the FormHandle struct and related functionality for
//! managing form state, validation, field operations, and form lifecycle.

pub mod array_operations;
pub mod core;
pub mod field_operations;
pub mod lifecycle;
pub mod validation;

// Re-export public API
pub use core::FormHandle;
pub use lifecycle::{FormLifecycleState, FormStateSnapshot, SubmissionStatus};
pub use validation::ValidationSummary;
