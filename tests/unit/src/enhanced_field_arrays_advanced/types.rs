//! Shared types for enhanced field arrays advanced testing

use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, Form, NumberType};
use leptos_forms_rs::validation::{ValidationErrors, Validator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdvancedArrayForm {
    pub complex_items: Vec<String>,
}

impl Default for AdvancedArrayForm {
    fn default() -> Self {
        Self {
            complex_items: Vec::new(),
        }
    }
}

impl Form for AdvancedArrayForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![FieldMetadata {
            name: "complex_items".to_string(),
            field_type: FieldType::Array(Box::new(FieldType::Text)),
            is_required: false,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![],
        }]
    }

    fn get_form_data(&self) -> HashMap<String, FieldValue> {
        let mut data = HashMap::new();
        data.insert(
            "complex_items".to_string(),
            FieldValue::Array(
                self.complex_items
                    .iter()
                    .map(|s| FieldValue::String(s.clone()))
                    .collect(),
            ),
        );
        data
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "complex_items" => {
                if let FieldValue::Array(arr) = value {
                    self.complex_items = arr
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
