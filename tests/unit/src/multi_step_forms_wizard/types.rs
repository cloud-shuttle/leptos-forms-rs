//! Shared types for multi-step forms wizard testing

use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, Form, NumberType};
use leptos_forms_rs::validation::{ValidationErrors, Validator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WizardForm {
    pub name: String,
    pub email: String,
    pub age: i32,
    pub preferences: String,
}

impl Default for WizardForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            age: 0,
            preferences: String::new(),
        }
    }
}

impl Form for WizardForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "name".to_string(),
                field_type: FieldType::Text,
                is_required: true,
                default_value: None,
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![Validator::Required],
            },
            FieldMetadata {
                name: "email".to_string(),
                field_type: FieldType::Email,
                is_required: true,
                default_value: None,
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![Validator::Required, Validator::Email],
            },
            FieldMetadata {
                name: "age".to_string(),
                field_type: FieldType::Number(NumberType::new()),
                is_required: true,
                default_value: None,
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![Validator::Required, Validator::Min(18.0)],
            },
            FieldMetadata {
                name: "preferences".to_string(),
                field_type: FieldType::Text,
                is_required: false,
                default_value: None,
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![],
            },
        ]
    }

    fn get_form_data(&self) -> HashMap<String, FieldValue> {
        let mut data = HashMap::new();
        data.insert("name".to_string(), FieldValue::String(self.name.clone()));
        data.insert("email".to_string(), FieldValue::String(self.email.clone()));
        data.insert("age".to_string(), FieldValue::Number(self.age as f64));
        data.insert("preferences".to_string(), FieldValue::String(self.preferences.clone()));
        data
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "name" => {
                if let FieldValue::String(s) = value {
                    self.name = s;
                }
            }
            "email" => {
                if let FieldValue::String(s) = value {
                    self.email = s;
                }
            }
            "age" => {
                if let FieldValue::Number(n) = value {
                    self.age = n as i32;
                }
            }
            "preferences" => {
                if let FieldValue::String(s) = value {
                    self.preferences = s;
                }
            }
            _ => {}
        }
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.name.is_empty() {
            errors.add_field_error("name", "Name is required".to_string());
        }

        if self.email.is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        } else if !regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$")
            .unwrap()
            .is_match(&self.email)
        {
            errors.add_field_error("email", "Invalid email format".to_string());
        }

        if self.age < 18 {
            errors.add_field_error("age", "Age must be at least 18".to_string());
        }

        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn default_values() -> Self {
        Self::default()
    }
}
