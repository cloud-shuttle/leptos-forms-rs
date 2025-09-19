//! Array tests for enhanced field arrays

use super::types::ArrayTestForm;
use leptos_forms_rs::Form;

#[test]
fn test_array_form_initialization() {
    let form = ArrayTestForm::default();
    assert!(form.items.is_empty());
    assert!(form.tags.is_empty());
    assert!(form.numbers.is_empty());
}

#[test]
fn test_array_form_validation() {
    let form = ArrayTestForm::default();
    let result = form.validate();
    assert!(result.is_ok());
}
