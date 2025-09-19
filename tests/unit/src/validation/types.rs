//! Shared types for validation testing
//!
//! This module provides the validation test form structure and related types.

use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, NumberType};
use leptos_forms_rs::validation::Validator;
use leptos_forms_rs::{Form, ValidationErrors};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationTestForm {
    // Basic validation fields
    pub required_field: String,
    pub email_field: String,
    pub url_field: String,
    pub phone_field: String,

    // Length validation fields
    pub min_length_field: String,
    pub max_length_field: String,
    pub exact_length_field: String,

    // Number validation fields
    pub min_value: i32,
    pub max_value: i32,
    pub range_value: f64,

    // Pattern validation fields
    pub regex_field: String,
    pub alphanumeric_field: String,

    // Custom validation fields
    pub custom_validation_field: String,

    // Conditional validation fields
    pub conditional_required: String,
    pub dependency_trigger: bool,
}

impl Default for ValidationTestForm {
    fn default() -> Self {
        Self {
            required_field: String::new(),
            email_field: String::new(),
            url_field: String::new(),
            phone_field: String::new(),
            min_length_field: String::new(),
            max_length_field: String::new(),
            exact_length_field: String::new(),
            min_value: 0,
            max_value: 0,
            range_value: 0.0,
            regex_field: String::new(),
            alphanumeric_field: String::new(),
            custom_validation_field: String::new(),
            conditional_required: String::new(),
            dependency_trigger: false,
        }
    }
}

impl Form for ValidationTestForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "required_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "email_field".to_string(),
                field_type: FieldType::Email,
                validators: vec![Validator::Email],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "url_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Url],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "phone_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Pattern(r"^\d{3}-\d{3}-\d{4}$".to_string())],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "min_length_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::MinLength(5)],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "max_length_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::MaxLength(10)],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "exact_length_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::MinLength(3), Validator::MaxLength(3)],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "min_value".to_string(),
                field_type: FieldType::Number(NumberType::new()),
                validators: vec![Validator::Min(10.0)],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "max_value".to_string(),
                field_type: FieldType::Number(NumberType::new()),
                validators: vec![Validator::Max(100.0)],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "range_value".to_string(),
                field_type: FieldType::Number(NumberType::new()),
                validators: vec![Validator::Range(1.0, 10.0)],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "regex_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Pattern(r"^[A-Z][a-z]+$".to_string())],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "alphanumeric_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Pattern(r"^[a-zA-Z0-9]+$".to_string())],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "custom_validation_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Custom("custom_validator".to_string())],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "conditional_required".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Required],
                is_required: false,
                default_value: None,
                dependencies: vec!["dependency_trigger".to_string()],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "dependency_trigger".to_string(),
                field_type: FieldType::Boolean,
                validators: vec![],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
        ]
    }

    fn get_form_data(&self) -> HashMap<String, FieldValue> {
        let mut data = HashMap::new();
        data.insert(
            "required_field".to_string(),
            FieldValue::String(self.required_field.clone()),
        );
        data.insert(
            "email_field".to_string(),
            FieldValue::String(self.email_field.clone()),
        );
        data.insert(
            "url_field".to_string(),
            FieldValue::String(self.url_field.clone()),
        );
        data.insert(
            "phone_field".to_string(),
            FieldValue::String(self.phone_field.clone()),
        );
        data.insert(
            "min_length_field".to_string(),
            FieldValue::String(self.min_length_field.clone()),
        );
        data.insert(
            "max_length_field".to_string(),
            FieldValue::String(self.max_length_field.clone()),
        );
        data.insert(
            "exact_length_field".to_string(),
            FieldValue::String(self.exact_length_field.clone()),
        );
        data.insert(
            "min_value".to_string(),
            FieldValue::Number(self.min_value as f64),
        );
        data.insert(
            "max_value".to_string(),
            FieldValue::Number(self.max_value as f64),
        );
        data.insert(
            "range_value".to_string(),
            FieldValue::Number(self.range_value),
        );
        data.insert(
            "regex_field".to_string(),
            FieldValue::String(self.regex_field.clone()),
        );
        data.insert(
            "alphanumeric_field".to_string(),
            FieldValue::String(self.alphanumeric_field.clone()),
        );
        data.insert(
            "custom_validation_field".to_string(),
            FieldValue::String(self.custom_validation_field.clone()),
        );
        data.insert(
            "conditional_required".to_string(),
            FieldValue::String(self.conditional_required.clone()),
        );
        data.insert(
            "dependency_trigger".to_string(),
            FieldValue::Boolean(self.dependency_trigger),
        );
        data
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "required_field" => {
                if let FieldValue::String(s) = value {
                    self.required_field = s;
                }
            }
            "email_field" => {
                if let FieldValue::String(s) = value {
                    self.email_field = s;
                }
            }
            "url_field" => {
                if let FieldValue::String(s) = value {
                    self.url_field = s;
                }
            }
            "phone_field" => {
                if let FieldValue::String(s) = value {
                    self.phone_field = s;
                }
            }
            "min_length_field" => {
                if let FieldValue::String(s) = value {
                    self.min_length_field = s;
                }
            }
            "max_length_field" => {
                if let FieldValue::String(s) = value {
                    self.max_length_field = s;
                }
            }
            "exact_length_field" => {
                if let FieldValue::String(s) = value {
                    self.exact_length_field = s;
                }
            }
            "min_value" => {
                if let FieldValue::Number(n) = value {
                    self.min_value = n as i32;
                }
            }
            "max_value" => {
                if let FieldValue::Number(n) = value {
                    self.max_value = n as i32;
                }
            }
            "range_value" => {
                if let FieldValue::Number(n) = value {
                    self.range_value = n;
                }
            }
            "regex_field" => {
                if let FieldValue::String(s) = value {
                    self.regex_field = s;
                }
            }
            "alphanumeric_field" => {
                if let FieldValue::String(s) = value {
                    self.alphanumeric_field = s;
                }
            }
            "custom_validation_field" => {
                if let FieldValue::String(s) = value {
                    self.custom_validation_field = s;
                }
            }
            "conditional_required" => {
                if let FieldValue::String(s) = value {
                    self.conditional_required = s;
                }
            }
            "dependency_trigger" => {
                if let FieldValue::Boolean(b) = value {
                    self.dependency_trigger = b;
                }
            }
            _ => {}
        }
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // Basic required field validation
        if self.required_field.trim().is_empty() {
            errors.add_field_error("required_field", "This field is required".to_string());
        }

        // Email validation
        if !self.email_field.is_empty() && !self.email_field.contains('@') {
            errors.add_field_error("email_field", "Invalid email format".to_string());
        }

        // URL validation
        if !self.url_field.is_empty() && !self.url_field.starts_with("http") {
            errors.add_field_error("url_field", "Invalid URL format".to_string());
        }

        // Phone validation
        if !self.phone_field.is_empty()
            && !self
                .phone_field
                .matches(r"^\d{3}-\d{3}-\d{4}$")
                .any(|_| true)
        {
            errors.add_field_error("phone_field", "Invalid phone format".to_string());
        }

        // Length validations
        if !self.min_length_field.is_empty() && self.min_length_field.len() < 5 {
            errors.add_field_error(
                "min_length_field",
                "Minimum length is 5 characters".to_string(),
            );
        }

        if !self.max_length_field.is_empty() && self.max_length_field.len() > 10 {
            errors.add_field_error(
                "max_length_field",
                "Maximum length is 10 characters".to_string(),
            );
        }

        if !self.exact_length_field.is_empty() && self.exact_length_field.len() != 3 {
            errors.add_field_error(
                "exact_length_field",
                "Must be exactly 3 characters".to_string(),
            );
        }

        // Number validations
        if self.min_value < 10 {
            errors.add_field_error("min_value", "Minimum value is 10".to_string());
        }

        if self.max_value > 100 {
            errors.add_field_error("max_value", "Maximum value is 100".to_string());
        }

        if self.range_value < 1.0 || self.range_value > 10.0 {
            errors.add_field_error(
                "range_value",
                "Value must be between 1.0 and 10.0".to_string(),
            );
        }

        // Pattern validations
        if !self.regex_field.is_empty() {
            let regex = regex::Regex::new(r"^[A-Z][a-z]+$").unwrap();
            if !regex.is_match(&self.regex_field) {
                errors.add_field_error(
                    "regex_field",
                    "Must match pattern ^[A-Z][a-z]+$".to_string(),
                );
            }
        }

        if !self.alphanumeric_field.is_empty() {
            let regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
            if !regex.is_match(&self.alphanumeric_field) {
                errors.add_field_error(
                    "alphanumeric_field",
                    "Must contain only alphanumeric characters".to_string(),
                );
            }
        }

        // Custom validation
        if !self.custom_validation_field.is_empty() && self.custom_validation_field != "valid" {
            errors.add_field_error(
                "custom_validation_field",
                "Custom validation failed".to_string(),
            );
        }

        // Conditional validation
        if self.dependency_trigger && self.conditional_required.trim().is_empty() {
            errors.add_field_error(
                "conditional_required",
                "This field is required when dependency_trigger is true".to_string(),
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
