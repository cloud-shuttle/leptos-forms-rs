//! Core types module - Fundamental type definitions for Leptos Forms
//!
//! This module provides:
//! - Field types and constraints
//! - Dynamic field value representation
//! - File data handling
//! - Field configuration and metadata
//! - Validator configuration
//! - Form options and settings

pub mod array_state;
pub mod field_config;
pub mod field_types;
pub mod field_value;
pub mod file_data;
pub mod form_error;
pub mod form_options;
pub mod validator_config;

// Re-export public API
pub use array_state::*;
pub use field_config::*;
pub use field_types::*;
pub use field_value::*;
pub use file_data::*;
pub use form_error::*;
pub use form_options::*;
pub use validator_config::*;
