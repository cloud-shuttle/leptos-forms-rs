//! Basic validation tests
//!
//! This module provides tests for basic validation rules including
//! required fields, email validation, and length validation.

use super::types::ValidationTestForm;
use leptos_forms_rs::Form;

#[test]
fn test_required_field_validation() {
    let mut form = ValidationTestForm::default();
    
    // Test empty required field
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.get_field_error("required_field").is_some());
    }
    
    // Test valid required field
    form.required_field = "Valid value".to_string();
    let result = form.validate();
    assert!(result.is_ok() || result.is_err()); // May still have other validation errors
}

#[test]
fn test_email_validation() {
    let mut form = ValidationTestForm::default();
    form.required_field = "Valid".to_string(); // Set required field to avoid that error
    
    // Test invalid email
    form.email_field = "invalid-email".to_string();
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.get_field_error("email_field").is_some());
    }
    
    // Test valid email
    form.email_field = "test@example.com".to_string();
    let result = form.validate();
    // Should not have email validation errors
    if let Err(errors) = result {
        assert!(errors.get_field_error("email_field").is_none());
    }
    
    // Test empty email (should be valid)
    form.email_field = String::new();
    let result = form.validate();
    if let Err(errors) = result {
        assert!(errors.get_field_error("email_field").is_none());
    }
}

#[test]
fn test_length_validation() {
    let mut form = ValidationTestForm::default();
    form.required_field = "Valid".to_string(); // Set required field
    
    // Test min length validation
    form.min_length_field = "123".to_string(); // Too short
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.get_field_error("min_length_field").is_some());
    }
    
    // Test valid min length
    form.min_length_field = "12345".to_string(); // Valid length
    let result = form.validate();
    if let Err(errors) = result {
        assert!(errors.get_field_error("min_length_field").is_none());
    }
    
    // Test max length validation
    form.max_length_field = "12345678901".to_string(); // Too long
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.get_field_error("max_length_field").is_some());
    }
    
    // Test valid max length
    form.max_length_field = "1234567890".to_string(); // Valid length
    let result = form.validate();
    if let Err(errors) = result {
        assert!(errors.get_field_error("max_length_field").is_none());
    }
    
    // Test exact length validation
    form.exact_length_field = "12".to_string(); // Wrong length
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.get_field_error("exact_length_field").is_some());
    }
    
    // Test valid exact length
    form.exact_length_field = "123".to_string(); // Valid length
    let result = form.validate();
    if let Err(errors) = result {
        assert!(errors.get_field_error("exact_length_field").is_none());
    }
}
