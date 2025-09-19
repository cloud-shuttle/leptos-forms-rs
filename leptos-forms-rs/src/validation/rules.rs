//! Validator rules module - Validator enum and rule definitions
//!
//! This module provides the Validator enum and related types for defining
//! validation rules, along with rule validation logic and validator configuration.

use crate::core::types::FieldValue;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Type alias for field validators
pub type FieldValidator = Box<dyn Fn(&FieldValue) -> Result<(), String> + Send + Sync>;

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

impl Validator {
    /// Create a required validator
    pub fn required() -> Self {
        Self::Required
    }

    /// Create an email validator
    pub fn email() -> Self {
        Self::Email
    }

    /// Create a URL validator
    pub fn url() -> Self {
        Self::Url
    }

    /// Create a minimum length validator
    pub fn min_length(min: usize) -> Self {
        Self::MinLength(min)
    }

    /// Create a maximum length validator
    pub fn max_length(max: usize) -> Self {
        Self::MaxLength(max)
    }

    /// Create a minimum value validator
    pub fn min(min: f64) -> Self {
        Self::Min(min)
    }

    /// Create a maximum value validator
    pub fn max(max: f64) -> Self {
        Self::Max(max)
    }

    /// Create a pattern validator
    pub fn pattern(pattern: String) -> Self {
        Self::Pattern(pattern)
    }

    /// Create a range validator
    pub fn range(min: f64, max: f64) -> Self {
        Self::Range(min, max)
    }

    /// Create a custom validator
    pub fn custom(name: String) -> Self {
        Self::Custom(name)
    }

    /// Validate a value against this validator
    pub fn validate(&self, value: &FieldValue) -> Result<(), String> {
        match self {
            Validator::Required => {
                if value.is_empty() {
                    Err("This field is required".to_string())
                } else {
                    Ok(())
                }
            }
            Validator::Email => {
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
            Validator::Url => {
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
            Validator::MinLength(min_len) => {
                if let FieldValue::String(s) = value {
                    if s.len() >= *min_len {
                        Ok(())
                    } else {
                        Err(format!("Minimum length is {} characters", min_len))
                    }
                } else {
                    Err("Value must be a string".to_string())
                }
            }
            Validator::MaxLength(max_len) => {
                if let FieldValue::String(s) = value {
                    if s.len() <= *max_len {
                        Ok(())
                    } else {
                        Err(format!("Maximum length is {} characters", max_len))
                    }
                } else {
                    Err("Value must be a string".to_string())
                }
            }
            Validator::Min(min_val) => {
                if let Some(num) = value.as_number() {
                    if num >= *min_val {
                        Ok(())
                    } else {
                        Err(format!("Minimum value is {}", min_val))
                    }
                } else {
                    Err("Value must be a number".to_string())
                }
            }
            Validator::Max(max_val) => {
                if let Some(num) = value.as_number() {
                    if num <= *max_val {
                        Ok(())
                    } else {
                        Err(format!("Maximum value is {}", max_val))
                    }
                } else {
                    Err("Value must be a number".to_string())
                }
            }
            Validator::Pattern(pattern) => {
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
            Validator::Range(min, max) => {
                if let Some(num) = value.as_number() {
                    if num >= *min && num <= *max {
                        Ok(())
                    } else {
                        Err(format!("Value must be between {} and {}", min, max))
                    }
                } else {
                    Err("Value must be a number".to_string())
                }
            }
            Validator::Custom(_name) => {
                // Custom validators are handled by the validation engine
                Err("Custom validator not implemented".to_string())
            }
        }
    }

    /// Get a human-readable description of this validator
    pub fn description(&self) -> String {
        match self {
            Validator::Required => "Required field".to_string(),
            Validator::Email => "Valid email address".to_string(),
            Validator::Url => "Valid URL".to_string(),
            Validator::MinLength(min) => format!("Minimum {} characters", min),
            Validator::MaxLength(max) => format!("Maximum {} characters", max),
            Validator::Min(min) => format!("Minimum value {}", min),
            Validator::Max(max) => format!("Maximum value {}", max),
            Validator::Pattern(pattern) => format!("Must match pattern: {}", pattern),
            Validator::Range(min, max) => format!("Value between {} and {}", min, max),
            Validator::Custom(name) => format!("Custom validation: {}", name),
        }
    }

    /// Check if this validator is a custom validator
    pub fn is_custom(&self) -> bool {
        matches!(self, Validator::Custom(_))
    }

    /// Get the custom validator name if this is a custom validator
    pub fn custom_name(&self) -> Option<&str> {
        if let Validator::Custom(name) = self {
            Some(name)
        } else {
            None
        }
    }
}

impl std::fmt::Display for Validator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}
