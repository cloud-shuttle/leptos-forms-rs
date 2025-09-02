//! Tests for form validation functionality

use leptos::*;
use leptos_forms_rs::*;
use wasm_bindgen_test::*;

#[derive(Clone, Debug, PartialEq)]
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
                validators: vec![ValidatorConfig::Required, ValidatorConfig::MinLength(3)],
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
        
        // Required field validation
        if self.required_field.is_empty() {
            errors.add_field_error("required_field", FieldError::new("Required field is mandatory"));
        }
        
        // Email validation
        if self.email_field.is_empty() {
            errors.add_field_error("email_field", FieldError::new("Email is required"));
        } else if !self.email_field.contains('@') || !self.email_field.contains('.') {
            errors.add_field_error("email_field", FieldError::new("Invalid email format"));
        }
        
        // Min length validation
        if self.min_length_field.len() < 3 {
            errors.add_field_error("min_length_field", FieldError::new("Must be at least 3 characters"));
        }
        
        // Max length validation
        if self.max_length_field.len() > 10 {
            errors.add_field_error("max_length_field", FieldError::new("Must be no more than 10 characters"));
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
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

    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), String> {
        match name {
            "required_field" => {
                if let FieldValue::String(s) = value {
                    self.required_field = s;
                    Ok(())
                } else {
                    Err("Expected string value for required_field".to_string())
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
            "min_length_field" => {
                if let FieldValue::String(s) = value {
                    self.min_length_field = s;
                    Ok(())
                } else {
                    Err("Expected string value for min_length_field".to_string())
                }
            }
            "max_length_field" => {
                if let FieldValue::String(s) = value {
                    self.max_length_field = s;
                    Ok(())
                } else {
                    Err("Expected string value for max_length_field".to_string())
                }
            }
            _ => Err(format!("Unknown field: {}", name)),
        }
    }

    fn default_values() -> Self {
        ValidationTestForm {
            required_field: String::new(),
            email_field: String::new(),
            min_length_field: String::new(),
            max_length_field: String::new(),
        }
    }

    fn schema() -> FormSchema {
        FormSchema::new(Self::field_metadata())
    }
}

#[wasm_bindgen_test]
fn test_required_field_validation() {
    let form = use_form::<ValidationTestForm>();
    
    // Initially should be invalid (empty required field)
    assert!(!form.is_valid().get());
    
    // Set required field
    let _ = form.set_field_value("required_field", FieldValue::String("Value".to_string()));
    
    // Still invalid because other fields are empty
    assert!(!form.is_valid().get());
}

#[wasm_bindgen_test]
fn test_email_validation() {
    let form = use_form::<ValidationTestForm>();
    
    // Set invalid email
    let _ = form.set_field_value("email_field", FieldValue::String("invalid-email".to_string()));
    
    // Should be invalid
    assert!(!form.is_valid().get());
    
    // Set valid email
    let _ = form.set_field_value("email_field", FieldValue::String("valid@email.com".to_string()));
    
    // Email should now be valid
    // (though form still invalid due to other empty fields)
}

#[wasm_bindgen_test]
fn test_min_length_validation() {
    let form = use_form::<ValidationTestForm>();
    
    // Set field below min length
    let _ = form.set_field_value("min_length_field", FieldValue::String("ab".to_string()));
    
    // Should be invalid
    assert!(!form.is_valid().get());
    
    // Set field at min length
    let _ = form.set_field_value("min_length_field", FieldValue::String("abc".to_string()));
    
    // Should now be valid for this field
}

#[wasm_bindgen_test]
fn test_max_length_validation() {
    let form = use_form::<ValidationTestForm>();
    
    // Set field above max length
    let _ = form.set_field_value("max_length_field", FieldValue::String("this is too long".to_string()));
    
    // Should be invalid
    assert!(!form.is_valid().get());
    
    // Set field at max length
    let _ = form.set_field_value("max_length_field", FieldValue::String("1234567890".to_string()));
    
    // Should now be valid for this field
}
