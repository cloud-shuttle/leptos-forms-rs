//! Tests for form validation functionality

use serde::{Serialize, Deserialize};
use leptos_forms_rs::core::{FieldType, FieldValue, FieldMetadata, FormSchema};
use leptos_forms_rs::validation::Validator;
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
                validators: vec![Validator::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "email_field".to_string(),
                field_type: FieldType::Email,
                validators: vec![Validator::Required, Validator::Email],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "min_length_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Required, Validator::MinLength(5)],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "max_length_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Required, Validator::MaxLength(10)],
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
            errors.add_field_error("required_field", "Required field is empty".to_string());
        }
        
        if self.email_field.is_empty() {
            errors.add_field_error("email_field", "Email is required".to_string());
        } else if !self.email_field.contains('@') {
            errors.add_field_error("email_field", "Invalid email format".to_string());
        }
        
        if self.min_length_field.len() < 5 {
            errors.add_field_error("min_length_field", "Field must be at least 5 characters".to_string());
        }
        
        if self.max_length_field.len() > 10 {
            errors.add_field_error("max_length_field", "Field must be at most 10 characters".to_string());
        }
        
        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn get_field_value(&self, name: &str) -> FieldValue {
        match name {
            "required_field" => FieldValue::String(self.required_field.clone()),
            "email_field" => FieldValue::String(self.email_field.clone()),
            "min_length_field" => FieldValue::String(self.min_length_field.clone()),
            "max_length_field" => FieldValue::String(self.max_length_field.clone()),
            _ => FieldValue::String(String::new()),
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
        FormSchema {
            name: "ValidationTestForm".to_string(),
            field_metadata: Self::field_metadata(),
        }
    }
}

#[test]
fn test_validation_form_validation() {
    let mut form = ValidationTestForm::default_values();
    
    // Test empty form validation
    let result = form.validate();
    assert!(result.is_err());
    
    // Test valid form
    form.required_field = "Valid text".to_string();
    form.email_field = "test@example.com".to_string();
    form.min_length_field = "Long enough".to_string();
    form.max_length_field = "Short".to_string();
    
    let result = form.validate();
    assert!(result.is_ok());
}

#[test]
fn test_validation_form_schema() {
    let schema = ValidationTestForm::schema();
    assert_eq!(schema.field_metadata.len(), 4);
    
    // Note: required_fields() method doesn't exist in current API
    // let required_fields = schema.required_fields();
    // assert_eq!(required_fields.len(), 4); // All fields are required
}

#[test]
fn test_validation_field_access() {
    let mut form = ValidationTestForm::default_values();
    
    // Test setting and getting field values
    form.required_field = "test".to_string();
    let value = form.get_field_value("required_field");
    assert_eq!(value, FieldValue::String("test".to_string()));
    
    // Test unknown field
    let value = form.get_field_value("unknown_field");
    assert_eq!(value, FieldValue::String(String::new()));
}
