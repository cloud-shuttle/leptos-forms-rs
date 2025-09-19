//! Advanced validation tests module - Complex validation scenarios
//!
//! This module provides comprehensive tests for advanced validation features,
//! including cross-field validation, business rules, and complex validation scenarios.

pub mod business_rules;
pub mod comprehensive_tests;
pub mod field_validation;
pub mod form_tests;
pub mod types;

// Re-export public API
pub use types::*;
