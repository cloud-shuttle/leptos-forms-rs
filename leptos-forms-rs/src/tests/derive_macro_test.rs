//! Derive macro test
//!
//! This module contains tests for the derive macro functionality.

#[cfg(test)]
mod tests {
    use crate::core::traits::Form;
    use crate::core::types::FieldValue;
    use crate::validation::ValidationErrors;
    use leptos_forms_rs_macro::Form;

    #[derive(Form, Clone, serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct TestForm {
        #[field(required = true, min_length = 3, max_length = 50)]
        name: String,

        #[field(email = true, required = true)]
        email: String,

        #[field(default = "en")]
        language: String,

        #[field(default = false)]
        newsletter: bool,
    }

    #[test]
    fn test_derive_macro_field_metadata() {
        let metadata = TestForm::field_metadata();

        assert_eq!(metadata.len(), 4);

        // Check name field
        let name_field = metadata.iter().find(|f| f.name == "name").unwrap();
        assert!(name_field.is_required);
        assert!(name_field.validators.len() >= 2); // required + min_length + max_length

        // Check email field
        let email_field = metadata.iter().find(|f| f.name == "email").unwrap();
        assert!(email_field.is_required);
        assert!(email_field
            .validators
            .iter()
            .any(|v| matches!(v, crate::validation::Validator::Email)));

        // Check language field
        let language_field = metadata.iter().find(|f| f.name == "language").unwrap();
        assert!(!language_field.is_required);
        assert_eq!(
            language_field.default_value,
            Some(FieldValue::String("en".to_string()))
        );

        // Check newsletter field
        let newsletter_field = metadata.iter().find(|f| f.name == "newsletter").unwrap();
        assert!(!newsletter_field.is_required);
        assert_eq!(
            newsletter_field.default_value,
            Some(FieldValue::Boolean(false))
        );
    }

    #[test]
    fn test_derive_macro_default_values() {
        let form = TestForm::default_values();

        assert_eq!(form.name, "");
        assert_eq!(form.email, "");
        assert_eq!(form.language, "en");
        assert_eq!(form.newsletter, false);
    }

    #[test]
    fn test_derive_macro_get_field_value() {
        let form = TestForm {
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
        assert_eq!(
            form.get_field_value("language"),
            FieldValue::String("en".to_string())
        );
        assert_eq!(
            form.get_field_value("newsletter"),
            FieldValue::Boolean(true)
        );
    }

    #[test]
    fn test_derive_macro_set_field_value() {
        let mut form = TestForm::default_values();

        form.set_field_value("name", FieldValue::String("Jane Doe".to_string()));
        form.set_field_value("email", FieldValue::String("jane@example.com".to_string()));
        form.set_field_value("language", FieldValue::String("fr".to_string()));
        form.set_field_value("newsletter", FieldValue::Boolean(true));

        assert_eq!(form.name, "Jane Doe");
        assert_eq!(form.email, "jane@example.com");
        assert_eq!(form.language, "fr");
        assert_eq!(form.newsletter, true);
    }

    #[test]
    fn test_derive_macro_validation_success() {
        let form = TestForm {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            language: "en".to_string(),
            newsletter: true,
        };

        let result = form.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_derive_macro_validation_failure() {
        let form = TestForm {
            name: "Jo".to_string(),             // Too short
            email: "invalid-email".to_string(), // Invalid email
            language: "en".to_string(),
            newsletter: true,
        };

        let result = form.validate();
        assert!(result.is_err());

        if let Err(errors) = result {
            assert!(!errors.is_empty());
        }
    }

    #[test]
    fn test_derive_macro_schema() {
        let schema = TestForm::schema();

        assert_eq!(schema.name, "TestForm");
        assert_eq!(schema.field_metadata.len(), 4);
    }
}
