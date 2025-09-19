//! Persistence tests for form persistence

use super::types::PersistenceForm;
use leptos_forms_rs::Form;

#[test]
fn test_persistence_form() {
    let form = PersistenceForm::default();
    assert_eq!(form.data, "");

    let result = form.validate();
    assert!(result.is_ok());
}
