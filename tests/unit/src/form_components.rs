//! Tests for form components functionality

use leptos::*;
use leptos_forms_rs::*;
use wasm_bindgen_test::*;

#[derive(Clone, Debug, PartialEq)]
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
                field_type: FieldType::Number,
                validators: vec![ValidatorConfig::Required],
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
            errors.add_field_error("text_field", FieldError::new("Text field is required"));
        }
        
        if self.email_field.is_empty() {
            errors.add_field_error("email_field", FieldError::new("Email is required"));
        } else if !self.email_field.contains('@') {
            errors.add_field_error("email_field", FieldError::new("Invalid email format"));
        }
        
        if self.password_field.len() < 8 {
            errors.add_field_error("password_field", FieldError::new("Password must be at least 8 characters"));
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn get_field(&self, name: &str) -> Option<FieldValue> {
        match name {
            "text_field" => Some(FieldValue::String(self.text_field.clone())),
            "email_field" => Some(FieldValue::String(self.email_field.clone())),
            "password_field" => Some(FieldValue::String(self.password_field.clone())),
            "number_field" => Some(FieldValue::Number(self.number_field)),
            "boolean_field" => Some(FieldValue::Boolean(self.boolean_field)),
            _ => None,
        }
    }

    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), String> {
        match name {
            "text_field" => {
                if let FieldValue::String(s) = value {
                    self.text_field = s;
                    Ok(())
                } else {
                    Err("Expected string value for text_field".to_string())
                }
            }
            "email_field" => {
                if let FieldValue::String(s) = value {
                    self.email_field = s;
                    Ok(())
                } else {
                    Err("Expected string value for email_field".to_string())
                }
            }
            "password_field" => {
                if let FieldValue::String(s) = value {
                    self.password_field = s;
                    Ok(())
                } else {
                    Err("Expected string value for password_field".to_string())
                }
            }
            "number_field" => {
                if let FieldValue::Number(n) = value {
                    self.number_field = n;
                    Ok(())
                } else {
                    Err("Expected number value for number_field".to_string())
                }
            }
            "boolean_field" => {
                if let FieldValue::Boolean(b) = value {
                    self.boolean_field = b;
                    Ok(())
                } else {
                    Err("Expected boolean value for boolean_field".to_string())
                }
            }
            _ => Err(format!("Unknown field: {}", name)),
        }
    }

    fn default_values() -> Self {
        ComponentTestForm {
            text_field: String::new(),
            email_field: String::new(),
            password_field: String::new(),
            number_field: 0,
            boolean_field: false,
        }
    }

    fn schema() -> FormSchema {
        FormSchema::new(Self::field_metadata())
    }
}

#[wasm_bindgen_test]
fn test_text_field_component() {
    let form = use_form::<ComponentTestForm>();
    
    let _ = form.set_field_value("text_field", FieldValue::String("Test text".to_string()));
    
    let values = form.get_values().get();
    assert_eq!(values.text_field, "Test text");
}

#[wasm_bindgen_test]
fn test_email_field_component() {
    let form = use_form::<ComponentTestForm>();
    
    let _ = form.set_field_value("email_field", FieldValue::String("test@example.com".to_string()));
    
    let values = form.get_values().get();
    assert_eq!(values.email_field, "test@example.com");
}

#[wasm_bindgen_test]
fn test_password_field_component() {
    let form = use_form::<ComponentTestForm>();
    
    let _ = form.set_field_value("password_field", FieldValue::String("securepass123".to_string()));
    
    let values = form.get_values().get();
    assert_eq!(values.password_field, "securepass123");
}

#[wasm_bindgen_test]
fn test_number_field_component() {
    let form = use_form::<ComponentTestForm>();
    
    let _ = form.set_field_value("number_field", FieldValue::Number(42));
    
    let values = form.get_values().get();
    assert_eq!(values.number_field, 42);
}

#[wasm_bindgen_test]
fn test_boolean_field_component() {
    let form = use_form::<ComponentTestForm>();
    
    let _ = form.set_field_value("boolean_field", FieldValue::Boolean(true));
    
    let values = form.get_values().get();
    assert_eq!(values.boolean_field, true);
}

#[wasm_bindgen_test]
fn test_field_type_validation() {
    let form = use_form::<ComponentTestForm>();
    
    // Try to set wrong type for number field
    let result = form.set_field_value("number_field", FieldValue::String("not a number".to_string()));
    assert!(result.is_err());
    
    // Try to set wrong type for boolean field
    let result = form.set_field_value("boolean_field", FieldValue::String("not a boolean".to_string()));
    assert!(result.is_err());
}
