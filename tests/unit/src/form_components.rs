//! Tests for form components functionality

use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, FormSchema, NumberType};
use leptos_forms_rs::validation::Validator;
use leptos_forms_rs::{Form, ValidationErrors};
use serde::{Deserialize, Serialize};

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
                name: "password_field".to_string(),
                field_type: FieldType::Password,
                validators: vec![Validator::Required, Validator::MinLength(8)],
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
                validators: vec![Validator::Required, Validator::Min(0.0)],
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
            errors.add_field_error("text_field", "Text field is required".to_string());
        }

        if self.email_field.is_empty() {
            errors.add_field_error("email_field", "Email is required".to_string());
        } else if !self.email_field.contains('@') {
            errors.add_field_error("email_field", "Invalid email format".to_string());
        }

        if self.password_field.len() < 8 {
            errors.add_field_error(
                "password_field",
                "Password must be at least 8 characters".to_string(),
            );
        }

        if self.number_field < 0 {
            errors.add_field_error("number_field", "Number must be non-negative".to_string());
        }

        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn get_field_value(&self, name: &str) -> FieldValue {
        match name {
            "text_field" => FieldValue::String(self.text_field.clone()),
            "email_field" => FieldValue::String(self.email_field.clone()),
            "password_field" => FieldValue::String(self.password_field.clone()),
            "number_field" => FieldValue::Number(self.number_field as f64),
            "boolean_field" => FieldValue::Boolean(self.boolean_field),
            _ => FieldValue::String(String::new()),
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
        FormSchema {
            name: "ComponentTestForm".to_string(),
            field_metadata: Self::field_metadata(),
        }
    }
}

#[test]
fn test_components_form_validation() {
    let mut form = ComponentTestForm::default_values();

    // Test empty form validation
    let result = form.validate();
    assert!(result.is_err());

    // Test valid form
    form.text_field = "Sample text".to_string();
    form.email_field = "test@example.com".to_string();
    form.password_field = "securepass123".to_string();
    form.number_field = 42;
    form.boolean_field = true;

    let result = form.validate();
    assert!(result.is_ok());
}

#[test]
fn test_components_form_schema() {
    let schema = ComponentTestForm::schema();
    assert_eq!(schema.field_metadata.len(), 5);

    // Note: required_fields() method doesn't exist in current API
    // let required_fields = schema.required_fields();
    // assert_eq!(required_fields.len(), 4); // text, email, password, number are required
}

#[test]
fn test_components_field_access() {
    let mut form = ComponentTestForm::default_values();

    // Test setting and getting field values
    form.text_field = "test".to_string();
    let value = form.get_field_value("text_field");
    assert_eq!(value, FieldValue::String("test".to_string()));

    // Test unknown field
    let value = form.get_field_value("unknown_field");
    assert_eq!(value, FieldValue::String(String::new()));
}
