//! Tests for FormHandle functionality

use leptos::*;
use leptos_forms_rs::*;
use wasm_bindgen_test::*;

#[derive(Clone, Debug, PartialEq)]
struct TestForm {
    name: String,
    email: String,
}

impl Form for TestForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "name".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "email".to_string(),
                field_type: FieldType::Email,
                validators: vec![ValidatorConfig::Required, ValidatorConfig::Email],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
        ]
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        
        if self.name.is_empty() {
            errors.add_field_error("name", FieldError::new("Name is required"));
        }
        
        if self.email.is_empty() {
            errors.add_field_error("email", FieldError::new("Email is required"));
        } else if !self.email.contains('@') {
            errors.add_field_error("email", FieldError::new("Invalid email format"));
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn get_field(&self, name: &str) -> Option<FieldValue> {
        match name {
            "name" => Some(FieldValue::String(self.name.clone())),
            "email" => Some(FieldValue::String(self.email.clone())),
            _ => None,
        }
    }

    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), String> {
        match name {
            "name" => {
                if let FieldValue::String(s) = value {
                    self.name = s;
                    Ok(())
                } else {
                    Err("Expected string value for name".to_string())
                }
            }
            "email" => {
                if let FieldValue::String(s) = value {
                    self.email = s;
                    Ok(())
                } else {
                    Err("Expected string value for email".to_string())
                }
            }
            _ => Err(format!("Unknown field: {}", name)),
        }
    }

    fn default_values() -> Self {
        TestForm {
            name: String::new(),
            email: String::new(),
        }
    }

    fn schema() -> FormSchema {
        FormSchema::new(Self::field_metadata())
    }
}

#[wasm_bindgen_test]
fn test_form_handle_creation() {
    let form = use_form::<TestForm>();
    assert!(form.get_values().get().name.is_empty());
    assert!(form.get_values().get().email.is_empty());
}

#[wasm_bindgen_test]
fn test_form_handle_set_field() {
    let form = use_form::<TestForm>();
    
    let _ = form.set_field_value("name", FieldValue::String("John".to_string()));
    let _ = form.set_field_value("email", FieldValue::String("john@example.com".to_string()));
    
    let values = form.get_values().get();
    assert_eq!(values.name, "John");
    assert_eq!(values.email, "john@example.com");
}

#[wasm_bindgen_test]
fn test_form_handle_validation() {
    let form = use_form::<TestForm>();
    
    // Initially should be invalid (empty fields)
    assert!(!form.is_valid().get());
    
    // Set valid values
    let _ = form.set_field_value("name", FieldValue::String("John".to_string()));
    let _ = form.set_field_value("email", FieldValue::String("john@example.com".to_string()));
    
    // Should now be valid
    assert!(form.is_valid().get());
}

#[wasm_bindgen_test]
fn test_form_handle_reset() {
    let form = use_form::<TestForm>();
    
    // Set some values
    let _ = form.set_field_value("name", FieldValue::String("John".to_string()));
    let _ = form.set_field_value("email", FieldValue::String("john@example.com".to_string()));
    
    // Reset the form
    form.reset();
    
    // Should be back to default values
    let values = form.get_values().get();
    assert!(values.name.is_empty());
    assert!(values.email.is_empty());
}
