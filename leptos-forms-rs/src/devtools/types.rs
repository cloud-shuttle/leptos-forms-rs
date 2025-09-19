//! DevTools types and data structures
//!
//! This module provides the core data structures used by the DevTools system
//! for form inspection, performance monitoring, and debugging.

use crate::core::types::FieldValue;
use crate::validation::ValidationErrors;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Type alias for form state change listeners
pub type ChangeListener = Box<dyn Fn(&FormStateSnapshot) + Send + Sync>;

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
