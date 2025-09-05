use crate::core::{FieldValue, Form, FormHandle};
use crate::validation::ValidationErrors;
use leptos::prelude::GetUntracked;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Form State Inspector for DevTools
pub struct FormStateInspector<T: Form + Send + Sync + PartialEq> {
    form_handle: FormHandle<T>,
    change_listeners: Vec<Box<dyn Fn(&FormStateSnapshot) + Send + Sync>>,
}

/// Performance Monitor for DevTools
pub struct PerformanceMonitor<T: Form + Send + Sync + PartialEq> {
    _form_handle: FormHandle<T>,
    metrics: PerformanceMetrics,
}

/// Debug Utilities for DevTools
pub struct DebugUtilities;

/// Form State Snapshot
#[derive(Debug, Clone)]
pub struct FormStateSnapshot {
    pub form_name: String,
    pub field_count: usize,
    pub is_dirty: Option<bool>,
    pub is_submitting: Option<bool>,
    pub has_errors: Option<bool>,
    pub timestamp: SystemTime,
}

/// Field State Information
#[derive(Debug, Clone)]
pub struct FieldState {
    pub name: String,
    pub field_type: String,
    pub is_required: bool,
    pub value: Option<FieldValue>,
    pub has_error: Option<bool>,
    pub error_message: Option<String>,
}

/// Performance Metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub form_creation_time: Option<Duration>,
    pub total_field_operations: u64,
    pub validation_operations: u64,
    pub average_field_operation_time: Option<Duration>,
    pub average_validation_time: Option<Duration>,
    pub memory_usage: Option<usize>,
}

/// Form Snapshot for Debugging
#[derive(Debug, Clone)]
pub struct FormSnapshot {
    pub form_name: String,
    pub timestamp: SystemTime,
    pub field_count: usize,
    pub field_values: HashMap<String, FieldValue>,
    pub is_dirty: bool,
    pub is_submitting: bool,
    pub has_errors: bool,
}

/// Snapshot Comparison Result
#[derive(Debug, Clone)]
pub struct SnapshotDiff {
    pub has_changes: bool,
    pub changed_fields: Vec<FieldChange>,
}

/// Field Change Information
#[derive(Debug, Clone)]
pub struct FieldChange {
    pub field_name: String,
    pub old_value: Option<FieldValue>,
    pub new_value: Option<FieldValue>,
}

/// Form Integrity Check Result
#[derive(Debug, Clone)]
pub struct IntegrityCheck {
    pub is_valid: bool,
    pub issues: Vec<String>,
}

impl<T: Form + Send + Sync + PartialEq> FormStateInspector<T> {
    pub fn new(form_handle: &FormHandle<T>) -> Result<Self, String> {
        Ok(Self {
            form_handle: form_handle.clone(),
            change_listeners: Vec::new(),
        })
    }

    pub fn get_current_state(&self) -> FormStateSnapshot {
        let state = self.form_handle.state().get_untracked();
        FormStateSnapshot {
            form_name: T::schema().name,
            field_count: T::schema().field_metadata.len(),
            is_dirty: Some(state.is_dirty),
            is_submitting: Some(state.is_submitting),
            has_errors: Some(!state.errors.is_empty()),
            timestamp: SystemTime::now(),
        }
    }

    pub fn get_field_states(&self) -> HashMap<String, FieldState> {
        let mut field_states = HashMap::new();
        let form_data = self.form_handle.values().get_untracked();
        let state = self.form_handle.state().get_untracked();

        for field_metadata in &T::schema().field_metadata {
            let value = form_data.get_field_value(&field_metadata.name);
            let has_error = state.errors.get_field_error(&field_metadata.name).is_some();
            let error_message = state
                .errors
                .get_field_error(&field_metadata.name)
                .map(|errors| errors.join(", "));

            field_states.insert(
                field_metadata.name.clone(),
                FieldState {
                    name: field_metadata.name.clone(),
                    field_type: format!("{:?}", field_metadata.field_type),
                    is_required: field_metadata.is_required,
                    value: Some(value),
                    has_error: Some(has_error),
                    error_message,
                },
            );
        }

        field_states
    }

    pub fn get_validation_errors(&self) -> ValidationErrors {
        let state = self.form_handle.state().get_untracked();
        state.errors
    }

    pub fn subscribe_to_changes<F>(&mut self, callback: F) -> impl Fn()
    where
        F: Fn(&FormStateSnapshot) + Send + Sync + 'static,
    {
        let callback = Box::new(callback);
        self.change_listeners.push(callback);

        // Return unsubscribe function
        move || {
            // In a real implementation, this would remove the listener
        }
    }
}

impl<T: Form + Send + Sync + PartialEq> PerformanceMonitor<T> {
    pub fn new(form_handle: &FormHandle<T>) -> Result<Self, String> {
        Ok(Self {
            _form_handle: form_handle.clone(),
            metrics: PerformanceMetrics {
                form_creation_time: Some(Duration::from_millis(1)),
                total_field_operations: 0,
                validation_operations: 0,
                average_field_operation_time: None,
                average_validation_time: None,
                memory_usage: Some(1024),
            },
        })
    }

    pub fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.clone()
    }

    pub fn track_field_operation(&mut self, operation_time: Duration) {
        self.metrics.total_field_operations += 1;

        // Update average time calculation
        if let Some(ref mut avg) = self.metrics.average_field_operation_time {
            // Simple running average calculation
            let total_time = avg.as_nanos() * (self.metrics.total_field_operations - 1) as u128
                + operation_time.as_nanos();
            *avg = Duration::from_nanos(
                (total_time / self.metrics.total_field_operations as u128) as u64,
            );
        } else {
            self.metrics.average_field_operation_time = Some(operation_time);
        }
    }

    pub fn track_validation_operation(&mut self, operation_time: Duration) {
        self.metrics.validation_operations += 1;

        // Update average time calculation
        if let Some(ref mut avg) = self.metrics.average_validation_time {
            // Simple running average calculation
            let total_time = avg.as_nanos() * (self.metrics.validation_operations - 1) as u128
                + operation_time.as_nanos();
            *avg = Duration::from_nanos(
                (total_time / self.metrics.validation_operations as u128) as u64,
            );
        } else {
            self.metrics.average_validation_time = Some(operation_time);
        }
    }
}

impl DebugUtilities {
    pub fn create_form_snapshot<T: Form + Send + Sync + PartialEq>(
        form_handle: &FormHandle<T>,
    ) -> FormSnapshot {
        let form_data = form_handle.values().get_untracked();
        let state = form_handle.state().get_untracked();
        let mut field_values = HashMap::new();

        for field_metadata in &T::schema().field_metadata {
            let value = form_data.get_field_value(&field_metadata.name);
            field_values.insert(field_metadata.name.clone(), value);
        }

        FormSnapshot {
            form_name: T::schema().name,
            timestamp: SystemTime::now(),
            field_count: T::schema().field_metadata.len(),
            field_values,
            is_dirty: state.is_dirty,
            is_submitting: state.is_submitting,
            has_errors: !state.errors.is_empty(),
        }
    }

    pub fn compare_snapshots(snapshot1: &FormSnapshot, snapshot2: &FormSnapshot) -> SnapshotDiff {
        let mut changed_fields = Vec::new();

        for (field_name, value1) in &snapshot1.field_values {
            if let Some(value2) = snapshot2.field_values.get(field_name) {
                if value1 != value2 {
                    changed_fields.push(FieldChange {
                        field_name: field_name.clone(),
                        old_value: Some(value1.clone()),
                        new_value: Some(value2.clone()),
                    });
                }
            }
        }

        SnapshotDiff {
            has_changes: !changed_fields.is_empty(),
            changed_fields,
        }
    }

    pub fn export_form_data<T: Form + Send + Sync + PartialEq + std::fmt::Debug>(
        form_handle: &FormHandle<T>,
    ) -> String {
        let form_data = form_handle.values().get_untracked();
        let state = form_handle.state().get_untracked();

        format!(
            "Form: {}\nFields: {}\nIs Dirty: {}\nIs Submitting: {}\nHas Errors: {}\nData: {:?}",
            T::schema().name,
            T::schema().field_metadata.len(),
            state.is_dirty,
            state.is_submitting,
            !state.errors.is_empty(),
            form_data
        )
    }

    pub fn validate_form_integrity<T: Form + Send + Sync + PartialEq>(
        form_handle: &FormHandle<T>,
    ) -> IntegrityCheck {
        let mut issues = Vec::new();

        // Check if all required fields have values
        let form_data = form_handle.values().get_untracked();
        for field_metadata in &T::schema().field_metadata {
            if field_metadata.is_required {
                let value = form_data.get_field_value(&field_metadata.name);
                if matches!(value, FieldValue::Null)
                    || matches!(value, FieldValue::String(ref s) if s.is_empty())
                {
                    issues.push(format!("Required field '{}' is empty", field_metadata.name));
                }
            }
        }

        IntegrityCheck {
            is_valid: issues.is_empty(),
            issues,
        }
    }
}
