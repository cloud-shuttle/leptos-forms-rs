//! Comprehensive tests for advanced validation
//!
//! This module provides comprehensive tests for advanced validation scenarios.

use super::types::AdvancedValidationForm;
use leptos_forms_rs::Form;

#[test]
fn test_advanced_validation_edge_cases() {
    let mut form = AdvancedValidationForm::default();

    // Test edge case: exactly minimum password length
    form.password = "Pass123!".to_string(); // Exactly 8 characters
    form.confirm_password = "Pass123!".to_string();
    form.email = "test@company.com".to_string();
    form.username = "testuser".to_string();
    form.age = 18; // Minimum age
    form.start_date = "2024-01-01".to_string();
    form.end_date = "2024-01-02".to_string();

    let result = form.validate();
    assert!(result.is_ok(), "Minimum valid values should pass");
}

#[test]
fn test_advanced_validation_boundary_values() {
    let mut form = AdvancedValidationForm::default();

    // Test boundary values
    form.password = "Password123!".to_string();
    form.confirm_password = "Password123!".to_string();
    form.email = "test@company.com".to_string();
    form.username = "testuser".to_string();
    form.age = 120; // Maximum age
    form.start_date = "2024-01-01".to_string();
    form.end_date = "2024-12-31".to_string();

    let result = form.validate();
    assert!(result.is_ok(), "Maximum valid values should pass");
}

#[test]
fn test_advanced_validation_password_complexity() {
    let mut form = AdvancedValidationForm::default();

    // Test password without uppercase
    form.password = "password123!".to_string();
    form.confirm_password = "password123!".to_string();
    form.email = "test@company.com".to_string();
    form.username = "testuser".to_string();
    form.age = 25;
    form.start_date = "2024-01-01".to_string();
    form.end_date = "2024-12-31".to_string();

    let result = form.validate();
    assert!(result.is_err(), "Password without uppercase should fail");
}

#[test]
fn test_advanced_validation_username_edge_cases() {
    let mut form = AdvancedValidationForm::default();

    // Test minimum username length
    form.password = "Password123!".to_string();
    form.confirm_password = "Password123!".to_string();
    form.email = "test@company.com".to_string();
    form.username = "abc".to_string(); // Minimum 3 characters
    form.age = 25;
    form.start_date = "2024-01-01".to_string();
    form.end_date = "2024-12-31".to_string();

    let result = form.validate();
    assert!(result.is_ok(), "Minimum username length should pass");

    // Test maximum username length
    form.username = "abcdefghijklmnopqrst".to_string(); // 20 characters
    let result = form.validate();
    assert!(result.is_ok(), "Maximum username length should pass");
}

#[test]
fn test_advanced_validation_date_edge_cases() {
    let mut form = AdvancedValidationForm::default();

    // Test same start and end date
    form.password = "Password123!".to_string();
    form.confirm_password = "Password123!".to_string();
    form.email = "test@company.com".to_string();
    form.username = "testuser".to_string();
    form.age = 25;
    form.start_date = "2024-01-01".to_string();
    form.end_date = "2024-01-01".to_string(); // Same date

    let result = form.validate();
    assert!(result.is_err(), "Same start and end date should fail");
}
