//! Form statistics utilities
//!
//! This module provides utilities for generating form statistics and analysis.

use crate::core::traits::Form;
use serde::{Deserialize, Serialize};

/// Get form statistics
pub fn get_form_stats<T: Form>(_form: &T) -> FormStats {
    let metadata = T::field_metadata();
    let total_fields = metadata.len();
    let required_fields = metadata.iter().filter(|meta| meta.is_required).count();
    let optional_fields = total_fields - required_fields;

    FormStats {
        total_fields,
        required_fields,
        optional_fields,
    }
}

/// Form statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormStats {
    pub total_fields: usize,
    pub required_fields: usize,
    pub optional_fields: usize,
}
