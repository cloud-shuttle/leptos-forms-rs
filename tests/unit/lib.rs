use wasm_bindgen_test::*;
use leptos_forms_rs::{
    core::types::*,
    validation::{ValidationErrors, Validators},
    utils::field_utils,
};
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

// Core type tests
#[cfg(test)]
mod core_types_tests {
    use super::*;

    #[test]
    fn test_field_value_string() {
        let value = FieldValue::String("test".to_string());
        assert_eq!(value.as_string(), Some(&"test".to_string()));
        assert_eq!(value.as_number(), None);
        assert_eq!(value.as_boolean(), None);
        assert!(!value.is_null());
        assert!(!value.is_empty());
    }

    #[test]
    fn test_field_value_number() {
        let value = FieldValue::Number(42.5);
        assert_eq!(value.as_number(), Some(42.5));
        assert_eq!(value.as_string(), None);
        assert!(!value.is_null());
        assert!(!value.is_empty());
    }

    #[test]
    fn test_field_value_integer() {
        let value = FieldValue::Integer(42);
        assert_eq!(value.as_number(), Some(42.0));
        assert_eq!(value.as_string(), None);
        assert!(!value.is_null());
        assert!(!value.is_empty());
    }

    #[test]
    fn test_field_value_boolean() {
        let value = FieldValue::Boolean(true);
        assert_eq!(value.as_boolean(), Some(true));
        assert_eq!(value.as_string(), None);
        assert!(!value.is_null());
        assert!(!value.is_empty());
    }

    #[test]
    fn test_field_value_null() {
        let value = FieldValue::Null;
        assert!(value.is_null());
        assert!(value.is_empty());
        assert_eq!(value.as_string(), None);
        assert_eq!(value.as_number(), None);
        assert_eq!(value.as_boolean(), None);
    }

    #[test]
    fn test_field_value_array() {
        let value = FieldValue::Array(vec![
            FieldValue::String("a".to_string()),
            FieldValue::Number(1.0),
        ]);
        
        assert_eq!(value.as_array().unwrap().len(), 2);
        assert!(!value.is_empty());
        
        let empty_array = FieldValue::Array(vec![]);
        assert!(empty_array.is_empty());
    }

    #[test]
    fn test_field_value_object() {
        let mut obj = HashMap::new();
        obj.insert("key1".to_string(), FieldValue::String("value1".to_string()));
        obj.insert("key2".to_string(), FieldValue::Number(42.0));
        
        let value = FieldValue::Object(obj);
        assert_eq!(value.as_object().unwrap().len(), 2);
        assert!(!value.is_empty());
        
        let empty_obj = FieldValue::Object(HashMap::new());
        assert!(empty_obj.is_empty());
    }

    #[test]
    fn test_field_value_empty_string() {
        let value = FieldValue::String("".to_string());
        assert!(value.is_empty());
        assert!(!value.is_null());
    }

    #[test]
    fn test_field_error() {
        let error = FieldError::new("email".to_string(), "Invalid email".to_string());
        assert_eq!(error.field, "email");
        assert_eq!(error.message, "Invalid email");
        assert_eq!(error.code, None);
        
        let error_with_code = error.with_code("EMAIL_INVALID".to_string());
        assert_eq!(error_with_code.code, Some("EMAIL_INVALID".to_string()));
        
        assert_eq!(format!("{}", error_with_code), "email: Invalid email");
    }

    #[test]
    fn test_form_error() {
        let field_error = FieldError::new("name".to_string(), "Required".to_string());
        let form_error = FormError::new("Form validation failed".to_string())
            .with_code("VALIDATION_FAILED".to_string())
            .with_field_errors(vec![field_error]);
        
        assert_eq!(form_error.message, "Form validation failed");
        assert_eq!(form_error.code, Some("VALIDATION_FAILED".to_string()));
        assert_eq!(form_error.field_errors.len(), 1);
        assert_eq!(form_error.field_errors[0].field, "name");
    }

    #[test]
    fn test_field_config_builder() {
        let config = FieldConfig::new("username".to_string())
            .with_label("Username".to_string())
            .with_placeholder("Enter your username".to_string())
            .with_help_text("Must be unique".to_string())
            .disabled()
            .readonly()
            .hidden()
            .with_attribute("data-test".to_string(), "username-field".to_string());
        
        assert_eq!(config.name, "username");
        assert_eq!(config.label, Some("Username".to_string()));
        assert_eq!(config.placeholder, Some("Enter your username".to_string()));
        assert_eq!(config.help_text, Some("Must be unique".to_string()));
        assert!(config.disabled);
        assert!(config.readonly);
        assert!(config.hidden);
        assert_eq!(config.attributes.get("data-test"), Some(&"username-field".to_string()));
    }

    #[test]
    fn test_form_submission_result() {
        let success_result = FormSubmissionResult::success("form_data".to_string())
            .with_warnings(vec!["Minor issue".to_string()]);
        
        assert!(success_result.success);
        assert_eq!(success_result.data, Some("form_data".to_string()));
        assert!(success_result.errors.is_empty());
        assert_eq!(success_result.warnings, vec!["Minor issue".to_string()]);
        
        let failure_result = FormSubmissionResult::<String>::failure(vec![
            FormError::new("Validation failed".to_string())
        ]);
        
        assert!(!failure_result.success);
        assert_eq!(failure_result.data, None);
        assert_eq!(failure_result.errors.len(), 1);
        assert!(failure_result.warnings.is_empty());
    }

    #[test]
    fn test_select_option() {
        let option = SelectOption {
            value: "us".to_string(),
            label: "United States".to_string(),
            disabled: false,
        };
        
        assert_eq!(option.value, "us");
        assert_eq!(option.label, "United States");
        assert!(!option.disabled);
    }

    #[test]
    fn test_number_type() {
        let number_type = NumberType {
            min: Some(0.0),
            max: Some(100.0),
            step: Some(0.1),
        };
        
        assert_eq!(number_type.min, Some(0.0));
        assert_eq!(number_type.max, Some(100.0));
        assert_eq!(number_type.step, Some(0.1));
    }

    #[test]
    fn test_file_constraints() {
        let constraints = FileConstraints {
            max_size: Some(1024 * 1024), // 1MB
            accept: vec!["image/jpeg".to_string(), "image/png".to_string()],
            multiple: false,
        };
        
        assert_eq!(constraints.max_size, Some(1024 * 1024));
        assert_eq!(constraints.accept.len(), 2);
        assert!(!constraints.multiple);
    }

    #[test]
    fn test_file_data() {
        let file_data = FileData {
            name: "test.jpg".to_string(),
            size: 1024,
            mime_type: "image/jpeg".to_string(),
            data: vec![0, 1, 2, 3],
        };
        
        assert_eq!(file_data.name, "test.jpg");
        assert_eq!(file_data.size, 1024);
        assert_eq!(file_data.mime_type, "image/jpeg");
        assert_eq!(file_data.data, vec![0, 1, 2, 3]);
    }
}

// Validation tests
#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_validation_errors_new() {
        let errors = ValidationErrors::new();
        assert!(errors.is_empty());
        assert!(errors.field_errors.is_empty());
        assert!(errors.form_errors.is_empty());
    }

    #[test]
    fn test_validation_errors_add_field_error() {
        let mut errors = ValidationErrors::new();
        errors.add_field_error("email".to_string(), "Invalid email".to_string());
        
        assert!(!errors.is_empty());
        assert!(errors.has_field_error("email"));
        assert_eq!(errors.get_field_error("email"), Some(&"Invalid email".to_string()));
        assert!(!errors.has_field_error("name"));
    }

    #[test]
    fn test_validation_errors_add_form_error() {
        let mut errors = ValidationErrors::new();
        errors.add_form_error("Form submission failed".to_string());
        
        assert!(!errors.is_empty());
        assert_eq!(errors.form_errors.len(), 1);
        assert_eq!(errors.form_errors[0], "Form submission failed");
    }

    #[test]
    fn test_validation_errors_clear_field() {
        let mut errors = ValidationErrors::new();
        errors.add_field_error("email".to_string(), "Invalid email".to_string());
        errors.add_field_error("name".to_string(), "Required".to_string());
        
        assert!(errors.has_field_error("email"));
        assert!(errors.has_field_error("name"));
        
        errors.clear_field("email");
        assert!(!errors.has_field_error("email"));
        assert!(errors.has_field_error("name"));
    }

    #[test]
    fn test_validation_errors_merge() {
        let mut errors1 = ValidationErrors::new();
        errors1.add_field_error("email".to_string(), "Invalid email".to_string());
        errors1.add_form_error("Form error 1".to_string());
        
        let mut errors2 = ValidationErrors::new();
        errors2.add_field_error("name".to_string(), "Required".to_string());
        errors2.add_form_error("Form error 2".to_string());
        
        errors1.merge(errors2);
        
        assert_eq!(errors1.field_errors.len(), 2);
        assert_eq!(errors1.form_errors.len(), 2);
        assert!(errors1.has_field_error("email"));
        assert!(errors1.has_field_error("name"));
    }

    #[test]
    fn test_validators_required() {
        // Valid cases
        assert!(Validators::required(&FieldValue::String("hello".to_string())).is_ok());
        assert!(Validators::required(&FieldValue::Number(42.0)).is_ok());
        assert!(Validators::required(&FieldValue::Boolean(false)).is_ok());
        assert!(Validators::required(&FieldValue::Array(vec![FieldValue::String("item".to_string())])).is_ok());
        
        // Invalid cases
        assert!(Validators::required(&FieldValue::String("".to_string())).is_err());
        assert!(Validators::required(&FieldValue::String("   ".to_string())).is_err());
        assert!(Validators::required(&FieldValue::Null).is_err());
        assert!(Validators::required(&FieldValue::Array(vec![])).is_err());
        
        // Error messages
        let result = Validators::required(&FieldValue::Null);
        assert_eq!(result.unwrap_err(), "This field is required");
    }

    #[test]
    fn test_validators_email() {
        // Valid emails
        assert!(Validators::email(&FieldValue::String("test@example.com".to_string())).is_ok());
        assert!(Validators::email(&FieldValue::String("user.name@domain.co.uk".to_string())).is_ok());
        assert!(Validators::email(&FieldValue::String("user+tag@example.org".to_string())).is_ok());
        
        // Invalid emails
        assert!(Validators::email(&FieldValue::String("invalid".to_string())).is_err());
        assert!(Validators::email(&FieldValue::String("@domain.com".to_string())).is_err());
        assert!(Validators::email(&FieldValue::String("user@".to_string())).is_err());
        assert!(Validators::email(&FieldValue::String("user@domain".to_string())).is_err());
        assert!(Validators::email(&FieldValue::String("user name@domain.com".to_string())).is_err());
        
        // Non-string values
        assert!(Validators::email(&FieldValue::Number(42.0)).is_err());
        assert!(Validators::email(&FieldValue::Null).is_err());
        
        // Error messages
        let result = Validators::email(&FieldValue::String("invalid".to_string()));
        assert_eq!(result.unwrap_err(), "Invalid email format");
    }

    #[test]
    fn test_validators_url() {
        // Valid URLs
        assert!(Validators::url(&FieldValue::String("https://example.com".to_string())).is_ok());
        assert!(Validators::url(&FieldValue::String("http://domain.org/path".to_string())).is_ok());
        assert!(Validators::url(&FieldValue::String("https://sub.domain.com:8080/path?query=1".to_string())).is_ok());
        
        // Invalid URLs
        assert!(Validators::url(&FieldValue::String("invalid".to_string())).is_err());
        assert!(Validators::url(&FieldValue::String("ftp://example.com".to_string())).is_err());
        assert!(Validators::url(&FieldValue::String("//example.com".to_string())).is_err());
        
        // Non-string values
        assert!(Validators::url(&FieldValue::Number(42.0)).is_err());
        
        // Error messages
        let result = Validators::url(&FieldValue::String("invalid".to_string()));
        assert_eq!(result.unwrap_err(), "Invalid URL format");
    }

    #[test]
    fn test_validators_min_length() {
        // Valid cases
        assert!(Validators::min_length(&FieldValue::String("hello".to_string()), 3).is_ok());
        assert!(Validators::min_length(&FieldValue::String("hello".to_string()), 5).is_ok());
        
        // Invalid cases
        assert!(Validators::min_length(&FieldValue::String("hi".to_string()), 3).is_err());
        assert!(Validators::min_length(&FieldValue::String("".to_string()), 1).is_err());
        
        // Non-string values
        assert!(Validators::min_length(&FieldValue::Number(42.0), 3).is_err());
        
        // Error message
        let result = Validators::min_length(&FieldValue::String("hi".to_string()), 5);
        assert_eq!(result.unwrap_err(), "Minimum length is 5 characters");
    }

    #[test]
    fn test_validators_max_length() {
        // Valid cases
        assert!(Validators::max_length(&FieldValue::String("hi".to_string()), 5).is_ok());
        assert!(Validators::max_length(&FieldValue::String("hello".to_string()), 5).is_ok());
        
        // Invalid cases
        assert!(Validators::max_length(&FieldValue::String("hello world".to_string()), 5).is_err());
        
        // Non-string values
        assert!(Validators::max_length(&FieldValue::Number(42.0), 5).is_err());
        
        // Error message
        let result = Validators::max_length(&FieldValue::String("hello world".to_string()), 5);
        assert_eq!(result.unwrap_err(), "Maximum length is 5 characters");
    }

    #[test]
    fn test_validators_min_max_numbers() {
        // Min validation
        assert!(Validators::min(&FieldValue::Number(10.0), 5.0).is_ok());
        assert!(Validators::min(&FieldValue::Number(5.0), 5.0).is_ok());
        assert!(Validators::min(&FieldValue::Number(3.0), 5.0).is_err());
        assert!(Validators::min(&FieldValue::Integer(10), 5.0).is_ok());
        
        // Max validation
        assert!(Validators::max(&FieldValue::Number(3.0), 5.0).is_ok());
        assert!(Validators::max(&FieldValue::Number(5.0), 5.0).is_ok());
        assert!(Validators::max(&FieldValue::Number(7.0), 5.0).is_err());
        assert!(Validators::max(&FieldValue::Integer(3), 5.0).is_ok());
        
        // Non-numeric values
        assert!(Validators::min(&FieldValue::String("hello".to_string()), 5.0).is_err());
        assert!(Validators::max(&FieldValue::String("hello".to_string()), 5.0).is_err());
    }

    #[test]
    fn test_validators_pattern() {
        // Valid patterns
        assert!(Validators::pattern(&FieldValue::String("123".to_string()), r"^\d+$").is_ok());
        assert!(Validators::pattern(&FieldValue::String("abc".to_string()), r"^[a-z]+$").is_ok());
        
        // Invalid patterns
        assert!(Validators::pattern(&FieldValue::String("123abc".to_string()), r"^\d+$").is_err());
        assert!(Validators::pattern(&FieldValue::String("ABC".to_string()), r"^[a-z]+$").is_err());
        
        // Non-string values
        assert!(Validators::pattern(&FieldValue::Number(123.0), r"^\d+$").is_err());
        
        // Invalid regex
        assert!(Validators::pattern(&FieldValue::String("test".to_string()), r"[").is_err());
        
        // Error message
        let result = Validators::pattern(&FieldValue::String("abc".to_string()), r"^\d+$");
        assert_eq!(result.unwrap_err(), "Value doesn't match required pattern");
    }

    #[test]
    fn test_validators_positive_negative() {
        // Positive
        assert!(Validators::positive(&FieldValue::Number(5.0)).is_ok());
        assert!(Validators::positive(&FieldValue::Integer(10)).is_ok());
        assert!(Validators::positive(&FieldValue::Number(0.0)).is_err());
        assert!(Validators::positive(&FieldValue::Number(-5.0)).is_err());
        
        // Negative
        assert!(Validators::negative(&FieldValue::Number(-5.0)).is_ok());
        assert!(Validators::negative(&FieldValue::Integer(-10)).is_ok());
        assert!(Validators::negative(&FieldValue::Number(0.0)).is_err());
        assert!(Validators::negative(&FieldValue::Number(5.0)).is_err());
        
        // Non-numeric values
        assert!(Validators::positive(&FieldValue::String("5".to_string())).is_err());
        assert!(Validators::negative(&FieldValue::String("-5".to_string())).is_err());
    }

    #[test]
    fn test_validators_integer() {
        // Valid integers
        assert!(Validators::integer(&FieldValue::Number(5.0)).is_ok());
        assert!(Validators::integer(&FieldValue::Number(-10.0)).is_ok());
        assert!(Validators::integer(&FieldValue::Number(0.0)).is_ok());
        assert!(Validators::integer(&FieldValue::Integer(42)).is_ok());
        
        // Invalid integers (floats)
        assert!(Validators::integer(&FieldValue::Number(5.5)).is_err());
        assert!(Validators::integer(&FieldValue::Number(-10.1)).is_err());
        
        // Non-numeric values
        assert!(Validators::integer(&FieldValue::String("5".to_string())).is_err());
        
        // Error message
        let result = Validators::integer(&FieldValue::Number(5.5));
        assert_eq!(result.unwrap_err(), "Value must be an integer");
    }

    #[test]
    fn test_validators_array_length() {
        let array = vec![
            FieldValue::String("a".to_string()),
            FieldValue::String("b".to_string()),
            FieldValue::String("c".to_string()),
        ];
        
        // Valid length
        assert!(Validators::array_length(&FieldValue::Array(array.clone()), 1, 5).is_ok());
        assert!(Validators::array_length(&FieldValue::Array(array.clone()), 3, 3).is_ok());
        
        // Invalid length
        assert!(Validators::array_length(&FieldValue::Array(array.clone()), 5, 10).is_err());
        assert!(Validators::array_length(&FieldValue::Array(array.clone()), 1, 2).is_err());
        
        // Non-array values
        assert!(Validators::array_length(&FieldValue::String("hello".to_string()), 1, 5).is_err());
        
        // Error message
        let result = Validators::array_length(&FieldValue::Array(array), 5, 10);
        assert_eq!(result.unwrap_err(), "Array must have between 5 and 10 items");
    }
}

// Field utils tests
#[cfg(test)]
mod field_utils_tests {
    use super::*;

    #[test]
    fn test_string_to_field_value() {
        assert_eq!(field_utils::string_to_field_value("hello"), FieldValue::String("hello".to_string()));
        assert_eq!(field_utils::string_to_field_value(""), FieldValue::Null);
    }

    #[test]
    fn test_number_to_field_value() {
        assert_eq!(field_utils::number_to_field_value(42.5), FieldValue::Number(42.5));
    }

    #[test]
    fn test_bool_to_field_value() {
        assert_eq!(field_utils::bool_to_field_value(true), FieldValue::Boolean(true));
        assert_eq!(field_utils::bool_to_field_value(false), FieldValue::Boolean(false));
    }

    #[test]
    fn test_field_value_to_string() {
        assert_eq!(field_utils::field_value_to_string(&FieldValue::String("hello".to_string())), "hello");
        assert_eq!(field_utils::field_value_to_string(&FieldValue::Number(42.5)), "42.5");
        assert_eq!(field_utils::field_value_to_string(&FieldValue::Integer(42)), "42");
        assert_eq!(field_utils::field_value_to_string(&FieldValue::Boolean(true)), "true");
        assert_eq!(field_utils::field_value_to_string(&FieldValue::Null), "");
        
        let array = FieldValue::Array(vec![
            FieldValue::String("a".to_string()),
            FieldValue::Number(1.0),
        ]);
        assert_eq!(field_utils::field_value_to_string(&array), "[a, 1]");
        
        let mut obj = HashMap::new();
        obj.insert("key".to_string(), FieldValue::String("value".to_string()));
        let object = FieldValue::Object(obj);
        assert_eq!(field_utils::field_value_to_string(&object), "{key: value}");
    }

    #[test]
    fn test_is_field_value_empty() {
        assert!(field_utils::is_field_value_empty(&FieldValue::String("".to_string())));
        assert!(field_utils::is_field_value_empty(&FieldValue::String("   ".to_string())));
        assert!(field_utils::is_field_value_empty(&FieldValue::Array(vec![])));
        assert!(field_utils::is_field_value_empty(&FieldValue::Object(HashMap::new())));
        assert!(field_utils::is_field_value_empty(&FieldValue::Null));
        
        assert!(!field_utils::is_field_value_empty(&FieldValue::String("hello".to_string())));
        assert!(!field_utils::is_field_value_empty(&FieldValue::Number(0.0)));
        assert!(!field_utils::is_field_value_empty(&FieldValue::Boolean(false)));
    }

    #[test]
    fn test_get_field_value_type() {
        assert_eq!(field_utils::get_field_value_type(&FieldValue::String("".to_string())), "string");
        assert_eq!(field_utils::get_field_value_type(&FieldValue::Number(42.0)), "number");
        assert_eq!(field_utils::get_field_value_type(&FieldValue::Integer(42)), "integer");
        assert_eq!(field_utils::get_field_value_type(&FieldValue::Boolean(true)), "boolean");
        assert_eq!(field_utils::get_field_value_type(&FieldValue::Array(vec![])), "array");
        assert_eq!(field_utils::get_field_value_type(&FieldValue::Object(HashMap::new())), "object");
        assert_eq!(field_utils::get_field_value_type(&FieldValue::Null), "null");
    }
}

// WASM-specific tests
#[wasm_bindgen_test]
fn test_field_value_serialization() {
    let value = FieldValue::String("test".to_string());
    let serialized = serde_json::to_string(&value).unwrap();
    let deserialized: FieldValue = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(value.as_string(), deserialized.as_string());
}

#[wasm_bindgen_test]
fn test_validation_errors_in_wasm() {
    let mut errors = ValidationErrors::new();
    errors.add_field_error("email".to_string(), "Invalid email".to_string());
    errors.add_form_error("Form submission failed".to_string());
    
    assert!(!errors.is_empty());
    assert!(errors.has_field_error("email"));
    assert_eq!(errors.form_errors.len(), 1);
}

#[wasm_bindgen_test]
fn test_validators_in_wasm() {
    // Test email validation in WASM context
    let valid_email = FieldValue::String("test@example.com".to_string());
    let invalid_email = FieldValue::String("invalid-email".to_string());
    
    assert!(Validators::email(&valid_email).is_ok());
    assert!(Validators::email(&invalid_email).is_err());
    
    // Test required validation
    let empty_value = FieldValue::String("".to_string());
    let filled_value = FieldValue::String("content".to_string());
    
    assert!(Validators::required(&empty_value).is_err());
    assert!(Validators::required(&filled_value).is_ok());
}

#[wasm_bindgen_test]
fn test_complex_field_value_operations() {
    // Test nested array/object operations
    let mut inner_obj = HashMap::new();
    inner_obj.insert("nested_key".to_string(), FieldValue::String("nested_value".to_string()));
    
    let complex_array = FieldValue::Array(vec![
        FieldValue::String("string_item".to_string()),
        FieldValue::Number(42.0),
        FieldValue::Object(inner_obj),
        FieldValue::Boolean(true),
    ]);
    
    assert_eq!(complex_array.as_array().unwrap().len(), 4);
    assert!(!complex_array.is_empty());
    
    // Test serialization round-trip
    let serialized = serde_json::to_string(&complex_array).unwrap();
    let deserialized: FieldValue = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(deserialized.as_array().unwrap().len(), 4);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_run_in_node() {
        // This test can run in Node.js environment
        assert_eq!(2 + 2, 4);
    }
}