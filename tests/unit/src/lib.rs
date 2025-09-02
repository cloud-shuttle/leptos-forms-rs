//! Unit tests for Leptos Forms RS
//! 
//! This module contains unit tests for the core functionality of the library.

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// Test modules
mod form_handle;
mod form_validation;
mod form_components;
mod form_hooks;
mod form_types;

// Re-export test modules for external access
pub mod form_handle {
    pub use super::form_handle::*;
}

pub mod form_validation {
    pub use super::form_validation::*;
}

pub mod form_components {
    pub use super::form_components::*;
}

pub mod form_hooks {
    pub use super::form_hooks::*;
}

pub mod form_types {
    pub use super::form_types::*;
}

// Main test runner
#[wasm_bindgen_test]
fn run_unit_tests() {
    // This function will run all unit tests
    // Individual test modules are run separately
}
