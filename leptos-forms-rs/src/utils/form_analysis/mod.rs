//! Form Analysis Module - Form metadata and statistics utilities
//!
//! This module provides utilities for analyzing forms including
//! field metadata, statistics, and form structure analysis.

pub mod field_metadata;
pub mod form_statistics;

// Re-export public API
pub use field_metadata::*;
pub use form_statistics::*;
