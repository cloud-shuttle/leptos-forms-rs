// Required for Leptos 0.8+ to handle complex type resolution
#![recursion_limit = "256"]

//! Leptos Forms - Type-safe, reactive form handling library for Leptos applications
//! 
//! This library provides a comprehensive form handling solution with:
//! - Type-safe form definitions with procedural macros
//! - Reactive form state management
//! - Built-in validation system
//! - Accessibility-first design
//! - Performance optimized for WASM
//! - **NEW**: Full compatibility with Leptos 0.6 and 0.8
//! 
//! # Quick Start
//! 
//! ```rust,ignore
//! use leptos::*;
//! use leptos_forms_rs::*;
//! 
//! #[derive(Form, Clone, Serialize, Deserialize)]
//! struct LoginForm {
//!     #[form(required, email)]
//!     email: String,
//!     #[form(required, min_length = 8)]
//!     password: String,
//!     #[form(default = true)]
//!     remember_me: bool,
//! }
//! 
//! #[component]
//! fn LoginPage() -> impl IntoView {
//!     let form = use_form::<LoginForm>();
//!     
//!         <Form form=form>
//!             <FormField name="email" />
//!             <FormField name="password" input_type="password" />
//!             <FormField name="remember_me" />
//!             <button type="submit">"Login"</button>
//!         </Form>
//!     }
//! }
//! ```
//! 
//! # Leptos Version Compatibility
//! 
//! This library now supports both Leptos 0.6 and 0.8 through feature flags:
//! 
//! ```toml
//! # For Leptos 0.6 (default)
//! leptos-forms-rs = { version = "0.1", features = ["leptos-0-6"] }
//! 
//! # For Leptos 0.8
//! leptos-forms-rs = { version = "0.1", features = ["leptos-0-8"] }
//! ```
//! 
//! The compatibility layer automatically handles:
//! - Signal API differences (Scope parameter removal in 0.8)
//! - Component definition patterns
//! - Effect lifecycle management
//! - Resource serialization requirements
//! - Context API evolution

pub mod core;
pub mod validation;
pub mod hooks;
pub mod components;
pub mod error;
pub mod utils;
pub mod compat;

// Re-export main types for convenience
pub use core::{Form, FormHandle, FormState, FieldMetadata, FormSchema};
pub use validation::{ValidationErrors, Validators, register_validator, validate_custom};
pub use hooks::{use_form, use_form_with_values, use_field_value, use_field_error, use_form_validation, use_field_array, use_form_wizard, FieldArrayHandle, FormWizardHandle};
pub use components::{Form as FormComponent, FormField, FormErrors, FormSubmit, FormReset, FormProgress, FormDebug};
pub use error::{FormError, FieldError, ErrorContext, FormResult};

// Re-export the Form derive macro
pub use leptos_forms_rs_macro::Form;

// Re-export compatibility layer
pub use compat::prelude as compat;

// Re-export common Leptos types based on version
#[cfg(feature = "leptos-0-6")]
pub use leptos_06 as leptos;

#[cfg(feature = "leptos-0-8")]
pub use leptos_08 as leptos;

// Re-export serde for form serialization
pub use serde::{Deserialize, Serialize};

/// Prelude module for common imports
pub mod prelude {
    pub use crate::{Form, FormHandle, FormState, FieldMetadata, FormSchema};
    pub use crate::{ValidationErrors, Validators, register_validator, validate_custom};
    pub use crate::{use_form, use_form_with_values, use_field_value, use_field_error, use_form_validation};
    pub use crate::{FormComponent, FormField, FormErrors, FormSubmit, FormReset, FormProgress, FormDebug};
    pub use crate::{FormError, FieldError, ErrorContext, FormResult};
    
    // Re-export Leptos based on version
    #[cfg(feature = "leptos-0-6")]
    pub use leptos_06::*;
    
    #[cfg(feature = "leptos-0-8")]
    pub use leptos_08::prelude::*;
    
    pub use serde::{Deserialize, Serialize};
    
    // Re-export compatibility layer
    pub use crate::compat::*;
}
