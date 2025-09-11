//! Consumer contract tests for leptos-forms-rs
//!
//! These tests verify that consumers can properly implement the Form trait
//! and use all the functionality provided by the library.

use crate::*;
use leptos::prelude::*;
use leptos_forms_rs::core::*;
use leptos_forms_rs::hooks::use_form;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Consumer form implementation for testing
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ConsumerForm {
    name: String,
    email: String,
    age: Option<i32>,
    preferences: Vec<String>,
    newsletter: bool,
}

impl Default for ConsumerForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            age: None,
            preferences: Vec::new(),
            newsletter: false,
        }
    }
}

impl Form for ConsumerForm {
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
                field_type: FieldType::Number(NumberType {
                    min: Some(18.0),
                    max: Some(100.0),
                    step: Some(1.0),
                }),
                is_required: false,
                default_value: Some(FieldValue::Null),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![Validator::Min(18.0), Validator::Max(100.0)],
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
            FieldMetadata {
                name: "newsletter".to_string(),
                field_type: FieldType::Boolean,
                is_required: false,
                default_value: Some(FieldValue::Boolean(false)),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: Vec::new(),
            },
        ]
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // Name validation
        if self.name.is_empty() {
            errors.add_field_error("name", "Name is required".to_string());
        } else if self.name.len() < 2 {
            errors.add_field_error("name", "Name must be at least 2 characters".to_string());
        }

        // Email validation
        if self.email.is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        } else if !self.email.contains('@') {
            errors.add_field_error("email", "Invalid email format".to_string());
        }

        // Age validation (optional)
        if let Some(age) = self.age {
            if age < 18 || age > 100 {
                errors.add_field_error("age", "Age must be between 18 and 100".to_string());
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
            "age" => {
                if let Some(age) = self.age {
                    FieldValue::Number(age as f64)
                } else {
                    FieldValue::Null
                }
            }
            "preferences" => FieldValue::Array(
                self.preferences
                    .iter()
                    .map(|p| FieldValue::String(p.clone()))
                    .collect(),
            ),
            "newsletter" => FieldValue::Boolean(self.newsletter),
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
            "age" => match value {
                FieldValue::Number(n) => self.age = Some(n as i32),
                FieldValue::Null => self.age = None,
                _ => {}
            },
            "preferences" => {
                if let FieldValue::Array(arr) = value {
                    self.preferences = arr
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
            "newsletter" => {
                if let FieldValue::Boolean(b) = value {
                    self.newsletter = b;
                }
            }
            _ => {}
        }
    }
}

/// Complex consumer form with nested data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ComplexConsumerForm {
    user: UserData,
    settings: UserSettings,
    tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct UserData {
    first_name: String,
    last_name: String,
    email: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct UserSettings {
    theme: String,
    notifications: bool,
    language: String,
}

impl Default for ComplexConsumerForm {
    fn default() -> Self {
        Self {
            user: UserData {
                first_name: String::new(),
                last_name: String::new(),
                email: String::new(),
            },
            settings: UserSettings {
                theme: "light".to_string(),
                notifications: true,
                language: "en".to_string(),
            },
            tags: Vec::new(),
        }
    }
}

impl Form for ComplexConsumerForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "user.first_name".to_string(),
                field_type: FieldType::Text,
                is_required: true,
                default_value: Some(FieldValue::String(String::new())),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![Validator::Required],
            },
            FieldMetadata {
                name: "user.last_name".to_string(),
                field_type: FieldType::Text,
                is_required: true,
                default_value: Some(FieldValue::String(String::new())),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![Validator::Required],
            },
            FieldMetadata {
                name: "user.email".to_string(),
                field_type: FieldType::Email,
                is_required: true,
                default_value: Some(FieldValue::String(String::new())),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: vec![Validator::Required, Validator::Email],
            },
            FieldMetadata {
                name: "settings.theme".to_string(),
                field_type: FieldType::Select(vec![
                    SelectOption {
                        value: "light".to_string(),
                        label: "Light".to_string(),
                        disabled: false,
                    },
                    SelectOption {
                        value: "dark".to_string(),
                        label: "Dark".to_string(),
                        disabled: false,
                    },
                ]),
                is_required: false,
                default_value: Some(FieldValue::String("light".to_string())),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: Vec::new(),
            },
            FieldMetadata {
                name: "settings.notifications".to_string(),
                field_type: FieldType::Boolean,
                is_required: false,
                default_value: Some(FieldValue::Boolean(true)),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: Vec::new(),
            },
            FieldMetadata {
                name: "settings.language".to_string(),
                field_type: FieldType::Select(vec![
                    SelectOption {
                        value: "en".to_string(),
                        label: "English".to_string(),
                        disabled: false,
                    },
                    SelectOption {
                        value: "es".to_string(),
                        label: "Spanish".to_string(),
                        disabled: false,
                    },
                ]),
                is_required: false,
                default_value: Some(FieldValue::String("en".to_string())),
                dependencies: Vec::new(),
                attributes: HashMap::new(),
                validators: Vec::new(),
            },
            FieldMetadata {
                name: "tags".to_string(),
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

        if self.user.first_name.is_empty() {
            errors.add_field_error("user.first_name", "First name is required".to_string());
        }

        if self.user.last_name.is_empty() {
            errors.add_field_error("user.last_name", "Last name is required".to_string());
        }

        if self.user.email.is_empty() {
            errors.add_field_error("user.email", "Email is required".to_string());
        } else if !self.user.email.contains('@') {
            errors.add_field_error("user.email", "Invalid email format".to_string());
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
            "user.first_name" => FieldValue::String(self.user.first_name.clone()),
            "user.last_name" => FieldValue::String(self.user.last_name.clone()),
            "user.email" => FieldValue::String(self.user.email.clone()),
            "settings.theme" => FieldValue::String(self.settings.theme.clone()),
            "settings.notifications" => FieldValue::Boolean(self.settings.notifications),
            "settings.language" => FieldValue::String(self.settings.language.clone()),
            "tags" => FieldValue::Array(
                self.tags
                    .iter()
                    .map(|t| FieldValue::String(t.clone()))
                    .collect(),
            ),
            _ => FieldValue::String(String::new()),
        }
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "user.first_name" => {
                if let FieldValue::String(s) = value {
                    self.user.first_name = s;
                }
            }
            "user.last_name" => {
                if let FieldValue::String(s) = value {
                    self.user.last_name = s;
                }
            }
            "user.email" => {
                if let FieldValue::String(s) = value {
                    self.user.email = s;
                }
            }
            "settings.theme" => {
                if let FieldValue::String(s) = value {
                    self.settings.theme = s;
                }
            }
            "settings.notifications" => {
                if let FieldValue::Boolean(b) = value {
                    self.settings.notifications = b;
                }
            }
            "settings.language" => {
                if let FieldValue::String(s) = value {
                    self.settings.language = s;
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
            _ => {}
        }
    }
}

#[test]
fn test_consumer_can_implement_form() {
    let form = ConsumerForm::default();
    let handle = FormHandle::new(form);

    // Consumer should be able to use all FormHandle functionality
    let result = handle.validate();
    assert!(result.is_err()); // Empty form should be invalid

    // Set valid values
    handle.set_field_value("name", FieldValue::String("John Doe".to_string()));
    handle.set_field_value("email", FieldValue::String("john@example.com".to_string()));
    handle.set_field_value("age", FieldValue::Number(25.0));
    handle.set_field_value("newsletter", FieldValue::Boolean(true));

    let result = handle.validate();
    assert!(result.is_ok()); // Valid form should pass
}

#[test]
fn test_consumer_field_access() {
    let form = ConsumerForm {
        name: "Jane Doe".to_string(),
        email: "jane@example.com".to_string(),
        age: Some(30),
        preferences: vec!["newsletter".to_string(), "updates".to_string()],
        newsletter: true,
    };

    let handle = FormHandle::new(form);

    // Test field value access
    let name_value = handle.get_field_value("name");
    assert_eq!(name_value, FieldValue::String("Jane Doe".to_string()));

    let email_value = handle.get_field_value("email");
    assert_eq!(
        email_value,
        FieldValue::String("jane@example.com".to_string())
    );

    let age_value = handle.get_field_value("age");
    assert_eq!(age_value, FieldValue::Number(30.0));

    let newsletter_value = handle.get_field_value("newsletter");
    assert_eq!(newsletter_value, FieldValue::Boolean(true));

    let preferences_value = handle.get_field_value("preferences");
    if let FieldValue::Array(arr) = preferences_value {
        assert_eq!(arr.len(), 2);
        assert!(arr.contains(&FieldValue::String("newsletter".to_string())));
        assert!(arr.contains(&FieldValue::String("updates".to_string())));
    } else {
        panic!("Expected array field value");
    }
}

#[test]
fn test_consumer_field_modification() {
    let form = ConsumerForm::default();
    let handle = FormHandle::new(form);

    // Test field modification
    handle.set_field_value("name", FieldValue::String("Modified Name".to_string()));
    let name_value = handle.get_field_value("name");
    assert_eq!(name_value, FieldValue::String("Modified Name".to_string()));

    handle.set_field_value("age", FieldValue::Number(42.0));
    let age_value = handle.get_field_value("age");
    assert_eq!(age_value, FieldValue::Number(42.0));

    handle.set_field_value("age", FieldValue::Null);
    let age_value = handle.get_field_value("age");
    assert_eq!(age_value, FieldValue::Null);

    handle.set_field_value("newsletter", FieldValue::Boolean(false));
    let newsletter_value = handle.get_field_value("newsletter");
    assert_eq!(newsletter_value, FieldValue::Boolean(false));
}

#[test]
fn test_consumer_validation() {
    let form = ConsumerForm::default();
    let handle = FormHandle::new(form);

    // Test validation with missing required fields
    let result = handle.validate();
    assert!(result.is_err());

    if let Err(errors) = result {
        assert!(errors.has_field_error("name"));
        assert!(errors.has_field_error("email"));
    }

    // Test validation with invalid data
    handle.set_field_value("name", FieldValue::String("A".to_string())); // Too short
    handle.set_field_value("email", FieldValue::String("invalid-email".to_string())); // Invalid format
    handle.set_field_value("age", FieldValue::Number(15.0)); // Too young

    let result = handle.validate();
    assert!(result.is_err());

    if let Err(errors) = result {
        assert!(errors.has_field_error("name"));
        assert!(errors.has_field_error("email"));
        assert!(errors.has_field_error("age"));
    }

    // Test validation with valid data
    handle.set_field_value("name", FieldValue::String("John Doe".to_string()));
    handle.set_field_value("email", FieldValue::String("john@example.com".to_string()));
    handle.set_field_value("age", FieldValue::Number(25.0));

    let result = handle.validate();
    assert!(result.is_ok());
}

#[test]
fn test_consumer_serialization() {
    let form = ConsumerForm {
        name: "Serialization Test".to_string(),
        email: "test@example.com".to_string(),
        age: Some(35),
        preferences: vec!["option1".to_string(), "option2".to_string()],
        newsletter: true,
    };

    // Test JSON serialization
    let json = serde_json::to_string(&form).unwrap();
    let deserialized: ConsumerForm = serde_json::from_str(&json).unwrap();
    assert_eq!(form, deserialized);

    // Test that deserialized form works with FormHandle
    let handle = FormHandle::new(deserialized);
    let result = handle.validate();
    assert!(result.is_ok());
}

#[test]
fn test_complex_consumer_form() {
    let form = ComplexConsumerForm::default();
    let handle = FormHandle::new(form);

    // Test complex form validation
    let result = handle.validate();
    assert!(result.is_err()); // Empty form should be invalid

    // Set valid values
    handle.set_field_value("user.first_name", FieldValue::String("John".to_string()));
    handle.set_field_value("user.last_name", FieldValue::String("Doe".to_string()));
    handle.set_field_value(
        "user.email",
        FieldValue::String("john@example.com".to_string()),
    );
    handle.set_field_value("settings.theme", FieldValue::String("dark".to_string()));
    handle.set_field_value("settings.notifications", FieldValue::Boolean(false));
    handle.set_field_value("settings.language", FieldValue::String("es".to_string()));

    let result = handle.validate();
    assert!(result.is_ok()); // Valid form should pass

    // Test field access
    let first_name = handle.get_field_value("user.first_name");
    assert_eq!(first_name, FieldValue::String("John".to_string()));

    let theme = handle.get_field_value("settings.theme");
    assert_eq!(theme, FieldValue::String("dark".to_string()));

    let notifications = handle.get_field_value("settings.notifications");
    assert_eq!(notifications, FieldValue::Boolean(false));
}

#[test]
fn test_consumer_with_hooks() {
    // Test that consumers can use the library's hooks
    let (form_handle, _submit, _reset) = use_form(ConsumerForm::default());

    // Test field value hook
    let name_value = use_field_value(&form_handle, "name");
    assert_eq!(
        name_value.get_untracked(),
        FieldValue::String(String::new())
    );

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
fn test_consumer_metadata_access() {
    let metadata = ConsumerForm::field_metadata();

    // Verify all expected fields are present
    let field_names: Vec<String> = metadata.iter().map(|f| f.name.clone()).collect();
    assert!(field_names.contains(&"name".to_string()));
    assert!(field_names.contains(&"email".to_string()));
    assert!(field_names.contains(&"age".to_string()));
    assert!(field_names.contains(&"preferences".to_string()));
    assert!(field_names.contains(&"newsletter".to_string()));

    // Verify field types
    let name_field = metadata.iter().find(|f| f.name == "name").unwrap();
    assert_eq!(name_field.field_type, FieldType::Text);
    assert!(name_field.is_required);

    let email_field = metadata.iter().find(|f| f.name == "email").unwrap();
    assert_eq!(email_field.field_type, FieldType::Email);
    assert!(email_field.is_required);

    let age_field = metadata.iter().find(|f| f.name == "age").unwrap();
    assert!(matches!(age_field.field_type, FieldType::Number(_)));
    assert!(!age_field.is_required);

    let preferences_field = metadata.iter().find(|f| f.name == "preferences").unwrap();
    assert!(matches!(preferences_field.field_type, FieldType::Array(_)));
    assert!(!preferences_field.is_required);

    let newsletter_field = metadata.iter().find(|f| f.name == "newsletter").unwrap();
    assert_eq!(newsletter_field.field_type, FieldType::Boolean);
    assert!(!newsletter_field.is_required);
}

#[test]
fn test_consumer_schema_generation() {
    let schema = ConsumerForm::schema();

    // Verify schema structure
    assert!(!schema.name.is_empty());
    assert_eq!(schema.field_metadata.len(), 5);

    // Test get_field method
    let name_field = schema.get_field("name");
    assert!(name_field.is_some());
    assert_eq!(name_field.unwrap().name, "name");

    let nonexistent_field = schema.get_field("nonexistent");
    assert!(nonexistent_field.is_none());
}
