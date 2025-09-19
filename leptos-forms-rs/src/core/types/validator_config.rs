//! Validator configuration module - Built-in validator configuration
//!
//! This module provides ValidatorConfig enum for built-in validator configuration,
//! including validation rule definitions and validator metadata.

use serde::{Deserialize, Serialize};

/// Built-in validator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidatorConfig {
    Required,
    Email,
    Url,
    MinLength(usize),
    MaxLength(usize),
    Min(f64),
    Max(f64),
    Pattern(String),
    Custom(String), // Function name for code generation
}

impl ValidatorConfig {
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

    /// Create a custom validator
    pub fn custom(name: String) -> Self {
        Self::Custom(name)
    }

    /// Get the validator name
    pub fn name(&self) -> &'static str {
        match self {
            ValidatorConfig::Required => "required",
            ValidatorConfig::Email => "email",
            ValidatorConfig::Url => "url",
            ValidatorConfig::MinLength(_) => "min_length",
            ValidatorConfig::MaxLength(_) => "max_length",
            ValidatorConfig::Min(_) => "min",
            ValidatorConfig::Max(_) => "max",
            ValidatorConfig::Pattern(_) => "pattern",
            ValidatorConfig::Custom(_) => "custom",
        }
    }

    /// Get a human-readable description of the validator
    pub fn description(&self) -> String {
        match self {
            ValidatorConfig::Required => "Field is required".to_string(),
            ValidatorConfig::Email => "Must be a valid email address".to_string(),
            ValidatorConfig::Url => "Must be a valid URL".to_string(),
            ValidatorConfig::MinLength(min) => format!("Minimum length is {} characters", min),
            ValidatorConfig::MaxLength(max) => format!("Maximum length is {} characters", max),
            ValidatorConfig::Min(min) => format!("Minimum value is {}", min),
            ValidatorConfig::Max(max) => format!("Maximum value is {}", max),
            ValidatorConfig::Pattern(pattern) => format!("Must match pattern: {}", pattern),
            ValidatorConfig::Custom(name) => format!("Custom validation: {}", name),
        }
    }

    /// Check if this is a required validator
    pub fn is_required(&self) -> bool {
        matches!(self, ValidatorConfig::Required)
    }

    /// Check if this is a length validator
    pub fn is_length_validator(&self) -> bool {
        matches!(
            self,
            ValidatorConfig::MinLength(_) | ValidatorConfig::MaxLength(_)
        )
    }

    /// Check if this is a numeric validator
    pub fn is_numeric_validator(&self) -> bool {
        matches!(self, ValidatorConfig::Min(_) | ValidatorConfig::Max(_))
    }

    /// Check if this is a pattern validator
    pub fn is_pattern_validator(&self) -> bool {
        matches!(self, ValidatorConfig::Pattern(_))
    }

    /// Check if this is a custom validator
    pub fn is_custom(&self) -> bool {
        matches!(self, ValidatorConfig::Custom(_))
    }

    /// Get the minimum length if this is a MinLength validator
    pub fn get_min_length(&self) -> Option<usize> {
        match self {
            ValidatorConfig::MinLength(min) => Some(*min),
            _ => None,
        }
    }

    /// Get the maximum length if this is a MaxLength validator
    pub fn get_max_length(&self) -> Option<usize> {
        match self {
            ValidatorConfig::MaxLength(max) => Some(*max),
            _ => None,
        }
    }

    /// Get the minimum value if this is a Min validator
    pub fn min_value(&self) -> Option<f64> {
        match self {
            ValidatorConfig::Min(min) => Some(*min),
            _ => None,
        }
    }

    /// Get the maximum value if this is a Max validator
    pub fn max_value(&self) -> Option<f64> {
        match self {
            ValidatorConfig::Max(max) => Some(*max),
            _ => None,
        }
    }

    /// Get the pattern if this is a Pattern validator
    pub fn get_pattern(&self) -> Option<&str> {
        match self {
            ValidatorConfig::Pattern(pattern) => Some(pattern),
            _ => None,
        }
    }

    /// Get the custom validator name if this is a Custom validator
    pub fn custom_name(&self) -> Option<&str> {
        match self {
            ValidatorConfig::Custom(name) => Some(name),
            _ => None,
        }
    }

    /// Check if this validator applies to string values
    pub fn applies_to_strings(&self) -> bool {
        matches!(
            self,
            ValidatorConfig::Required
                | ValidatorConfig::Email
                | ValidatorConfig::Url
                | ValidatorConfig::MinLength(_)
                | ValidatorConfig::MaxLength(_)
                | ValidatorConfig::Pattern(_)
                | ValidatorConfig::Custom(_)
        )
    }

    /// Check if this validator applies to numeric values
    pub fn applies_to_numbers(&self) -> bool {
        matches!(
            self,
            ValidatorConfig::Required
                | ValidatorConfig::Min(_)
                | ValidatorConfig::Max(_)
                | ValidatorConfig::Custom(_)
        )
    }

    /// Check if this validator applies to all value types
    pub fn applies_to_all(&self) -> bool {
        matches!(self, ValidatorConfig::Required | ValidatorConfig::Custom(_))
    }

    /// Get the validator priority (lower numbers = higher priority)
    pub fn priority(&self) -> u8 {
        match self {
            ValidatorConfig::Required => 1,
            ValidatorConfig::Email => 2,
            ValidatorConfig::Url => 2,
            ValidatorConfig::MinLength(_) => 3,
            ValidatorConfig::MaxLength(_) => 3,
            ValidatorConfig::Min(_) => 3,
            ValidatorConfig::Max(_) => 3,
            ValidatorConfig::Pattern(_) => 4,
            ValidatorConfig::Custom(_) => 5,
        }
    }
}

impl std::fmt::Display for ValidatorConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl PartialEq for ValidatorConfig {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ValidatorConfig::Required, ValidatorConfig::Required) => true,
            (ValidatorConfig::Email, ValidatorConfig::Email) => true,
            (ValidatorConfig::Url, ValidatorConfig::Url) => true,
            (ValidatorConfig::MinLength(a), ValidatorConfig::MinLength(b)) => a == b,
            (ValidatorConfig::MaxLength(a), ValidatorConfig::MaxLength(b)) => a == b,
            (ValidatorConfig::Min(a), ValidatorConfig::Min(b)) => (a - b).abs() < f64::EPSILON,
            (ValidatorConfig::Max(a), ValidatorConfig::Max(b)) => (a - b).abs() < f64::EPSILON,
            (ValidatorConfig::Pattern(a), ValidatorConfig::Pattern(b)) => a == b,
            (ValidatorConfig::Custom(a), ValidatorConfig::Custom(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for ValidatorConfig {}
