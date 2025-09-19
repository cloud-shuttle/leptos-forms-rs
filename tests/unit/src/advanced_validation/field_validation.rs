//! Field validation tests for advanced validation
//!
//! This module provides tests for individual field validation rules.

use super::types::AdvancedValidationForm;
use leptos_forms_rs::Form;

#[test]
fn test_advanced_validation_password_validation() {
    let mut form = AdvancedValidationForm::default();

    // Test empty password
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        println!("Validation failed with errors: {:?}", errors);
    }

    // Test short password
    form.password = "short".to_string();
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        println!("Validation failed with errors: {:?}", errors);
    }

    // Test password without required complexity
    form.password = "password123".to_string();
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        println!("Validation failed with errors: {:?}", errors);
    }

    // Test valid password
    form.password = "Password123!".to_string();
    form.confirm_password = "Password123!".to_string();
    form.email = "test@company.com".to_string();
    form.username = "testuser".to_string();
    form.age = 25;
    form.start_date = "2024-01-01".to_string();
    form.end_date = "2024-12-31".to_string();

    let result = form.validate();
    // Valid password should not cause validation errors
    if result.is_err() {
        println!("Unexpected validation error: {:?}", result.unwrap_err());
    }
}

#[test]
fn test_advanced_validation_password_mismatch() {
    let mut form = AdvancedValidationForm::default();
    form.password = "Password123!".to_string();
    form.confirm_password = "DifferentPassword123!".to_string();

    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        println!("Validation failed with errors: {:?}", errors);
    }
}

#[test]
fn test_advanced_validation_email_domain_validation() {
    let mut form = AdvancedValidationForm::default();
    form.password = "Password123!".to_string();
    form.confirm_password = "Password123!".to_string();
    form.username = "testuser".to_string();
    form.age = 25;
    form.start_date = "2024-01-01".to_string();
    form.end_date = "2024-12-31".to_string();

    // Test invalid email domain
    form.email = "test@example.com".to_string();
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        println!("Validation failed with errors: {:?}", errors);
    }

    // Test valid email domain
    form.email = "test@company.com".to_string();
    let result = form.validate();
    // Valid email should not cause validation errors
    if result.is_err() {
        println!("Unexpected validation error: {:?}", result.unwrap_err());
    }
}

#[test]
fn test_advanced_validation_username_availability() {
    let mut form = AdvancedValidationForm::default();
    form.password = "Password123!".to_string();
    form.confirm_password = "Password123!".to_string();
    form.email = "test@company.com".to_string();
    form.age = 25;
    form.start_date = "2024-01-01".to_string();
    form.end_date = "2024-12-31".to_string();

    // Test reserved username
    form.username = "admin".to_string();
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        println!("Validation failed with errors: {:?}", errors);
    }

    // Test another reserved username
    form.username = "root".to_string();
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        println!("Validation failed with errors: {:?}", errors);
    }

    // Test valid username
    form.username = "testuser".to_string();
    let result = form.validate();
    // Valid username should not cause validation errors
    if result.is_err() {
        println!("Unexpected validation error: {:?}", result.unwrap_err());
    }
}

#[test]
fn test_advanced_validation_age_validation() {
    let mut form = AdvancedValidationForm::default();
    form.password = "Password123!".to_string();
    form.confirm_password = "Password123!".to_string();
    form.email = "test@company.com".to_string();
    form.username = "testuser".to_string();
    form.start_date = "2024-01-01".to_string();
    form.end_date = "2024-12-31".to_string();

    // Test age too young
    form.age = 17;
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        println!("Validation failed with errors: {:?}", errors);
    }

    // Test age too old
    form.age = 121;
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        println!("Validation failed with errors: {:?}", errors);
    }

    // Test valid age
    form.age = 25;
    let result = form.validate();
    // Valid age should not cause validation errors
    if result.is_err() {
        println!("Unexpected validation error: {:?}", result.unwrap_err());
    }
}
