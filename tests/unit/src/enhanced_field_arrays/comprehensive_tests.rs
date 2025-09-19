//! Comprehensive tests for enhanced field arrays
//!
//! This module provides comprehensive tests for array field operations.

use super::types::ArrayTestForm;
use leptos_forms_rs::Form;

#[test]
fn test_array_comprehensive_operations() {
    let mut form = ArrayTestForm::default();

    // Test adding many items
    for i in 0..50 {
        form.items.push(format!("item_{}", i));
        form.tags.push(format!("tag_{}", i));
        form.numbers.push(i);
    }

    assert_eq!(form.items.len(), 50);
    assert_eq!(form.tags.len(), 50);
    assert_eq!(form.numbers.len(), 50);

    let result = form.validate();
    assert!(result.is_ok(), "Large arrays should pass validation");
}

#[test]
fn test_array_mixed_types() {
    let mut form = ArrayTestForm::default();

    // Test mixed content types
    form.items.push("item1".to_string());
    form.items.push("item2".to_string());
    form.items.push("item3".to_string());

    form.tags.push("tag1".to_string());
    form.tags.push("tag2".to_string());

    form.numbers.push(1);
    form.numbers.push(2);
    form.numbers.push(3);

    let result = form.validate();
    assert!(result.is_ok(), "Mixed array types should pass validation");
}

#[test]
fn test_array_edge_cases() {
    let mut form = ArrayTestForm::default();

    // Test empty arrays
    assert!(form.items.is_empty());
    assert!(form.tags.is_empty());
    assert!(form.numbers.is_empty());

    let result = form.validate();
    assert!(result.is_ok(), "Empty arrays should pass validation");

    // Test single item arrays
    form.items.push("single_item".to_string());
    form.tags.push("single_tag".to_string());
    form.numbers.push(42);

    let result = form.validate();
    assert!(result.is_ok(), "Single item arrays should pass validation");
}

#[test]
fn test_array_duplicate_values() {
    let mut form = ArrayTestForm::default();

    // Test duplicate values
    form.items.push("duplicate".to_string());
    form.items.push("duplicate".to_string());
    form.items.push("duplicate".to_string());

    form.tags.push("tag".to_string());
    form.tags.push("tag".to_string());

    form.numbers.push(5);
    form.numbers.push(5);
    form.numbers.push(5);

    let result = form.validate();
    assert!(result.is_ok(), "Duplicate values should pass validation");
}
