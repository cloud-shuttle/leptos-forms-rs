//! Operations tests for enhanced field arrays

use super::types::ArrayTestForm;
use leptos_forms_rs::Form;

#[test]
fn test_array_operations() {
    let mut form = ArrayTestForm::default();
    
    // Test setting array values
    form.set_field_value("items", leptos_forms_rs::core::FieldValue::Array(vec![
        leptos_forms_rs::core::FieldValue::String("item1".to_string()),
        leptos_forms_rs::core::FieldValue::String("item2".to_string()),
    ]));
    
    assert_eq!(form.items.len(), 2);
    assert_eq!(form.items[0], "item1");
    assert_eq!(form.items[1], "item2");
}
