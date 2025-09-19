//! Form types for the wizard example
//!
//! This module defines the data structures used in the form wizard example.

use crate::core::traits::Form;
use crate::core::types::{FieldType, FieldValue};
use crate::FieldMetadata;
use crate::validation::ValidationErrors;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main wizard form structure
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WizardForm {
    pub personal_info: PersonalInfo,
    pub preferences: Preferences,
    pub confirmation: Confirmation,
}

/// Personal information step data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersonalInfo {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

/// Preferences step data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    pub theme: String,
    pub notifications: bool,
    pub newsletter: bool,
}

/// Confirmation step data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Confirmation {
    pub terms_accepted: bool,
    pub privacy_accepted: bool,
}

impl Default for WizardForm {
    fn default() -> Self {
        Self {
            personal_info: PersonalInfo {
                first_name: "".to_string(),
                last_name: "".to_string(),
                email: "".to_string(),
            },
            preferences: Preferences {
                theme: "light".to_string(),
                notifications: true,
                newsletter: false,
            },
            confirmation: Confirmation {
                terms_accepted: false,
                privacy_accepted: false,
            },
        }
    }
}

impl Form for WizardForm {
    fn get_form_data(&self) -> HashMap<String, FieldValue> {
        let mut data = HashMap::new();
        data.insert("first_name".to_string(), FieldValue::String(self.personal_info.first_name.clone()));
        data.insert("last_name".to_string(), FieldValue::String(self.personal_info.last_name.clone()));
        data.insert("email".to_string(), FieldValue::String(self.personal_info.email.clone()));
        data.insert("theme".to_string(), FieldValue::String(self.preferences.theme.clone()));
        data.insert("notifications".to_string(), FieldValue::Boolean(self.preferences.notifications));
        data.insert("newsletter".to_string(), FieldValue::Boolean(self.preferences.newsletter));
        data.insert("terms_accepted".to_string(), FieldValue::Boolean(self.confirmation.terms_accepted));
        data.insert("privacy_accepted".to_string(), FieldValue::Boolean(self.confirmation.privacy_accepted));
        data
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "first_name" => {
                if let FieldValue::String(s) = value {
                    self.personal_info.first_name = s;
                }
            }
            "last_name" => {
                if let FieldValue::String(s) = value {
                    self.personal_info.last_name = s;
                }
            }
            "email" => {
                if let FieldValue::String(s) = value {
                    self.personal_info.email = s;
                }
            }
            "theme" => {
                if let FieldValue::String(s) = value {
                    self.preferences.theme = s;
                }
            }
            "notifications" => {
                if let FieldValue::Boolean(b) = value {
                    self.preferences.notifications = b;
                }
            }
            "newsletter" => {
                if let FieldValue::Boolean(b) = value {
                    self.preferences.newsletter = b;
                }
            }
            "terms_accepted" => {
                if let FieldValue::Boolean(b) = value {
                    self.confirmation.terms_accepted = b;
                }
            }
            "privacy_accepted" => {
                if let FieldValue::Boolean(b) = value {
                    self.confirmation.privacy_accepted = b;
                }
            }
            _ => {}
        }
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // Validate personal info
        if self.personal_info.first_name.trim().is_empty() {
            errors.add_field_error("first_name", "First name is required".to_string());
        }
        if self.personal_info.last_name.trim().is_empty() {
            errors.add_field_error("last_name", "Last name is required".to_string());
        }
        if self.personal_info.email.trim().is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        } else if !self.personal_info.email.contains('@') {
            errors.add_field_error("email", "Invalid email format".to_string());
        }

        // Validate confirmation
        if !self.confirmation.terms_accepted {
            errors.add_field_error("terms_accepted", "You must accept the terms and conditions".to_string());
        }
        if !self.confirmation.privacy_accepted {
            errors.add_field_error("privacy_accepted", "You must accept the privacy policy".to_string());
        }

        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "first_name".to_string(),
                field_type: FieldType::Text,
                is_required: true,
                default_value: Some(FieldValue::String("".to_string())),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
                validators: vec![],
            },
            FieldMetadata {
                name: "last_name".to_string(),
                field_type: FieldType::Text,
                is_required: true,
                default_value: Some(FieldValue::String("".to_string())),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
                validators: vec![],
            },
            FieldMetadata {
                name: "email".to_string(),
                field_type: FieldType::Email,
                is_required: true,
                default_value: Some(FieldValue::String("".to_string())),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
                validators: vec![],
            },
            FieldMetadata {
                name: "theme".to_string(),
                field_type: FieldType::Text,
                is_required: false,
                default_value: Some(FieldValue::String("light".to_string())),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
                validators: vec![],
            },
            FieldMetadata {
                name: "notifications".to_string(),
                field_type: FieldType::Boolean,
                is_required: false,
                default_value: Some(FieldValue::Boolean(true)),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
                validators: vec![],
            },
            FieldMetadata {
                name: "newsletter".to_string(),
                field_type: FieldType::Boolean,
                is_required: false,
                default_value: Some(FieldValue::Boolean(false)),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
                validators: vec![],
            },
            FieldMetadata {
                name: "terms_accepted".to_string(),
                field_type: FieldType::Boolean,
                is_required: true,
                default_value: Some(FieldValue::Boolean(false)),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
                validators: vec![],
            },
            FieldMetadata {
                name: "privacy_accepted".to_string(),
                field_type: FieldType::Boolean,
                is_required: true,
                default_value: Some(FieldValue::Boolean(false)),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
                validators: vec![],
            },
        ]
    }

    fn default_values() -> Self {
        Self::default()
    }
}
