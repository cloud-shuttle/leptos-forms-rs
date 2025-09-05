//! Basic WASM tests for leptos-forms-rs
//!
//! These tests verify that the core functionality works in a WASM environment

use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, Form, FormHandle};
use leptos_forms_rs::validation::{ValidationErrors, Validator};
use serde::{Deserialize, Serialize};
use wasm_bindgen_test::*;

// Simple test form for WASM testing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct WasmTestForm {
    name: String,
    email: String,
    age: i32,
}

impl Default for WasmTestForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            age: 0,
        }
    }
}

impl Form for WasmTestForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "name".to_string(),
                field_type: FieldType::Text,
                is_required: true,
                default_value: Some(FieldValue::String(String::new())),
                dependencies: Vec::new(),
                attributes: std::collections::HashMap::new(),
                validators: vec![Validator::Required],
            },
            FieldMetadata {
                name: "email".to_string(),
                field_type: FieldType::Email,
                is_required: true,
                default_value: Some(FieldValue::String(String::new())),
                dependencies: Vec::new(),
                attributes: std::collections::HashMap::new(),
                validators: vec![Validator::Required, Validator::Email],
            },
            FieldMetadata {
                name: "age".to_string(),
                field_type: FieldType::Number(leptos_forms_rs::core::NumberType {
                    min: Some(0.0),
                    max: Some(120.0),
                    step: Some(1.0),
                }),
                is_required: false,
                default_value: Some(FieldValue::Number(0.0)),
                dependencies: Vec::new(),
                attributes: std::collections::HashMap::new(),
                validators: vec![Validator::Min(0.0), Validator::Max(120.0)],
            },
        ]
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.name.is_empty() {
            errors.add_field_error("name", "Name is required".to_string());
        }

        if self.email.is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        } else if !self.email.contains('@') {
            errors.add_field_error("email", "Invalid email format".to_string());
        }

        if self.age < 0 || self.age > 120 {
            errors.add_field_error("age", "Age must be between 0 and 120".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn default_values() -> Self {
        Self::default()
    }

    fn get_field_value(&self, field_name: &str) -> FieldValue {
        match field_name {
            "name" => FieldValue::String(self.name.clone()),
            "email" => FieldValue::String(self.email.clone()),
            "age" => FieldValue::Number(self.age as f64),
            _ => FieldValue::String(String::new()),
        }
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "name" => {
                if let FieldValue::String(s) = value {
                    self.name = s;
                }
            }
            "email" => {
                if let FieldValue::String(s) = value {
                    self.email = s;
                }
            }
            "age" => {
                if let FieldValue::Number(n) = value {
                    self.age = n as i32;
                }
            }
            _ => {}
        }
    }
}

// WASM-specific tests
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_wasm_form_creation() {
    let form = WasmTestForm::default();
    assert_eq!(form.name, "");
    assert_eq!(form.email, "");
    assert_eq!(form.age, 0);
}

#[wasm_bindgen_test]
fn test_wasm_form_handle_creation() {
    let form = WasmTestForm::default();
    let _form_handle = FormHandle::new(form);
    // If we get here without panicking, the form handle was created successfully
}

#[wasm_bindgen_test]
fn test_wasm_form_validation_valid() {
    let form = WasmTestForm {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: 25,
    };

    let result = form.validate();
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn test_wasm_form_validation_invalid() {
    let form = WasmTestForm {
        name: "".to_string(),               // Invalid: empty name
        email: "invalid-email".to_string(), // Invalid: no @
        age: -5,                            // Invalid: negative age
    };

    let result = form.validate();
    assert!(result.is_err());

    if let Err(errors) = result {
        assert!(errors.has_field_error("name"));
        assert!(errors.has_field_error("email"));
        assert!(errors.has_field_error("age"));
    }
}

#[wasm_bindgen_test]
fn test_wasm_field_metadata() {
    let metadata = WasmTestForm::field_metadata();
    assert_eq!(metadata.len(), 3);

    let name_field = metadata.iter().find(|f| f.name == "name").unwrap();
    assert!(matches!(name_field.field_type, FieldType::Text));
    assert!(name_field.is_required);

    let email_field = metadata.iter().find(|f| f.name == "email").unwrap();
    assert!(matches!(email_field.field_type, FieldType::Email));
    assert!(email_field.is_required);

    let age_field = metadata.iter().find(|f| f.name == "age").unwrap();
    assert!(matches!(age_field.field_type, FieldType::Number(_)));
    assert!(!age_field.is_required);
}

#[wasm_bindgen_test]
fn test_wasm_field_value_operations() {
    let mut form = WasmTestForm::default();

    // Test setting and getting field values
    form.set_field_value("name", FieldValue::String("Alice".to_string()));
    form.set_field_value("email", FieldValue::String("alice@example.com".to_string()));
    form.set_field_value("age", FieldValue::Number(30.0));

    assert_eq!(
        form.get_field_value("name"),
        FieldValue::String("Alice".to_string())
    );
    assert_eq!(
        form.get_field_value("email"),
        FieldValue::String("alice@example.com".to_string())
    );
    assert_eq!(form.get_field_value("age"), FieldValue::Number(30.0));
}

#[wasm_bindgen_test]
fn test_wasm_serialization() {
    let form = WasmTestForm {
        name: "Bob Smith".to_string(),
        email: "bob@example.com".to_string(),
        age: 35,
    };

    // Test serialization
    let serialized = serde_json::to_string(&form).unwrap();
    assert!(serialized.contains("Bob Smith"));
    assert!(serialized.contains("bob@example.com"));
    assert!(serialized.contains("35"));

    // Test deserialization
    let deserialized: WasmTestForm = serde_json::from_str(&serialized).unwrap();
    assert_eq!(form, deserialized);
}
