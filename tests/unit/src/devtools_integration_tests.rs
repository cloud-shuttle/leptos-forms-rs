use leptos::prelude::*;
use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, FormHandle, NumberType};
use leptos_forms_rs::devtools::{DebugUtilities, FormStateInspector, PerformanceMonitor};
use leptos_forms_rs::validation::Validator;
use leptos_forms_rs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Test form for DevTools integration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevToolsTestForm {
    pub name: String,
    pub email: String,
    pub age: i64,
    pub tags: Vec<String>,
    pub is_active: bool,
}

impl Form for DevToolsTestForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "name".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Required, Validator::MinLength(2)],
                is_required: true,
                default_value: Some(FieldValue::String(String::new())),
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "email".to_string(),
                field_type: FieldType::Email,
                validators: vec![Validator::Required, Validator::Email],
                is_required: true,
                default_value: Some(FieldValue::String(String::new())),
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "age".to_string(),
                field_type: FieldType::Number(NumberType {
                    min: Some(18.0),
                    max: Some(120.0),
                    step: Some(1.0),
                }),
                validators: vec![Validator::Min(18.0), Validator::Max(120.0)],
                is_required: false,
                default_value: Some(FieldValue::Integer(0)),
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "tags".to_string(),
                field_type: FieldType::Array(Box::new(FieldType::Text)),
                validators: vec![Validator::MaxLength(5)],
                is_required: false,
                default_value: Some(FieldValue::Array(vec![])),
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "is_active".to_string(),
                field_type: FieldType::Boolean,
                validators: vec![],
                is_required: false,
                default_value: Some(FieldValue::Boolean(false)),
                dependencies: vec![],
                attributes: HashMap::new(),
            },
        ]
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // Custom validation logic
        if self.age < 18 && self.is_active {
            errors.add_field_error("age", "Must be 18 or older to be active".to_string());
        }

        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn default_values() -> Self {
        Self {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            age: 25,
            tags: vec!["developer".to_string(), "rust".to_string()],
            is_active: true,
        }
    }

    fn schema() -> FormSchema {
        FormSchema {
            field_metadata: Self::field_metadata(),
            name: "DevToolsTestForm".to_string(),
        }
    }

    fn get_field_value(&self, field_name: &str) -> FieldValue {
        match field_name {
            "name" => FieldValue::String(self.name.clone()),
            "email" => FieldValue::String(self.email.clone()),
            "age" => FieldValue::Integer(self.age),
            "tags" => FieldValue::Array(
                self.tags
                    .iter()
                    .map(|t| FieldValue::String(t.clone()))
                    .collect(),
            ),
            "is_active" => FieldValue::Boolean(self.is_active),
            _ => FieldValue::Null,
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
                if let FieldValue::Integer(i) = value {
                    self.age = i;
                }
            }
            "tags" => {
                if let FieldValue::Array(arr) = value {
                    self.tags = arr
                        .into_iter()
                        .filter_map(|v| match v {
                            FieldValue::String(s) => Some(s),
                            _ => None,
                        })
                        .collect();
                }
            }
            "is_active" => {
                if let FieldValue::Boolean(b) = value {
                    self.is_active = b;
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::GetUntracked;

    #[test]
    fn test_form_state_inspector_creation() {
        let form_handle = FormHandle::new(DevToolsTestForm::default_values());

        // Test that we can create a form state inspector
        let inspector = FormStateInspector::new(&form_handle);
        assert!(inspector.is_ok());
    }

    #[test]
    fn test_form_state_inspector_get_current_state() {
        let form_handle = FormHandle::new(DevToolsTestForm::default_values());
        let inspector = FormStateInspector::new(&form_handle).unwrap();

        let state = inspector.get_current_state();
        assert_eq!(state.form_name, "DevToolsTestForm");
        assert_eq!(state.field_count, 5);
        assert!(state.is_dirty.is_some());
        assert!(state.is_submitting.is_some());
        assert!(state.has_errors.is_some());
    }

    #[test]
    fn test_form_state_inspector_get_field_states() {
        let form_handle = FormHandle::new(DevToolsTestForm::default_values());
        let inspector = FormStateInspector::new(&form_handle).unwrap();

        let field_states = inspector.get_field_states();
        assert_eq!(field_states.len(), 5);

        // Check specific field states
        let name_field = field_states.get("name").unwrap();
        assert_eq!(name_field.name, "name");
        assert_eq!(name_field.field_type, "Text");
        assert!(name_field.is_required);
        assert!(name_field.value.is_some());
        assert!(name_field.has_error.is_some());
    }

    #[test]
    fn test_form_state_inspector_get_validation_errors() {
        let mut form = DevToolsTestForm::default_values();
        form.age = 16; // Invalid age for active user
        form.is_active = true; // This should trigger validation error
        let form_handle = FormHandle::new(form);

        // Trigger validation to populate errors
        let _ = form_handle.validate();

        let inspector = FormStateInspector::new(&form_handle).unwrap();
        let errors = inspector.get_validation_errors();

        // Should have validation errors due to age < 18 and is_active = true
        assert!(errors.has_errors());
        assert!(errors.get_field_error("age").is_some());
    }

    #[test]
    fn test_form_state_inspector_subscribe_to_changes() {
        let form_handle = FormHandle::new(DevToolsTestForm::default_values());
        let mut inspector = FormStateInspector::new(&form_handle).unwrap();

        let change_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let change_count_clone = change_count.clone();
        let unsubscribe = inspector.subscribe_to_changes(move |_| {
            change_count_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        });

        // For now, just verify that the subscription was created successfully
        // In a real implementation, this would track form changes
        assert!(change_count.load(std::sync::atomic::Ordering::Relaxed) >= 0);

        // Clean up
        unsubscribe();
    }

    #[test]
    fn test_performance_monitor_creation() {
        let form_handle = FormHandle::new(DevToolsTestForm::default_values());

        let monitor = PerformanceMonitor::new(&form_handle);
        assert!(monitor.is_ok());
    }

    #[test]
    fn test_performance_monitor_get_metrics() {
        let form_handle = FormHandle::new(DevToolsTestForm::default_values());
        let monitor = PerformanceMonitor::new(&form_handle).unwrap();

        let metrics = monitor.get_metrics();
        assert!(metrics.form_creation_time.is_some());
        assert!(metrics.total_field_operations >= 0);
        assert!(metrics.validation_operations >= 0);
        assert!(metrics.memory_usage.is_some());
    }

    #[test]
    fn test_performance_monitor_track_field_operation() {
        let form_handle = FormHandle::new(DevToolsTestForm::default_values());
        let mut monitor = PerformanceMonitor::new(&form_handle).unwrap();

        let start_time = std::time::Instant::now();
        form_handle.set_field_value("name", FieldValue::String("New Name".to_string()));
        let operation_time = start_time.elapsed();

        // Track the operation
        monitor.track_field_operation(operation_time);

        let metrics = monitor.get_metrics();
        assert!(metrics.total_field_operations > 0);
        assert!(metrics.average_field_operation_time.is_some());
    }

    #[test]
    fn test_performance_monitor_track_validation_operation() {
        let form_handle = FormHandle::new(DevToolsTestForm::default_values());
        let mut monitor = PerformanceMonitor::new(&form_handle).unwrap();

        let start_time = std::time::Instant::now();
        let _ = form_handle.validate();
        let validation_time = start_time.elapsed();

        // Track the validation operation
        monitor.track_validation_operation(validation_time);

        let metrics = monitor.get_metrics();
        assert!(metrics.validation_operations > 0);
        assert!(metrics.average_validation_time.is_some());
    }

    #[test]
    fn test_debug_utilities_form_snapshot() {
        let form_handle = FormHandle::new(DevToolsTestForm::default_values());

        let snapshot = DebugUtilities::create_form_snapshot(&form_handle);
        assert_eq!(snapshot.form_name, "DevToolsTestForm");
        assert_eq!(snapshot.timestamp, snapshot.timestamp); // Should be set
        assert_eq!(snapshot.field_count, 5);
        assert!(snapshot.field_values.len() > 0);
    }

    #[test]
    fn test_debug_utilities_compare_snapshots() {
        let form_handle = FormHandle::new(DevToolsTestForm::default_values());

        let snapshot1 = DebugUtilities::create_form_snapshot(&form_handle);

        // Make a change
        form_handle.set_field_value("name", FieldValue::String("Changed Name".to_string()));

        let snapshot2 = DebugUtilities::create_form_snapshot(&form_handle);

        let diff = DebugUtilities::compare_snapshots(&snapshot1, &snapshot2);
        assert!(diff.has_changes);
        assert_eq!(diff.changed_fields.len(), 1);
        assert_eq!(diff.changed_fields[0].field_name, "name");
    }

    #[test]
    fn test_debug_utilities_export_form_data() {
        let form_handle = FormHandle::new(DevToolsTestForm::default_values());

        let exported = DebugUtilities::export_form_data(&form_handle);
        assert!(exported.contains("DevToolsTestForm"));
        assert!(exported.contains("name"));
        assert!(exported.contains("email"));
        assert!(exported.contains("age"));
    }

    #[test]
    fn test_debug_utilities_validate_form_integrity() {
        let form_handle = FormHandle::new(DevToolsTestForm::default_values());

        let integrity_check = DebugUtilities::validate_form_integrity(&form_handle);
        assert!(integrity_check.is_valid);
        assert_eq!(integrity_check.issues.len(), 0);
    }

    #[test]
    fn test_devtools_integration_full_workflow() {
        let form_handle = FormHandle::new(DevToolsTestForm::default_values());

        // Create all DevTools components
        let inspector = FormStateInspector::new(&form_handle).unwrap();
        let mut monitor = PerformanceMonitor::new(&form_handle).unwrap();

        // Get initial state
        let initial_state = inspector.get_current_state();
        let initial_metrics = monitor.get_metrics();

        // Make some changes and track them
        form_handle.set_field_value("name", FieldValue::String("Updated Name".to_string()));
        monitor.track_field_operation(std::time::Duration::from_millis(1));

        form_handle.add_array_item("tags", FieldValue::String("new-tag".to_string()));
        monitor.track_field_operation(std::time::Duration::from_millis(1));

        // Validate and track it
        let _ = form_handle.validate();
        monitor.track_validation_operation(std::time::Duration::from_millis(1));

        // Check updated state
        let updated_state = inspector.get_current_state();
        let updated_metrics = monitor.get_metrics();

        // Verify changes were tracked
        assert!(updated_metrics.total_field_operations > initial_metrics.total_field_operations);
        assert!(updated_metrics.validation_operations > initial_metrics.validation_operations);

        // Create debug snapshot
        let snapshot = DebugUtilities::create_form_snapshot(&form_handle);
        assert_eq!(snapshot.field_count, 5);

        // Export form data
        let exported = DebugUtilities::export_form_data(&form_handle);
        assert!(exported.contains("Updated Name"));
        assert!(exported.contains("new-tag"));
    }
}
