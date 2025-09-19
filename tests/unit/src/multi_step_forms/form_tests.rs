//! Form tests for multi-step forms
//!
//! This module provides tests for form initialization, metadata, and basic form operations.

use super::types::MultiStepTestForm;
use leptos_forms_rs::core::FormHandle;
use leptos_forms_rs::Form;
use leptos::prelude::GetUntracked;

#[test]
fn test_multi_step_form_initialization() {
    let form = MultiStepTestForm::default();
    
    // Test default values
    assert_eq!(form.first_name, "");
    assert_eq!(form.last_name, "");
    assert_eq!(form.email, "");
    assert_eq!(form.phone, "");
    assert_eq!(form.address, "");
    assert_eq!(form.city, "");
    assert_eq!(form.newsletter, false);
    assert_eq!(form.notifications, false);
    assert_eq!(form.theme, "light");
}

#[test]
fn test_multi_step_form_field_metadata() {
    let metadata = MultiStepTestForm::field_metadata();
    
    // Test that we have the expected number of fields
    assert_eq!(metadata.len(), 9);
    
    // Test step 1 fields
    let first_name_field = metadata.iter().find(|f| f.name == "first_name").unwrap();
    assert!(first_name_field.is_required);
    assert_eq!(first_name_field.validators.len(), 2);
    
    // Test step 2 fields
    let phone_field = metadata.iter().find(|f| f.name == "phone").unwrap();
    assert!(phone_field.is_required);
    assert_eq!(phone_field.validators.len(), 2);
    
    // Test step 3 fields
    let newsletter_field = metadata.iter().find(|f| f.name == "newsletter").unwrap();
    assert!(!newsletter_field.is_required);
    assert_eq!(newsletter_field.validators.len(), 0);
}

#[test]
fn test_multi_step_form_handle_creation() {
    let form_handle = FormHandle::new(MultiStepTestForm::default());
    
    // Test that form handle was created successfully
    let state = form_handle.state().get_untracked();
    assert!(!state.is_dirty);
    assert!(!state.is_submitting);
}

#[test]
fn test_multi_step_form_handle_field_updates() {
    let form_handle = FormHandle::new(MultiStepTestForm::default());
    
    // Test field updates
    form_handle.set_field_value("first_name", leptos_forms_rs::core::FieldValue::String("John".to_string()));
    form_handle.set_field_value("last_name", leptos_forms_rs::core::FieldValue::String("Doe".to_string()));
    form_handle.set_field_value("email", leptos_forms_rs::core::FieldValue::String("john@example.com".to_string()));
    
    // Verify updates
    let form_data = form_handle.values().get_untracked();
    assert_eq!(form_data.first_name, "John");
    assert_eq!(form_data.last_name, "Doe");
    assert_eq!(form_data.email, "john@example.com");
    
    // Verify form is marked as dirty
    let state = form_handle.state().get_untracked();
    assert!(state.is_dirty);
}
