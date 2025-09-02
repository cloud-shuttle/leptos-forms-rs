//! Tests for form hooks functionality

use leptos::*;
use leptos_forms_rs::*;
use wasm_bindgen_test::*;

#[derive(Clone, Debug, PartialEq)]
struct HooksTestForm {
    name: String,
    email: String,
    age: i32,
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
                field_type: FieldType::Number,
                validators: vec![ValidatorConfig::Required],
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
            errors.add_field_error("name", FieldError::new("Name is required"));
        }
        
        if self.email.is_empty() {
            errors.add_field_error("email", FieldError::new("Email is required"));
        } else if !self.email.contains('@') {
            errors.add_field_error("email", FieldError::new("Invalid email format"));
        }
        
        if self.age < 0 {
            errors.add_field_error("age", FieldError::new("Age must be positive"));
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn get_field(&self, name: &str) -> Option<FieldValue> {
        match name {
            "name" => Some(FieldValue::String(self.name.clone())),
            "email" => Some(FieldValue::String(self.email.clone())),
            "age" => Some(FieldValue::Number(self.age)),
            _ => None,
        }
    }

    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), String> {
        match name {
            "name" => {
                if let FieldValue::String(s) = value {
                    self.name = s;
                    Ok(())
                } else {
                    Err("Expected string value for name".to_string())
                }
            }
            "email" => {
                if let FieldValue::String(s) = value {
                    self.email = s;
                    Ok(())
                } else {
                    Err("Expected string value for email".to_string())
                }
            }
            "age" => {
                if let FieldValue::Number(n) = value {
                    self.age = n;
                    Ok(())
                } else {
                    Err("Expected number value for age".to_string())
                }
            }
            _ => Err(format!("Unknown field: {}", name)),
        }
    }

    fn default_values() -> Self {
        HooksTestForm {
            name: String::new(),
            email: String::new(),
            age: 0,
        }
    }

    fn schema() -> FormSchema {
        FormSchema::new(Self::field_metadata())
    }
}

#[wasm_bindgen_test]
fn test_use_form_hook() {
    let form = use_form::<HooksTestForm>();
    
    // Test initial state
    let values = form.get_values().get();
    assert!(values.name.is_empty());
    assert!(values.email.is_empty());
    assert_eq!(values.age, 0);
    
    // Test form validity
    assert!(!form.is_valid().get());
}

#[wasm_bindgen_test]
fn test_use_field_value_hook() {
    let form = use_form::<HooksTestForm>();
    
    // Set field values
    let _ = form.set_field_value("name", FieldValue::String("John Doe".to_string()));
    let _ = form.set_field_value("email", FieldValue::String("john@example.com".to_string()));
    let _ = form.set_field_value("age", FieldValue::Number(30));
    
    // Test field value hooks
    let name_value = use_field_value(&form, "name");
    let email_value = use_field_value(&form, "email");
    let age_value = use_field_value(&form, "age");
    
    assert_eq!(name_value.get(), "John Doe");
    assert_eq!(email_value.get(), "john@example.com");
    assert_eq!(age_value.get(), 30);
}

#[wasm_bindgen_test]
fn test_use_field_error_hook() {
    let form = use_form::<HooksTestForm>();
    
    // Initially no errors
    let name_error = use_field_error(&form, "name");
    assert!(name_error.get().is_none());
    
    // Trigger validation by checking form validity
    let _ = form.is_valid();
    
    // Now should have errors for empty required fields
    let name_error = use_field_error(&form, "name");
    assert!(name_error.get().is_some());
}

#[wasm_bindgen_test]
fn test_use_field_dirty_hook() {
    let form = use_form::<HooksTestForm>();
    
    // Initially not dirty
    let name_dirty = use_field_dirty(&form, "name");
    assert!(!name_dirty.get());
    
    // Change a field
    let _ = form.set_field_value("name", FieldValue::String("New Name".to_string()));
    
    // Now should be dirty
    let name_dirty = use_field_dirty(&form, "name");
    assert!(name_dirty.get());
}

#[wasm_bindgen_test]
fn test_use_field_touched_hook() {
    let form = use_form::<HooksTestForm>();
    
    // Initially not touched
    let name_touched = use_field_touched(&form, "name");
    assert!(!name_touched.get());
    
    // Mark field as touched (this would typically happen on blur)
    // Note: In a real implementation, this would be triggered by user interaction
    
    // For now, we'll just test the hook exists
    let name_touched = use_field_touched(&form, "name");
    assert!(!name_touched.get()); // Should still be false
}

#[wasm_bindgen_test]
fn test_form_state_hooks() {
    let form = use_form::<HooksTestForm>();
    
    // Test form state hooks
    let is_valid = form.is_valid();
    let is_dirty = form.is_dirty();
    let is_touched = form.is_touched();
    
    // Initially should be invalid, not dirty, not touched
    assert!(!is_valid.get());
    assert!(!is_dirty.get());
    assert!(!is_touched.get());
    
    // Make some changes
    let _ = form.set_field_value("name", FieldValue::String("John".to_string()));
    
    // Should now be dirty
    assert!(is_dirty.get());
    
    // Still invalid because other fields are empty
    assert!(!is_valid.get());
}
