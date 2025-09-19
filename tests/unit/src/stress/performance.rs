//! Performance stress tests
//!
//! This module provides performance benchmarks for form creation, validation, and serialization.

use super::types::LargeStressForm;
use leptos_forms_rs::core::{Form, FormHandle};
use leptos::prelude::GetUntracked;
use std::time::Instant;

#[test]
fn test_large_form_creation_performance() {
    let start = Instant::now();
    
    // Create a large form
    let form = LargeStressForm::default();
    
    let creation_time = start.elapsed();
    
    // Verify the form was created successfully
    assert!(creation_time.as_millis() < 100, "Form creation took too long: {:?}", creation_time);
    
    // Verify form has expected number of fields
    let metadata = LargeStressForm::field_metadata();
    assert!(metadata.len() >= 30, "Form should have at least 30 fields");
}

#[test]
fn test_large_form_handle_creation_performance() {
    let start = Instant::now();
    
    // Create a form handle
    let form_handle = FormHandle::new(LargeStressForm::default());
    
    let creation_time = start.elapsed();
    
    // Verify the form handle was created successfully
    assert!(creation_time.as_millis() < 100, "Form handle creation took too long: {:?}", creation_time);
    
    // Verify form handle has expected properties
    let state = form_handle.state().get_untracked();
    assert!(!state.is_dirty);
    assert!(!state.is_submitting);
}

#[test]
fn test_large_form_validation_performance() {
    let form_handle = FormHandle::new(LargeStressForm::default());
    
    let start = Instant::now();
    
    // Validate the form
    let result = form_handle.validate();
    
    let validation_time = start.elapsed();
    
    // Verify validation completed in reasonable time
    assert!(validation_time.as_millis() < 50, "Form validation took too long: {:?}", validation_time);
    
    // Verify validation result
    assert!(result.is_err(), "Empty form should have validation errors");
}

#[test]
fn test_large_form_serialization_performance() {
    let form = LargeStressForm::default();
    
    let start = Instant::now();
    
    // Serialize the form
    let serialized = serde_json::to_string(&form).unwrap();
    
    let serialization_time = start.elapsed();
    
    // Verify serialization completed in reasonable time
    assert!(serialization_time.as_millis() < 50, "Form serialization took too long: {:?}", serialization_time);
    
    // Verify serialized data is not empty
    assert!(!serialized.is_empty(), "Serialized form should not be empty");
    
    // Verify we can deserialize it back
    let deserialized: LargeStressForm = serde_json::from_str(&serialized).unwrap();
    assert_eq!(form, deserialized);
}
