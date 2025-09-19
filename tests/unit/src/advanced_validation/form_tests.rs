//! Form tests for advanced validation
//!
//! This module provides tests for form initialization, metadata, and basic form operations.

use super::types::AdvancedValidationForm;
use leptos::prelude::GetUntracked;
use leptos_forms_rs::core::FormHandle;
use leptos_forms_rs::Form;

#[test]
fn test_advanced_validation_form_initialization() {
    let form = AdvancedValidationForm::default();

    // Test default values
    assert_eq!(form.password, "");
    assert_eq!(form.confirm_password, "");
    assert_eq!(form.email, "");
    assert_eq!(form.username, "");
    assert_eq!(form.age, 0);
    assert_eq!(form.start_date, "");
    assert_eq!(form.end_date, "");
    assert_eq!(form.custom_field, "");
}

#[test]
fn test_advanced_validation_form_field_metadata() {
    let metadata = AdvancedValidationForm::field_metadata();

    // Test that we have the expected number of fields
    assert_eq!(metadata.len(), 8);

    // Test password field metadata
    let password_field = metadata.iter().find(|f| f.name == "password").unwrap();
    assert!(password_field.is_required);
    assert_eq!(password_field.validators.len(), 2);

    // Test email field metadata
    let email_field = metadata.iter().find(|f| f.name == "email").unwrap();
    assert!(email_field.is_required);
    assert_eq!(email_field.validators.len(), 2);

    // Test username field metadata
    let username_field = metadata.iter().find(|f| f.name == "username").unwrap();
    assert!(username_field.is_required);
    assert_eq!(username_field.validators.len(), 4);

    // Test age field metadata
    let age_field = metadata.iter().find(|f| f.name == "age").unwrap();
    assert!(age_field.is_required);
    assert_eq!(age_field.validators.len(), 2);

    // Test custom field metadata
    let custom_field = metadata.iter().find(|f| f.name == "custom_field").unwrap();
    assert!(!custom_field.is_required);
    assert_eq!(custom_field.validators.len(), 1);
}

#[test]
fn test_advanced_validation_form_handle_creation() {
    let form_handle = FormHandle::new(AdvancedValidationForm::default());

    // Test that form handle was created successfully
    let state = form_handle.state().get_untracked();
    assert!(!state.is_dirty);
    assert!(!state.is_submitting);
}

#[test]
fn test_advanced_validation_form_handle_field_updates() {
    let form_handle = FormHandle::new(AdvancedValidationForm::default());

    // Test field updates
    form_handle.set_field_value(
        "username",
        leptos_forms_rs::core::FieldValue::String("testuser".to_string()),
    );
    form_handle.set_field_value(
        "email",
        leptos_forms_rs::core::FieldValue::String("test@company.com".to_string()),
    );
    form_handle.set_field_value("age", leptos_forms_rs::core::FieldValue::Number(25.0));

    // Verify updates
    let form_data = form_handle.values().get_untracked();
    assert_eq!(form_data.username, "testuser");
    assert_eq!(form_data.email, "test@company.com");
    assert_eq!(form_data.age, 25);

    // Verify form is marked as dirty
    let state = form_handle.state().get_untracked();
    assert!(state.is_dirty);
}
