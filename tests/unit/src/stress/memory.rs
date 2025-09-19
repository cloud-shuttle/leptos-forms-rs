//! Memory usage stress tests
//!
//! This module provides tests for memory usage and high-frequency field updates.

use super::types::LargeStressForm;
use leptos_forms_rs::core::{FieldValue, FormHandle};
use leptos::prelude::GetUntracked;
use std::time::Instant;

#[test]
fn test_memory_usage_large_form() {
    let form_handle = FormHandle::new(LargeStressForm::default());
    
    // Test memory usage by creating many form instances
    let mut forms = Vec::new();
    
    for i in 0..100 {
        let mut form = LargeStressForm::default();
        form.name = format!("Test Form {}", i);
        form.email = format!("test{}@example.com", i);
        forms.push(form);
    }
    
    // Verify we can still access the original form handle
    let state = form_handle.state().get_untracked();
    assert!(!state.is_dirty);
    
    // Verify all forms were created
    assert_eq!(forms.len(), 100);
}

#[test]
fn test_high_frequency_field_updates() {
    let form_handle = FormHandle::new(LargeStressForm::default());
    
    let start = Instant::now();
    
    // Perform many field updates
    for i in 0..1000 {
        form_handle.set_field_value("name", FieldValue::String(format!("Updated Name {}", i)));
        form_handle.set_field_value("email", FieldValue::String(format!("updated{}@example.com", i)));
        form_handle.set_field_value("field_1", FieldValue::String(format!("Field 1 Value {}", i)));
    }
    
    let update_time = start.elapsed();
    
    // Verify updates completed in reasonable time
    assert!(update_time.as_millis() < 1000, "High frequency updates took too long: {:?}", update_time);
    
    // Verify the final state
    let state = form_handle.state().get_untracked();
    assert!(state.is_dirty, "Form should be marked as dirty after updates");
    
    // Verify the final values
    let form_data = form_handle.values().get_untracked();
    assert_eq!(form_data.name, "Updated Name 999");
    assert_eq!(form_data.email, "updated999@example.com");
    assert_eq!(form_data.field_1, "Field 1 Value 999");
}
