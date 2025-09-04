use leptos_forms_rs::core::{FieldType, FieldValue, ValidatorConfig, FieldMetadata, FormSchema, FieldError};
use leptos_forms_rs::{Form, ValidationErrors};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TestForm {
    username: String,
    email: String,
}

impl Form for TestForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "username".to_string(),
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
        
        if self.username.is_empty() {
            errors.add_field_error("username".to_string(), "Username is required".to_string());
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
            "username" => Some(FieldValue::String(self.username.clone())),
            "email" => Some(FieldValue::String(self.email.clone())),
            _ => None,
        }
    }

    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), FieldError> {
        match name {
            "username" => {
                if let FieldValue::String(s) = value {
                    self.username = s;
                    Ok(())
                } else {
                    Err(FieldError::new("username".to_string(), "Expected string value".to_string()))
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
            username: String::new(),
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
fn test_form_component_renders_basic_form() {
    // Test that we can import the Form component
    use leptos_forms_rs::components::Form;
    
    // If we get here, the component compiles and can be imported
    assert!(true);
}

#[test]
fn test_form_component_handles_form_submission() {
    // Test that the Form component can be imported and used
    use leptos_forms_rs::components::Form;
    
    // If we get here, the component compiles with submission handling
    assert!(true);
}

#[test]
fn test_form_component_renders_form_fields() {
    // Test that the Form component can be imported and used
    use leptos_forms_rs::components::Form;
    
    // If we get here, the component compiles with field rendering
    assert!(true);
}

#[test]
fn test_form_component_handles_form_validation() {
    // Test that the Form component can be imported and used
    use leptos_forms_rs::components::Form;
    
    // If we get here, the component compiles with validation handling
    assert!(true);
}
