//! Tests for form hooks functionality

use serde::{Serialize, Deserialize};
use leptos_forms_rs::core::{FieldType, FieldValue, NumberType, FieldMetadata, FormSchema};
use leptos_forms_rs::validation::Validator;
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
            FieldMetadata {
                name: "age".to_string(),
                field_type: FieldType::Number(NumberType {
                    min: Some(18.0),
                    max: None,
                    step: None,
                }),
                validators: vec![Validator::Required, Validator::Min(18.0)],
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
            errors.add_field_error("name", "Name is required".to_string());
        }
        
        if self.email.is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        } else if !self.email.contains('@') {
            errors.add_field_error("email", "Invalid email format".to_string());
        }
        
        if self.age < 18 {
            errors.add_field_error("age", "Age must be at least 18".to_string());
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
            "age" => FieldValue::Number(self.age as f64),
            "is_active" => FieldValue::Boolean(self.is_active),
            _ => FieldValue::String(String::new()),
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
        FormSchema {
            name: "HooksTestForm".to_string(),
            field_metadata: Self::field_metadata(),
        }
    }
}

#[test]
fn test_hooks_form_validation() {
    let mut form = HooksTestForm::default_values();
    
    // Test empty form validation
    let result = form.validate();
    assert!(result.is_err());
    
    // Test valid form
    form.name = "John Doe".to_string();
    form.email = "john@example.com".to_string();
    form.age = 25;
    form.is_active = true;
    
    let result = form.validate();
    assert!(result.is_ok());
}

#[test]
fn test_hooks_form_schema() {
    let schema = HooksTestForm::schema();
    assert_eq!(schema.field_metadata.len(), 4);
    
    // Note: required_fields() method doesn't exist in current API
    // let required_fields = schema.required_fields();
    // assert_eq!(required_fields.len(), 3); // name, email, age are required
}

#[test]
fn test_hooks_field_access() {
    let mut form = HooksTestForm::default_values();
    
    // Test setting and getting field values
    form.name = "Jane Doe".to_string();
    let value = form.get_field_value("name");
    assert_eq!(value, FieldValue::String("Jane Doe".to_string()));
    
    // Test unknown field
    let value = form.get_field_value("unknown_field");
    assert_eq!(value, FieldValue::String(String::new()));
}
