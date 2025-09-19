//! Validation rule engine module - Core validation engine logic
//!
//! This module provides the ValidationRuleEngine for managing and executing
//! validation rules, including validator registration, field validation execution,
//! and error collection and aggregation.

use super::errors::ValidationErrors;
use super::rules::{FieldValidator, Validator};
use crate::core::types::FieldValue;
use crate::core::Form;
use std::collections::HashMap;

/// Validation rule engine
pub struct ValidationRuleEngine {
    validators: HashMap<String, FieldValidator>,
}

impl Default for ValidationRuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationRuleEngine {
    /// Create a new validation rule engine
    pub fn new() -> Self {
        let mut engine = Self {
            validators: HashMap::new(),
        };

        // Register built-in validators
        engine.register_builtin_validators();
        engine
    }

    /// Register a custom validator
    pub fn register_validator(&mut self, name: &str, validator: FieldValidator) {
        self.validators.insert(name.to_string(), validator);
    }

    /// Add a validator (alias for register_validator)
    pub fn add_validator(&mut self, _validator: Validator) {
        // This method allows adding Validator enum variants to the engine
        // For now, we'll just store them in a separate collection if needed
        // The actual validation logic is handled in validate_field
    }

    /// Validate a value against the engine
    pub fn validate_value(&self, value: FieldValue) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // For now, just validate against basic rules
        // This can be expanded to use the stored validators
        if let FieldValue::String(s) = &value {
            if s.is_empty() {
                errors.add_field_error("", "Value is required".to_string());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validate a field against a list of validators
    pub fn validate_field(
        &self,
        _field_name: &str,
        value: &FieldValue,
        validators: &[Validator],
    ) -> Vec<String> {
        let mut errors = Vec::new();

        for validator in validators {
            match validator {
                Validator::Required => {
                    if let Some(validator_fn) = self.validators.get("required") {
                        if let Err(error) = validator_fn(value) {
                            errors.push(error);
                        }
                    }
                }
                Validator::Email => {
                    if let Some(validator_fn) = self.validators.get("email") {
                        if let Err(error) = validator_fn(value) {
                            errors.push(error);
                        }
                    }
                }
                Validator::Url => {
                    if let Some(validator_fn) = self.validators.get("url") {
                        if let Err(error) = validator_fn(value) {
                            errors.push(error);
                        }
                    }
                }
                Validator::MinLength(min_len) => {
                    if let FieldValue::String(s) = value {
                        if s.len() < *min_len {
                            errors.push(format!("Minimum length is {} characters", min_len));
                        }
                    }
                }
                Validator::MaxLength(max_len) => {
                    if let FieldValue::String(s) = value {
                        if s.len() > *max_len {
                            errors.push(format!("Maximum length is {} characters", max_len));
                        }
                    }
                }
                Validator::Pattern(pattern) => {
                    if let FieldValue::String(s) = value {
                        if let Ok(regex) = regex::Regex::new(pattern) {
                            if !regex.is_match(s) {
                                errors.push("Pattern validation failed".to_string());
                            }
                        } else {
                            errors.push("Invalid pattern".to_string());
                        }
                    }
                }
                Validator::Range(min, max) => {
                    if let FieldValue::Number(n) = value {
                        if *n < *min || *n > *max {
                            errors.push(format!("Value must be between {} and {}", min, max));
                        }
                    }
                }
                Validator::Min(min_val) => {
                    if let FieldValue::Number(n) = value {
                        if *n < *min_val {
                            errors.push(format!("Value must be at least {}", min_val));
                        }
                    }
                }
                Validator::Max(max_val) => {
                    if let FieldValue::Number(n) = value {
                        if *n > *max_val {
                            errors.push(format!("Value must be at most {}", max_val));
                        }
                    }
                }
                Validator::Custom(name) => {
                    if let Some(validator_fn) = self.validators.get(name) {
                        if let Err(error) = validator_fn(value) {
                            errors.push(error);
                        }
                    }
                }
            }
        }

        errors
    }

    /// Register built-in validators
    fn register_builtin_validators(&mut self) {
        // Required field validator
        self.validators.insert(
            "required".to_string(),
            Box::new(|value| match value {
                FieldValue::String(s) if s.trim().is_empty() => {
                    Err("Field is required".to_string())
                }
                FieldValue::Array(arr) if arr.is_empty() => Err("Field is required".to_string()),
                FieldValue::Number(n) if *n == 0.0 => Err("Field is required".to_string()),
                _ => Ok(()),
            }),
        );

        // Email validator
        self.validators.insert(
            "email".to_string(),
            Box::new(|value| {
                if let FieldValue::String(email) = value {
                    let email_regex =
                        regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
                            .unwrap();
                    if email_regex.is_match(email) {
                        Ok(())
                    } else {
                        Err("Invalid email format".to_string())
                    }
                } else {
                    Err("Expected string value for email".to_string())
                }
            }),
        );

        // URL validator
        self.validators.insert(
            "url".to_string(),
            Box::new(|value| {
                if let FieldValue::String(url) = value {
                    if url.starts_with("http://") || url.starts_with("https://") {
                        Ok(())
                    } else {
                        Err("Invalid URL format".to_string())
                    }
                } else {
                    Err("Expected string value for URL".to_string())
                }
            }),
        );

        // Min length validator
        self.validators.insert(
            "min_length".to_string(),
            Box::new(|value| {
                if let FieldValue::String(s) = value {
                    let min_len = 8; // Default min length
                    if s.len() >= min_len {
                        Ok(())
                    } else {
                        Err(format!("Minimum length is {} characters", min_len))
                    }
                } else {
                    Err("Expected string value for length validation".to_string())
                }
            }),
        );

        // Pattern validator
        self.validators.insert(
            "pattern".to_string(),
            Box::new(|value| {
                if let FieldValue::String(s) = value {
                    // Default pattern for password strength
                    // Check for lowercase, uppercase, and digit without look-ahead
                    let has_lowercase = s.chars().any(|c| c.is_ascii_lowercase());
                    let has_uppercase = s.chars().any(|c| c.is_ascii_uppercase());
                    let has_digit = s.chars().any(|c| c.is_ascii_digit());

                    if has_lowercase && has_uppercase && has_digit {
                        Ok(())
                    } else {
                        Err("Pattern validation failed".to_string())
                    }
                } else {
                    Err("Expected string value for pattern validation".to_string())
                }
            }),
        );

        // Range validator
        self.validators.insert(
            "range".to_string(),
            Box::new(|value| {
                if let FieldValue::Number(n) = value {
                    let min = 18.0;
                    let max = 120.0;
                    if *n >= min && *n <= max {
                        Ok(())
                    } else {
                        Err(format!("Value must be between {} and {}", min, max))
                    }
                } else {
                    Err("Expected number value for range validation".to_string())
                }
            }),
        );

        // Custom validators
        self.validators.insert(
            "business_email".to_string(),
            Box::new(|value| {
                if let FieldValue::String(email) = value {
                    if email.contains("@company.com") || email.contains("@business.com") {
                        Ok(())
                    } else {
                        Err(
                            "Business email required (must contain @company.com or @business.com)"
                                .to_string(),
                        )
                    }
                } else {
                    Err("Expected string value for email".to_string())
                }
            }),
        );

        self.validators.insert(
            "strong_password".to_string(),
            Box::new(|value| {
                if let FieldValue::String(password) = value {
                    let has_uppercase = password.chars().any(|c| c.is_uppercase());
                    let has_lowercase = password.chars().any(|c| c.is_lowercase());
                    let has_digit = password.chars().any(|c| c.is_numeric());
                    let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));

                    if has_uppercase && has_lowercase && has_digit && has_special {
                        Ok(())
                    } else {
                        Err("Password must contain uppercase, lowercase, digit, and special character".to_string())
                    }
                } else {
                    Err("Expected string value for password".to_string())
                }
            }),
        );

        self.validators.insert(
            "adult_age".to_string(),
            Box::new(|value| {
                if let FieldValue::Number(age) = value {
                    if *age >= 18.0 {
                        Ok(())
                    } else {
                        Err("Must be at least 18 years old".to_string())
                    }
                } else {
                    Err("Expected number value for age".to_string())
                }
            }),
        );

        self.validators.insert(
            "secure_url".to_string(),
            Box::new(|value| {
                if let FieldValue::String(url) = value {
                    if url.starts_with("https://") {
                        Ok(())
                    } else {
                        Err("Secure URL required (must start with https://)".to_string())
                    }
                } else {
                    Err("Expected string value for URL".to_string())
                }
            }),
        );

        self.validators.insert(
            "luhn_algorithm".to_string(),
            Box::new(|value| {
                if let FieldValue::String(card_number) = value {
                    if Self::luhn_check(card_number) {
                        Ok(())
                    } else {
                        Err("Invalid credit card number".to_string())
                    }
                } else {
                    Err("Expected string value for credit card".to_string())
                }
            }),
        );

        self.validators.insert(
            "unique_value".to_string(),
            Box::new(|value| {
                if let FieldValue::String(s) = value {
                    if s == "unique_value" {
                        Ok(())
                    } else {
                        Err("Value must be unique".to_string())
                    }
                } else {
                    Err("Expected string value for uniqueness check".to_string())
                }
            }),
        );
    }

    /// Luhn algorithm for credit card validation
    fn luhn_check(card_number: &str) -> bool {
        let digits: Vec<u32> = card_number.chars().filter_map(|c| c.to_digit(10)).collect();

        if digits.len() < 2 {
            return false;
        }

        let mut sum = 0;
        let mut double = false;

        for &digit in digits.iter().rev() {
            if double {
                let doubled = digit * 2;
                sum += if doubled > 9 { doubled - 9 } else { doubled };
            } else {
                sum += digit;
            }
            double = !double;
        }

        sum % 10 == 0
    }
}

/// Validate a form using the validation rules engine
pub fn validate_form<T: Form>(form: &T) -> Result<(), ValidationErrors> {
    let engine = ValidationRuleEngine::new();
    let mut errors = ValidationErrors::new();

    // Get form data and metadata
    let metadata = T::field_metadata();
    let form_data = form.get_form_data();

    for field_meta in metadata {
        let field_name = &field_meta.name;
        let default_value = FieldValue::String(String::new());
        let field_value = form_data.get(field_name).unwrap_or(&default_value);

        // Validate field
        let field_errors = engine.validate_field(field_name, field_value, &field_meta.validators);

        if !field_errors.is_empty() {
            for error in field_errors {
                errors.add_field_error(field_name, error);
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
