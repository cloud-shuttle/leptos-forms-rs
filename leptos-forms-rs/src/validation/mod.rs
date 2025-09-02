use std::collections::HashMap;
use regex::Regex;
use crate::core::types::FieldValue;

/// Container for all validation errors
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ValidationErrors {
    pub field_errors: HashMap<String, String>,
    pub form_errors: Vec<String>,
}

impl ValidationErrors {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn is_empty(&self) -> bool {
        self.field_errors.is_empty() && self.form_errors.is_empty()
    }
    
    pub fn has_errors(&self) -> bool {
        !self.is_empty()
    }
    
    pub fn add_field_error(&mut self, field: String, message: String) {
        self.field_errors.insert(field, message);
    }
    
    pub fn add_form_error(&mut self, message: String) {
        self.form_errors.push(message);
    }
    
    pub fn clear_field(&mut self, field: &str) {
        self.field_errors.remove(field);
    }
    
    pub fn has_field_error(&self, field: &str) -> bool {
        self.field_errors.contains_key(field)
    }
    
    pub fn get_field_error(&self, field: &str) -> Option<&String> {
        self.field_errors.get(field)
    }
    
    pub fn merge(&mut self, other: ValidationErrors) {
        for (field, error) in other.field_errors {
            self.field_errors.insert(field, error);
        }
        self.form_errors.extend(other.form_errors);
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

/// Custom validator function type
pub type CustomValidator = Box<dyn Fn(&FieldValue) -> Result<(), String> + Send + Sync>;

/// Validator registry for custom validators
pub struct ValidatorRegistry {
    validators: HashMap<String, CustomValidator>,
}

impl ValidatorRegistry {
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
        }
    }
    
    pub fn register(&mut self, name: String, validator: CustomValidator) {
        self.validators.insert(name, validator);
    }
    
    pub fn get(&self, name: &str) -> Option<&CustomValidator> {
        self.validators.get(name)
    }
    
    pub fn validate(&self, name: &str, value: &FieldValue) -> Result<(), String> {
        if let Some(validator) = self.get(name) {
            validator(value)
        } else {
            Err(format!("Unknown validator: {}", name))
        }
    }
}

impl Default for ValidatorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Global validator registry instance
lazy_static::lazy_static! {
    pub static ref GLOBAL_REGISTRY: std::sync::Mutex<ValidatorRegistry> = 
        std::sync::Mutex::new(ValidatorRegistry::new());
}

/// Register a custom validator globally
pub fn register_validator(name: String, validator: CustomValidator) {
    if let Ok(mut registry) = GLOBAL_REGISTRY.lock() {
        registry.register(name, validator);
    }
}

/// Validate a value using a custom validator
pub fn validate_custom(name: &str, value: &FieldValue) -> Result<(), String> {
    if let Ok(registry) = GLOBAL_REGISTRY.lock() {
        registry.validate(name, value)
    } else {
        Err("Failed to access validator registry".to_string())
    }
}

/// Validate field value against a specific validator
pub fn validate_field_value(value: &FieldValue, validator: &crate::core::types::ValidatorConfig) -> Result<(), String> {
    match validator {
        crate::core::types::ValidatorConfig::Required => Validators::required(value),
        crate::core::types::ValidatorConfig::Email => Validators::email(value),
        crate::core::types::ValidatorConfig::Url => Validators::url(value),
        crate::core::types::ValidatorConfig::MinLength(min) => Validators::min_length(value, *min),
        crate::core::types::ValidatorConfig::MaxLength(max) => Validators::max_length(value, *max),
        crate::core::types::ValidatorConfig::Pattern(pattern) => Validators::pattern(value, pattern),
        crate::core::types::ValidatorConfig::Min(min) => Validators::min(value, *min),
        crate::core::types::ValidatorConfig::Max(max) => Validators::max(value, *max),
        crate::core::types::ValidatorConfig::Custom(_) => Ok(()), // Custom validators handled separately
    }
}
