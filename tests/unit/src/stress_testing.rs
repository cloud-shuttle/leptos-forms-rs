//! Stress tests for leptos-forms-rs
//!
//! These tests verify that the library can handle large forms and high-frequency updates

use leptos::prelude::*;
use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, Form, FormHandle};
use leptos_forms_rs::hooks::use_form;
use leptos_forms_rs::validation::{ValidationErrors, Validator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

// Large form for stress testing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct LargeStressForm {
    // Basic fields
    name: String,
    email: String,
    age: i32,

    // Many text fields
    field_1: String,
    field_2: String,
    field_3: String,
    field_4: String,
    field_5: String,
    field_6: String,
    field_7: String,
    field_8: String,
    field_9: String,
    field_10: String,
    field_11: String,
    field_12: String,
    field_13: String,
    field_14: String,
    field_15: String,
    field_16: String,
    field_17: String,
    field_18: String,
    field_19: String,
    field_20: String,

    // Number fields
    number_1: f64,
    number_2: f64,
    number_3: f64,
    number_4: f64,
    number_5: f64,

    // Boolean fields
    bool_1: bool,
    bool_2: bool,
    bool_3: bool,
    bool_4: bool,
    bool_5: bool,

    // Array fields
    tags: Vec<String>,
    categories: Vec<String>,
    items: Vec<String>,
}

impl Default for LargeStressForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            age: 0,
            field_1: String::new(),
            field_2: String::new(),
            field_3: String::new(),
            field_4: String::new(),
            field_5: String::new(),
            field_6: String::new(),
            field_7: String::new(),
            field_8: String::new(),
            field_9: String::new(),
            field_10: String::new(),
            field_11: String::new(),
            field_12: String::new(),
            field_13: String::new(),
            field_14: String::new(),
            field_15: String::new(),
            field_16: String::new(),
            field_17: String::new(),
            field_18: String::new(),
            field_19: String::new(),
            field_20: String::new(),
            number_1: 0.0,
            number_2: 0.0,
            number_3: 0.0,
            number_4: 0.0,
            number_5: 0.0,
            bool_1: false,
            bool_2: false,
            bool_3: false,
            bool_4: false,
            bool_5: false,
            tags: Vec::new(),
            categories: Vec::new(),
            items: Vec::new(),
        }
    }
}

impl Form for LargeStressForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        let mut metadata = Vec::new();

        // Basic fields
        metadata.push(FieldMetadata {
            name: "name".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: Some(FieldValue::String(String::new())),
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::MinLength(2)],
        });

        metadata.push(FieldMetadata {
            name: "email".to_string(),
            field_type: FieldType::Email,
            is_required: true,
            default_value: Some(FieldValue::String(String::new())),
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::Email],
        });

        metadata.push(FieldMetadata {
            name: "age".to_string(),
            field_type: FieldType::Number(leptos_forms_rs::core::NumberType {
                min: Some(0.0),
                max: Some(120.0),
                step: Some(1.0),
            }),
            is_required: false,
            default_value: Some(FieldValue::Number(0.0)),
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Min(0.0), Validator::Max(120.0)],
        });

        // Add many text fields
        for i in 1..=20 {
            metadata.push(FieldMetadata {
                name: format!("field_{}", i),
                field_type: FieldType::Text,
                is_required: false,
                default_value: Some(FieldValue::String(String::new())),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![Validator::MaxLength(100)],
            });
        }

        // Add number fields
        for i in 1..=5 {
            metadata.push(FieldMetadata {
                name: format!("number_{}", i),
                field_type: FieldType::Number(leptos_forms_rs::core::NumberType {
                    min: Some(-1000.0),
                    max: Some(1000.0),
                    step: Some(0.1),
                }),
                is_required: false,
                default_value: Some(FieldValue::Number(0.0)),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![Validator::Min(-1000.0), Validator::Max(1000.0)],
            });
        }

        // Add boolean fields
        for i in 1..=5 {
            metadata.push(FieldMetadata {
                name: format!("bool_{}", i),
                field_type: FieldType::Boolean,
                is_required: false,
                default_value: Some(FieldValue::Boolean(false)),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: Vec::new(),
            });
        }

        // Add array fields
        metadata.push(FieldMetadata {
            name: "tags".to_string(),
            field_type: FieldType::Array(Box::new(FieldType::Text)),
            is_required: false,
            default_value: Some(FieldValue::Array(Vec::new())),
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: Vec::new(),
        });

        metadata.push(FieldMetadata {
            name: "categories".to_string(),
            field_type: FieldType::Array(Box::new(FieldType::Text)),
            is_required: false,
            default_value: Some(FieldValue::Array(Vec::new())),
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: Vec::new(),
        });

        metadata.push(FieldMetadata {
            name: "items".to_string(),
            field_type: FieldType::Array(Box::new(FieldType::Text)),
            is_required: false,
            default_value: Some(FieldValue::Array(Vec::new())),
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: Vec::new(),
        });

        metadata
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // Basic validation
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

        // Validate text fields
        let text_fields = [
            &self.field_1,
            &self.field_2,
            &self.field_3,
            &self.field_4,
            &self.field_5,
            &self.field_6,
            &self.field_7,
            &self.field_8,
            &self.field_9,
            &self.field_10,
            &self.field_11,
            &self.field_12,
            &self.field_13,
            &self.field_14,
            &self.field_15,
            &self.field_16,
            &self.field_17,
            &self.field_18,
            &self.field_19,
            &self.field_20,
        ];

        for (i, field) in text_fields.iter().enumerate() {
            if field.len() > 100 {
                errors.add_field_error(&format!("field_{}", i + 1), "Field too long".to_string());
            }
        }

        // Validate number fields
        let numbers = [
            self.number_1,
            self.number_2,
            self.number_3,
            self.number_4,
            self.number_5,
        ];
        for (i, num) in numbers.iter().enumerate() {
            if *num < -1000.0 || *num > 1000.0 {
                errors.add_field_error(
                    &format!("number_{}", i + 1),
                    "Number out of range".to_string(),
                );
            }
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
            "field_1" => FieldValue::String(self.field_1.clone()),
            "field_2" => FieldValue::String(self.field_2.clone()),
            "field_3" => FieldValue::String(self.field_3.clone()),
            "field_4" => FieldValue::String(self.field_4.clone()),
            "field_5" => FieldValue::String(self.field_5.clone()),
            "field_6" => FieldValue::String(self.field_6.clone()),
            "field_7" => FieldValue::String(self.field_7.clone()),
            "field_8" => FieldValue::String(self.field_8.clone()),
            "field_9" => FieldValue::String(self.field_9.clone()),
            "field_10" => FieldValue::String(self.field_10.clone()),
            "field_11" => FieldValue::String(self.field_11.clone()),
            "field_12" => FieldValue::String(self.field_12.clone()),
            "field_13" => FieldValue::String(self.field_13.clone()),
            "field_14" => FieldValue::String(self.field_14.clone()),
            "field_15" => FieldValue::String(self.field_15.clone()),
            "field_16" => FieldValue::String(self.field_16.clone()),
            "field_17" => FieldValue::String(self.field_17.clone()),
            "field_18" => FieldValue::String(self.field_18.clone()),
            "field_19" => FieldValue::String(self.field_19.clone()),
            "field_20" => FieldValue::String(self.field_20.clone()),
            "number_1" => FieldValue::Number(self.number_1),
            "number_2" => FieldValue::Number(self.number_2),
            "number_3" => FieldValue::Number(self.number_3),
            "number_4" => FieldValue::Number(self.number_4),
            "number_5" => FieldValue::Number(self.number_5),
            "bool_1" => FieldValue::Boolean(self.bool_1),
            "bool_2" => FieldValue::Boolean(self.bool_2),
            "bool_3" => FieldValue::Boolean(self.bool_3),
            "bool_4" => FieldValue::Boolean(self.bool_4),
            "bool_5" => FieldValue::Boolean(self.bool_5),
            "tags" => FieldValue::Array(
                self.tags
                    .iter()
                    .map(|t| FieldValue::String(t.clone()))
                    .collect(),
            ),
            "categories" => FieldValue::Array(
                self.categories
                    .iter()
                    .map(|c| FieldValue::String(c.clone()))
                    .collect(),
            ),
            "items" => FieldValue::Array(
                self.items
                    .iter()
                    .map(|i| FieldValue::String(i.clone()))
                    .collect(),
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
            "field_1" => {
                if let FieldValue::String(s) = value {
                    self.field_1 = s;
                }
            }
            "field_2" => {
                if let FieldValue::String(s) = value {
                    self.field_2 = s;
                }
            }
            "field_3" => {
                if let FieldValue::String(s) = value {
                    self.field_3 = s;
                }
            }
            "field_4" => {
                if let FieldValue::String(s) = value {
                    self.field_4 = s;
                }
            }
            "field_5" => {
                if let FieldValue::String(s) = value {
                    self.field_5 = s;
                }
            }
            "field_6" => {
                if let FieldValue::String(s) = value {
                    self.field_6 = s;
                }
            }
            "field_7" => {
                if let FieldValue::String(s) = value {
                    self.field_7 = s;
                }
            }
            "field_8" => {
                if let FieldValue::String(s) = value {
                    self.field_8 = s;
                }
            }
            "field_9" => {
                if let FieldValue::String(s) = value {
                    self.field_9 = s;
                }
            }
            "field_10" => {
                if let FieldValue::String(s) = value {
                    self.field_10 = s;
                }
            }
            "field_11" => {
                if let FieldValue::String(s) = value {
                    self.field_11 = s;
                }
            }
            "field_12" => {
                if let FieldValue::String(s) = value {
                    self.field_12 = s;
                }
            }
            "field_13" => {
                if let FieldValue::String(s) = value {
                    self.field_13 = s;
                }
            }
            "field_14" => {
                if let FieldValue::String(s) = value {
                    self.field_14 = s;
                }
            }
            "field_15" => {
                if let FieldValue::String(s) = value {
                    self.field_15 = s;
                }
            }
            "field_16" => {
                if let FieldValue::String(s) = value {
                    self.field_16 = s;
                }
            }
            "field_17" => {
                if let FieldValue::String(s) = value {
                    self.field_17 = s;
                }
            }
            "field_18" => {
                if let FieldValue::String(s) = value {
                    self.field_18 = s;
                }
            }
            "field_19" => {
                if let FieldValue::String(s) = value {
                    self.field_19 = s;
                }
            }
            "field_20" => {
                if let FieldValue::String(s) = value {
                    self.field_20 = s;
                }
            }
            "number_1" => {
                if let FieldValue::Number(n) = value {
                    self.number_1 = n;
                }
            }
            "number_2" => {
                if let FieldValue::Number(n) = value {
                    self.number_2 = n;
                }
            }
            "number_3" => {
                if let FieldValue::Number(n) = value {
                    self.number_3 = n;
                }
            }
            "number_4" => {
                if let FieldValue::Number(n) = value {
                    self.number_4 = n;
                }
            }
            "number_5" => {
                if let FieldValue::Number(n) = value {
                    self.number_5 = n;
                }
            }
            "bool_1" => {
                if let FieldValue::Boolean(b) = value {
                    self.bool_1 = b;
                }
            }
            "bool_2" => {
                if let FieldValue::Boolean(b) = value {
                    self.bool_2 = b;
                }
            }
            "bool_3" => {
                if let FieldValue::Boolean(b) = value {
                    self.bool_3 = b;
                }
            }
            "bool_4" => {
                if let FieldValue::Boolean(b) = value {
                    self.bool_4 = b;
                }
            }
            "bool_5" => {
                if let FieldValue::Boolean(b) = value {
                    self.bool_5 = b;
                }
            }
            "tags" => {
                if let FieldValue::Array(arr) = value {
                    self.tags = arr
                        .into_iter()
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
            "categories" => {
                if let FieldValue::Array(arr) = value {
                    self.categories = arr
                        .into_iter()
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
            "items" => {
                if let FieldValue::Array(arr) = value {
                    self.items = arr
                        .into_iter()
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
fn test_large_form_creation_performance() {
    let start = Instant::now();
    let form = LargeStressForm::default();
    let creation_time = start.elapsed();

    // Form creation should be fast (less than 1ms)
    assert!(
        creation_time.as_micros() < 1000,
        "Form creation took too long: {:?}",
        creation_time
    );

    // Verify form has all expected fields
    let metadata = LargeStressForm::field_metadata();
    assert_eq!(metadata.len(), 36); // 3 basic + 20 text + 5 number + 5 bool + 3 arrays
}

#[test]
fn test_large_form_handle_creation_performance() {
    let start = Instant::now();
    let form = LargeStressForm::default();
    let form_handle = FormHandle::new(form);
    let creation_time = start.elapsed();

    // FormHandle creation should be fast (less than 5ms)
    assert!(
        creation_time.as_micros() < 5000,
        "FormHandle creation took too long: {:?}",
        creation_time
    );

    // Verify we can access all fields
    let metadata = LargeStressForm::field_metadata();
    for field in metadata {
        let _value = form_handle.get_field_value(&field.name);
        // Should not panic
    }
}

#[test]
fn test_high_frequency_field_updates() {
    let form = LargeStressForm::default();
    let form_handle = FormHandle::new(form);

    let start = Instant::now();

    // Perform 1000 rapid field updates
    for i in 0..1000 {
        let field_name = format!("field_{}", (i % 20) + 1);
        let value = FieldValue::String(format!("value_{}", i));
        form_handle.set_field_value(&field_name, value);
    }

    let update_time = start.elapsed();

    // 1000 updates should complete in reasonable time (less than 100ms)
    assert!(
        update_time.as_millis() < 100,
        "High frequency updates took too long: {:?}",
        update_time
    );

    // Verify some values were set correctly
    let value_1 = form_handle.get_field_value("field_1");
    if let Some(FieldValue::String(s)) = value_1 {
        assert!(s.starts_with("value_"));
    } else {
        panic!("Expected string value");
    }
}

#[test]
fn test_large_form_validation_performance() {
    let mut form = LargeStressForm::default();

    // Fill form with valid data
    form.name = "John Doe".to_string();
    form.email = "john@example.com".to_string();
    form.age = 30;

    for i in 1..=20 {
        let field_name = format!("field_{}", i);
        form.set_field_value(
            &field_name,
            FieldValue::String(format!("Valid field {}", i)),
        );
    }

    for i in 1..=5 {
        let field_name = format!("number_{}", i);
        form.set_field_value(&field_name, FieldValue::Number(i as f64 * 10.0));
    }

    for i in 1..=5 {
        let field_name = format!("bool_{}", i);
        form.set_field_value(&field_name, FieldValue::Boolean(i % 2 == 0));
    }

    form.tags = vec!["tag1".to_string(), "tag2".to_string()];
    form.categories = vec!["cat1".to_string(), "cat2".to_string()];
    form.items = vec!["item1".to_string(), "item2".to_string()];

    let form_handle = FormHandle::new(form);

    let start = Instant::now();
    let result = form_handle.validate();
    let validation_time = start.elapsed();

    // Validation should be fast (less than 10ms)
    assert!(
        validation_time.as_micros() < 10000,
        "Validation took too long: {:?}",
        validation_time
    );
    assert!(result.is_ok(), "Valid form should pass validation");
}

#[test]
fn test_large_form_serialization_performance() {
    let mut form = LargeStressForm::default();

    // Fill form with data
    form.name = "Serialization Test".to_string();
    form.email = "test@example.com".to_string();
    form.age = 25;

    for i in 1..=20 {
        let field_name = format!("field_{}", i);
        form.set_field_value(
            &field_name,
            FieldValue::String(format!("Serialized field {}", i)),
        );
    }

    for i in 1..=5 {
        let field_name = format!("number_{}", i);
        form.set_field_value(&field_name, FieldValue::Number(i as f64 * 5.0));
    }

    for i in 1..=5 {
        let field_name = format!("bool_{}", i);
        form.set_field_value(&field_name, FieldValue::Boolean(true));
    }

    form.tags = (1..=10).map(|i| format!("tag_{}", i)).collect();
    form.categories = (1..=5).map(|i| format!("category_{}", i)).collect();
    form.items = (1..=15).map(|i| format!("item_{}", i)).collect();

    // Test serialization performance
    let start = Instant::now();
    let json = serde_json::to_string(&form).unwrap();
    let serialization_time = start.elapsed();

    // Serialization should be fast (less than 5ms)
    assert!(
        serialization_time.as_micros() < 5000,
        "Serialization took too long: {:?}",
        serialization_time
    );

    // Test deserialization performance
    let start = Instant::now();
    let deserialized: LargeStressForm = serde_json::from_str(&json).unwrap();
    let deserialization_time = start.elapsed();

    // Deserialization should be fast (less than 5ms)
    assert!(
        deserialization_time.as_micros() < 5000,
        "Deserialization took too long: {:?}",
        deserialization_time
    );

    // Verify data integrity
    assert_eq!(form, deserialized);
}

#[test]
fn test_memory_usage_large_form() {
    // Test that large forms don't consume excessive memory
    let forms: Vec<FormHandle<LargeStressForm>> = (0..100)
        .map(|_| {
            let form = LargeStressForm::default();
            FormHandle::new(form)
        })
        .collect();

    // 100 large forms should be manageable
    assert_eq!(forms.len(), 100);

    // Test that we can still perform operations
    for (i, form_handle) in forms.iter().enumerate() {
        form_handle.set_field_value("name", FieldValue::String(format!("Form {}", i)));
        let name = form_handle.get_field_value("name");
        if let Some(FieldValue::String(s)) = name {
            assert_eq!(s, format!("Form {}", i));
        }
    }
}

#[test]
fn test_concurrent_field_access() {
    let form = LargeStressForm::default();
    let form_handle = FormHandle::new(form);

    // Simulate concurrent access by rapidly reading and writing different fields
    let start = Instant::now();

    for i in 0..500 {
        // Read from one field
        let read_field = format!("field_{}", (i % 20) + 1);
        let _value = form_handle.get_field_value(&read_field);

        // Write to another field
        let write_field = format!("field_{}", ((i + 1) % 20) + 1);
        form_handle.set_field_value(
            &write_field,
            FieldValue::String(format!("concurrent_{}", i)),
        );
    }

    let access_time = start.elapsed();

    // Concurrent access should be fast (less than 50ms)
    assert!(
        access_time.as_millis() < 50,
        "Concurrent access took too long: {:?}",
        access_time
    );
}

#[test]
fn test_array_field_stress() {
    let form = LargeStressForm::default();
    let form_handle = FormHandle::new(form);

    // Test large arrays
    let large_tags: Vec<FieldValue> = (1..=1000)
        .map(|i| FieldValue::String(format!("tag_{}", i)))
        .collect();

    let start = Instant::now();
    form_handle.set_field_value("tags", FieldValue::Array(large_tags));
    let set_time = start.elapsed();

    // Setting large array should be reasonable (less than 20ms)
    assert!(
        set_time.as_millis() < 20,
        "Large array set took too long: {:?}",
        set_time
    );

    // Test reading large array
    let start = Instant::now();
    let retrieved = form_handle.get_field_value("tags");
    let get_time = start.elapsed();

    // Reading large array should be fast (less than 5ms)
    assert!(
        get_time.as_millis() < 5,
        "Large array get took too long: {:?}",
        get_time
    );

    if let Some(FieldValue::Array(arr)) = retrieved {
        assert_eq!(arr.len(), 1000);
        // Verify some values
        if let FieldValue::String(first_tag) = &arr[0] {
            assert_eq!(first_tag, "tag_1");
        }
        if let FieldValue::String(last_tag) = &arr[999] {
            assert_eq!(last_tag, "tag_1000");
        }
    } else {
        panic!("Expected array value");
    }
}
