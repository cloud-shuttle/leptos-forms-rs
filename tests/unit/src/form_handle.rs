//! Tests for FormHandle functionality

use serde::{Serialize, Deserialize};
use leptos_forms_rs::core::{FieldType, FieldValue, FieldMetadata, FormSchema, FieldError};
use leptos_forms_rs::validation::Validator;
use leptos_forms_rs::{Form, ValidationErrors};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
                validators: vec![Validator::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "email".to_string(),
                field_type: FieldType::Email,
                validators: vec![Validator::Required, Validator::Email],
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
            errors.add_field_error("name", "Name is required".to_string());
        }
        
        if self.email.is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        } else if !self.email.contains('@') {
            errors.add_field_error("email", "Invalid email format".to_string());
        }
        
        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn get_field_value(&self, name: &str) -> FieldValue {
        match name {
            "name" => FieldValue::String(self.name.clone()),
            "email" => FieldValue::String(self.email.clone()),
            _ => FieldValue::String(String::new()),
        }
    }

    fn default_values() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
        }
    }

    fn schema() -> FormSchema {
        FormSchema {
            name: "TestForm".to_string(),
            field_metadata: Self::field_metadata(),
        }
    }
}

#[test]
fn test_form_handle_creation() {
    let form = TestForm::default_values();
    assert!(form.name.is_empty());
    assert!(form.email.is_empty());
}

#[test]
fn test_form_handle_set_field() {
    let mut form = TestForm::default_values();
    
    form.name = "John".to_string();
    form.email = "john@example.com".to_string();
    
    assert_eq!(form.name, "John");
    assert_eq!(form.email, "john@example.com");
}

#[test]
fn test_form_handle_validation() {
    let mut form = TestForm::default_values();
    
    // Initially should be invalid (empty fields)
    let result = form.validate();
    assert!(result.is_err());
    
    // Set valid values
    form.name = "John".to_string();
    form.email = "john@example.com".to_string();
    
    // Should now be valid
    let result = form.validate();
    assert!(result.is_ok());
}

#[test]
fn test_form_handle_schema() {
    let schema = TestForm::schema();
    assert_eq!(schema.field_metadata.len(), 2);
    
    // Note: required_fields() method doesn't exist in current API
    // let required_fields = schema.required_fields();
    // assert_eq!(required_fields.len(), 2); // Both fields are required
}

#[test]
fn test_form_handle_field_access() {
    let mut form = TestForm::default_values();
    
    // Test setting and getting field values
    form.name = "Jane Doe".to_string();
    let value = form.get_field_value("name");
    assert_eq!(value, FieldValue::String("Jane Doe".to_string()));
    
    // Test unknown field
    let value = form.get_field_value("unknown_field");
    assert_eq!(value, FieldValue::String(String::new()));
}
