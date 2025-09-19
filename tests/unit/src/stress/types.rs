//! Shared types for stress testing
//!
//! This module provides the large form structure and related types used in stress tests.

use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, Form, FormHandle, NumberType};
use leptos_forms_rs::validation::{ValidationErrors, Validator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Large form for stress testing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LargeStressForm {
    // Basic fields
    pub name: String,
    pub email: String,
    pub age: i32,

    // Many text fields
    pub field_1: String,
    pub field_2: String,
    pub field_3: String,
    pub field_4: String,
    pub field_5: String,
    pub field_6: String,
    pub field_7: String,
    pub field_8: String,
    pub field_9: String,
    pub field_10: String,
    pub field_11: String,
    pub field_12: String,
    pub field_13: String,
    pub field_14: String,
    pub field_15: String,
    pub field_16: String,
    pub field_17: String,
    pub field_18: String,
    pub field_19: String,
    pub field_20: String,

    // Number fields
    pub number_1: f64,
    pub number_2: f64,
    pub number_3: f64,
    pub number_4: f64,
    pub number_5: f64,

    // Boolean fields
    pub bool_1: bool,
    pub bool_2: bool,
    pub bool_3: bool,
    pub bool_4: bool,
    pub bool_5: bool,

    // Array fields
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub items: Vec<String>,
}

impl Default for LargeStressForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            age: 0,
            field_1: String::new(),
            field_2: String::new(),
            field_3: String::new(),
            field_4: String::new(),
            field_5: String::new(),
            field_6: String::new(),
            field_7: String::new(),
            field_8: String::new(),
            field_9: String::new(),
            field_10: String::new(),
            field_11: String::new(),
            field_12: String::new(),
            field_13: String::new(),
            field_14: String::new(),
            field_15: String::new(),
            field_16: String::new(),
            field_17: String::new(),
            field_18: String::new(),
            field_19: String::new(),
            field_20: String::new(),
            number_1: 0.0,
            number_2: 0.0,
            number_3: 0.0,
            number_4: 0.0,
            number_5: 0.0,
            bool_1: false,
            bool_2: false,
            bool_3: false,
            bool_4: false,
            bool_5: false,
            tags: Vec::new(),
            categories: Vec::new(),
            items: Vec::new(),
        }
    }
}

impl Form for LargeStressForm {
    fn get_form_data(&self) -> HashMap<String, FieldValue> {
        let mut data = HashMap::new();
        data.insert("name".to_string(), FieldValue::String(self.name.clone()));
        data.insert("email".to_string(), FieldValue::String(self.email.clone()));
        data.insert("age".to_string(), FieldValue::Number(self.age as f64));
        
        // Add all text fields
        for i in 1..=20 {
            let field_name = format!("field_{}", i);
            let field_value = match i {
                1 => &self.field_1, 2 => &self.field_2, 3 => &self.field_3, 4 => &self.field_4, 5 => &self.field_5,
                6 => &self.field_6, 7 => &self.field_7, 8 => &self.field_8, 9 => &self.field_9, 10 => &self.field_10,
                11 => &self.field_11, 12 => &self.field_12, 13 => &self.field_13, 14 => &self.field_14, 15 => &self.field_15,
                16 => &self.field_16, 17 => &self.field_17, 18 => &self.field_18, 19 => &self.field_19, 20 => &self.field_20,
                _ => continue,
            };
            data.insert(field_name, FieldValue::String(field_value.clone()));
        }
        
        // Add number fields
        data.insert("number_1".to_string(), FieldValue::Number(self.number_1));
        data.insert("number_2".to_string(), FieldValue::Number(self.number_2));
        data.insert("number_3".to_string(), FieldValue::Number(self.number_3));
        data.insert("number_4".to_string(), FieldValue::Number(self.number_4));
        data.insert("number_5".to_string(), FieldValue::Number(self.number_5));
        
        // Add boolean fields
        data.insert("bool_1".to_string(), FieldValue::Boolean(self.bool_1));
        data.insert("bool_2".to_string(), FieldValue::Boolean(self.bool_2));
        data.insert("bool_3".to_string(), FieldValue::Boolean(self.bool_3));
        data.insert("bool_4".to_string(), FieldValue::Boolean(self.bool_4));
        data.insert("bool_5".to_string(), FieldValue::Boolean(self.bool_5));
        
        // Add array fields
        data.insert("tags".to_string(), FieldValue::Array(self.tags.iter().map(|s| FieldValue::String(s.clone())).collect()));
        data.insert("categories".to_string(), FieldValue::Array(self.categories.iter().map(|s| FieldValue::String(s.clone())).collect()));
        data.insert("items".to_string(), FieldValue::Array(self.items.iter().map(|s| FieldValue::String(s.clone())).collect()));
        
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
            _ => {
                // Handle dynamic field names
                if field_name.starts_with("field_") {
                    if let FieldValue::String(s) = value {
                        match field_name {
                            "field_1" => self.field_1 = s,
                            "field_2" => self.field_2 = s,
                            "field_3" => self.field_3 = s,
                            "field_4" => self.field_4 = s,
                            "field_5" => self.field_5 = s,
                            "field_6" => self.field_6 = s,
                            "field_7" => self.field_7 = s,
                            "field_8" => self.field_8 = s,
                            "field_9" => self.field_9 = s,
                            "field_10" => self.field_10 = s,
                            "field_11" => self.field_11 = s,
                            "field_12" => self.field_12 = s,
                            "field_13" => self.field_13 = s,
                            "field_14" => self.field_14 = s,
                            "field_15" => self.field_15 = s,
                            "field_16" => self.field_16 = s,
                            "field_17" => self.field_17 = s,
                            "field_18" => self.field_18 = s,
                            "field_19" => self.field_19 = s,
                            "field_20" => self.field_20 = s,
                            _ => {}
                        }
                    }
                } else if field_name.starts_with("number_") {
                    if let FieldValue::Number(n) = value {
                        match field_name {
                            "number_1" => self.number_1 = n,
                            "number_2" => self.number_2 = n,
                            "number_3" => self.number_3 = n,
                            "number_4" => self.number_4 = n,
                            "number_5" => self.number_5 = n,
                            _ => {}
                        }
                    }
                } else if field_name.starts_with("bool_") {
                    if let FieldValue::Boolean(b) = value {
                        match field_name {
                            "bool_1" => self.bool_1 = b,
                            "bool_2" => self.bool_2 = b,
                            "bool_3" => self.bool_3 = b,
                            "bool_4" => self.bool_4 = b,
                            "bool_5" => self.bool_5 = b,
                            _ => {}
                        }
                    }
                } else if field_name == "tags" {
                    if let FieldValue::Array(arr) = value {
                        self.tags = arr.into_iter()
                            .filter_map(|v| if let FieldValue::String(s) = v { Some(s) } else { None })
                            .collect();
                    }
                } else if field_name == "categories" {
                    if let FieldValue::Array(arr) = value {
                        self.categories = arr.into_iter()
                            .filter_map(|v| if let FieldValue::String(s) = v { Some(s) } else { None })
                            .collect();
                    }
                } else if field_name == "items" {
                    if let FieldValue::Array(arr) = value {
                        self.items = arr.into_iter()
                            .filter_map(|v| if let FieldValue::String(s) = v { Some(s) } else { None })
                            .collect();
                    }
                }
            }
        }
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.name.trim().is_empty() {
            errors.add_field_error("name", "Name is required".to_string());
        }
        if self.email.trim().is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        } else if !self.email.contains('@') {
            errors.add_field_error("email", "Invalid email format".to_string());
        }
        if self.age < 0 {
            errors.add_field_error("age", "Age must be positive".to_string());
        }

        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn field_metadata() -> Vec<FieldMetadata> {
        let mut metadata = vec![
            FieldMetadata {
                name: "name".to_string(),
                field_type: FieldType::Text,
                is_required: true,
                default_value: Some(FieldValue::String(String::new())),
                dependencies: vec![],
                attributes: HashMap::new(),
                validators: vec![Validator::Required],
            },
            FieldMetadata {
                name: "email".to_string(),
                field_type: FieldType::Email,
                is_required: true,
                default_value: Some(FieldValue::String(String::new())),
                dependencies: vec![],
                attributes: HashMap::new(),
                validators: vec![Validator::Required, Validator::Email],
            },
            FieldMetadata {
                name: "age".to_string(),
                field_type: FieldType::Number(NumberType::new()),
                is_required: false,
                default_value: Some(FieldValue::Number(0.0)),
                dependencies: vec![],
                attributes: HashMap::new(),
                validators: vec![Validator::Min(0.0)],
            },
        ];

        // Add metadata for all text fields
        for i in 1..=20 {
            metadata.push(FieldMetadata {
                name: format!("field_{}", i),
                field_type: FieldType::Text,
                is_required: false,
                default_value: Some(FieldValue::String(String::new())),
                dependencies: vec![],
                attributes: HashMap::new(),
                validators: vec![],
            });
        }

        // Add metadata for number fields
        for i in 1..=5 {
            metadata.push(FieldMetadata {
                name: format!("number_{}", i),
                field_type: FieldType::Number(NumberType::new()),
                is_required: false,
                default_value: Some(FieldValue::Number(0.0)),
                dependencies: vec![],
                attributes: HashMap::new(),
                validators: vec![],
            });
        }

        // Add metadata for boolean fields
        for i in 1..=5 {
            metadata.push(FieldMetadata {
                name: format!("bool_{}", i),
                field_type: FieldType::Boolean,
                is_required: false,
                default_value: Some(FieldValue::Boolean(false)),
                dependencies: vec![],
                attributes: HashMap::new(),
                validators: vec![],
            });
        }

        // Add metadata for array fields
        metadata.push(FieldMetadata {
            name: "tags".to_string(),
            field_type: FieldType::Array(Box::new(FieldType::Text)),
            is_required: false,
            default_value: Some(FieldValue::Array(vec![])),
            dependencies: vec![],
            attributes: HashMap::new(),
            validators: vec![],
        });

        metadata.push(FieldMetadata {
            name: "categories".to_string(),
            field_type: FieldType::Array(Box::new(FieldType::Text)),
            is_required: false,
            default_value: Some(FieldValue::Array(vec![])),
            dependencies: vec![],
            attributes: HashMap::new(),
            validators: vec![],
        });

        metadata.push(FieldMetadata {
            name: "items".to_string(),
            field_type: FieldType::Array(Box::new(FieldType::Text)),
            is_required: false,
            default_value: Some(FieldValue::Array(vec![])),
            dependencies: vec![],
            attributes: HashMap::new(),
            validators: vec![],
        });

        metadata
    }

    fn default_values() -> Self {
        Self::default()
    }
}
