//! Shared types for real-time validation testing

use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, Form, NumberType};
use leptos_forms_rs::validation::{ValidationErrors, Validator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealTimeForm {
    pub email: String,
}

impl Default for RealTimeForm {
    fn default() -> Self {
        Self {
            email: String::new(),
        }
    }
}

impl Form for RealTimeForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "email".to_string(),
                field_type: FieldType::Email,
                is_required: true,
                default_value: None,
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![Validator::Required, Validator::Email],
            },
        ]
    }

    fn get_form_data(&self) -> HashMap<String, FieldValue> {
        let mut data = HashMap::new();
        data.insert("email".to_string(), FieldValue::String(self.email.clone()));
        data
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "email" => {
                if let FieldValue::String(s) = value {
                    self.email = s;
                }
            }
            _ => {}
        }
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.email.is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        } else if !regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$")
            .unwrap()
            .is_match(&self.email)
        {
            errors.add_field_error("email", "Invalid email format".to_string());
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
