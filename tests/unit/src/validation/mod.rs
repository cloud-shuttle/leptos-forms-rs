//! Validation tests module - Comprehensive validation engine tests
//!
//! This module provides comprehensive tests for the validation engine,
//! including basic validation, field validation, complex scenarios, and error handling.

pub mod basic_validation;
pub mod complex_scenarios;
pub mod edge_cases;
pub mod field_validation;
pub mod types;

// Re-export public API
pub use types::*;
