//! JSON serialization utilities
//!
//! This module provides utilities for serializing and deserializing forms to/from JSON.

use crate::core::traits::Form;

/// Serialize a form to JSON
pub fn serialize_form<T: Form>(form: &T) -> Result<String, String> {
    serde_json::to_string(form).map_err(|e| format!("Failed to serialize form: {}", e))
}

/// Deserialize a form from JSON
pub fn deserialize_form<T: Form>(json: &str) -> Result<T, String> {
    serde_json::from_str(json).map_err(|e| format!("Failed to deserialize form: {}", e))
}

/// Create a form from a JSON object
pub fn form_from_json<T: Form>(json: serde_json::Value) -> Result<T, String> {
    serde_json::from_value(json).map_err(|e| format!("Failed to create form from JSON: {}", e))
}

/// Convert a form to a JSON object
pub fn form_to_json<T: Form>(form: &T) -> Result<serde_json::Value, String> {
    serde_json::to_value(form).map_err(|e| format!("Failed to convert form to JSON: {}", e))
}

/// Check if two forms are equal
pub fn forms_are_equal<T: Form + PartialEq>(form1: &T, form2: &T) -> bool {
    form1 == form2
}
