//! Conditional tests for conditional validation

use super::types::ConditionalForm;
use leptos_forms_rs::Form;

#[test]
fn test_conditional_validation() {
    let form = ConditionalForm::default();
    assert_eq!(form.trigger, false);
    assert_eq!(form.conditional_field, "");

    let result = form.validate();
    assert!(result.is_ok());
}
