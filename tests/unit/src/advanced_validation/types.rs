//! Shared types for advanced validation testing
//!
//! This module provides the advanced validation form structure and related types.

use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, Form, NumberType};
use leptos_forms_rs::validation::{ValidationErrors, Validator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdvancedValidationForm {
    pub password: String,
    pub confirm_password: String,
    pub email: String,
    pub username: String,
    pub age: i32,
    pub start_date: String,
    pub end_date: String,
    pub custom_field: String,
}

impl Default for AdvancedValidationForm {
    fn default() -> Self {
        Self {
            password: String::new(),
            confirm_password: String::new(),
            email: String::new(),
            username: String::new(),
            age: 0,
            start_date: String::new(),
            end_date: String::new(),
            custom_field: String::new(),
        }
    }
}

impl Form for AdvancedValidationForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        let mut metadata = Vec::new();

        metadata.push(FieldMetadata {
            name: "password".to_string(),
            field_type: FieldType::Password,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            validators: vec![
                Validator::MinLength(8),
                Validator::Pattern(r"^[A-Za-z\d@$!%*?&]+$".to_string()),
            ],
            attributes: HashMap::new(),
        });

        metadata.push(FieldMetadata {
            name: "confirm_password".to_string(),
            field_type: FieldType::Password,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            validators: vec![
                Validator::MinLength(8),
                Validator::Custom("password_match".to_string()),
            ],
            attributes: HashMap::new(),
        });

        metadata.push(FieldMetadata {
            name: "email".to_string(),
            field_type: FieldType::Email,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            validators: vec![
                Validator::Email,
                Validator::Custom("email_domain_check".to_string()),
            ],
            attributes: HashMap::new(),
        });

        metadata.push(FieldMetadata {
            name: "username".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            validators: vec![
                Validator::MinLength(3),
                Validator::MaxLength(20),
                Validator::Pattern(r"^[a-zA-Z0-9_]+$".to_string()),
                Validator::Custom("username_availability".to_string()),
            ],
            attributes: HashMap::new(),
        });

        metadata.push(FieldMetadata {
            name: "age".to_string(),
            field_type: FieldType::Number(NumberType {
                min: Some(18.0),
                max: Some(120.0),
                step: Some(1.0),
            }),
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            validators: vec![Validator::Min(18.0), Validator::Max(120.0)],
            attributes: HashMap::new(),
        });

        metadata.push(FieldMetadata {
            name: "start_date".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            validators: vec![
                Validator::Pattern(r"^\d{4}-\d{2}-\d{2}$".to_string()),
                Validator::Custom("date_range_validation".to_string()),
            ],
            attributes: HashMap::new(),
        });

        metadata.push(FieldMetadata {
            name: "end_date".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            validators: vec![
                Validator::Pattern(r"^\d{4}-\d{2}-\d{2}$".to_string()),
                Validator::Custom("date_range_validation".to_string()),
            ],
            attributes: HashMap::new(),
        });

        metadata.push(FieldMetadata {
            name: "custom_field".to_string(),
            field_type: FieldType::Text,
            is_required: false,
            default_value: None,
            dependencies: Vec::new(),
            validators: vec![Validator::Custom("complex_business_rule".to_string())],
            attributes: HashMap::new(),
        });

        metadata
    }

    fn get_form_data(&self) -> HashMap<String, FieldValue> {
        let mut data = HashMap::new();
        data.insert(
            "password".to_string(),
            FieldValue::String(self.password.clone()),
        );
        data.insert(
            "confirm_password".to_string(),
            FieldValue::String(self.confirm_password.clone()),
        );
        data.insert("email".to_string(), FieldValue::String(self.email.clone()));
        data.insert(
            "username".to_string(),
            FieldValue::String(self.username.clone()),
        );
        data.insert("age".to_string(), FieldValue::Number(self.age as f64));
        data.insert(
            "start_date".to_string(),
            FieldValue::String(self.start_date.clone()),
        );
        data.insert(
            "end_date".to_string(),
            FieldValue::String(self.end_date.clone()),
        );
        data.insert(
            "custom_field".to_string(),
            FieldValue::String(self.custom_field.clone()),
        );
        data
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "password" => {
                if let FieldValue::String(s) = value {
                    self.password = s;
                }
            }
            "confirm_password" => {
                if let FieldValue::String(s) = value {
                    self.confirm_password = s;
                }
            }
            "email" => {
                if let FieldValue::String(s) = value {
                    self.email = s;
                }
            }
            "username" => {
                if let FieldValue::String(s) = value {
                    self.username = s;
                }
            }
            "age" => {
                if let FieldValue::Number(n) = value {
                    self.age = n as i32;
                }
            }
            "start_date" => {
                if let FieldValue::String(s) = value {
                    self.start_date = s;
                }
            }
            "end_date" => {
                if let FieldValue::String(s) = value {
                    self.end_date = s;
                }
            }
            "custom_field" => {
                if let FieldValue::String(s) = value {
                    self.custom_field = s;
                }
            }
            _ => {}
        }
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // Basic field validation
        if self.password.is_empty() {
            errors.add_field_error("password", "Password is required".to_string());
        } else if self.password.len() < 8 {
            errors.add_field_error(
                "password",
                "Password must be at least 8 characters".to_string(),
            );
        } else {
            // Check password complexity manually
            let has_lowercase = self.password.chars().any(|c| c.is_lowercase());
            let has_uppercase = self.password.chars().any(|c| c.is_uppercase());
            let has_digit = self.password.chars().any(|c| c.is_digit(10));
            let has_special = self.password.chars().any(|c| "@$!%*?&".contains(c));

            if !has_lowercase || !has_uppercase || !has_digit || !has_special {
                errors.add_field_error(
                    "password",
                    "Password must contain uppercase, lowercase, number, and special character"
                        .to_string(),
                );
            }
        }

        if self.confirm_password.is_empty() {
            errors.add_field_error(
                "confirm_password",
                "Confirm password is required".to_string(),
            );
        } else if self.confirm_password != self.password {
            errors.add_field_error("confirm_password", "Passwords do not match".to_string());
        }

        if self.email.is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        } else if !regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$")
            .unwrap()
            .is_match(&self.email)
        {
            errors.add_field_error("email", "Invalid email format".to_string());
        } else if !self.email.ends_with("@company.com") {
            errors.add_field_error("email", "Email must be from company.com domain".to_string());
        }

        if self.username.is_empty() {
            errors.add_field_error("username", "Username is required".to_string());
        } else if self.username.len() < 3 {
            errors.add_field_error(
                "username",
                "Username must be at least 3 characters".to_string(),
            );
        } else if self.username.len() > 20 {
            errors.add_field_error(
                "username",
                "Username must be at most 20 characters".to_string(),
            );
        } else if !regex::Regex::new(r"^[a-zA-Z0-9_]+$")
            .unwrap()
            .is_match(&self.username)
        {
            errors.add_field_error(
                "username",
                "Username can only contain letters, numbers, and underscores".to_string(),
            );
        } else if self.username == "admin" || self.username == "root" {
            errors.add_field_error("username", "Username is not available".to_string());
        }

        if self.age < 18 {
            errors.add_field_error("age", "Age must be at least 18".to_string());
        } else if self.age > 120 {
            errors.add_field_error("age", "Age must be at most 120".to_string());
        }

        if self.start_date.is_empty() {
            errors.add_field_error("start_date", "Start date is required".to_string());
        } else if !regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$")
            .unwrap()
            .is_match(&self.start_date)
        {
            errors.add_field_error("start_date", "Invalid date format (YYYY-MM-DD)".to_string());
        }

        if self.end_date.is_empty() {
            errors.add_field_error("end_date", "End date is required".to_string());
        } else if !regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$")
            .unwrap()
            .is_match(&self.end_date)
        {
            errors.add_field_error("end_date", "Invalid date format (YYYY-MM-DD)".to_string());
        } else if !self.start_date.is_empty() && self.end_date <= self.start_date {
            errors.add_field_error("end_date", "End date must be after start date".to_string());
        }

        if !self.custom_field.is_empty() && self.custom_field != "valid_value" {
            errors.add_field_error("custom_field", "Custom field validation failed".to_string());
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
