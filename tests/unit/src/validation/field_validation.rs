//! Field validation tests
//!
//! This module provides tests for field-specific validation including
//! number validation, pattern validation, and custom validation.

use super::types::ValidationTestForm;
use leptos_forms_rs::Form;

#[test]
fn test_number_validation() {
    let mut form = ValidationTestForm::default();
    form.required_field = "Valid".to_string(); // Set required field

    // Test min value validation
    form.min_value = 5; // Below minimum
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.get_field_error("min_value").is_some());
    }

    // Test valid min value
    form.min_value = 15; // Valid value
    let result = form.validate();
    if let Err(errors) = result {
        assert!(errors.get_field_error("min_value").is_none());
    }

    // Test max value validation
    form.max_value = 150; // Above maximum
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.get_field_error("max_value").is_some());
    }

    // Test valid max value
    form.max_value = 50; // Valid value
    let result = form.validate();
    if let Err(errors) = result {
        assert!(errors.get_field_error("max_value").is_none());
    }

    // Test range validation
    form.range_value = 0.5; // Below range
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.get_field_error("range_value").is_some());
    }

    form.range_value = 15.0; // Above range
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.get_field_error("range_value").is_some());
    }

    // Test valid range value
    form.range_value = 5.5; // Valid value
    let result = form.validate();
    if let Err(errors) = result {
        assert!(errors.get_field_error("range_value").is_none());
    }
}

#[test]
fn test_pattern_validation() {
    let mut form = ValidationTestForm::default();
    form.required_field = "Valid".to_string(); // Set required field

    // Test regex field validation
    form.regex_field = "invalid".to_string(); // Doesn't match pattern
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.get_field_error("regex_field").is_some());
    }

    // Test valid regex field
    form.regex_field = "Valid".to_string(); // Matches pattern
    form.min_value = 15; // Set valid min value
    form.range_value = 5.5; // Set valid range value
    let result = form.validate();
    if let Err(errors) = result {
        assert!(errors.get_field_error("regex_field").is_none());
    }

    // Test alphanumeric field validation
    form.alphanumeric_field = "invalid@field".to_string(); // Contains special characters
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.get_field_error("alphanumeric_field").is_some());
    }

    // Test valid alphanumeric field
    form.alphanumeric_field = "Valid123".to_string(); // Valid alphanumeric
    let result = form.validate();
    if let Err(errors) = result {
        assert!(errors.get_field_error("alphanumeric_field").is_none());
    }
}

#[test]
fn test_custom_validation() {
    let mut form = ValidationTestForm::default();
    form.required_field = "Valid".to_string(); // Set required field

    // Test custom validation failure
    form.custom_validation_field = "invalid".to_string();
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.get_field_error("custom_validation_field").is_some());
    }

    // Test custom validation success
    form.custom_validation_field = "valid".to_string();
    let result = form.validate();
    if let Err(errors) = result {
        assert!(errors.get_field_error("custom_validation_field").is_none());
    }

    // Test empty custom validation field (should be valid)
    form.custom_validation_field = String::new();
    let result = form.validate();
    if let Err(errors) = result {
        assert!(errors.get_field_error("custom_validation_field").is_none());
    }
}
