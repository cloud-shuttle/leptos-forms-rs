use leptos::prelude::*;
use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, Form, FormSchema, NumberType};
use leptos_forms_rs::hooks::use_form;
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

        // Cross-field validation for dates
        if !self.start_date.is_empty() && !self.end_date.is_empty() {
            if let (Ok(start), Ok(end)) = (
                chrono::NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d"),
                chrono::NaiveDate::parse_from_str(&self.end_date, "%Y-%m-%d"),
            ) {
                if start >= end {
                    errors.add_field_error(
                        "end_date",
                        "End date must be after start date".to_string(),
                    );
                }
            }
        }

        // Custom business rule validation
        if !self.custom_field.is_empty() {
            if self.custom_field.len() % 2 != 0 {
                errors.add_field_error(
                    "custom_field",
                    "Custom field length must be even".to_string(),
                );
            }
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

    fn schema() -> FormSchema {
        FormSchema {
            name: "AdvancedValidationForm".to_string(),
            field_metadata: Self::field_metadata(),
        }
    }

    fn get_field_value(&self, field_name: &str) -> FieldValue {
        match field_name {
            "password" => FieldValue::String(self.password.clone()),
            "confirm_password" => FieldValue::String(self.confirm_password.clone()),
            "email" => FieldValue::String(self.email.clone()),
            "username" => FieldValue::String(self.username.clone()),
            "age" => FieldValue::Number(self.age as f64),
            "start_date" => FieldValue::String(self.start_date.clone()),
            "end_date" => FieldValue::String(self.end_date.clone()),
            "custom_field" => FieldValue::String(self.custom_field.clone()),
            _ => FieldValue::String(String::new()),
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_validation_form_initialization() {
        let form = AdvancedValidationForm::default();
        assert_eq!(form.password, "");
        assert_eq!(form.confirm_password, "");
        assert_eq!(form.email, "");
        assert_eq!(form.username, "");
        assert_eq!(form.age, 0);
        assert_eq!(form.start_date, "");
        assert_eq!(form.end_date, "");
        assert_eq!(form.custom_field, "");
    }

    #[test]
    fn test_advanced_validation_form_field_metadata() {
        let metadata = AdvancedValidationForm::field_metadata();
        assert_eq!(metadata.len(), 8);

        let password_field = metadata.iter().find(|f| f.name == "password").unwrap();
        assert!(matches!(password_field.field_type, FieldType::Password));
        assert!(password_field.is_required);
        assert!(password_field.validators.contains(&Validator::MinLength(8)));
        assert!(password_field
            .validators
            .iter()
            .any(|v| matches!(v, Validator::Pattern(_))));

        let confirm_password_field = metadata
            .iter()
            .find(|f| f.name == "confirm_password")
            .unwrap();
        assert!(confirm_password_field
            .validators
            .iter()
            .any(|v| matches!(v, Validator::Custom(_))));

        let email_field = metadata.iter().find(|f| f.name == "email").unwrap();
        assert!(matches!(email_field.field_type, FieldType::Email));
        assert!(email_field.validators.contains(&Validator::Email));
        assert!(email_field
            .validators
            .iter()
            .any(|v| matches!(v, Validator::Custom(_))));

        let age_field = metadata.iter().find(|f| f.name == "age").unwrap();
        assert!(matches!(age_field.field_type, FieldType::Number(_)));
        assert!(age_field.validators.contains(&Validator::Min(18.0)));
        assert!(age_field.validators.contains(&Validator::Max(120.0)));
    }

    #[test]
    fn test_advanced_validation_form_handle_creation() {
        let (form_handle, _submit, _reset) = use_form(AdvancedValidationForm::default());
        let values = form_handle.values().get_untracked();
        assert_eq!(values.password, "");
        assert_eq!(values.confirm_password, "");
        assert_eq!(values.email, "");
        assert_eq!(values.username, "");
        assert_eq!(values.age, 0);
    }

    #[test]
    fn test_advanced_validation_form_handle_field_updates() {
        let (form_handle, _submit, _reset) = use_form(AdvancedValidationForm::default());

        form_handle.set_field_value("password", FieldValue::String("SecurePass123!".to_string()));
        form_handle.set_field_value(
            "confirm_password",
            FieldValue::String("SecurePass123!".to_string()),
        );
        form_handle.set_field_value("email", FieldValue::String("user@company.com".to_string()));
        form_handle.set_field_value("username", FieldValue::String("testuser".to_string()));
        form_handle.set_field_value("age", FieldValue::Number(25.0));

        let values = form_handle.values().get_untracked();
        assert_eq!(values.password, "SecurePass123!");
        assert_eq!(values.confirm_password, "SecurePass123!");
        assert_eq!(values.email, "user@company.com");
        assert_eq!(values.username, "testuser");
        assert_eq!(values.age, 25);
    }

    #[test]
    fn test_advanced_validation_password_validation() {
        let mut form = AdvancedValidationForm::default();

        // Test weak password
        form.password = "weak".to_string();
        let result = form.validate();
        assert!(result.is_err());

        // Test password without special character
        form.password = "StrongPass123".to_string();
        let result = form.validate();
        assert!(result.is_err());

        // Test valid password
        form.password = "StrongPass123!".to_string();
        form.confirm_password = "StrongPass123!".to_string();
        form.email = "user@company.com".to_string();
        form.username = "testuser".to_string();
        form.age = 25;
        let result = form.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_advanced_validation_password_mismatch() {
        let mut form = AdvancedValidationForm::default();
        form.password = "StrongPass123!".to_string();
        form.confirm_password = "DifferentPass123!".to_string();
        form.email = "user@company.com".to_string();
        form.username = "testuser".to_string();
        form.age = 25;

        let result = form.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_advanced_validation_email_domain_validation() {
        let mut form = AdvancedValidationForm::default();
        form.password = "StrongPass123!".to_string();
        form.confirm_password = "StrongPass123!".to_string();
        form.username = "testuser".to_string();
        form.age = 25;

        // Test invalid domain
        form.email = "user@gmail.com".to_string();
        let result = form.validate();
        assert!(result.is_err());

        // Test valid domain
        form.email = "user@company.com".to_string();
        let result = form.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_advanced_validation_username_availability() {
        let mut form = AdvancedValidationForm::default();
        form.password = "StrongPass123!".to_string();
        form.confirm_password = "StrongPass123!".to_string();
        form.email = "user@company.com".to_string();
        form.age = 25;

        // Test reserved username
        form.username = "admin".to_string();
        let result = form.validate();
        assert!(result.is_err());

        // Test valid username
        form.username = "testuser".to_string();
        let result = form.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_advanced_validation_age_validation() {
        let mut form = AdvancedValidationForm::default();
        form.password = "StrongPass123!".to_string();
        form.confirm_password = "StrongPass123!".to_string();
        form.email = "user@company.com".to_string();
        form.username = "testuser".to_string();

        // Test underage
        form.age = 17;
        let result = form.validate();
        assert!(result.is_err());

        // Test valid age
        form.age = 25;
        let result = form.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_advanced_validation_cross_field_date_validation() {
        let mut form = AdvancedValidationForm::default();
        form.password = "StrongPass123!".to_string();
        form.confirm_password = "StrongPass123!".to_string();
        form.email = "user@company.com".to_string();
        form.username = "testuser".to_string();
        form.age = 25;

        // Test invalid date range
        form.start_date = "2024-12-31".to_string();
        form.end_date = "2024-01-01".to_string();
        let result = form.validate();
        assert!(result.is_err());

        // Test valid date range
        form.start_date = "2024-01-01".to_string();
        form.end_date = "2024-12-31".to_string();
        let result = form.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_advanced_validation_custom_business_rule() {
        let mut form = AdvancedValidationForm::default();
        form.password = "StrongPass123!".to_string();
        form.confirm_password = "StrongPass123!".to_string();
        form.email = "user@company.com".to_string();
        form.username = "testuser".to_string();
        form.age = 25;

        // Test odd length custom field
        form.custom_field = "odd".to_string();
        let result = form.validate();
        assert!(result.is_err());

        // Test even length custom field
        form.custom_field = "even".to_string();
        let result = form.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_advanced_validation_complete_valid_form() {
        let mut form = AdvancedValidationForm::default();
        form.password = "StrongPass123!".to_string();
        form.confirm_password = "StrongPass123!".to_string();
        form.email = "user@company.com".to_string();
        form.username = "testuser".to_string();
        form.age = 25;
        form.start_date = "2024-01-01".to_string();
        form.end_date = "2024-12-31".to_string();
        form.custom_field = "even".to_string();

        let result = form.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_advanced_validation_form_handle_validation() {
        let (form_handle, _submit, _reset) = use_form(AdvancedValidationForm::default());

        // Set valid values
        form_handle.set_field_value("password", FieldValue::String("StrongPass123!".to_string()));
        form_handle.set_field_value(
            "confirm_password",
            FieldValue::String("StrongPass123!".to_string()),
        );
        form_handle.set_field_value("email", FieldValue::String("user@company.com".to_string()));
        form_handle.set_field_value("username", FieldValue::String("testuser".to_string()));
        form_handle.set_field_value("age", FieldValue::Number(25.0));
        form_handle.set_field_value("start_date", FieldValue::String("2024-01-01".to_string()));
        form_handle.set_field_value("end_date", FieldValue::String("2024-12-31".to_string()));
        form_handle.set_field_value("custom_field", FieldValue::String("even".to_string()));

        let result = form_handle.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_advanced_validation_form_handle_invalid_data() {
        let (form_handle, _submit, _reset) = use_form(AdvancedValidationForm::default());

        // Set invalid values
        form_handle.set_field_value("password", FieldValue::String("weak".to_string()));
        form_handle.set_field_value(
            "confirm_password",
            FieldValue::String("different".to_string()),
        );
        form_handle.set_field_value("email", FieldValue::String("invalid-email".to_string()));
        form_handle.set_field_value("username", FieldValue::String("admin".to_string()));
        form_handle.set_field_value("age", FieldValue::Number(17.0));

        let result = form_handle.validate();
        assert!(result.is_err());
    }
}
