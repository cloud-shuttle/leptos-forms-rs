//! Shared types for form persistence testing

use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, Form, NumberType};
use leptos_forms_rs::validation::{ValidationErrors, Validator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersistenceForm {
    pub data: String,
}

impl Default for PersistenceForm {
    fn default() -> Self {
        Self {
            data: String::new(),
        }
    }
}

impl Form for PersistenceForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![FieldMetadata {
            name: "data".to_string(),
            field_type: FieldType::Text,
            is_required: false,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![],
        }]
    }

    fn get_form_data(&self) -> HashMap<String, FieldValue> {
        let mut data = HashMap::new();
        data.insert("data".to_string(), FieldValue::String(self.data.clone()));
        data
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "data" => {
                if let FieldValue::String(s) = value {
                    self.data = s;
                }
            }
            _ => {}
        }
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(())
    }

    fn default_values() -> Self {
        Self::default()
    }
}
