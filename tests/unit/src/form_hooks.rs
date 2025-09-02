//! Tests for form hooks functionality

use serde::{Serialize, Deserialize};
use leptos_forms_rs::core::{FieldType, FieldValue, ValidatorConfig, NumberType, FieldMetadata, FormSchema};
use leptos_forms_rs::{Form, ValidationErrors};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct HooksTestForm {
    name: String,
    email: String,
    age: i32,
    is_active: bool,
}

impl Form for HooksTestForm {
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
            FieldMetadata {
                name: "age".to_string(),
                field_type: FieldType::Number(NumberType {
                    min: Some(18.0),
                    max: None,
                    step: None,
                }),
                validators: vec![ValidatorConfig::Required, ValidatorConfig::Min(18.0)],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "is_active".to_string(),
                field_type: FieldType::Boolean,
                validators: vec![],
                is_required: false,
                default_value: Some(FieldValue::Boolean(false)),
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
        
        if self.age < 18 {
            errors.add_field_error("age".to_string(), "Age must be at least 18".to_string());
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
            "age" => Some(FieldValue::Number(self.age as f64)),
            "is_active" => Some(FieldValue::Boolean(self.is_active)),
            _ => None,
        }
    }

    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), leptos_forms_rs::core::FieldError> {
        match name {
            "name" => {
                if let FieldValue::String(s) = value {
                    self.name = s;
                    Ok(())
                } else {
                    Err(leptos_forms_rs::core::FieldError::new("name".to_string(), "Expected string value".to_string()))
                }
            },
            "email" => {
                if let FieldValue::String(s) = value {
                    self.email = s;
                    Ok(())
                } else {
                    Err(leptos_forms_rs::core::FieldError::new("email".to_string(), "Expected string value".to_string()))
                }
            },
            "age" => {
                if let FieldValue::Number(n) = value {
                    self.age = n as i32;
                    Ok(())
                } else {
                    Err(leptos_forms_rs::core::FieldError::new("age".to_string(), "Expected number value".to_string()))
                }
            },
            "is_active" => {
                if let FieldValue::Boolean(b) = value {
                    self.is_active = b;
                    Ok(())
                } else {
                    Err(leptos_forms_rs::core::FieldError::new("is_active".to_string(), "Expected boolean value".to_string()))
                }
            },
            _ => Err(leptos_forms_rs::core::FieldError::new(name.to_string(), "Unknown field".to_string())),
        }
    }

    fn default_values() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            age: 18,
            is_active: false,
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
fn test_hooks_form_validation() {
    let mut form = HooksTestForm::default_values();
    
    // Test empty form validation
    let result = form.validate();
    assert!(result.is_err());
    
    // Test valid form
    let _ = form.set_field("name", FieldValue::String("John Doe".to_string()));
    let _ = form.set_field("email", FieldValue::String("john@example.com".to_string()));
    let _ = form.set_field("age", FieldValue::Number(25.0));
    let _ = form.set_field("is_active", FieldValue::Boolean(true));
    
    let result = form.validate();
    assert!(result.is_ok());
}

#[test]
fn test_hooks_form_schema() {
    let schema = HooksTestForm::schema();
    assert_eq!(schema.fields.len(), 4);
    
    let required_fields = schema.required_fields();
    assert_eq!(required_fields.len(), 3); // name, email, age are required
}

#[test]
fn test_hooks_field_access() {
    let mut form = HooksTestForm::default_values();
    
    // Test setting and getting field values
    let _ = form.set_field("name", FieldValue::String("Jane Doe".to_string()));
    let value = form.get_field("name");
    assert_eq!(value, Some(FieldValue::String("Jane Doe".to_string())));
    
    // Test unknown field
    let value = form.get_field("unknown_field");
    assert_eq!(value, None);
}
