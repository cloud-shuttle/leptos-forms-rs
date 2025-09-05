use leptos::prelude::Get;
use leptos_forms_rs::core::types::FieldValue;
use leptos_forms_rs::core::Form;
use leptos_forms_rs::core::FormHandle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ValidationRulesForm {
    email: String,
    password: String,
    age: i32,
    website: String,
    phone: String,
    zip_code: String,
    credit_card: String,
    custom_field: String,
}

impl Form for ValidationRulesForm {
    fn field_metadata() -> Vec<leptos_forms_rs::core::FieldMetadata> {
        vec![
            leptos_forms_rs::core::FieldMetadata {
                name: "email".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Email,
                is_required: true,
                validators: vec![
                    leptos_forms_rs::validation::Validator::Email,
                    leptos_forms_rs::validation::Validator::Custom("business_email".to_string()),
                ],
                ..Default::default()
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "password".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Password,
                is_required: true,
                validators: vec![
                    leptos_forms_rs::validation::Validator::MinLength(8),
                    leptos_forms_rs::validation::Validator::Pattern(
                        r"^[a-zA-Z0-9!@#$%^&*()_+\-=\[\]{}|;:,.<>?]{8,}$".to_string(),
                    ),
                    leptos_forms_rs::validation::Validator::Custom("strong_password".to_string()),
                ],
                ..Default::default()
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "age".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Number(
                    leptos_forms_rs::core::types::NumberType {
                        min: Some(0.0),
                        max: Some(120.0),
                        step: Some(1.0),
                    },
                ),
                is_required: true,
                validators: vec![
                    leptos_forms_rs::validation::Validator::Range(18.0, 120.0),
                    leptos_forms_rs::validation::Validator::Custom("adult_age".to_string()),
                ],
                ..Default::default()
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "website".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Text,
                is_required: false,
                validators: vec![
                    leptos_forms_rs::validation::Validator::Url,
                    leptos_forms_rs::validation::Validator::Custom("secure_url".to_string()),
                ],
                ..Default::default()
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "phone".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Text,
                is_required: false,
                validators: vec![leptos_forms_rs::validation::Validator::Pattern(
                    r"^\+?[\d\s\-\(\)]+$".to_string(),
                )],
                ..Default::default()
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "zip_code".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Text,
                is_required: false,
                validators: vec![leptos_forms_rs::validation::Validator::Pattern(
                    r"^\d{5}(-\d{4})?$".to_string(),
                )],
                ..Default::default()
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "credit_card".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Text,
                is_required: false,
                validators: vec![leptos_forms_rs::validation::Validator::Custom(
                    "luhn_algorithm".to_string(),
                )],
                ..Default::default()
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "custom_field".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Text,
                is_required: false,
                validators: vec![leptos_forms_rs::validation::Validator::Custom(
                    "unique_value".to_string(),
                )],
                ..Default::default()
            },
        ]
    }

    fn validate(&self) -> Result<(), leptos_forms_rs::validation::ValidationErrors> {
        // This will use the new validation rules engine
        leptos_forms_rs::validation::validate_form(self)
    }

    fn default_values() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            age: 0,
            website: String::new(),
            phone: String::new(),
            zip_code: String::new(),
            credit_card: String::new(),
            custom_field: String::new(),
        }
    }

    fn get_field_value(&self, field_name: &str) -> FieldValue {
        match field_name {
            "email" => FieldValue::String(self.email.clone()),
            "password" => FieldValue::String(self.password.clone()),
            "age" => FieldValue::Number(self.age as f64),
            "website" => FieldValue::String(self.website.clone()),
            "phone" => FieldValue::String(self.phone.clone()),
            "zip_code" => FieldValue::String(self.zip_code.clone()),
            "credit_card" => FieldValue::String(self.credit_card.clone()),
            "custom_field" => FieldValue::String(self.custom_field.clone()),
            _ => FieldValue::String(String::new()),
        }
    }
}

#[test]
fn test_validation_rules_engine_creation() {
    let form: FormHandle<ValidationRulesForm> =
        FormHandle::new(ValidationRulesForm::default_values());
    let schema = ValidationRulesForm::schema();
    assert!(schema.name == "ValidationRulesForm");
}

#[test]
fn test_email_validation_rules() {
    let mut form = ValidationRulesForm::default_values();

    // Set valid values for all required fields
    form.password = "ValidPass123!".to_string(); // Added special character
    form.age = 25;
    form.website = "https://example.com".to_string();
    form.phone = "123-456-7890".to_string();
    form.zip_code = "12345".to_string();
    form.credit_card = "4111111111111111".to_string();
    form.custom_field = "unique_value".to_string(); // Use the exact value expected by validator

    // Test valid email
    form.email = "user@company.com".to_string();
    let validation_result = form.validate();
    if let Err(errors) = &validation_result {
        println!("Validation errors: {:?}", errors);
    }
    assert!(validation_result.is_ok());

    // Test invalid email format
    form.email = "invalid-email".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_err());

    // Test business email validation
    form.email = "user@gmail.com".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_err()); // Should fail business email rule
}

#[test]
fn test_password_validation_rules() {
    let mut form = ValidationRulesForm::default_values();

    // Set valid values for all required fields
    form.email = "user@company.com".to_string();
    form.age = 25;
    form.website = "https://example.com".to_string();
    form.phone = "123-456-7890".to_string();
    form.zip_code = "12345".to_string();
    form.credit_card = "4111111111111111".to_string();
    form.custom_field = "unique_value".to_string();

    // Test valid password
    form.password = "StrongPass123!".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_ok());

    // Test too short password
    form.password = "Short1".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_err());

    // Test password without uppercase
    form.password = "strongpass123!".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_err());

    // Test password without numbers
    form.password = "StrongPass!".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_err());
}

#[test]
fn test_age_validation_rules() {
    let mut form = ValidationRulesForm::default_values();

    // Set valid values for all required fields
    form.email = "user@company.com".to_string();
    form.password = "StrongPass123!".to_string();
    form.website = "https://example.com".to_string();
    form.phone = "123-456-7890".to_string();
    form.zip_code = "12345".to_string();
    form.credit_card = "4111111111111111".to_string();
    form.custom_field = "unique_value".to_string();

    // Test valid age
    form.age = 25;
    let validation_result = form.validate();
    assert!(validation_result.is_ok());

    // Test underage
    form.age = 16;
    let validation_result = form.validate();
    assert!(validation_result.is_err());

    // Test too old
    form.age = 150;
    let validation_result = form.validate();
    assert!(validation_result.is_err());
}

#[test]
fn test_url_validation_rules() {
    let mut form = ValidationRulesForm::default_values();

    // Set valid values for all required fields
    form.email = "user@company.com".to_string();
    form.password = "StrongPass123!".to_string();
    form.age = 25;
    form.phone = "123-456-7890".to_string();
    form.zip_code = "12345".to_string();
    form.credit_card = "4111111111111111".to_string();
    form.custom_field = "unique_value".to_string();

    // Test valid URL
    form.website = "https://example.com".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_ok());

    // Test invalid URL
    form.website = "not-a-url".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_err());

    // Test secure URL requirement
    form.website = "http://example.com".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_err()); // Should fail secure URL rule
}

#[test]
fn test_pattern_validation_rules() {
    let mut form = ValidationRulesForm::default_values();

    // Set valid values for all required fields
    form.email = "user@company.com".to_string();
    form.password = "StrongPass123!".to_string();
    form.age = 25;
    form.website = "https://example.com".to_string();
    form.zip_code = "12345".to_string();
    form.credit_card = "4111111111111111".to_string();
    form.custom_field = "unique_value".to_string();

    // Test valid phone number
    form.phone = "+1 (555) 123-4567".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_ok());

    // Test invalid phone number
    form.phone = "invalid-phone".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_err());

    // Test valid ZIP code
    form.phone = "+1 (555) 123-4567".to_string(); // Reset phone to valid value
    form.zip_code = "12345".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_ok());

    // Test valid ZIP+4
    form.zip_code = "12345-6789".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_ok());

    // Test invalid ZIP code
    form.zip_code = "123".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_err());
}

#[test]
fn test_custom_validation_rules() {
    let mut form = ValidationRulesForm::default_values();

    // Set valid values for all required fields
    form.email = "user@company.com".to_string();
    form.password = "StrongPass123!".to_string();
    form.age = 25;
    form.website = "https://example.com".to_string();
    form.phone = "123-456-7890".to_string();
    form.zip_code = "12345".to_string();
    form.custom_field = "unique_value".to_string();

    // Test Luhn algorithm for credit card
    form.credit_card = "4532015112830366".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_ok());

    // Test invalid credit card
    form.credit_card = "4532015112830367".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_err());

    // Test unique value validation
    form.credit_card = "4532015112830366".to_string(); // Reset credit card to valid value
    form.custom_field = "unique_value".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_ok());

    // Test duplicate value (should fail unique validation)
    form.custom_field = "duplicate_value".to_string();
    let validation_result = form.validate();
    assert!(validation_result.is_err());
}

#[test]
fn test_validation_error_aggregation() {
    let mut form = ValidationRulesForm::default_values();

    // Set multiple invalid values
    form.email = "invalid-email".to_string();
    form.password = "weak".to_string();
    form.age = 15;

    let validation_result = form.validate();
    assert!(validation_result.is_err());

    if let Err(errors) = validation_result {
        assert!(errors.field_errors.len() >= 3);
        assert!(errors.field_errors.contains_key("email"));
        assert!(errors.field_errors.contains_key("password"));
        assert!(errors.field_errors.contains_key("age"));
    }
}

#[test]
fn test_validation_rules_metadata() {
    let metadata = ValidationRulesForm::field_metadata();

    let email_field = metadata.iter().find(|f| f.name == "email").unwrap();
    assert!(email_field.validators.len() >= 2);
    assert!(email_field
        .validators
        .contains(&leptos_forms_rs::validation::Validator::Email));

    let password_field = metadata.iter().find(|f| f.name == "password").unwrap();
    assert!(password_field.validators.len() >= 3);
    assert!(password_field
        .validators
        .contains(&leptos_forms_rs::validation::Validator::MinLength(8)));
}
