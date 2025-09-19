//! Hooks module - React-like hooks for form management
//!
//! This module provides a comprehensive set of hooks for building forms
//! with the Leptos Forms library, including core form hooks, field hooks,
//! validation hooks, and specialized hooks for arrays, wizards, persistence,
//! analytics, and performance monitoring.

pub mod analytics;
pub mod array;
pub mod core;
pub mod field;
pub mod performance;
pub mod persistence;
pub mod validation;
pub mod wizard;

// Re-export core hooks
pub use core::*;

// Re-export field hooks
pub use field::*;

// Re-export validation hooks
pub use validation::*;

// Re-export array hooks
pub use array::*;

// Re-export wizard hooks
pub use wizard::*;

// Re-export persistence hooks
pub use persistence::*;

// Re-export analytics hooks
pub use analytics::*;

// Re-export performance hooks
pub use performance::*;
