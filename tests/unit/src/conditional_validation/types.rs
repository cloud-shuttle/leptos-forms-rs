//! Shared types for conditional validation testing

use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, Form, NumberType};
use leptos_forms_rs::validation::{ValidationErrors, Validator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionalForm {
    pub trigger: bool,
    pub conditional_field: String,
}

impl Default for ConditionalForm {
    fn default() -> Self {
        Self {
            trigger: false,
            conditional_field: String::new(),
        }
    }
}

impl Form for ConditionalForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "trigger".to_string(),
                field_type: FieldType::Boolean,
                is_required: false,
                default_value: None,
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![],
            },
            FieldMetadata {
                name: "conditional_field".to_string(),
                field_type: FieldType::Text,
                is_required: false,
                default_value: None,
                dependencies: vec!["trigger".to_string()],
                attributes: HashMap::new(),
                validators: vec![Validator::Custom("conditional_required".to_string())],
            },
        ]
    }

    fn get_form_data(&self) -> HashMap<String, FieldValue> {
        let mut data = HashMap::new();
        data.insert("trigger".to_string(), FieldValue::Boolean(self.trigger));
        data.insert(
            "conditional_field".to_string(),
            FieldValue::String(self.conditional_field.clone()),
        );
        data
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "trigger" => {
                if let FieldValue::Boolean(b) = value {
                    self.trigger = b;
                }
            }
            "conditional_field" => {
                if let FieldValue::String(s) = value {
                    self.conditional_field = s;
                }
            }
            _ => {}
        }
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.trigger && self.conditional_field.is_empty() {
            errors.add_field_error(
                "conditional_field",
                "Field is required when trigger is true".to_string(),
            );
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
