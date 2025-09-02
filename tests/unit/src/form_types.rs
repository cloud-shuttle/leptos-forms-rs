//! Tests for form types functionality

use leptos::*;
use leptos_forms_rs::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_field_type_variants() {
    // Test that all FieldType variants exist and work correctly
    let text_type = FieldType::Text;
    let email_type = FieldType::Email;
    let password_type = FieldType::Password;
    let number_type = FieldType::Number;
    let boolean_type = FieldType::Boolean;
    
    // These should compile without errors
    assert!(matches!(text_type, FieldType::Text));
    assert!(matches!(email_type, FieldType::Email));
    assert!(matches!(password_type, FieldType::Password));
    assert!(matches!(number_type, FieldType::Number));
    assert!(matches!(boolean_type, FieldType::Boolean));
}

#[wasm_bindgen_test]
fn test_field_value_variants() {
    // Test FieldValue variants
    let string_value = FieldValue::String("test".to_string());
    let number_value = FieldValue::Number(42);
    let boolean_value = FieldValue::Boolean(true);
    
    // Test string value
    if let FieldValue::String(s) = &string_value {
        assert_eq!(s, "test");
    } else {
        panic!("Expected String variant");
    }
    
    // Test number value
    if let FieldValue::Number(n) = &number_value {
        assert_eq!(*n, 42);
    } else {
        panic!("Expected Number variant");
    }
    
    // Test boolean value
    if let FieldValue::Boolean(b) = &boolean_value {
        assert_eq!(*b, true);
    } else {
        panic!("Expected Boolean variant");
    }
}

#[wasm_bindgen_test]
fn test_validator_config_variants() {
    // Test ValidatorConfig variants
    let required = ValidatorConfig::Required;
    let email = ValidatorConfig::Email;
    let min_length = ValidatorConfig::MinLength(5);
    let max_length = ValidatorConfig::MaxLength(10);
    let pattern = ValidatorConfig::Pattern(r"^\d+$".to_string());
    let custom = ValidatorConfig::Custom("custom_validator".to_string());
    
    // Test required
    assert!(matches!(required, ValidatorConfig::Required));
    
    // Test email
    assert!(matches!(email, ValidatorConfig::Email));
    
    // Test min length
    if let ValidatorConfig::MinLength(len) = min_length {
        assert_eq!(len, 5);
    } else {
        panic!("Expected MinLength variant");
    }
    
    // Test max length
    if let ValidatorConfig::MaxLength(len) = max_length {
        assert_eq!(len, 10);
    } else {
        panic!("Expected MaxLength variant");
    }
    
    // Test pattern
    if let ValidatorConfig::Pattern(pat) = pattern {
        assert_eq!(pat, r"^\d+$");
    } else {
        panic!("Expected Pattern variant");
    }
    
    // Test custom
    if let ValidatorConfig::Custom(name) = custom {
        assert_eq!(name, "custom_validator");
    } else {
        panic!("Expected Custom variant");
    }
}

#[wasm_bindgen_test]
fn test_field_metadata_creation() {
    // Test creating FieldMetadata
    let metadata = FieldMetadata {
        name: "test_field".to_string(),
        field_type: FieldType::Text,
        validators: vec![ValidatorConfig::Required],
        is_required: true,
        default_value: Some(FieldValue::String("default".to_string())),
        dependencies: vec!["other_field".to_string()],
        attributes: {
            let mut map = std::collections::HashMap::new();
            map.insert("placeholder".to_string(), "Enter text".to_string());
            map
        },
    };
    
    assert_eq!(metadata.name, "test_field");
    assert!(matches!(metadata.field_type, FieldType::Text));
    assert_eq!(metadata.validators.len(), 1);
    assert!(metadata.is_required);
    assert!(metadata.default_value.is_some());
    assert_eq!(metadata.dependencies.len(), 1);
    assert_eq!(metadata.attributes.len(), 1);
}

#[wasm_bindgen_test]
fn test_form_schema_creation() {
    // Test creating FormSchema
    let metadata = vec![
        FieldMetadata {
            name: "field1".to_string(),
            field_type: FieldType::Text,
            validators: vec![ValidatorConfig::Required],
            is_required: true,
            default_value: None,
            dependencies: vec![],
            attributes: std::collections::HashMap::new(),
        },
        FieldMetadata {
            name: "field2".to_string(),
            field_type: FieldType::Email,
            validators: vec![ValidatorConfig::Required, ValidatorConfig::Email],
            is_required: true,
            default_value: None,
            dependencies: vec![],
            attributes: std::collections::HashMap::new(),
        },
    ];
    
    let schema = FormSchema::new(metadata);
    assert_eq!(schema.fields().len(), 2);
    
    // Test getting field by name
    let field1 = schema.get_field("field1");
    assert!(field1.is_some());
    assert_eq!(field1.unwrap().name, "field1");
    
    let field2 = schema.get_field("field2");
    assert!(field2.is_some());
    assert_eq!(field2.unwrap().name, "field2");
    
    // Test getting non-existent field
    let non_existent = schema.get_field("field3");
    assert!(non_existent.is_none());
}

#[wasm_bindgen_test]
fn test_validation_errors() {
    // Test ValidationErrors creation and manipulation
    let mut errors = ValidationErrors::new();
    
    // Initially empty
    assert!(errors.is_empty());
    assert_eq!(errors.field_errors().len(), 0);
    
    // Add field error
    let field_error = FieldError::new("This field is required");
    errors.add_field_error("test_field", field_error);
    
    // Should now have one error
    assert!(!errors.is_empty());
    assert_eq!(errors.field_errors().len(), 1);
    
    // Check if specific field has errors
    assert!(errors.has_field_error("test_field"));
    assert!(!errors.has_field_error("other_field"));
    
    // Get field errors
    let field_errors = errors.get_field_errors("test_field");
    assert_eq!(field_errors.len(), 1);
    assert_eq!(field_errors[0].message(), "This field is required");
}

#[wasm_bindgen_test]
fn test_field_error() {
    // Test FieldError creation and properties
    let error = FieldError::new("Custom error message");
    
    assert_eq!(error.message(), "Custom error message");
    
    // Test with different message
    let error2 = FieldError::new("Another error");
    assert_eq!(error2.message(), "Another error");
}
