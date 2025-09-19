//! Shared types for multi-step forms testing
//!
//! This module provides the multi-step form structure and related types.

use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, Form, NumberType};
use leptos_forms_rs::validation::{ValidationErrors, Validator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Test form for multi-step functionality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiStepTestForm {
    // Step 1: Personal Information
    pub first_name: String,
    pub last_name: String,
    pub email: String,

    // Step 2: Contact Information
    pub phone: String,
    pub address: String,
    pub city: String,

    // Step 3: Preferences
    pub newsletter: bool,
    pub notifications: bool,
    pub theme: String,
}

impl Default for MultiStepTestForm {
    fn default() -> Self {
        Self {
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            phone: String::new(),
            address: String::new(),
            city: String::new(),
            newsletter: false,
            notifications: false,
            theme: "light".to_string(),
        }
    }
}

impl Form for MultiStepTestForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        let mut metadata = Vec::new();

        // Step 1 fields
        metadata.push(FieldMetadata {
            name: "first_name".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::MinLength(2)],
        });

        metadata.push(FieldMetadata {
            name: "last_name".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::MinLength(2)],
        });

        metadata.push(FieldMetadata {
            name: "email".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::Email],
        });

        // Step 2 fields
        metadata.push(FieldMetadata {
            name: "phone".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![
                Validator::Required,
                Validator::Pattern(r"^\d{10}$".to_string()),
            ],
        });

        metadata.push(FieldMetadata {
            name: "address".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::MinLength(10)],
        });

        metadata.push(FieldMetadata {
            name: "city".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::MinLength(2)],
        });

        // Step 3 fields
        metadata.push(FieldMetadata {
            name: "newsletter".to_string(),
            field_type: FieldType::Boolean,
            is_required: false,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![],
        });

        metadata.push(FieldMetadata {
            name: "notifications".to_string(),
            field_type: FieldType::Boolean,
            is_required: false,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![],
        });

        metadata.push(FieldMetadata {
            name: "theme".to_string(),
            field_type: FieldType::Text,
            is_required: false,
            default_value: Some(FieldValue::String("light".to_string())),
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Pattern(r"^(light|dark)$".to_string())],
        });

        metadata
    }

    fn get_form_data(&self) -> HashMap<String, FieldValue> {
        let mut data = HashMap::new();
        data.insert("first_name".to_string(), FieldValue::String(self.first_name.clone()));
        data.insert("last_name".to_string(), FieldValue::String(self.last_name.clone()));
        data.insert("email".to_string(), FieldValue::String(self.email.clone()));
        data.insert("phone".to_string(), FieldValue::String(self.phone.clone()));
        data.insert("address".to_string(), FieldValue::String(self.address.clone()));
        data.insert("city".to_string(), FieldValue::String(self.city.clone()));
        data.insert("newsletter".to_string(), FieldValue::Boolean(self.newsletter));
        data.insert("notifications".to_string(), FieldValue::Boolean(self.notifications));
        data.insert("theme".to_string(), FieldValue::String(self.theme.clone()));
        data
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "first_name" => {
                if let FieldValue::String(s) = value {
                    self.first_name = s;
                }
            }
            "last_name" => {
                if let FieldValue::String(s) = value {
                    self.last_name = s;
                }
            }
            "email" => {
                if let FieldValue::String(s) = value {
                    self.email = s;
                }
            }
            "phone" => {
                if let FieldValue::String(s) = value {
                    self.phone = s;
                }
            }
            "address" => {
                if let FieldValue::String(s) = value {
                    self.address = s;
                }
            }
            "city" => {
                if let FieldValue::String(s) = value {
                    self.city = s;
                }
            }
            "newsletter" => {
                if let FieldValue::Boolean(b) = value {
                    self.newsletter = b;
                }
            }
            "notifications" => {
                if let FieldValue::Boolean(b) = value {
                    self.notifications = b;
                }
            }
            "theme" => {
                if let FieldValue::String(s) = value {
                    self.theme = s;
                }
            }
            _ => {}
        }
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // Step 1 validation
        if self.first_name.is_empty() {
            errors.add_field_error("first_name", "First name is required".to_string());
        } else if self.first_name.len() < 2 {
            errors.add_field_error("first_name", "First name must be at least 2 characters".to_string());
        }

        if self.last_name.is_empty() {
            errors.add_field_error("last_name", "Last name is required".to_string());
        } else if self.last_name.len() < 2 {
            errors.add_field_error("last_name", "Last name must be at least 2 characters".to_string());
        }

        if self.email.is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        } else if !regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$")
            .unwrap()
            .is_match(&self.email)
        {
            errors.add_field_error("email", "Invalid email format".to_string());
        }

        // Step 2 validation
        if self.phone.is_empty() {
            errors.add_field_error("phone", "Phone is required".to_string());
        } else if !regex::Regex::new(r"^\d{10}$")
            .unwrap()
            .is_match(&self.phone)
        {
            errors.add_field_error("phone", "Phone must be 10 digits".to_string());
        }

        if self.address.is_empty() {
            errors.add_field_error("address", "Address is required".to_string());
        } else if self.address.len() < 10 {
            errors.add_field_error("address", "Address must be at least 10 characters".to_string());
        }

        if self.city.is_empty() {
            errors.add_field_error("city", "City is required".to_string());
        } else if self.city.len() < 2 {
            errors.add_field_error("city", "City must be at least 2 characters".to_string());
        }

        // Step 3 validation
        if !self.theme.is_empty() && !regex::Regex::new(r"^(light|dark)$")
            .unwrap()
            .is_match(&self.theme)
        {
            errors.add_field_error("theme", "Theme must be 'light' or 'dark'".to_string());
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
