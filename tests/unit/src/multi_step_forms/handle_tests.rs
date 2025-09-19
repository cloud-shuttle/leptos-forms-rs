//! Form handle tests for multi-step forms
//!
//! This module provides tests for form handle operations and validation.

use super::types::MultiStepTestForm;
use leptos_forms_rs::Form;

#[test]
fn test_multi_step_form_validation_errors() {
    let mut form = MultiStepTestForm::default();

    // Test validation with empty form
    let result = form.validate();
    assert!(result.is_err());

    // Set some valid data
    form.set_field_value(
        "first_name",
        leptos_forms_rs::core::FieldValue::String("John".to_string()),
    );
    form.set_field_value(
        "last_name",
        leptos_forms_rs::core::FieldValue::String("Doe".to_string()),
    );
    form.set_field_value(
        "email",
        leptos_forms_rs::core::FieldValue::String("john@example.com".to_string()),
    );
    form.set_field_value(
        "phone",
        leptos_forms_rs::core::FieldValue::String("1234567890".to_string()),
    );
    form.set_field_value(
        "address",
        leptos_forms_rs::core::FieldValue::String("123 Main Street".to_string()),
    );
    form.set_field_value(
        "city",
        leptos_forms_rs::core::FieldValue::String("New York".to_string()),
    );

    // Test validation with valid data
    let result = form.validate();
    assert!(
        result.is_ok(),
        "Form with valid data should pass validation"
    );
}

#[test]
fn test_multi_step_form_step_validation() {
    let mut form = MultiStepTestForm::default();

    // Test step-by-step validation
    // Step 1: Set only first name
    form.set_field_value(
        "first_name",
        leptos_forms_rs::core::FieldValue::String("John".to_string()),
    );
    let result = form.validate();
    assert!(result.is_err(), "Should fail with incomplete step 1");

    // Complete step 1
    form.set_field_value(
        "last_name",
        leptos_forms_rs::core::FieldValue::String("Doe".to_string()),
    );
    form.set_field_value(
        "email",
        leptos_forms_rs::core::FieldValue::String("john@example.com".to_string()),
    );

    // Should still fail because step 2 is incomplete
    let result = form.validate();
    assert!(result.is_err(), "Should fail with incomplete step 2");

    // Complete step 2
    form.set_field_value(
        "phone",
        leptos_forms_rs::core::FieldValue::String("1234567890".to_string()),
    );
    form.set_field_value(
        "address",
        leptos_forms_rs::core::FieldValue::String("123 Main Street".to_string()),
    );
    form.set_field_value(
        "city",
        leptos_forms_rs::core::FieldValue::String("New York".to_string()),
    );

    // Should pass now
    let result = form.validate();
    assert!(result.is_ok(), "Should pass with complete form");
}
