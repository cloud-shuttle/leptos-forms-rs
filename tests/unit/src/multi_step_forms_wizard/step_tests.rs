//! Step tests for multi-step forms wizard

use super::types::WizardForm;
use leptos_forms_rs::Form;

#[test]
fn test_wizard_step_validation() {
    let mut form = WizardForm::default();

    // Test step 1 validation
    form.name = "John Doe".to_string();
    let result = form.validate();
    assert!(result.is_err()); // Still missing email and age

    // Test step 2 validation
    form.email = "john@example.com".to_string();
    let result = form.validate();
    assert!(result.is_err()); // Still missing age

    // Test complete validation
    form.age = 25;
    let result = form.validate();
    assert!(result.is_ok());
}
