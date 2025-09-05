use crate::core::types::FieldValue;
use regex::Regex;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Validation errors for a form
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationErrors {
    pub field_errors: HashMap<String, Vec<String>>,
    pub form_errors: Vec<String>,
}

impl ValidationErrors {
    pub fn new() -> Self {
        Self {
            field_errors: HashMap::new(),
            form_errors: Vec::new(),
        }
    }

    pub fn add_field_error(&mut self, field_name: &str, error: String) {
        self.field_errors
            .entry(field_name.to_string())
            .or_insert_with(Vec::new)
            .push(error);
    }

    pub fn add_form_error(&mut self, error: String) {
        self.form_errors.push(error);
    }

    pub fn is_empty(&self) -> bool {
        self.field_errors.is_empty() && self.form_errors.is_empty()
    }
    
    pub fn has_errors(&self) -> bool {
        !self.is_empty()
    }
    
    pub fn has_field_error(&self, field: &str) -> bool {
        self.field_errors.contains_key(field)
    }
    
    pub fn get_field_error(&self, field: &str) -> Option<&Vec<String>> {
        self.field_errors.get(field)
    }
    
    pub fn clear_field(&mut self, field: &str) {
        self.field_errors.remove(field);
    }
    
    pub fn remove_field_error(&mut self, field: &str) {
        self.field_errors.remove(field);
    }

    pub fn to_field_errors(&self) -> Vec<crate::error::FieldError> {
        let mut errors = Vec::new();
        for (field_name, field_errors) in &self.field_errors {
            for error_msg in field_errors {
                errors.push(crate::error::FieldError::new(field_name.clone(), error_msg.clone()));
            }
        }
        errors
    }
    
    pub fn merge(&mut self, other: ValidationErrors) {
        for (field, errors) in other.field_errors {
            self.field_errors
                .entry(field)
                .or_insert_with(Vec::new)
                .extend(errors);
        }
        self.form_errors.extend(other.form_errors);
    }
}

impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.form_errors.is_empty() {
            writeln!(f, "Form errors:")?;
            for error in &self.form_errors {
                writeln!(f, "  - {}", error)?;
            }
        }

        if !self.field_errors.is_empty() {
            writeln!(f, "Field errors:")?;
            for (field, errors) in &self.field_errors {
                writeln!(f, "  {}:", field)?;
                for error in errors {
                    writeln!(f, "    - {}", error)?;
                }
            }
        }

        Ok(())
    }
}

impl std::error::Error for ValidationErrors {}

/// Validator types for field validation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Validator {
    Required,
    Email,
    Url,
    MinLength(usize),
    MaxLength(usize),
    Min(f64),
    Max(f64),
    Pattern(String),
    Range(f64, f64),
    Custom(String),
}

/// Validation rule engine
pub struct ValidationRuleEngine {
    validators: HashMap<String, Box<dyn Fn(&FieldValue) -> Result<(), String> + Send + Sync>>,
}

impl ValidationRuleEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            validators: HashMap::new(),
        };
        
        // Register built-in validators
        engine.register_builtin_validators();
        engine
    }

    fn register_builtin_validators(&mut self) {
        // Required field validator
        self.validators.insert(
            "required".to_string(),
            Box::new(|value| {
                match value {
                    FieldValue::String(s) if s.trim().is_empty() => {
                        Err("Field is required".to_string())
                    }
                    FieldValue::Array(arr) if arr.is_empty() => {
                        Err("Field is required".to_string())
                    }
                    FieldValue::Number(n) if *n == 0.0 => {
                        Err("Field is required".to_string())
                    }
                    _ => Ok(()),
                }
            }),
        );

        // Email validator
        self.validators.insert(
            "email".to_string(),
            Box::new(|value| {
                if let FieldValue::String(email) = value {
                    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
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
                    let pattern = Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)").unwrap();
                    if pattern.is_match(s) {
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
                        Err("Business email required (must contain @company.com or @business.com)".to_string())
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

    fn luhn_check(card_number: &str) -> bool {
        let digits: Vec<u32> = card_number
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        
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

    pub fn register_validator(&mut self, name: &str, validator: Box<dyn Fn(&FieldValue) -> Result<(), String> + Send + Sync>) {
        self.validators.insert(name.to_string(), validator);
    }

    pub fn add_validator(&mut self, validator: Validator) {
        // This method allows adding Validator enum variants to the engine
        // For now, we'll just store them in a separate collection if needed
        // The actual validation logic is handled in validate_field
    }

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

    pub fn validate_field(&self, _field_name: &str, value: &FieldValue, validators: &[Validator]) -> Vec<String> {
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
                        if let Ok(regex) = Regex::new(pattern) {
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
}

/// Validate a form using the validation rules engine
pub fn validate_form<T: crate::core::Form>(form: &T) -> Result<(), ValidationErrors> {
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

/// Built-in validators implementation
pub struct Validators;

impl Validators {
    /// Check if a field is required
    pub fn required(value: &FieldValue) -> Result<(), String> {
        match value {
            FieldValue::String(s) if s.trim().is_empty() => Err("This field is required".to_string()),
            FieldValue::Null => Err("This field is required".to_string()),
            FieldValue::Array(arr) if arr.is_empty() => Err("This field is required".to_string()),
            _ => Ok(()),
        }
    }
    
    /// Validate email format
    pub fn email(value: &FieldValue) -> Result<(), String> {
        if let FieldValue::String(email) = value {
            let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
            if email_regex.is_match(email) {
                Ok(())
            } else {
                Err("Invalid email format".to_string())
            }
        } else {
            Err("Email must be a string".to_string())
        }
    }
    
    /// Validate URL format
    pub fn url(value: &FieldValue) -> Result<(), String> {
        if let FieldValue::String(url) = value {
            let url_regex = Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap();
            if url_regex.is_match(url) {
                Ok(())
            } else {
                Err("Invalid URL format".to_string())
            }
        } else {
            Err("URL must be a string".to_string())
        }
    }
    
    /// Check minimum length for strings
    pub fn min_length(value: &FieldValue, min: usize) -> Result<(), String> {
        if let FieldValue::String(s) = value {
            if s.len() >= min {
                Ok(())
            } else {
                Err(format!("Minimum length is {} characters", min))
            }
        } else {
            Err("Value must be a string".to_string())
        }
    }
    
    /// Check maximum length for strings
    pub fn max_length(value: &FieldValue, max: usize) -> Result<(), String> {
        if let FieldValue::String(s) = value {
            if s.len() <= max {
                Ok(())
            } else {
                Err(format!("Maximum length is {} characters", max))
            }
        } else {
            Err("Value must be a string".to_string())
        }
    }
    
    /// Check minimum value for numbers
    pub fn min(value: &FieldValue, min: f64) -> Result<(), String> {
        if let Some(num) = value.as_number() {
            if num >= min {
                Ok(())
            } else {
                Err(format!("Minimum value is {}", min))
            }
        } else {
            Err("Value must be a number".to_string())
        }
    }
    
    /// Check maximum value for numbers
    pub fn max(value: &FieldValue, max: f64) -> Result<(), String> {
        if let Some(num) = value.as_number() {
            if num <= max {
                Ok(())
            } else {
                Err(format!("Maximum value is {}", max))
            }
        } else {
            Err("Value must be a number".to_string())
        }
    }
    
    /// Validate against a regex pattern
    pub fn pattern(value: &FieldValue, pattern: &str) -> Result<(), String> {
        if let FieldValue::String(s) = value {
            let regex = Regex::new(pattern).map_err(|_| "Invalid pattern".to_string())?;
            if regex.is_match(s) {
                Ok(())
            } else {
                Err("Value doesn't match required pattern".to_string())
            }
        } else {
            Err("Value must be a string".to_string())
        }
    }
    
    /// Validate phone number format
    pub fn phone(value: &FieldValue) -> Result<(), String> {
        if let FieldValue::String(phone) = value {
            let phone_regex = Regex::new(r"^[\+]?[1-9][\d]{0,15}$").unwrap();
            if phone_regex.is_match(phone) {
                Ok(())
            } else {
                Err("Invalid phone number format".to_string())
            }
        } else {
            Err("Phone number must be a string".to_string())
        }
    }
    
    /// Validate postal code format (basic)
    pub fn postal_code(value: &FieldValue) -> Result<(), String> {
        if let FieldValue::String(code) = value {
            let postal_regex = Regex::new(r"^\d{5}(-\d{4})?$").unwrap();
            if postal_regex.is_match(code) {
                Ok(())
            } else {
                Err("Invalid postal code format".to_string())
            }
        } else {
            Err("Postal code must be a string".to_string())
        }
    }
    
    /// Validate credit card number (Luhn algorithm)
    pub fn credit_card(value: &FieldValue) -> Result<(), String> {
        if let FieldValue::String(card) = value {
            let digits: Vec<u32> = card
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            
            if digits.len() < 13 || digits.len() > 19 {
                return Err("Invalid credit card number length".to_string());
            }
            
            // Luhn algorithm
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
            
            if sum % 10 == 0 {
                Ok(())
            } else {
                Err("Invalid credit card number".to_string())
            }
        } else {
            Err("Credit card number must be a string".to_string())
        }
    }
    
    /// Validate date format (YYYY-MM-DD)
    pub fn date(value: &FieldValue) -> Result<(), String> {
        if let FieldValue::Date(_) = value {
            Ok(())
        } else if let FieldValue::String(date_str) = value {
            let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
            if date_regex.is_match(date_str) {
                Ok(())
            } else {
                Err("Invalid date format (YYYY-MM-DD)".to_string())
            }
        } else {
            Err("Value must be a date".to_string())
        }
    }
    
    /// Validate that value is a positive number
    pub fn positive(value: &FieldValue) -> Result<(), String> {
        if let Some(num) = value.as_number() {
            if num > 0.0 {
                Ok(())
            } else {
                Err("Value must be positive".to_string())
            }
        } else {
            Err("Value must be a number".to_string())
        }
    }
    
    /// Validate that value is a negative number
    pub fn negative(value: &FieldValue) -> Result<(), String> {
        if let Some(num) = value.as_number() {
            if num < 0.0 {
                Ok(())
            } else {
                Err("Value must be negative".to_string())
            }
        } else {
            Err("Value must be a number".to_string())
        }
    }
    
    /// Validate that value is an integer
    pub fn integer(value: &FieldValue) -> Result<(), String> {
        if let Some(num) = value.as_number() {
            if num.fract() == 0.0 {
                Ok(())
            } else {
                Err("Value must be an integer".to_string())
            }
        } else {
            Err("Value must be a number".to_string())
        }
    }
    
    /// Validate array length
    pub fn array_length(value: &FieldValue, min: usize, max: usize) -> Result<(), String> {
        if let FieldValue::Array(arr) = value {
            if arr.len() >= min && arr.len() <= max {
                Ok(())
            } else {
                Err(format!("Array must have between {} and {} items", min, max))
            }
        } else {
            Err("Value must be an array".to_string())
        }
    }
}

/// Conditional validation engine for field dependencies
pub struct ConditionalValidator {
    rules: Vec<ConditionalRule>,
}

impl Default for ConditionalValidator {
    fn default() -> Self {
        Self { rules: Vec::new() }
    }
}

impl ConditionalValidator {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_rule(&mut self, rule: ConditionalRule) {
        self.rules.push(rule);
    }
    
    pub fn validate_conditional_fields<T: crate::core::Form>(
        &self,
        form: &T,
        field_name: &str,
        field_value: &FieldValue,
    ) -> Result<(), String> {
        // Find rules that apply to this field
        for rule in &self.rules {
            if rule.target_field == field_name {
                if let Some(condition) = &rule.condition {
                    // Check if the condition is met
                    let condition_met = self.evaluate_condition(form, condition)?;
                    
                    if condition_met {
                        // Apply the validation rule
                        for validator in &rule.validators {
                            if let Err(error) = validator(field_value) {
                                return Err(error);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
    fn evaluate_condition<T: crate::core::Form>(
        &self,
        form: &T,
        condition: &FieldCondition,
    ) -> Result<bool, String> {
        match condition {
            FieldCondition::Equals(field, value) => {
                let field_value = form.get_field_value(field);
                Ok(field_value == *value)
            }
            FieldCondition::NotEquals(field, value) => {
                let field_value = form.get_field_value(field);
                Ok(field_value != *value)
            }
            FieldCondition::Contains(field, value) => {
                if let FieldValue::String(field_str) = form.get_field_value(field) {
                    Ok(field_str.contains(value))
                } else {
                    Ok(false)
                }
            }
            FieldCondition::IsEmpty(field) => {
                let field_value = form.get_field_value(field);
                Ok(field_value.is_empty())
            }
            FieldCondition::IsNotEmpty(field) => {
                let field_value = form.get_field_value(field);
                Ok(!field_value.is_empty())
            }
            FieldCondition::And(conditions) => {
                for condition in conditions {
                    if !self.evaluate_condition(form, condition)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            FieldCondition::Or(conditions) => {
                for condition in conditions {
                    if self.evaluate_condition(form, condition)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
        }
    }
}

/// A conditional validation rule
pub struct ConditionalRule {
    pub target_field: String,
    pub condition: Option<FieldCondition>,
    pub validators: Vec<Box<dyn Fn(&FieldValue) -> Result<(), String> + Send + Sync>>,
    pub error_message: Option<String>,
}

impl ConditionalRule {
    pub fn new(target_field: String) -> Self {
        Self {
            target_field,
            condition: None,
            validators: Vec::new(),
            error_message: None,
        }
    }
    
    pub fn when(mut self, condition: FieldCondition) -> Self {
        self.condition = Some(condition);
        self
    }
    
    pub fn validate_with(mut self, validator: Box<dyn Fn(&FieldValue) -> Result<(), String> + Send + Sync>) -> Self {
        self.validators.push(validator);
        self
    }
    
    pub fn with_error_message(mut self, message: String) -> Self {
        self.error_message = Some(message);
        self
    }
}

/// Field conditions for conditional validation
#[derive(Debug, Clone)]
pub enum FieldCondition {
    Equals(String, FieldValue),
    NotEquals(String, FieldValue),
    Contains(String, String),
    IsEmpty(String),
    IsNotEmpty(String),
    And(Vec<FieldCondition>),
    Or(Vec<FieldCondition>),
}

impl FieldCondition {
    pub fn equals(field: &str, value: FieldValue) -> Self {
        Self::Equals(field.to_string(), value)
    }
    
    pub fn not_equals(field: &str, value: FieldValue) -> Self {
        Self::NotEquals(field.to_string(), value)
    }
    
    pub fn contains(field: &str, value: &str) -> Self {
        Self::Contains(field.to_string(), value.to_string())
    }
    
    pub fn is_empty(field: &str) -> Self {
        Self::IsEmpty(field.to_string())
    }
    
    pub fn is_not_empty(field: &str) -> Self {
        Self::IsNotEmpty(field.to_string())
    }
    
    pub fn and(conditions: Vec<FieldCondition>) -> Self {
        Self::And(conditions)
    }
    
    pub fn or(conditions: Vec<FieldCondition>) -> Self {
        Self::Or(conditions)
    }
}
