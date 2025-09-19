//! Complex validation scenario tests
//!
//! This module provides tests for complex validation scenarios including
//! conditional validation and validation error aggregation.

use super::types::ValidationTestForm;
use leptos_forms_rs::Form;

#[test]
fn test_conditional_validation() {
    let mut form = ValidationTestForm::default();
    form.required_field = "Valid".to_string(); // Set required field
    
    // Test conditional validation when trigger is false (should not require field)
    form.dependency_trigger = false;
    form.conditional_required = String::new(); // Empty field
    let result = form.validate();
    // Should not have conditional validation errors
    if let Err(errors) = result {
        assert!(errors.get_field_error("conditional_required").is_none());
    }
    
    // Test conditional validation when trigger is true (should require field)
    form.dependency_trigger = true;
    form.conditional_required = String::new(); // Empty field
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.get_field_error("conditional_required").is_some());
    }
    
    // Test conditional validation when trigger is true and field is filled
    form.dependency_trigger = true;
    form.conditional_required = "Valid value".to_string();
    let result = form.validate();
    if let Err(errors) = result {
        assert!(errors.get_field_error("conditional_required").is_none());
    }
}

#[test]
fn test_complex_validation_scenario() {
    // Test a form with multiple validation errors
    let mut form = ValidationTestForm::default();
    
    // Set up multiple validation failures
    form.required_field = String::new(); // Required field empty
    form.email_field = "invalid-email".to_string(); // Invalid email
    form.min_length_field = "123".to_string(); // Too short
    form.max_length_field = "12345678901".to_string(); // Too long
    form.min_value = 5; // Below minimum
    form.max_value = 150; // Above maximum
    form.range_value = 15.0; // Above range
    form.regex_field = "invalid".to_string(); // Doesn't match pattern
    form.alphanumeric_field = "invalid@field".to_string(); // Contains special characters
    form.custom_validation_field = "invalid".to_string(); // Custom validation failure
    form.dependency_trigger = true;
    form.conditional_required = String::new(); // Conditional required field empty
    
    let result = form.validate();
    assert!(result.is_err());
    
    if let Err(errors) = result {
        // Should have multiple validation errors
        assert!(errors.get_field_error("required_field").is_some());
        assert!(errors.get_field_error("email_field").is_some());
        assert!(errors.get_field_error("min_length_field").is_some());
        assert!(errors.get_field_error("max_length_field").is_some());
        assert!(errors.get_field_error("min_value").is_some());
        assert!(errors.get_field_error("max_value").is_some());
        assert!(errors.get_field_error("range_value").is_some());
        assert!(errors.get_field_error("regex_field").is_some());
        assert!(errors.get_field_error("alphanumeric_field").is_some());
        assert!(errors.get_field_error("custom_validation_field").is_some());
        assert!(errors.get_field_error("conditional_required").is_some());
        
        // Should have at least 11 errors
        assert!(errors.error_count() >= 11);
    }
}

#[test]
fn test_validation_error_aggregation() {
    let mut form = ValidationTestForm::default();
    
    // Set up multiple validation failures
    form.required_field = String::new();
    form.email_field = "invalid-email".to_string();
    form.min_length_field = "123".to_string();
    
    let result = form.validate();
    assert!(result.is_err());
    
    if let Err(errors) = result {
        // Test error aggregation
        let error_count = errors.error_count();
        assert!(error_count >= 3);
        
        // Test individual field error access
        let required_errors = errors.get_field_error("required_field");
        assert!(required_errors.is_some());
        assert!(!required_errors.unwrap().is_empty());
        
        let email_errors = errors.get_field_error("email_field");
        assert!(email_errors.is_some());
        assert!(!email_errors.unwrap().is_empty());
        
        let min_length_errors = errors.get_field_error("min_length_field");
        assert!(min_length_errors.is_some());
        assert!(!min_length_errors.unwrap().is_empty());
        
        // Test has_errors method
        assert!(errors.has_errors());
        
        // Test is_empty method
        assert!(!errors.is_empty());
    }
}
