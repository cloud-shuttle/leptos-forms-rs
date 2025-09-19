//! Shared types for performance benchmarking testing

use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, Form, NumberType};
use leptos_forms_rs::validation::{ValidationErrors, Validator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenchmarkForm {
    pub field1: String,
    pub field2: String,
}

impl Default for BenchmarkForm {
    fn default() -> Self {
        Self {
            field1: String::new(),
            field2: String::new(),
        }
    }
}

impl Form for BenchmarkForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "field1".to_string(),
                field_type: FieldType::Text,
                is_required: false,
                default_value: None,
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![],
            },
            FieldMetadata {
                name: "field2".to_string(),
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
        data.insert("field1".to_string(), FieldValue::String(self.field1.clone()));
        data.insert("field2".to_string(), FieldValue::String(self.field2.clone()));
        data
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "field1" => {
                if let FieldValue::String(s) = value {
                    self.field1 = s;
                }
            }
            "field2" => {
                if let FieldValue::String(s) = value {
                    self.field2 = s;
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
