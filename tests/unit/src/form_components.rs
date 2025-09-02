//! Tests for form components functionality

use serde::{Serialize, Deserialize};
use leptos_forms_rs::core::{FieldType, FieldValue, ValidatorConfig, NumberType, FieldMetadata, FormSchema};
use leptos_forms_rs::{Form, ValidationErrors};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ComponentTestForm {
    text_field: String,
    email_field: String,
    password_field: String,
    number_field: i32,
    boolean_field: bool,
}

impl Form for ComponentTestForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "text_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "email_field".to_string(),
                field_type: FieldType::Email,
                validators: vec![ValidatorConfig::Required, ValidatorConfig::Email],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "password_field".to_string(),
                field_type: FieldType::Password,
                validators: vec![ValidatorConfig::Required, ValidatorConfig::MinLength(8)],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "number_field".to_string(),
                field_type: FieldType::Number(NumberType {
                    min: Some(0.0),
                    max: None,
                    step: None,
                }),
                validators: vec![ValidatorConfig::Required, ValidatorConfig::Min(0.0)],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "boolean_field".to_string(),
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
        
        if self.text_field.is_empty() {
            errors.add_field_error("text_field".to_string(), "Text field is required".to_string());
        }
        
        if self.email_field.is_empty() {
            errors.add_field_error("email_field".to_string(), "Email is required".to_string());
        } else if !self.email_field.contains('@') {
            errors.add_field_error("email_field".to_string(), "Invalid email format".to_string());
        }
        
        if self.password_field.len() < 8 {
            errors.add_field_error("password_field".to_string(), "Password must be at least 8 characters".to_string());
        }
        
        if self.number_field < 0 {
            errors.add_field_error("number_field".to_string(), "Number must be non-negative".to_string());
        }
        
        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn get_field(&self, name: &str) -> Option<FieldValue> {
        match name {
            "text_field" => Some(FieldValue::String(self.text_field.clone())),
            "email_field" => Some(FieldValue::String(self.email_field.clone())),
            "password_field" => Some(FieldValue::String(self.password_field.clone())),
            "number_field" => Some(FieldValue::Number(self.number_field as f64)),
            "boolean_field" => Some(FieldValue::Boolean(self.boolean_field)),
            _ => None,
        }
    }

    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), leptos_forms_rs::core::FieldError> {
        match name {
            "text_field" => {
                if let FieldValue::String(s) = value {
                    self.text_field = s;
                    Ok(())
                } else {
                    Err(leptos_forms_rs::core::FieldError::new("text_field".to_string(), "Expected string value".to_string()))
                }
            },
            "email_field" => {
                if let FieldValue::String(s) = value {
                    self.email_field = s;
                    Ok(())
                } else {
                    Err(leptos_forms_rs::core::FieldError::new("email_field".to_string(), "Expected string value".to_string()))
                }
            },
            "password_field" => {
                if let FieldValue::String(s) = value {
                    self.password_field = s;
                    Ok(())
                } else {
                    Err(leptos_forms_rs::core::FieldError::new("password_field".to_string(), "Expected string value".to_string()))
                }
            },
            "number_field" => {
                if let FieldValue::Number(n) = value {
                    self.number_field = n as i32;
                    Ok(())
                } else {
                    Err(leptos_forms_rs::core::FieldError::new("number_field".to_string(), "Expected number value".to_string()))
                }
            },
            "boolean_field" => {
                if let FieldValue::Boolean(b) = value {
                    self.boolean_field = b;
                    Ok(())
                } else {
                    Err(leptos_forms_rs::core::FieldError::new("boolean_field".to_string(), "Expected boolean value".to_string()))
                }
            },
            _ => Err(leptos_forms_rs::core::FieldError::new(name.to_string(), "Unknown field".to_string())),
        }
    }

    fn default_values() -> Self {
        Self {
            text_field: String::new(),
            email_field: String::new(),
            password_field: String::new(),
            number_field: 0,
            boolean_field: false,
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
fn test_components_form_validation() {
    let mut form = ComponentTestForm::default_values();
    
    // Test empty form validation
    let result = form.validate();
    assert!(result.is_err());
    
    // Test valid form
    let _ = form.set_field("text_field", FieldValue::String("Sample text".to_string()));
    let _ = form.set_field("email_field", FieldValue::String("test@example.com".to_string()));
    let _ = form.set_field("password_field", FieldValue::String("securepass123".to_string()));
    let _ = form.set_field("number_field", FieldValue::Number(42.0));
    let _ = form.set_field("boolean_field", FieldValue::Boolean(true));
    
    let result = form.validate();
    assert!(result.is_ok());
}

#[test]
fn test_components_form_schema() {
    let schema = ComponentTestForm::schema();
    assert_eq!(schema.fields.len(), 5);
    
    let required_fields = schema.required_fields();
    assert_eq!(required_fields.len(), 4); // text, email, password, number are required
}

#[test]
fn test_components_field_access() {
    let mut form = ComponentTestForm::default_values();
    
    // Test setting and getting field values
    let _ = form.set_field("text_field", FieldValue::String("test".to_string()));
    let value = form.get_field("text_field");
    assert_eq!(value, Some(FieldValue::String("test".to_string())));
    
    // Test unknown field
    let value = form.get_field("unknown_field");
    assert_eq!(value, None);
}
