//! Validation tests for multi-step forms
//!
//! This module provides tests for step-by-step validation and complete form validation.

use super::types::MultiStepTestForm;
use leptos_forms_rs::Form;

#[test]
fn test_multi_step_form_validation_step1() {
    let mut form = MultiStepTestForm::default();

    // Test step 1 validation with empty fields
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        println!("Step 1 validation errors: {:?}", errors);
    }

    // Test step 1 validation with valid data
    form.first_name = "John".to_string();
    form.last_name = "Doe".to_string();
    form.email = "john@example.com".to_string();

    let result = form.validate();
    // Should still fail because step 2 fields are empty
    assert!(result.is_err());
}

#[test]
fn test_multi_step_form_validation_step2() {
    let mut form = MultiStepTestForm::default();

    // Set step 1 data
    form.first_name = "John".to_string();
    form.last_name = "Doe".to_string();
    form.email = "john@example.com".to_string();

    // Test step 2 validation with empty fields
    let result = form.validate();
    assert!(result.is_err());
    if let Err(errors) = result {
        println!("Step 2 validation errors: {:?}", errors);
    }

    // Test step 2 validation with valid data
    form.phone = "1234567890".to_string();
    form.address = "123 Main Street".to_string();
    form.city = "New York".to_string();

    let result = form.validate();
    // Should pass now
    assert!(result.is_ok());
}

#[test]
fn test_multi_step_form_validation_complete() {
    let mut form = MultiStepTestForm::default();

    // Set all required fields
    form.first_name = "John".to_string();
    form.last_name = "Doe".to_string();
    form.email = "john@example.com".to_string();
    form.phone = "1234567890".to_string();
    form.address = "123 Main Street".to_string();
    form.city = "New York".to_string();
    form.newsletter = true;
    form.notifications = false;
    form.theme = "dark".to_string();

    let result = form.validate();
    assert!(result.is_ok(), "Complete form should pass validation");
}

#[test]
fn test_multi_step_form_field_values() {
    let mut form = MultiStepTestForm::default();
    form.first_name = "John".to_string();
    form.last_name = "Doe".to_string();
    form.email = "john@example.com".to_string();

    let form_data = form.get_form_data();
    assert_eq!(
        form_data.get("first_name").unwrap(),
        &leptos_forms_rs::core::FieldValue::String("John".to_string())
    );
    assert_eq!(
        form_data.get("last_name").unwrap(),
        &leptos_forms_rs::core::FieldValue::String("Doe".to_string())
    );
    assert_eq!(
        form_data.get("email").unwrap(),
        &leptos_forms_rs::core::FieldValue::String("john@example.com".to_string())
    );
}

#[test]
fn test_multi_step_form_set_field_values() {
    let mut form = MultiStepTestForm::default();

    form.set_field_value(
        "first_name",
        leptos_forms_rs::core::FieldValue::String("Jane".to_string()),
    );
    form.set_field_value(
        "newsletter",
        leptos_forms_rs::core::FieldValue::Boolean(true),
    );

    assert_eq!(form.first_name, "Jane");
    assert_eq!(form.newsletter, true);
}
