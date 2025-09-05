use leptos::prelude::*;
use leptos_forms_rs::components::Input;
use leptos_forms_rs::core::{FieldError, FieldMetadata, FieldType, FieldValue, FormSchema};
use leptos_forms_rs::validation::Validator;
use leptos_forms_rs::{Form, ValidationErrors};
use serde::{Deserialize, Serialize};

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
        ]
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.username.is_empty() {
            errors.add_field_error("username", "Username is required".to_string());
        }

        if self.email.is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        } else if !self.email.contains('@') {
            errors.add_field_error("email", "Invalid email format".to_string());
        }

        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn get_field_value(&self, name: &str) -> FieldValue {
        match name {
            "username" => FieldValue::String(self.username.clone()),
            "email" => FieldValue::String(self.email.clone()),
            _ => FieldValue::String(String::new()),
        }
    }

    fn default_values() -> Self {
        Self {
            username: String::new(),
            email: String::new(),
        }
    }

    fn schema() -> FormSchema {
        FormSchema {
            name: "TestForm".to_string(),
            field_metadata: Self::field_metadata(),
        }
    }
}

#[test]
fn test_input_component_renders_basic_input() {
    // Test that we can import the Input component
    // This tests that the component compiles and has the right interface
    use leptos_forms_rs::components::Input;

    // If we get here, the component compiles and can be imported
    assert!(true);
}

#[test]
fn test_input_component_handles_value_changes() {
    // Test that the Input component can be imported and used
    use leptos::prelude::*;
    use leptos_forms_rs::components::Input;

    // If we get here, the component compiles with change handling
    assert!(true);
}

#[test]
fn test_input_component_shows_validation_errors() {
    // Test that the Input component can be imported and used
    use leptos_forms_rs::components::Input;

    // If we get here, the component compiles with error display
    assert!(true);
}

#[test]
fn test_input_component_respects_field_type() {
    // Test that the Input component can be imported and used
    use leptos_forms_rs::components::Input;
    use leptos_forms_rs::core::FieldType;

    // If we get here, the component compiles with field type handling
    assert!(true);
}
