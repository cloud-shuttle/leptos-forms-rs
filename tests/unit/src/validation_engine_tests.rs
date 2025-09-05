//! Comprehensive validation engine tests

use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, FormSchema};
use leptos_forms_rs::validation::Validator;
use leptos_forms_rs::{Form, ValidationErrors};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ValidationTestForm {
    // Basic validation fields
    required_field: String,
    email_field: String,
    url_field: String,
    phone_field: String,

    // Length validation fields
    min_length_field: String,
    max_length_field: String,
    exact_length_field: String,

    // Number validation fields
    min_value: i32,
    max_value: i32,
    range_value: f64,

    // Pattern validation fields
    regex_field: String,
    alphanumeric_field: String,

    // Custom validation fields
    custom_validation_field: String,

    // Conditional validation fields
    conditional_required: String,
    dependency_trigger: bool,
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
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "email_field".to_string(),
                field_type: FieldType::Email,
                validators: vec![Validator::Required, Validator::Email],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "url_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Pattern(r"^https?://.*".to_string())],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "phone_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Pattern(r"^\+?[\d\s\-\(\)]+$".to_string())],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "min_length_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::MinLength(5)],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "max_length_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::MaxLength(10)],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "exact_length_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Pattern(r"^.{8}$".to_string())],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "min_value".to_string(),
                field_type: FieldType::Number(leptos_forms_rs::core::NumberType {
                    min: Some(0.0),
                    max: None,
                    step: None,
                }),
                validators: vec![Validator::Min(0.0)],
                is_required: false,
                default_value: Some(FieldValue::Number(0.0)),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "max_value".to_string(),
                field_type: FieldType::Number(leptos_forms_rs::core::NumberType {
                    min: None,
                    max: Some(100.0),
                    step: None,
                }),
                validators: vec![Validator::Max(100.0)],
                is_required: false,
                default_value: Some(FieldValue::Number(50.0)),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "range_value".to_string(),
                field_type: FieldType::Number(leptos_forms_rs::core::NumberType {
                    min: Some(10.0),
                    max: Some(90.0),
                    step: Some(0.1),
                }),
                validators: vec![Validator::Min(10.0), Validator::Max(90.0)],
                is_required: false,
                default_value: Some(FieldValue::Number(50.0)),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "regex_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Pattern(r"^[A-Z]{2,4}\d{3,6}$".to_string())],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "alphanumeric_field".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Pattern(r"^[a-zA-Z0-9]+$".to_string())],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "conditional_required".to_string(),
                field_type: FieldType::Text,
                validators: vec![], // Conditional validation implemented in validate()
                is_required: false,
                default_value: None,
                dependencies: vec!["dependency_trigger".to_string()],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "dependency_trigger".to_string(),
                field_type: FieldType::Boolean,
                validators: vec![],
                is_required: false,
                default_value: Some(FieldValue::Boolean(false)),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
        ]
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // Basic required validation
        if self.required_field.is_empty() {
            errors.add_field_error("required_field", "This field is required".to_string());
        }

        // Email validation
        if !self.email_field.is_empty() {
            if !self.email_field.contains('@') || !self.email_field.contains('.') {
                errors.add_field_error(
                    "email_field",
                    "Please enter a valid email address".to_string(),
                );
            }
        } else {
            errors.add_field_error("email_field", "Email is required".to_string());
        }

        // URL validation
        if !self.url_field.is_empty() {
            if !(self.url_field.starts_with("http://") || self.url_field.starts_with("https://")) {
                errors.add_field_error(
                    "url_field",
                    "URL must start with http:// or https://".to_string(),
                );
            }
        }

        // Phone validation (basic pattern check)
        if !self.phone_field.is_empty() {
            let phone_regex = regex::Regex::new(r"^\+?[\d\s\-\(\)]+$").unwrap();
            if !phone_regex.is_match(&self.phone_field) {
                errors.add_field_error(
                    "phone_field",
                    "Please enter a valid phone number".to_string(),
                );
            }
        }

        // Length validations
        if !self.min_length_field.is_empty() && self.min_length_field.len() < 5 {
            errors.add_field_error(
                "min_length_field",
                "Must be at least 5 characters".to_string(),
            );
        }

        if !self.max_length_field.is_empty() && self.max_length_field.len() > 10 {
            errors.add_field_error(
                "max_length_field",
                "Must be at most 10 characters".to_string(),
            );
        }

        if !self.exact_length_field.is_empty() && self.exact_length_field.len() != 8 {
            errors.add_field_error(
                "exact_length_field",
                "Must be exactly 8 characters".to_string(),
            );
        }

        // Number validations
        if self.min_value < 0 {
            errors.add_field_error("min_value", "Value must be at least 0".to_string());
        }

        if self.max_value > 100 {
            errors.add_field_error("max_value", "Value must be at most 100".to_string());
        }

        if self.range_value < 10.0 || self.range_value > 90.0 {
            errors.add_field_error("range_value", "Value must be between 10 and 90".to_string());
        }

        // Pattern validations
        if !self.regex_field.is_empty() {
            let pattern_regex = regex::Regex::new(r"^[A-Z]{2,4}\d{3,6}$").unwrap();
            if !pattern_regex.is_match(&self.regex_field) {
                errors.add_field_error(
                    "regex_field",
                    "Must match format: 2-4 uppercase letters followed by 3-6 digits".to_string(),
                );
            }
        }

        if !self.alphanumeric_field.is_empty() {
            let alphanumeric_regex = regex::Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
            if !alphanumeric_regex.is_match(&self.alphanumeric_field) {
                errors.add_field_error(
                    "alphanumeric_field",
                    "Must contain only letters and numbers".to_string(),
                );
            }
        }

        // Conditional validation
        if self.dependency_trigger && self.conditional_required.is_empty() {
            errors.add_field_error(
                "conditional_required",
                "This field is required when dependency is enabled".to_string(),
            );
        }

        // Custom validation example
        if !self.custom_validation_field.is_empty() {
            if self.custom_validation_field.to_lowercase() == "forbidden" {
                errors.add_field_error(
                    "custom_validation_field",
                    "This value is not allowed".to_string(),
                );
            }
        }

        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn get_field_value(&self, name: &str) -> FieldValue {
        match name {
            "required_field" => FieldValue::String(self.required_field.clone()),
            "email_field" => FieldValue::String(self.email_field.clone()),
            "url_field" => FieldValue::String(self.url_field.clone()),
            "phone_field" => FieldValue::String(self.phone_field.clone()),
            "min_length_field" => FieldValue::String(self.min_length_field.clone()),
            "max_length_field" => FieldValue::String(self.max_length_field.clone()),
            "exact_length_field" => FieldValue::String(self.exact_length_field.clone()),
            "min_value" => FieldValue::Number(self.min_value as f64),
            "max_value" => FieldValue::Number(self.max_value as f64),
            "range_value" => FieldValue::Number(self.range_value),
            "regex_field" => FieldValue::String(self.regex_field.clone()),
            "alphanumeric_field" => FieldValue::String(self.alphanumeric_field.clone()),
            "custom_validation_field" => FieldValue::String(self.custom_validation_field.clone()),
            "conditional_required" => FieldValue::String(self.conditional_required.clone()),
            "dependency_trigger" => FieldValue::Boolean(self.dependency_trigger),
            _ => FieldValue::String(String::new()),
        }
    }

    fn default_values() -> Self {
        Self {
            required_field: String::new(),
            email_field: String::new(),
            url_field: String::new(),
            phone_field: String::new(),
            min_length_field: String::new(),
            max_length_field: String::new(),
            exact_length_field: String::new(),
            min_value: 0,
            max_value: 50,
            range_value: 50.0,
            regex_field: String::new(),
            alphanumeric_field: String::new(),
            custom_validation_field: String::new(),
            conditional_required: String::new(),
            dependency_trigger: false,
        }
    }

    fn schema() -> FormSchema {
        FormSchema {
            name: "ValidationTestForm".to_string(),
            field_metadata: Self::field_metadata(),
        }
    }
}

// Test: Basic required field validation
#[test]
fn test_required_field_validation() {
    let mut form = ValidationTestForm::default_values();

    // Empty form should fail validation
    let result = form.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.has_field_error("required_field"));
    assert!(errors.has_field_error("email_field"));

    // Fill required field
    form.required_field = "test".to_string();
    form.email_field = "test@example.com".to_string();

    let result = form.validate();
    assert!(result.is_ok());
}

// Test: Email validation scenarios
#[test]
fn test_email_validation() {
    let mut form = ValidationTestForm::default_values();
    form.required_field = "test".to_string();

    // Invalid email formats
    let invalid_emails = vec!["invalid-email", "missing-at-sign.com", ""];

    for invalid_email in invalid_emails {
        form.email_field = invalid_email.to_string();
        let result = form.validate();
        assert!(result.is_err(), "Should fail for email: {}", invalid_email);
        let errors = result.unwrap_err();
        assert!(errors.has_field_error("email_field"));
    }

    // Valid email formats
    let valid_emails = vec![
        "user@example.com",
        "test.email@domain.co.uk",
        "user+tag@example.org",
        "123@456.com",
    ];

    for valid_email in valid_emails {
        form.email_field = valid_email.to_string();
        let result = form.validate();
        assert!(result.is_ok(), "Should pass for email: {}", valid_email);
    }
}

// Test: Length validation
#[test]
fn test_length_validation() {
    let mut form = ValidationTestForm::default_values();
    form.required_field = "test".to_string();
    form.email_field = "test@example.com".to_string();

    // Min length validation
    form.min_length_field = "1234".to_string(); // Too short
    let result = form.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.has_field_error("min_length_field"));

    form.min_length_field = "12345".to_string(); // Just right
    let result = form.validate();
    assert!(result.is_ok());

    // Max length validation
    form.max_length_field = "12345678901".to_string(); // Too long
    let result = form.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.has_field_error("max_length_field"));

    form.max_length_field = "1234567890".to_string(); // Just right
    let result = form.validate();
    assert!(result.is_ok());

    // Exact length validation
    form.exact_length_field = "1234567".to_string(); // Too short
    let result = form.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.has_field_error("exact_length_field"));

    form.exact_length_field = "12345678".to_string(); // Exactly right
    let result = form.validate();
    assert!(result.is_ok());
}

// Test: Number range validation
#[test]
fn test_number_validation() {
    let mut form = ValidationTestForm::default_values();
    form.required_field = "test".to_string();
    form.email_field = "test@example.com".to_string();

    // Min value validation
    form.min_value = -1;
    let result = form.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.has_field_error("min_value"));

    // Max value validation
    form.max_value = 101;
    let result = form.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.has_field_error("max_value"));

    // Range validation
    form.range_value = 5.0; // Too low
    let result = form.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.has_field_error("range_value"));

    form.range_value = 95.0; // Too high
    let result = form.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.has_field_error("range_value"));

    // Valid ranges
    form.min_value = 5;
    form.max_value = 95;
    form.range_value = 50.0;
    let result = form.validate();
    assert!(result.is_ok());
}

// Test: Pattern validation
#[test]
fn test_pattern_validation() {
    let mut form = ValidationTestForm::default_values();
    form.required_field = "test".to_string();
    form.email_field = "test@example.com".to_string();

    // Regex pattern validation
    form.regex_field = "invalid".to_string();
    let result = form.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.has_field_error("regex_field"));

    form.regex_field = "ABC123".to_string(); // Valid format
    let result = form.validate();
    assert!(result.is_ok());

    // Alphanumeric validation
    form.alphanumeric_field = "invalid-chars!".to_string();
    let result = form.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.has_field_error("alphanumeric_field"));

    form.alphanumeric_field = "ValidChars123".to_string();
    let result = form.validate();
    assert!(result.is_ok());
}

// Test: Conditional validation
#[test]
fn test_conditional_validation() {
    let mut form = ValidationTestForm::default_values();
    form.required_field = "test".to_string();
    form.email_field = "test@example.com".to_string();

    // Dependency trigger is false, conditional field not required
    form.dependency_trigger = false;
    form.conditional_required = "".to_string();
    let result = form.validate();
    assert!(result.is_ok());

    // Dependency trigger is true, conditional field now required
    form.dependency_trigger = true;
    let result = form.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.has_field_error("conditional_required"));

    // Fill conditional field
    form.conditional_required = "now filled".to_string();
    let result = form.validate();
    assert!(result.is_ok());
}

// Test: Custom validation logic
#[test]
fn test_custom_validation() {
    let mut form = ValidationTestForm::default_values();
    form.required_field = "test".to_string();
    form.email_field = "test@example.com".to_string();

    // Test forbidden value
    form.custom_validation_field = "forbidden".to_string();
    let result = form.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.has_field_error("custom_validation_field"));

    // Test allowed value
    form.custom_validation_field = "allowed".to_string();
    let result = form.validate();
    assert!(result.is_ok());
}

// Test: Complex validation scenario
#[test]
fn test_complex_validation_scenario() {
    let mut form = ValidationTestForm::default_values();

    // Fill all fields with valid values
    form.required_field = "required value".to_string();
    form.email_field = "user@example.com".to_string();
    form.url_field = "https://example.com".to_string();
    form.phone_field = "+1-234-567-8900".to_string();
    form.min_length_field = "12345".to_string();
    form.max_length_field = "1234567890".to_string();
    form.exact_length_field = "12345678".to_string();
    form.min_value = 10;
    form.max_value = 90;
    form.range_value = 50.0;
    form.regex_field = "ABC123".to_string();
    form.alphanumeric_field = "ValidInput123".to_string();
    form.custom_validation_field = "allowed".to_string();
    form.dependency_trigger = true;
    form.conditional_required = "required because dependency is true".to_string();

    // Should pass all validations
    let result = form.validate();
    assert!(result.is_ok());

    // Now introduce one invalid value
    form.email_field = "invalid-email".to_string();
    let result = form.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.has_field_error("email_field"));
    assert!(!errors.has_field_error("required_field")); // Other fields should still be valid
}

// Test: Validation error aggregation
#[test]
fn test_validation_error_aggregation() {
    let mut form = ValidationTestForm::default_values();

    // Leave multiple fields invalid
    // required_field is empty
    // email_field is empty
    // min_length_field is too short
    // max_value is too high
    form.min_length_field = "123".to_string();
    form.max_value = 150;
    form.custom_validation_field = "forbidden".to_string();

    let result = form.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();

    // Should have multiple errors
    assert!(errors.has_field_error("required_field"));
    assert!(errors.has_field_error("email_field"));
    assert!(errors.has_field_error("min_length_field"));
    assert!(errors.has_field_error("max_value"));
    assert!(errors.has_field_error("custom_validation_field"));

    // Count total errors
    let total_errors = errors.field_errors.len();
    assert!(total_errors >= 5);
}
