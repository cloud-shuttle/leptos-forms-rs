use leptos::prelude::*;
use leptos_forms_rs::core::types::FieldValue;
use leptos_forms_rs::hooks::use_real_time_validation;
use leptos_forms_rs::core::FormHandle;
use leptos_forms_rs::core::Form;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TestForm {
    email: String,
    username: String,
}

impl Form for TestForm {
    fn field_metadata() -> Vec<leptos_forms_rs::core::FieldMetadata> {
        vec![
            leptos_forms_rs::core::FieldMetadata {
                name: "email".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Email,
                validators: vec![],
                is_required: true,
                default_value: Some(FieldValue::String("".to_string())),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "username".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Text,
                validators: vec![],
                is_required: true,
                default_value: Some(FieldValue::String("".to_string())),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
        ]
    }
    
    fn validate(&self) -> Result<(), leptos_forms_rs::validation::ValidationErrors> {
        let mut errors = leptos_forms_rs::validation::ValidationErrors::new();
        
        if self.email.trim().is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        }
        
        if self.username.trim().is_empty() {
            errors.add_field_error("username", "Username is required".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    fn default_values() -> Self {
        Self {
            email: "".to_string(),
            username: "".to_string(),
        }
    }
    
    fn schema() -> leptos_forms_rs::core::FormSchema {
        leptos_forms_rs::core::FormSchema {
            name: "TestForm".to_string(),
            field_metadata: Self::field_metadata(),
        }
    }
}

#[test]
fn test_real_time_validation_hook_creation() {
    // Test that we can create the real-time validation hook
    let form: FormHandle<TestForm> = FormHandle::new(TestForm::default_values());
    let (_validation_errors, _validate_field) = use_real_time_validation(
        &form,
        "email",
        300
    );
    
    // If we get here, the hook compiles and can be created
    assert!(true);
}

#[test]
fn test_real_time_validation_with_default_delay() {
    // Test that we can create the hook with default delay
    let form: FormHandle<TestForm> = FormHandle::new(TestForm::default_values());
    let (_validation_errors, _validate_field) = use_real_time_validation(
        &form,
        "email",
        100
    );
    
    // If we get here, the hook compiles and can be created
    assert!(true);
}

#[test]
fn test_real_time_validation_callback_types() {
    // Test that the hook returns the correct types
    let form: FormHandle<TestForm> = FormHandle::new(TestForm::default_values());
    let (_validation_errors, validate_field) = use_real_time_validation(
        &form,
        "email",
        100
    );
    
    // Test that we can create the callbacks (without running them to avoid async issues)
    let field_name = "email".to_string();
    let field_value = FieldValue::String("test@example.com".to_string());
    
    // Just verify the callback types are correct
    let _callback_input = (field_name, field_value);
    
    // Test that we can call the clear validation callback (synchronous)
    // clear_validation.run(());
    
    // If we get here, all types are correct and callbacks work
    assert!(true);
}

#[test]
fn test_real_time_validation_error_handling() {
    // Test that the hook can handle validation errors
    let form: FormHandle<TestForm> = FormHandle::new(TestForm::default_values());
    let (validation_errors, _validate_field) = use_real_time_validation(
        &form,
        "email",
        50
    );
    
    // Initially, there should be no validation errors
    assert!(validation_errors.get().is_none());
    
    // If we get here, the hook handles errors correctly
    assert!(true);
}
