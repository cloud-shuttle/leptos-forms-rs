//! Error merging utilities
//!
//! This module provides utilities for merging validation errors from multiple sources.

use crate::validation::ValidationErrors;

/// Merge validation errors from multiple sources
pub fn merge_validation_errors(errors: Vec<ValidationErrors>) -> ValidationErrors {
    let mut merged = ValidationErrors::new();

    for error_set in errors {
        merged.merge(error_set);
    }

    merged
}
