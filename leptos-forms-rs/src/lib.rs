//! Leptos Forms - A comprehensive form management library for Leptos
//!
//! This library provides a complete solution for building forms in Leptos applications,
//! including form state management, validation, field arrays, event handling, and more.
//!
//! ## Features
//!
//! - **Type-safe forms** with compile-time validation
//! - **Reactive state management** using Leptos signals
//! - **Event handling system** with comprehensive form and field events
//! - **Hook system** with React-like hooks for form state management
//! - **Component integration** with pre-built form components
//! - **Field arrays and dynamic forms** support
//! - **Form persistence** with localStorage support
//! - **WASM-powered** for high performance
//!
//! ## Quick Start
//!
//! ```rust
//! use leptos::*;
//! use leptos_forms_rs::*;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
//! struct LoginForm {
//!     email: String,
//!     password: String,
//! }
//!
//! impl Form for LoginForm {
//!     fn default_values() -> Self {
//!         LoginForm {
//!             email: "".to_string(),
//!             password: "".to_string(),
//!         }
//!     }
//!
//!     fn field_metadata() -> Vec<FieldMetadata> {
//!         vec![
//!             FieldMetadata {
//!                 field_name: "email".to_string(),
//!                 field_type: FieldType::Email,
//!                 ..Default::default()
//!             },
//!             FieldMetadata {
//!                 field_name: "password".to_string(),
//!                 field_type: FieldType::Password,
//!                 ..Default::default()
//!             }
//!         ]
//!     }
//!
//!     fn validate(&self) -> Result<(), ValidationErrors> { Ok(()) }
//! }
//!
//! #[component]
//! pub fn LoginPage() -> impl IntoView {
//!     let (form, _submit_callback, _reset_callback) = use_form(LoginForm::default_values());
//!
//!     view! {
//!         <div class="form">
//!             <FormField form=form.clone() name="email".to_string() />
//!             <FormField form=form.clone() name="password".to_string() />
//!         </div>
//!     }
//! }
//! ```

pub mod components;
pub mod core;
pub mod devtools;
pub mod error;
pub mod events;
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

// Re-export events
pub use events::{
    EventSystem, EventSystemBuilder, EventSystemUtils, FieldBlurHandler, FieldChangeHandler,
    FieldEvent, FieldEventType, FieldFocusHandler, FieldInputHandler, FormEvent, FormEventType,
    FormResetHandler, FormSubmitHandler,
};

// Re-export devtools
pub use devtools::{
    DebugUtilities, FieldChange, FieldState, FormSnapshot, FormStateInspector, FormStateSnapshot,
    IntegrityCheck, PerformanceMetrics, PerformanceMonitor, SnapshotDiff,
};
