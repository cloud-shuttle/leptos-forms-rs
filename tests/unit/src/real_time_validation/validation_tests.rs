//! Validation tests for real-time validation

use super::types::RealTimeForm;
use leptos_forms_rs::Form;

#[test]
fn test_real_time_validation() {
    let form = RealTimeForm::default();
    assert_eq!(form.email, "");
    
    let result = form.validate();
    assert!(result.is_err());
}
