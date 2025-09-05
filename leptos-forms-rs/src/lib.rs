//! Leptos Forms - A comprehensive form management library for Leptos
//!
//! This library provides a complete solution for building forms in Leptos applications,
//! including form state management, validation, field arrays, and more.

pub mod components;
pub mod core;
pub mod devtools;
pub mod error;
pub mod hooks;
pub mod utils;
pub mod validation;

// WASM optimizations module
#[cfg(target_arch = "wasm32")]
pub mod wasm_optimizations;

// Re-export core types and traits
pub use core::traits::FormState;
pub use core::{FieldMetadata, Form, FormHandle, FormSchema};

// Re-export validation types
pub use validation::{validate_form, ValidationErrors, Validators};

// Re-export error types
pub use error::{FieldError, FormError};

// Re-export hooks
pub use hooks::{
    use_conditional_validation, use_field_array, use_field_dirty, use_field_error,
    use_field_touched, use_field_value, use_form, use_form_analytics, use_form_performance,
    use_form_persistence, use_form_submission, use_form_validation, use_form_wizard,
    use_real_time_validation, FieldArrayHandle,
};

// Re-export components
pub use components::{
    form_wizard as FormWizard, FieldArray, Form as FormComponent, FormDebug, FormField, FormReset,
};

// Re-export utility functions
pub use utils::{
    deserialize_form, form_from_json, form_to_json, form_to_map, forms_are_equal, get_field_type,
    get_form_field_names, get_form_stats, get_required_field_names, has_validation_errors,
    is_field_required, map_to_form, merge_validation_errors, serialize_form, validate_field_value,
    validate_form_detailed, FormStats, FormValidationResult,
};

// Re-export devtools
pub use devtools::{
    DebugUtilities, FieldChange, FieldState, FormSnapshot, FormStateInspector, FormStateSnapshot,
    IntegrityCheck, PerformanceMetrics, PerformanceMonitor, SnapshotDiff,
};
