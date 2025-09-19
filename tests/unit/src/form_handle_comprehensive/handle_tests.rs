//! Handle tests for form handle comprehensive

use super::types::ComprehensiveForm;
use leptos_forms_rs::Form;

#[test]
fn test_comprehensive_form_handle() {
    let form = ComprehensiveForm::default();
    assert_eq!(form.name, "");
    assert_eq!(form.email, "");
    assert_eq!(form.age, 0);

    let result = form.validate();
    assert!(result.is_err());
}
