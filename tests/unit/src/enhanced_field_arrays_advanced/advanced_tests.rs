//! Advanced tests for enhanced field arrays

use super::types::AdvancedArrayForm;
use leptos_forms_rs::Form;

#[test]
fn test_advanced_array_form() {
    let form = AdvancedArrayForm::default();
    assert!(form.complex_items.is_empty());

    let result = form.validate();
    assert!(result.is_ok());
}
