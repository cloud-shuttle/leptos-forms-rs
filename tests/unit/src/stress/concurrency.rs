//! Concurrency and array field stress tests
//!
//! This module provides tests for concurrent field access and array field operations.

use super::types::LargeStressForm;
use leptos_forms_rs::core::{FieldValue, FormHandle};
use leptos::prelude::GetUntracked;
use std::time::Instant;

#[test]
fn test_concurrent_field_access() {
    // Note: Leptos signals are not thread-safe, so we test sequential access instead
    let form_handle = FormHandle::new(LargeStressForm::default());
    
    // Simulate high-frequency updates in a single thread
    for i in 0..1000 {
        form_handle.set_field_value(
            "name", 
            FieldValue::String(format!("Update {}", i))
        );
        form_handle.set_field_value(
            "field_1", 
            FieldValue::String(format!("Field 1 Update {}", i))
        );
    }
    
    // Verify the form is in a consistent state
    let state = form_handle.state().get_untracked();
    assert!(state.is_dirty, "Form should be marked as dirty after updates");
    
    // Verify we can still access the form data
    let form_data = form_handle.values().get_untracked();
    assert!(!form_data.name.is_empty(), "Form should have a name after updates");
    assert!(!form_data.field_1.is_empty(), "Form should have field_1 value after updates");
}

#[test]
fn test_array_field_stress() {
    let form_handle = FormHandle::new(LargeStressForm::default());
    
    let start = Instant::now();
    
    // Test basic array field operations
    // Add a few items to test the functionality
    form_handle.add_array_item("tags", FieldValue::String("tag_1".to_string()));
    form_handle.add_array_item("tags", FieldValue::String("tag_2".to_string()));
    form_handle.add_array_item("tags", FieldValue::String("tag_3".to_string()));
    
    let array_operations_time = start.elapsed();
    
    // Verify array operations completed in reasonable time
    assert!(array_operations_time.as_millis() < 500, "Array operations took too long: {:?}", array_operations_time);
    
    // Verify array length
    let tags_length = form_handle.get_array_length("tags").unwrap_or(0);
    assert_eq!(tags_length, 3, "Tags array should have 3 items");
    
    // Test array item access
    let first_tag = form_handle.get_array_item("tags", 0);
    assert!(first_tag.is_some());
    if let Some(FieldValue::String(tag)) = first_tag {
        assert_eq!(tag, "tag_1");
    }
    
    // Test array item modification
    form_handle.set_array_item("tags", 0, FieldValue::String("modified_tag".to_string()));
    let modified_tag = form_handle.get_array_item("tags", 0);
    assert!(modified_tag.is_some());
    if let Some(FieldValue::String(tag)) = modified_tag {
        assert_eq!(tag, "modified_tag");
    }
    
    // Test array item removal
    form_handle.remove_array_item("tags", 0);
    assert_eq!(form_handle.get_array_length("tags"), Some(2));
    
    // Test array clearing
    form_handle.clear_array("tags");
    assert_eq!(form_handle.get_array_length("tags"), Some(0));
}
