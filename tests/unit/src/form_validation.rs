//! Tests for form validation functionality

use serde::{Serialize, Deserialize};
use leptos_forms_rs::core::{FieldType, FieldValue, ValidatorConfig, FieldMetadata, FormSchema};
use leptos_forms_rs::{Form, ValidationErrors};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ValidationTestForm {
    required_field: String,
    email_field: String,
    min_length_field: String,
    max_length_field: String,
}

impl Form for ValidationTestForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "required_field".to_string(),
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
                name: "min_length_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required, ValidatorConfig::MinLength(5)],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "max_length_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required, ValidatorConfig::MaxLength(10)],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
        ]
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        
        if self.required_field.is_empty() {
            errors.add_field_error("required_field".to_string(), "Required field is empty".to_string());
        }
        
        if self.email_field.is_empty() {
            errors.add_field_error("email_field".to_string(), "Email is required".to_string());
        } else if !self.email_field.contains('@') {
            errors.add_field_error("email_field".to_string(), "Invalid email format".to_string());
        }
        
        if self.min_length_field.len() < 5 {
            errors.add_field_error("min_length_field".to_string(), "Field must be at least 5 characters".to_string());
        }
        
        if self.max_length_field.len() > 10 {
            errors.add_field_error("max_length_field".to_string(), "Field must be at most 10 characters".to_string());
        }
        
        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn get_field(&self, name: &str) -> Option<FieldValue> {
        match name {
            "required_field" => Some(FieldValue::String(self.required_field.clone())),
            "email_field" => Some(FieldValue::String(self.email_field.clone())),
            "min_length_field" => Some(FieldValue::String(self.min_length_field.clone())),
            "max_length_field" => Some(FieldValue::String(self.max_length_field.clone())),
            _ => None,
        }
    }

    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), leptos_forms_rs::core::FieldError> {
        match name {
            "required_field" => {
                if let FieldValue::String(s) = value {
                    self.required_field = s;
                    Ok(())
                } else {
                    Err(leptos_forms_rs::core::FieldError::new("required_field".to_string(), "Expected string value".to_string()))
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
            "min_length_field" => {
                if let FieldValue::String(s) = value {
                    self.min_length_field = s;
                    Ok(())
                } else {
                    Err(leptos_forms_rs::core::FieldError::new("min_length_field".to_string(), "Expected string value".to_string()))
                }
            },
            "max_length_field" => {
                if let FieldValue::String(s) = value {
                    self.max_length_field = s;
                    Ok(())
                } else {
                    Err(leptos_forms_rs::core::FieldError::new("max_length_field".to_string(), "Expected string value".to_string()))
                }
            },
            _ => Err(leptos_forms_rs::core::FieldError::new(name.to_string(), "Unknown field".to_string())),
        }
    }

    fn default_values() -> Self {
        Self {
            required_field: String::new(),
            email_field: String::new(),
            min_length_field: String::new(),
            max_length_field: String::new(),
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
fn test_validation_form_validation() {
    let mut form = ValidationTestForm::default_values();
    
    // Test empty form validation
    let result = form.validate();
    assert!(result.is_err());
    
    // Test valid form
    let _ = form.set_field("required_field", FieldValue::String("Valid text".to_string()));
    let _ = form.set_field("email_field", FieldValue::String("test@example.com".to_string()));
    let _ = form.set_field("min_length_field", FieldValue::String("Long enough".to_string()));
    let _ = form.set_field("max_length_field", FieldValue::String("Short".to_string()));
    
    let result = form.validate();
    assert!(result.is_ok());
}

#[test]
fn test_validation_form_schema() {
    let schema = ValidationTestForm::schema();
    assert_eq!(schema.fields.len(), 4);
    
    let required_fields = schema.required_fields();
    assert_eq!(required_fields.len(), 4); // All fields are required
}

#[test]
fn test_validation_field_access() {
    let mut form = ValidationTestForm::default_values();
    
    // Test setting and getting field values
    let _ = form.set_field("required_field", FieldValue::String("test".to_string()));
    let value = form.get_field("required_field");
    assert_eq!(value, Some(FieldValue::String("test".to_string())));
    
    // Test unknown field
    let value = form.get_field("unknown_field");
    assert_eq!(value, None);
}
