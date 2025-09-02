//! Tests for FormHandle functionality

use serde::{Serialize, Deserialize};
use leptos_forms_rs::core::{FieldType, FieldValue, ValidatorConfig, FieldMetadata, FormSchema, FieldError};
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
            errors.add_field_error("name".to_string(), "Name is required".to_string());
        }
        
        if self.email.is_empty() {
            errors.add_field_error("email".to_string(), "Email is required".to_string());
        } else if !self.email.contains('@') {
            errors.add_field_error("email".to_string(), "Invalid email format".to_string());
        }
        
        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn get_field(&self, name: &str) -> Option<FieldValue> {
        match name {
            "name" => Some(FieldValue::String(self.name.clone())),
            "email" => Some(FieldValue::String(self.email.clone())),
            _ => None,
        }
    }

    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), FieldError> {
        match name {
            "name" => {
                if let FieldValue::String(s) = value {
                    self.name = s;
                    Ok(())
                } else {
                    Err(FieldError::new("name".to_string(), "Expected string value".to_string()))
                }
            },
            "email" => {
                if let FieldValue::String(s) = value {
                    self.email = s;
                    Ok(())
                } else {
                    Err(FieldError::new("email".to_string(), "Expected string value".to_string()))
                }
            },
            _ => Err(FieldError::new(name.to_string(), "Unknown field".to_string())),
        }
    }

    fn default_values() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
        }
    }

    fn schema() -> FormSchema {
        let mut schema = FormSchema::new();
        for field in Self::field_metadata() {
            schema.add_field(field);
        }
        schema
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
    
    let _ = form.set_field("name", FieldValue::String("John".to_string()));
    let _ = form.set_field("email", FieldValue::String("john@example.com".to_string()));
    
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
    let _ = form.set_field("name", FieldValue::String("John".to_string()));
    let _ = form.set_field("email", FieldValue::String("john@example.com".to_string()));
    
    // Should now be valid
    let result = form.validate();
    assert!(result.is_ok());
}

#[test]
fn test_form_handle_schema() {
    let schema = TestForm::schema();
    assert_eq!(schema.fields.len(), 2);
    
    let required_fields = schema.required_fields();
    assert_eq!(required_fields.len(), 2); // Both fields are required
}

#[test]
fn test_form_handle_field_access() {
    let mut form = TestForm::default_values();
    
    // Test setting and getting field values
    let _ = form.set_field("name", FieldValue::String("Jane Doe".to_string()));
    let value = form.get_field("name");
    assert_eq!(value, Some(FieldValue::String("Jane Doe".to_string())));
    
    // Test unknown field
    let value = form.get_field("unknown_field");
    assert_eq!(value, None);
}
