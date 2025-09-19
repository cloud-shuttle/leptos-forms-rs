//! Shared types for enhanced field arrays testing

use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, Form, NumberType};
use leptos_forms_rs::validation::{ValidationErrors, Validator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayTestForm {
    pub items: Vec<String>,
    pub tags: Vec<String>,
    pub numbers: Vec<i32>,
}

impl Default for ArrayTestForm {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            tags: Vec::new(),
            numbers: Vec::new(),
        }
    }
}

impl Form for ArrayTestForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "items".to_string(),
                field_type: FieldType::Array(Box::new(FieldType::Text)),
                is_required: false,
                default_value: None,
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![],
            },
            FieldMetadata {
                name: "tags".to_string(),
                field_type: FieldType::Array(Box::new(FieldType::Text)),
                is_required: false,
                default_value: None,
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![],
            },
            FieldMetadata {
                name: "numbers".to_string(),
                field_type: FieldType::Array(Box::new(FieldType::Number(NumberType::new()))),
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
        data.insert(
            "items".to_string(),
            FieldValue::Array(
                self.items
                    .iter()
                    .map(|s| FieldValue::String(s.clone()))
                    .collect(),
            ),
        );
        data.insert(
            "tags".to_string(),
            FieldValue::Array(
                self.tags
                    .iter()
                    .map(|s| FieldValue::String(s.clone()))
                    .collect(),
            ),
        );
        data.insert(
            "numbers".to_string(),
            FieldValue::Array(
                self.numbers
                    .iter()
                    .map(|n| FieldValue::Number(*n as f64))
                    .collect(),
            ),
        );
        data
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "items" => {
                if let FieldValue::Array(arr) = value {
                    self.items = arr
                        .into_iter()
                        .filter_map(|v| {
                            if let FieldValue::String(s) = v {
                                Some(s)
                            } else {
                                None
                            }
                        })
                        .collect();
                }
            }
            "tags" => {
                if let FieldValue::Array(arr) = value {
                    self.tags = arr
                        .into_iter()
                        .filter_map(|v| {
                            if let FieldValue::String(s) = v {
                                Some(s)
                            } else {
                                None
                            }
                        })
                        .collect();
                }
            }
            "numbers" => {
                if let FieldValue::Array(arr) = value {
                    self.numbers = arr
                        .into_iter()
                        .filter_map(|v| {
                            if let FieldValue::Number(n) = v {
                                Some(n as i32)
                            } else {
                                None
                            }
                        })
                        .collect();
                }
            }
            _ => {}
        }
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        // Basic validation - arrays are optional
        Ok(())
    }

    fn default_values() -> Self {
        Self::default()
    }
}
