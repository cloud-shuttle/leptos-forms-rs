//! Navigation tests for multi-step forms wizard

use super::types::WizardForm;
use leptos_forms_rs::Form;

#[test]
fn test_wizard_navigation_flow() {
    let mut form = WizardForm::default();
    
    // Simulate step-by-step navigation
    form.name = "John Doe".to_string();
    // Step 1 complete
    
    form.email = "john@example.com".to_string();
    // Step 2 complete
    
    form.age = 25;
    // Step 3 complete
    
    let result = form.validate();
    assert!(result.is_ok(), "All steps should be valid");
}
