//! Form Wizard Example Module - Example implementation of form wizard
//!
//! This module provides a comprehensive example of how to use the FormWizard
//! component with step-by-step form navigation, validation, and completion handling.

pub mod form_types;
pub mod step_components;
pub mod styles;
pub mod wizard_component;

// Re-export public API
pub use form_types::*;
pub use step_components::*;
pub use styles::*;
pub use wizard_component::*;
