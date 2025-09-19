//! Wizard tests for multi-step forms wizard

use super::types::WizardForm;
use leptos_forms_rs::Form;

#[test]
fn test_wizard_form_initialization() {
    let form = WizardForm::default();
    assert_eq!(form.name, "");
    assert_eq!(form.email, "");
    assert_eq!(form.age, 0);
    assert_eq!(form.preferences, "");
}

#[test]
fn test_wizard_form_validation() {
    let mut form = WizardForm::default();
    
    // Test empty form
    let result = form.validate();
    assert!(result.is_err());
    
    // Test valid form
    form.name = "John Doe".to_string();
    form.email = "john@example.com".to_string();
    form.age = 25;
    
    let result = form.validate();
    assert!(result.is_ok());
}
