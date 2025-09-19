//! Form conversion utilities
//!
//! This module provides utilities for converting forms to/from different data structures.

use crate::core::traits::Form;
use crate::core::types::FieldValue;
use std::collections::HashMap;

/// Convert a form to a HashMap of field values
pub fn form_to_map<T: Form>(form: &T) -> HashMap<String, FieldValue> {
    form.get_form_data()
}

/// Convert a HashMap of field values to a form
pub fn map_to_form<T: Form>(map: &HashMap<String, FieldValue>) -> Result<T, String> {
    let form = T::default_values();

    for (field_name, value) in map {
        // For now, we'll just log the operation
        // In a real implementation, you'd need to mutate the form
        log::info!("Setting field {} to {:?}", field_name, value);
    }

    Ok(form)
}
