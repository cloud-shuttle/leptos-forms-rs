//! Edge case tests for validation
//!
//! This module provides tests for edge cases and boundary conditions in validation.

use super::types::ValidationTestForm;
use leptos_forms_rs::Form;

#[test]
fn test_validation_edge_cases() {
    let mut form = ValidationTestForm::default();

    // Test exactly minimum length
    form.required_field = "Valid".to_string();
    form.email_field = "test@example.com".to_string();
    form.min_length_field = "12345".to_string(); // Exactly 5 characters
    form.min_value = 15; // Set valid min value
    form.range_value = 5.5; // Set valid range value

    let result = form.validate();
    assert!(result.is_ok(), "Exactly minimum length should pass");
}

#[test]
fn test_validation_boundary_values() {
    let mut form = ValidationTestForm::default();

    // Test exactly maximum length
    form.required_field = "Valid".to_string();
    form.email_field = "test@example.com".to_string();
    form.max_length_field = "1234567890".to_string(); // Exactly 10 characters
    form.min_value = 15; // Set valid min value
    form.range_value = 5.5; // Set valid range value

    let result = form.validate();
    assert!(result.is_ok(), "Exactly maximum length should pass");
}

#[test]
fn test_validation_number_boundaries() {
    let mut form = ValidationTestForm::default();

    // Test exactly minimum value
    form.required_field = "Valid".to_string();
    form.email_field = "test@example.com".to_string();
    form.min_value = 10; // Exactly minimum
    form.max_value = 100; // Exactly maximum
    form.range_value = 5.5; // Valid range value

    let result = form.validate();
    assert!(result.is_ok(), "Boundary values should pass");
}

#[test]
fn test_validation_pattern_edge_cases() {
    let mut form = ValidationTestForm::default();

    // Test valid regex pattern
    form.required_field = "Valid".to_string();
    form.email_field = "test@example.com".to_string();
    form.regex_field = "Valid".to_string(); // Valid pattern (matches ^[A-Z][a-z]+$)
    form.min_value = 15; // Set valid min value
    form.range_value = 5.5; // Set valid range value

    let result = form.validate();
    assert!(result.is_ok(), "Valid regex pattern should pass");
}

#[test]
fn test_validation_unicode_edge_cases() {
    let mut form = ValidationTestForm::default();

    // Test unicode characters
    form.required_field = "Valid".to_string();
    form.email_field = "test@example.com".to_string();
    form.min_length_field = "héllo".to_string(); // Unicode characters
    form.min_value = 15; // Set valid min value
    form.range_value = 5.5; // Set valid range value

    let result = form.validate();
    assert!(
        result.is_ok(),
        "Unicode characters should be handled correctly"
    );
}
