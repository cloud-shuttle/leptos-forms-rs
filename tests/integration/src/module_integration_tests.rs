//! Integration tests for module interactions
//! 
//! These tests verify that different modules work together correctly

use leptos::prelude::*;
use leptos_forms_rs::core::{Form, FormHandle, FieldMetadata, FieldType, FieldValue};
use leptos_forms_rs::validation::{ValidationErrors, Validator};
use leptos_forms_rs::hooks::{use_form, use_field_value, use_field_error};
use leptos_forms_rs::devtools::{FormStateInspector, PerformanceMonitor, DebugUtilities};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Test form for integration testing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct IntegrationForm {
    name: String,
    email: String,
    age: i32,
    preferences: Vec<String>,
    metadata: HashMap<String, String>,
}

impl Default for IntegrationForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            age: 0,
            preferences: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}

impl Form for IntegrationForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "name".to_string(),
                field_type: FieldType::Text,
                is_required: true,
                default_value: Some(FieldValue::String(String::new())),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![Validator::Required, Validator::MinLength(2)],
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
                field_type: FieldType::Number(leptos_forms_rs::core::NumberType { 
                    min: Some(0.0), 
                    max: Some(120.0), 
                    step: Some(1.0) 
                }),
                is_required: false,
                default_value: Some(FieldValue::Number(0.0)),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![Validator::Min(0.0), Validator::Max(120.0)],
            },
            FieldMetadata {
                name: "preferences".to_string(),
                field_type: FieldType::Array(Box::new(FieldType::Text)),
                is_required: false,
                default_value: Some(FieldValue::Array(Vec::new())),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: Vec::new(),
            },
        ]
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        
        if self.name.is_empty() {
            errors.add_field_error("name", "Name is required".to_string());
        } else if self.name.len() < 2 {
            errors.add_field_error("name", "Name must be at least 2 characters".to_string());
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
            "preferences" => FieldValue::Array(
                self.preferences.iter()
                    .map(|p| FieldValue::String(p.clone()))
                    .collect()
            ),
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
            "preferences" => {
                if let FieldValue::Array(arr) = value {
                    self.preferences = arr.into_iter()
                        .filter_map(|v| {
                            if let FieldValue::String(s) = v {
                                Some(s)
                            } else {
                                None
                            }
                        })
                        .collect();
                }
            }
            _ => {}
        }
    }
}

#[test]
fn test_core_validation_integration() {
    // Test that core validation works with form metadata
    let form = IntegrationForm {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: 25,
        preferences: vec!["newsletter".to_string(), "updates".to_string()],
        metadata: HashMap::new(),
    };
    
    let result = form.validate();
    assert!(result.is_ok());
    
    // Test invalid form
    let invalid_form = IntegrationForm {
        name: "A".to_string(), // Too short
        email: "invalid-email".to_string(), // Invalid format
        age: -5, // Invalid age
        preferences: Vec::new(),
        metadata: HashMap::new(),
    };
    
    let result = invalid_form.validate();
    assert!(result.is_err());
    
    if let Err(errors) = result {
        assert!(errors.has_field_error("name"));
        assert!(errors.has_field_error("email"));
        assert!(errors.has_field_error("age"));
    }
}

#[test]
fn test_form_handle_validation_integration() {
    // Test that FormHandle integrates with validation
    let form = IntegrationForm::default();
    let form_handle = FormHandle::new(form);
    
    // Initially should be invalid (empty required fields)
    let result = form_handle.validate();
    assert!(result.is_err());
    
    // Set valid values
    form_handle.set_field_value("name", FieldValue::String("Jane Doe".to_string()));
    form_handle.set_field_value("email", FieldValue::String("jane@example.com".to_string()));
    form_handle.set_field_value("age", FieldValue::Number(30.0));
    
    // Now should be valid
    let result = form_handle.validate();
    assert!(result.is_ok());
}

#[test]
fn test_hooks_form_handle_integration() {
    // Test that hooks work with FormHandle
    let (form_handle, _submit, _reset) = use_form(IntegrationForm::default());
    
    // Test field value hook
    let name_value = use_field_value(&form_handle, "name");
    assert_eq!(name_value.get_untracked(), FieldValue::String(String::new()));
    
    // Test field error hook
    let name_error = use_field_error(&form_handle, "name");
    // Initially no error since we haven't validated yet
    assert_eq!(name_error.get_untracked(), None);
    
    // Set a value and validate
    form_handle.set_field_value("name", FieldValue::String("Test".to_string()));
    let _result = form_handle.validate();
    
    // Now check if error hook reflects validation state
    // (This would depend on the specific implementation of use_field_error)
}

#[test]
fn test_devtools_form_handle_integration() {
    // Test that DevTools work with FormHandle
    let form_handle = FormHandle::new(IntegrationForm::default());
    
    // Test FormStateInspector
    let inspector = FormStateInspector::new(&form_handle).unwrap();
    let state = inspector.get_current_state();
    assert_eq!(state.field_count, 4); // name, email, age, preferences
    
    // Test PerformanceMonitor
    let monitor = PerformanceMonitor::new(&form_handle).unwrap();
    let metrics = monitor.get_metrics();
    assert!(metrics.form_creation_time.is_some());
    
    // Test DebugUtilities
    let snapshot = DebugUtilities::create_form_snapshot(&form_handle);
    assert_eq!(snapshot.field_count, 4);
}

#[test]
fn test_serialization_integration() {
    // Test that serialization works across all modules
    let form = IntegrationForm {
        name: "Serialization Test".to_string(),
        email: "test@example.com".to_string(),
        age: 42,
        preferences: vec!["option1".to_string(), "option2".to_string()],
        metadata: {
            let mut map = HashMap::new();
            map.insert("key1".to_string(), "value1".to_string());
            map
        },
    };
    
    // Test JSON serialization
    let json = serde_json::to_string(&form).unwrap();
    let deserialized: IntegrationForm = serde_json::from_str(&json).unwrap();
    assert_eq!(form, deserialized);
    
    // Test that FormHandle can work with serialized data
    let form_handle = FormHandle::new(deserialized);
    let result = form_handle.validate();
    assert!(result.is_ok());
}

#[test]
fn test_field_array_integration() {
    // Test that field arrays work with the form system
    let mut form = IntegrationForm::default();
    
    // Test setting array values
    let preferences = vec![
        FieldValue::String("newsletter".to_string()),
        FieldValue::String("updates".to_string()),
        FieldValue::String("promotions".to_string()),
    ];
    
    form.set_field_value("preferences", FieldValue::Array(preferences));
    
    // Verify the values were set correctly
    let retrieved = form.get_field_value("preferences");
    if let FieldValue::Array(arr) = retrieved {
        assert_eq!(arr.len(), 3);
        assert!(arr.contains(&FieldValue::String("newsletter".to_string())));
        assert!(arr.contains(&FieldValue::String("updates".to_string())));
        assert!(arr.contains(&FieldValue::String("promotions".to_string())));
    } else {
        panic!("Expected array field value");
    }
    
    // Test with FormHandle
    let form_handle = FormHandle::new(form);
    let retrieved_handle = form_handle.get_field_value("preferences");
    if let FieldValue::Array(arr) = retrieved_handle {
        assert_eq!(arr.len(), 3);
    } else {
        panic!("Expected array field value from FormHandle");
    }
}

#[test]
fn test_cross_module_error_handling() {
    // Test that error handling works across modules
    let form_handle = FormHandle::new(IntegrationForm::default());
    
    // Try to validate empty form (should fail)
    let result = form_handle.validate();
    assert!(result.is_err());
    
    if let Err(errors) = result {
        // Test that DevTools can handle validation errors
        let inspector = FormStateInspector::new(&form_handle).unwrap();
        let field_errors = inspector.get_validation_errors();
        assert!(!field_errors.is_empty());
        
        // Test that error information is accessible
        assert!(errors.has_field_error("name"));
        assert!(errors.has_field_error("email"));
    }
}

#[test]
fn test_performance_monitoring_integration() {
    // Test that performance monitoring works with form operations
    let form_handle = FormHandle::new(IntegrationForm::default());
    let monitor = PerformanceMonitor::new(&form_handle).unwrap();
    
    // Perform some operations
    form_handle.set_field_value("name", FieldValue::String("Performance Test".to_string()));
    form_handle.set_field_value("email", FieldValue::String("perf@example.com".to_string()));
    
    // Track field operations
    monitor.track_field_operation("name", "set");
    monitor.track_field_operation("email", "set");
    
    // Track validation
    let _result = form_handle.validate();
    monitor.track_validation_operation("full_form");
    
    // Check metrics
    let metrics = monitor.get_metrics();
    assert!(metrics.total_field_operations >= 2);
    assert!(metrics.validation_operations >= 1);
}
