//! Contract validation tests for leptos-forms-rs API
//!
//! These tests ensure that the API contract is maintained across versions
//! and that breaking changes are properly detected.

use crate::*;
use leptos::prelude::*;
use leptos_forms_rs::core::*;
use leptos_forms_rs::hooks::use_form;
use leptos_forms_rs::validation::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Test form for contract validation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TestContractForm {
    name: String,
    email: String,
    age: i32,
}

impl Default for TestContractForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            age: 0,
        }
    }
}

impl Form for TestContractForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "name".to_string(),
                field_type: FieldType::Text,
                is_required: true,
                default_value: Some(FieldValue::String(String::new())),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![Validator::Required],
            },
            FieldMetadata {
                name: "email".to_string(),
                field_type: FieldType::Email,
                is_required: true,
                default_value: Some(FieldValue::String(String::new())),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![Validator::Required, Validator::Email],
            },
            FieldMetadata {
                name: "age".to_string(),
                field_type: FieldType::Number(NumberType {
                    min: Some(0.0),
                    max: Some(120.0),
                    step: Some(1.0),
                }),
                is_required: false,
                default_value: Some(FieldValue::Number(0.0)),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![Validator::Min(0.0), Validator::Max(120.0)],
            },
        ]
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.name.is_empty() {
            errors.add_field_error("name", "Name is required".to_string());
        }

        if self.email.is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        } else if !self.email.contains('@') {
            errors.add_field_error("email", "Invalid email format".to_string());
        }

        if self.age < 0 || self.age > 120 {
            errors.add_field_error("age", "Age must be between 0 and 120".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn default_values() -> Self {
        Self::default()
    }

    fn get_field_value(&self, field_name: &str) -> FieldValue {
        match field_name {
            "name" => FieldValue::String(self.name.clone()),
            "email" => FieldValue::String(self.email.clone()),
            "age" => FieldValue::Number(self.age as f64),
            _ => FieldValue::String(String::new()),
        }
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "name" => {
                if let FieldValue::String(s) = value {
                    self.name = s;
                }
            }
            "email" => {
                if let FieldValue::String(s) = value {
                    self.email = s;
                }
            }
            "age" => {
                if let FieldValue::Number(n) = value {
                    self.age = n as i32;
                }
            }
            _ => {}
        }
    }
}

/// Legacy form for backward compatibility testing
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct LegacyForm {
    name: String,
}

impl Default for LegacyForm {
    fn default() -> Self {
        Self {
            name: String::new(),
        }
    }
}

impl Form for LegacyForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![FieldMetadata {
            name: "name".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: Some(FieldValue::String(String::new())),
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required],
        }]
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.name.is_empty() {
            errors.add_field_error("name", "Name is required".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn default_values() -> Self {
        Self::default()
    }

    fn get_field_value(&self, field_name: &str) -> FieldValue {
        match field_name {
            "name" => FieldValue::String(self.name.clone()),
            _ => FieldValue::String(String::new()),
        }
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "name" => {
                if let FieldValue::String(s) = value {
                    self.name = s;
                }
            }
            _ => {}
        }
    }
}

#[test]
fn test_form_trait_contract() {
    // Test that Form trait maintains required methods
    let test_form = TestContractForm::default();

    // Verify required methods exist and work
    let metadata = TestContractForm::field_metadata();
    assert!(!metadata.is_empty());
    assert_eq!(metadata.len(), 3); // name, email, age

    let validation = test_form.validate();
    // Should not panic, regardless of validation result
    assert!(validation.is_err()); // Empty form should be invalid

    let defaults = TestContractForm::default_values();
    assert_eq!(defaults, TestContractForm::default());

    let schema = TestContractForm::schema();
    assert!(!schema.name.is_empty());
    assert_eq!(schema.field_metadata.len(), 3);

    let field_value = test_form.get_field_value("name");
    assert_eq!(field_value, FieldValue::String(String::new()));

    let mut mutable_form = test_form.clone();
    mutable_form.set_field_value("name", FieldValue::String("test".to_string()));
    assert_eq!(mutable_form.name, "test");
}

#[test]
fn test_form_handle_contract() {
    let form = TestContractForm::default();
    let handle = FormHandle::new(form);

    // Test required FormHandle methods
    let _validation = handle.validate();
    let _is_valid = handle.is_valid();
    let _is_dirty = handle.is_dirty();
    let _is_submitting = handle.is_submitting();

    handle.set_field_value("name", FieldValue::String("test".to_string()));
    let field_value = handle.get_field_value("name");
    assert_eq!(field_value, FieldValue::String("test".to_string()));
}

#[test]
fn test_backward_compatibility() {
    // Test that old API usage still works
    let form = LegacyForm::default();
    let handle = FormHandle::new(form);

    // These should work exactly as they did in previous versions
    let result = handle.validate();
    assert!(result.is_err()); // Empty form should be invalid

    handle.set_field_value("name", FieldValue::String("Test".to_string()));
    let result = handle.validate();
    assert!(result.is_ok()); // Valid form should pass
}

#[test]
fn test_field_metadata_contract() {
    let metadata = TestContractForm::field_metadata();

    // Verify all required fields are present
    let name_field = metadata.iter().find(|f| f.name == "name").unwrap();
    assert_eq!(name_field.field_type, FieldType::Text);
    assert!(name_field.is_required);
    assert!(name_field.validators.contains(&Validator::Required));

    let email_field = metadata.iter().find(|f| f.name == "email").unwrap();
    assert_eq!(email_field.field_type, FieldType::Email);
    assert!(email_field.is_required);
    assert!(email_field.validators.contains(&Validator::Required));
    assert!(email_field.validators.contains(&Validator::Email));

    let age_field = metadata.iter().find(|f| f.name == "age").unwrap();
    assert!(matches!(age_field.field_type, FieldType::Number(_)));
    assert!(!age_field.is_required);
}

#[test]
fn test_validation_errors_contract() {
    let mut errors = ValidationErrors::new();

    // Test required methods
    assert!(errors.is_empty());
    assert!(!errors.has_field_error("nonexistent"));

    errors.add_field_error("test_field", "Test error".to_string());
    assert!(!errors.is_empty());
    assert!(errors.has_field_error("test_field"));
    assert!(!errors.has_field_error("other_field"));
}

#[test]
fn test_form_schema_contract() {
    let schema = TestContractForm::schema();

    // Verify schema structure
    assert!(!schema.name.is_empty());
    assert_eq!(schema.field_metadata.len(), 3);

    // Test get_field method
    let name_field = schema.get_field("name");
    assert!(name_field.is_some());
    assert_eq!(name_field.unwrap().name, "name");

    let nonexistent_field = schema.get_field("nonexistent");
    assert!(nonexistent_field.is_none());
}

#[test]
fn test_field_value_contract() {
    // Test all FieldValue variants
    let string_value = FieldValue::String("test".to_string());
    let number_value = FieldValue::Number(42.0);
    let boolean_value = FieldValue::Boolean(true);
    let array_value = FieldValue::Array(vec![
        FieldValue::String("item1".to_string()),
        FieldValue::String("item2".to_string()),
    ]);
    let null_value = FieldValue::Null;

    // Verify they can be created and used
    assert!(matches!(string_value, FieldValue::String(_)));
    assert!(matches!(number_value, FieldValue::Number(_)));
    assert!(matches!(boolean_value, FieldValue::Boolean(_)));
    assert!(matches!(array_value, FieldValue::Array(_)));
    assert!(matches!(null_value, FieldValue::Null));
}

#[test]
fn test_form_handle_integration() {
    // Test that FormHandle integrates properly with the Form trait
    let form = TestContractForm {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: 30,
    };

    let handle = FormHandle::new(form);

    // Test validation
    let result = handle.validate();
    assert!(result.is_ok());

    // Test field access
    let name_value = handle.get_field_value("name");
    assert_eq!(name_value, FieldValue::String("John Doe".to_string()));

    let email_value = handle.get_field_value("email");
    assert_eq!(
        email_value,
        FieldValue::String("john@example.com".to_string())
    );

    let age_value = handle.get_field_value("age");
    assert_eq!(age_value, FieldValue::Number(30.0));

    // Test field modification
    handle.set_field_value("name", FieldValue::String("Jane Doe".to_string()));
    let updated_name = handle.get_field_value("name");
    assert_eq!(updated_name, FieldValue::String("Jane Doe".to_string()));
}

#[test]
fn test_serialization_contract() {
    // Test that forms can be serialized and deserialized
    let form = TestContractForm {
        name: "Serialization Test".to_string(),
        email: "test@example.com".to_string(),
        age: 25,
    };

    // Test JSON serialization
    let json = serde_json::to_string(&form).unwrap();
    let deserialized: TestContractForm = serde_json::from_str(&json).unwrap();
    assert_eq!(form, deserialized);

    // Test that deserialized form works with FormHandle
    let handle = FormHandle::new(deserialized);
    let result = handle.validate();
    assert!(result.is_ok());
}

#[test]
fn test_contract_stability() {
    // Test that the API contract is stable and doesn't change unexpectedly
    let schema1 = generate_current_schema();
    let schema2 = generate_current_schema();

    // Schemas should be identical
    assert_eq!(schema1.version, schema2.version);
    assert_eq!(schema1.traits.len(), schema2.traits.len());
    assert_eq!(schema1.types.len(), schema2.types.len());

    // Monitor should detect no violations
    let monitor = ContractMonitor::new(schema1, schema2);
    let violations = monitor.detect_violations();
    assert!(violations.is_empty());
    assert!(!monitor.should_fail_build());
}
