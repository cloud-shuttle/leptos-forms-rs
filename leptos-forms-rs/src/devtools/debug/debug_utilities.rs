//! Debug Utilities implementation
//!
//! This module provides debug utilities for form inspection and debugging.

use crate::core::{Form, FormHandle};
use crate::core::types::FieldValue;
use leptos::prelude::GetUntracked;
use std::collections::HashMap;
use std::time::SystemTime;

use super::super::types::{FieldChange, FormSnapshot, IntegrityCheck, SnapshotDiff};

/// Debug Utilities for DevTools
pub struct DebugUtilities;

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
