//! Derive macro example
//!
//! This module demonstrates the derive macro functionality.

use crate::core::traits::Form;
use crate::core::types::FieldValue;
use leptos_forms_rs_macro::Form;

// #[derive(Form, Clone, serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct UserForm {
    #[field(required = true, min_length = 3, max_length = 50)]
    name: String,

    #[field(email = true, required = true)]
    email: String,

    #[field(default = "en")]
    language: String,

    #[field(default = false)]
    newsletter: bool,
}

pub fn demonstrate_derive_macro() {
    // Test default values
    let form = UserForm::default_values();
    println!("Default form: {:?}", form);

    // Test field metadata
    let metadata = UserForm::field_metadata();
    println!("Field metadata count: {}", metadata.len());

    // Test field access
    let mut form = UserForm {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        language: "en".to_string(),
        newsletter: true,
    };

    // Test get_field_value
    let name_value = form.get_field_value("name");
    println!("Name value: {:?}", name_value);

    // Test set_field_value
    form.set_field_value("name", FieldValue::String("Jane Doe".to_string()));
    println!("Updated form: {:?}", form);

    // Test validation
    let validation_result = form.validate();
    println!("Validation result: {:?}", validation_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_macro_basic() {
        let form = UserForm::default_values();
        assert_eq!(form.name, "");
        assert_eq!(form.email, "");
        assert_eq!(form.language, "en");
        assert_eq!(form.newsletter, false);
    }

    #[test]
    fn test_derive_macro_field_metadata() {
        let metadata = UserForm::field_metadata();
        assert_eq!(metadata.len(), 4);

        let name_field = metadata.iter().find(|f| f.name == "name").unwrap();
        assert!(name_field.is_required);

        let email_field = metadata.iter().find(|f| f.name == "email").unwrap();
        assert!(email_field.is_required);
    }

    #[test]
    fn test_derive_macro_field_access() {
        let mut form = UserForm {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            language: "en".to_string(),
            newsletter: true,
        };

        assert_eq!(
            form.get_field_value("name"),
            FieldValue::String("John Doe".to_string())
        );
        assert_eq!(
            form.get_field_value("email"),
            FieldValue::String("john@example.com".to_string())
        );

        form.set_field_value("name", FieldValue::String("Jane Doe".to_string()));
        assert_eq!(form.name, "Jane Doe");
    }

    #[test]
    fn test_derive_macro_validation() {
        let valid_form = UserForm {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            language: "en".to_string(),
            newsletter: true,
        };

        assert!(valid_form.validate().is_ok());

        let invalid_form = UserForm {
            name: "Jo".to_string(),             // Too short
            email: "invalid-email".to_string(), // Invalid email
            language: "en".to_string(),
            newsletter: true,
        };

        assert!(invalid_form.validate().is_err());
    }
}
