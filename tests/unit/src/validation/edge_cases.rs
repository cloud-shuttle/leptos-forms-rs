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
    form.range_value = 20.0; // Exactly minimum range
    
    let result = form.validate();
    assert!(result.is_ok(), "Boundary values should pass");
}

#[test]
fn test_validation_pattern_edge_cases() {
    let mut form = ValidationTestForm::default();
    
    // Test valid regex pattern
    form.required_field = "Valid".to_string();
    form.email_field = "test@example.com".to_string();
    form.regex_field = "ABC123".to_string(); // Valid pattern
    
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
    
    let result = form.validate();
    assert!(result.is_ok(), "Unicode characters should be handled correctly");
}
