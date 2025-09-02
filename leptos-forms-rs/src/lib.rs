//! # Leptos Forms RS
//! 
//! A type-safe, reactive form handling library for Leptos applications.
//! 
//! ## Quick Start
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
//!     view! {
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
//! ## Features
//! 
//! - **Type-safe forms** with compile-time validation
//! - **Reactive state management** with Leptos signals
//! - **Built-in validation** with customizable rules
//! - **Form persistence** with localStorage support
//! - **Accessibility** with proper ARIA attributes
//! - **Mobile-friendly** with touch event handling

pub mod core;
pub mod validation;
pub mod hooks;
pub mod components;
pub mod error;
pub mod utils;

#[cfg(test)]
pub mod tests;

// Re-export main types for convenience
pub use core::{Form, FormHandle, FormState, FieldMetadata, FormSchema};
pub use validation::{ValidationErrors, Validators, register_validator, validate_custom};
pub use hooks::{use_form, use_form_with_values, use_field_value, use_field_error, use_form_validation, use_field_array, use_form_wizard, FieldArrayHandle, FormWizardHandle};
pub use components::{Form as FormComponent, FormField, FormErrors, FormSubmit, FormReset, FormProgress, FormDebug};
pub use error::{FormError, FieldError, ErrorContext, FormResult};

// Re-export the Form derive macro
pub use leptos_forms_rs_macro::Form;

// Re-export Leptos for convenience
pub use leptos;

// Re-export serde for form serialization
pub use serde::{Deserialize, Serialize};

/// Prelude module for common imports
pub mod prelude {
    pub use crate::{Form, FormHandle, FormState, FieldMetadata, FormSchema};
    pub use crate::{ValidationErrors, Validators, register_validator, validate_custom};
    pub use crate::{use_form, use_form_with_values, use_field_value, use_field_error, use_form_validation};
    pub use crate::{FormComponent, FormField, FormErrors, FormSubmit, FormReset, FormProgress, FormDebug};
    pub use crate::{FormError, FieldError, ErrorContext, FormResult};
    
    // Re-export Leptos
    pub use leptos::*;
    
    pub use serde::{Deserialize, Serialize};
}
