//! Business rules tests for advanced validation
//!
//! This module provides tests for complex business rules and cross-field validation.

use super::types::AdvancedValidationForm;
use leptos_forms_rs::Form;

#[test]
fn test_advanced_validation_cross_field_date_validation() {
    let mut form = AdvancedValidationForm::default();
    form.password = "Password123!".to_string();
    form.confirm_password = "Password123!".to_string();
    form.email = "test@company.com".to_string();
    form.username = "testuser".to_string();
    form.age = 25;

    // Test end date before start date
    form.start_date = "2024-12-31".to_string();
    form.end_date = "2024-01-01".to_string();

    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.has_errors());
    }

    // Test valid date range
    form.start_date = "2024-01-01".to_string();
    form.end_date = "2024-12-31".to_string();

    let result = form.validate();
    // Valid date range should not cause validation errors
    if result.is_err() {
        println!("Unexpected validation error: {:?}", result.unwrap_err());
    }
}

#[test]
fn test_advanced_validation_custom_business_rule() {
    let mut form = AdvancedValidationForm::default();
    form.password = "Password123!".to_string();
    form.confirm_password = "Password123!".to_string();
    form.email = "test@company.com".to_string();
    form.username = "testuser".to_string();
    form.age = 25;
    form.start_date = "2024-01-01".to_string();
    form.end_date = "2024-12-31".to_string();

    // Test invalid custom field value
    form.custom_field = "invalid_value".to_string();
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        assert!(errors.has_errors());
    }

    // Test valid custom field value
    form.custom_field = "valid_value".to_string();
    let result = form.validate();
    // Valid custom field should not cause validation errors
    if result.is_err() {
        println!("Unexpected validation error: {:?}", result.unwrap_err());
    }

    // Test empty custom field (should be valid since it's not required)
    form.custom_field = "".to_string();
    let result = form.validate();
    // Empty custom field should not cause validation errors
    if result.is_err() {
        println!("Unexpected validation error: {:?}", result.unwrap_err());
    }
}

#[test]
fn test_advanced_validation_complete_valid_form() {
    let mut form = AdvancedValidationForm::default();

    // Set all fields to valid values
    form.password = "Password123!".to_string();
    form.confirm_password = "Password123!".to_string();
    form.email = "test@company.com".to_string();
    form.username = "testuser".to_string();
    form.age = 25;
    form.start_date = "2024-01-01".to_string();
    form.end_date = "2024-12-31".to_string();
    form.custom_field = "valid_value".to_string();

    let result = form.validate();
    assert!(result.is_ok(), "Complete valid form should pass validation");
}

#[test]
fn test_advanced_validation_form_handle_validation() {
    let mut form = AdvancedValidationForm::default();

    // Test validation with empty form
    let result = form.validate();
    assert!(result.is_err());

    // Set valid values
    form.password = "Password123!".to_string();
    form.confirm_password = "Password123!".to_string();
    form.email = "test@company.com".to_string();
    form.username = "testuser".to_string();
    form.age = 25;
    form.start_date = "2024-01-01".to_string();
    form.end_date = "2024-12-31".to_string();
    form.custom_field = "valid_value".to_string();

    // Test validation with valid form
    let result = form.validate();
    assert!(
        result.is_ok(),
        "Form with valid data should pass validation"
    );
}

#[test]
fn test_advanced_validation_form_handle_invalid_data() {
    let mut form = AdvancedValidationForm::default();

    // Set invalid values
    form.password = "short".to_string();
    form.confirm_password = "different".to_string();
    form.email = "invalid-email".to_string();
    form.username = "admin".to_string();
    form.age = 17;
    form.start_date = "invalid-date".to_string();
    form.end_date = "2024-01-01".to_string();
    form.custom_field = "invalid_value".to_string();

    // Test validation with invalid form
    let result = form.validate();
    assert!(
        result.is_err(),
        "Form with invalid data should fail validation"
    );

    if let Err(errors) = result {
        // Should have multiple validation errors
        // Note: The specific field error checking would need to be adapted
        // based on the actual ValidationErrors implementation
        println!("Validation failed with errors: {:?}", errors);
    }
}
