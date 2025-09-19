//! Serialization Module - Form serialization utilities
//!
//! This module provides utilities for serializing and deserializing forms
//! to/from JSON and other formats.

pub mod json_serialization;

// Re-export public API
pub use json_serialization::*;
