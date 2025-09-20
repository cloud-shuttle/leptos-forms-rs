//! Components module - UI components for the Leptos Forms library
//!
//! This module provides a comprehensive set of UI components for building forms
//! with the Leptos Forms library, including core form components, input components,
//! specialized components, and example implementations.

pub mod checkbox;
pub mod code_input;
pub mod core;
pub mod examples;
pub mod field;
// pub mod field_array; // Temporarily disabled due to compilation issues
pub mod file_input;
pub mod file_upload_input;
pub mod form_wizard;
pub mod input;
pub mod markdown_input;
pub mod radio;
pub mod rich_text_input;
pub mod select;
pub mod textarea;

// Re-export core components
pub use core::*;

// Re-export specialized components
pub use code_input::CodeInput;
// pub use field_array::FieldArray; // Temporarily disabled due to compilation issues
pub use file_upload_input::FileUploadInput;
pub use input::*;
pub use markdown_input::MarkdownInput;
pub use rich_text_input::RichTextInput;

// Re-export examples
pub use examples::*;
