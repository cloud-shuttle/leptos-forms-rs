//! Leptos Forms - A comprehensive form management library for Leptos
//! 
//! This library provides a complete solution for building forms in Leptos applications,
//! including form state management, validation, field arrays, and more.

pub mod core;
pub mod validation;
pub mod error;
pub mod hooks;
pub mod components;
pub mod utils;

// Re-export core types and traits
pub use core::{Form, FormHandle, FieldMetadata, FormSchema};
pub use core::types::FormState;

// Re-export validation types
pub use validation::{ValidationErrors, Validators, validate_form};

// Re-export error types
pub use error::{FormError, FieldError};

// Re-export hooks
pub use hooks::{use_form, use_field_value, use_field_error, use_field_dirty, use_field_touched, use_form_validation, use_form_submission, use_form_persistence, use_form_analytics, use_field_array, use_form_wizard, use_real_time_validation, use_conditional_validation, use_form_performance, FieldArrayHandle};

// Re-export components
pub use components::{Form as FormComponent, FormField, FormReset, FormDebug, FieldArray, form_wizard as FormWizard};

// Re-export utility functions
pub use utils::{form_to_map, map_to_form, merge_validation_errors, has_validation_errors, get_form_field_names, get_required_field_names, is_field_required, get_field_type, validate_field_value, serialize_form, deserialize_form, form_from_json, form_to_json, forms_are_equal, get_form_stats, validate_form_detailed, FormStats, FormValidationResult};
