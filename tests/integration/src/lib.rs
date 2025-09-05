//! Integration tests for Leptos Forms RS
//!
//! This module contains integration tests that test the library as a whole.

use leptos::*;
use leptos_forms_rs::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Clone, Debug, PartialEq)]
struct IntegrationTestForm {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
    age: i32,
    newsletter: bool,
}

impl Form for IntegrationTestForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "username".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required, ValidatorConfig::MinLength(3)],
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
                name: "password".to_string(),
                field_type: FieldType::Password,
                validators: vec![ValidatorConfig::Required, ValidatorConfig::MinLength(8)],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "confirm_password".to_string(),
                field_type: FieldType::Password,
                validators: vec![ValidatorConfig::Required],
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
            FieldMetadata {
                name: "newsletter".to_string(),
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

        // Username validation
        if self.username.is_empty() {
            errors.add_field_error("username", FieldError::new("Username is required"));
        } else if self.username.len() < 3 {
            errors.add_field_error(
                "username",
                FieldError::new("Username must be at least 3 characters"),
            );
        }

        // Email validation
        if self.email.is_empty() {
            errors.add_field_error("email", FieldError::new("Email is required"));
        } else if !self.email.contains('@') || !self.email.contains('.') {
            errors.add_field_error("email", FieldError::new("Invalid email format"));
        }

        // Password validation
        if self.password.len() < 8 {
            errors.add_field_error(
                "password",
                FieldError::new("Password must be at least 8 characters"),
            );
        }

        // Confirm password validation
        if self.password != self.confirm_password {
            errors.add_field_error(
                "confirm_password",
                FieldError::new("Passwords do not match"),
            );
        }

        // Age validation
        if self.age < 13 {
            errors.add_field_error("age", FieldError::new("Must be at least 13 years old"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn get_field(&self, name: &str) -> Option<FieldValue> {
        match name {
            "username" => Some(FieldValue::String(self.username.clone())),
            "email" => Some(FieldValue::String(self.email.clone())),
            "password" => Some(FieldValue::String(self.password.clone())),
            "confirm_password" => Some(FieldValue::String(self.confirm_password.clone())),
            "age" => Some(FieldValue::Number(self.age)),
            "newsletter" => Some(FieldValue::Boolean(self.newsletter)),
            _ => None,
        }
    }

    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), String> {
        match name {
            "username" => {
                if let FieldValue::String(s) = value {
                    self.username = s;
                    Ok(())
                } else {
                    Err("Expected string value for username".to_string())
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
            "password" => {
                if let FieldValue::String(s) = value {
                    self.password = s;
                    Ok(())
                } else {
                    Err("Expected string value for password".to_string())
                }
            }
            "confirm_password" => {
                if let FieldValue::String(s) = value {
                    self.confirm_password = s;
                    Ok(())
                } else {
                    Err("Expected string value for confirm_password".to_string())
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
            "newsletter" => {
                if let FieldValue::Boolean(b) = value {
                    self.newsletter = b;
                    Ok(())
                } else {
                    Err("Expected boolean value for newsletter".to_string())
                }
            }
            _ => Err(format!("Unknown field: {}", name)),
        }
    }

    fn default_values() -> Self {
        IntegrationTestForm {
            username: String::new(),
            email: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            age: 0,
            newsletter: false,
        }
    }

    fn schema() -> FormSchema {
        FormSchema::new(Self::field_metadata())
    }
}

#[wasm_bindgen_test]
fn test_complete_form_workflow() {
    // Create a form
    let form = use_form::<IntegrationTestForm>();

    // Initially should be invalid
    assert!(!form.is_valid().get());

    // Fill out the form step by step
    let _ = form.set_field_value("username", FieldValue::String("johndoe".to_string()));
    let _ = form.set_field_value("email", FieldValue::String("john@example.com".to_string()));
    let _ = form.set_field_value("password", FieldValue::String("securepass123".to_string()));
    let _ = form.set_field_value(
        "confirm_password",
        FieldValue::String("securepass123".to_string()),
    );
    let _ = form.set_field_value("age", FieldValue::Number(25));
    let _ = form.set_field_value("newsletter", FieldValue::Boolean(true));

    // Now should be valid
    assert!(form.is_valid().get());

    // Check all values are set correctly
    let values = form.get_values().get();
    assert_eq!(values.username, "johndoe");
    assert_eq!(values.email, "john@example.com");
    assert_eq!(values.password, "securepass123");
    assert_eq!(values.confirm_password, "securepass123");
    assert_eq!(values.age, 25);
    assert_eq!(values.newsletter, true);
}

#[wasm_bindgen_test]
fn test_form_validation_errors() {
    let form = use_form::<IntegrationTestForm>();

    // Set some invalid values
    let _ = form.set_field_value("username", FieldValue::String("ab".to_string())); // Too short
    let _ = form.set_field_value("email", FieldValue::String("invalid-email".to_string())); // Invalid email
    let _ = form.set_field_value("password", FieldValue::String("123".to_string())); // Too short
    let _ = form.set_field_value(
        "confirm_password",
        FieldValue::String("different".to_string()),
    ); // Mismatch
    let _ = form.set_field_value("age", FieldValue::Number(10)); // Too young

    // Should be invalid
    assert!(!form.is_valid().get());

    // Check that validation errors exist
    let validation_errors = form.validation_errors();
    assert!(validation_errors.is_some());

    if let Some(errors) = validation_errors {
        assert!(!errors.is_empty());
        assert!(errors.has_field_error("username"));
        assert!(errors.has_field_error("email"));
        assert!(errors.has_field_error("password"));
        assert!(errors.has_field_error("confirm_password"));
        assert!(errors.has_field_error("age"));
    }
}

#[wasm_bindgen_test]
fn test_form_reset_functionality() {
    let form = use_form::<IntegrationTestForm>();

    // Fill out the form
    let _ = form.set_field_value("username", FieldValue::String("testuser".to_string()));
    let _ = form.set_field_value("email", FieldValue::String("test@example.com".to_string()));
    let _ = form.set_field_value("password", FieldValue::String("password123".to_string()));
    let _ = form.set_field_value(
        "confirm_password",
        FieldValue::String("password123".to_string()),
    );
    let _ = form.set_field_value("age", FieldValue::Number(30));
    let _ = form.set_field_value("newsletter", FieldValue::Boolean(true));

    // Verify values are set
    let values = form.get_values().get();
    assert_eq!(values.username, "testuser");

    // Reset the form
    form.reset();

    // Verify form is back to default values
    let values = form.get_values().get();
    assert!(values.username.is_empty());
    assert!(values.email.is_empty());
    assert!(values.password.is_empty());
    assert!(values.confirm_password.is_empty());
    assert_eq!(values.age, 0);
    assert_eq!(values.newsletter, false);

    // Should be invalid again
    assert!(!form.is_valid().get());
}

#[wasm_bindgen_test]
fn test_form_schema_integration() {
    let form = use_form::<IntegrationTestForm>();

    // Test that the schema is properly integrated
    let schema = form.schema();
    assert_eq!(schema.fields().len(), 6);

    // Check specific fields exist
    let username_field = schema.get_field("username");
    assert!(username_field.is_some());
    assert_eq!(username_field.unwrap().name, "username");
    assert!(username_field.unwrap().is_required);

    let newsletter_field = schema.get_field("newsletter");
    assert!(newsletter_field.is_some());
    assert!(!newsletter_field.unwrap().is_required);
}

// Include module integration tests
mod module_integration_tests;
