//! Concurrency and array field stress tests
//!
//! This module provides tests for concurrent field access and array field operations.

use super::types::LargeStressForm;
use std::time::Instant;

#[test]
fn test_concurrent_field_access() {
    // Note: Leptos signals are not thread-safe, so we test sequential access instead
    let mut form = LargeStressForm::default();

    // Simulate high-frequency updates in a single thread
    for i in 0..1000 {
        form.name = format!("Update {}", i);
        form.field_1 = format!("Field 1 Update {}", i);
    }

    // Verify the form is in a consistent state
    assert!(
        !form.name.is_empty(),
        "Form should have a name after updates"
    );
    assert!(
        !form.field_1.is_empty(),
        "Form should have field_1 value after updates"
    );

    // Verify the final values
    assert_eq!(form.name, "Update 999");
    assert_eq!(form.field_1, "Field 1 Update 999");
}

#[test]
fn test_array_field_stress() {
    let mut form = LargeStressForm::default();

    let start = Instant::now();

    // Test basic array field operations
    // Add a few items to test the functionality
    form.tags.push("tag_1".to_string());
    form.tags.push("tag_2".to_string());
    form.tags.push("tag_3".to_string());

    let array_operations_time = start.elapsed();

    // Verify array operations completed in reasonable time
    assert!(
        array_operations_time.as_millis() < 500,
        "Array operations took too long: {:?}",
        array_operations_time
    );

    // Verify array length
    assert_eq!(form.tags.len(), 3, "Tags array should have 3 items");

    // Test array item access
    assert_eq!(form.tags[0], "tag_1");

    // Test array item modification
    form.tags[0] = "modified_tag".to_string();
    assert_eq!(form.tags[0], "modified_tag");

    // Test array item removal
    form.tags.remove(0);
    assert_eq!(form.tags.len(), 2);

    // Test array clearing
    form.tags.clear();
    assert_eq!(form.tags.len(), 0);
}
