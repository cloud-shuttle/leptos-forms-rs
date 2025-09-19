//! Field types module - Field type definitions and constraints
//!
//! This module provides the FieldType enum and related types including
//! NumberType, SelectOption, FileConstraints, and field type validation.

use serde::{Deserialize, Serialize};

/// Supported field types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldType {
    Text,
    Email,
    Password,
    Number(NumberType),
    Boolean,
    Select(Vec<SelectOption>),
    MultiSelect(Vec<SelectOption>),
    Date,
    DateTime,
    File(FileConstraints),
    RichText,
    Markdown,
    Code,
    Array(Box<FieldType>),
    Nested(String), // Type name for nested forms
}

impl FieldType {
    /// Check if this field type is a text input type
    pub fn is_text_input(&self) -> bool {
        matches!(
            self,
            FieldType::Text | FieldType::Email | FieldType::Password
        )
    }

    /// Check if this field type is a number input type
    pub fn is_number_input(&self) -> bool {
        matches!(self, FieldType::Number(_))
    }

    /// Check if this field type is a select type
    pub fn is_select_type(&self) -> bool {
        matches!(self, FieldType::Select(_) | FieldType::MultiSelect(_))
    }

    /// Check if this field type is a date type
    pub fn is_date_type(&self) -> bool {
        matches!(self, FieldType::Date | FieldType::DateTime)
    }

    /// Check if this field type is a file type
    pub fn is_file_type(&self) -> bool {
        matches!(self, FieldType::File(_))
    }

    /// Check if this field type is an array type
    pub fn is_array_type(&self) -> bool {
        matches!(self, FieldType::Array(_))
    }

    /// Check if this field type is a nested type
    pub fn is_nested_type(&self) -> bool {
        matches!(self, FieldType::Nested(_))
    }

    /// Get the inner type for array fields
    pub fn array_inner_type(&self) -> Option<&FieldType> {
        match self {
            FieldType::Array(inner) => Some(inner),
            _ => None,
        }
    }

    /// Get the nested type name for nested fields
    pub fn nested_type_name(&self) -> Option<&str> {
        match self {
            FieldType::Nested(name) => Some(name),
            _ => None,
        }
    }

    /// Get the number type configuration
    pub fn number_type(&self) -> Option<&NumberType> {
        match self {
            FieldType::Number(number_type) => Some(number_type),
            _ => None,
        }
    }

    /// Get the select options
    pub fn select_options(&self) -> Option<&Vec<SelectOption>> {
        match self {
            FieldType::Select(options) | FieldType::MultiSelect(options) => Some(options),
            _ => None,
        }
    }

    /// Get the file constraints
    pub fn file_constraints(&self) -> Option<&FileConstraints> {
        match self {
            FieldType::File(constraints) => Some(constraints),
            _ => None,
        }
    }

    /// Check if this field type supports multiple values
    pub fn supports_multiple_values(&self) -> bool {
        matches!(self, FieldType::MultiSelect(_) | FieldType::Array(_))
    }

    /// Get a human-readable name for the field type
    pub fn display_name(&self) -> &'static str {
        match self {
            FieldType::Text => "Text",
            FieldType::Email => "Email",
            FieldType::Password => "Password", // pragma: allowlist secret
            FieldType::Number(_) => "Number",
            FieldType::Boolean => "Boolean",
            FieldType::Select(_) => "Select",
            FieldType::MultiSelect(_) => "Multi-Select",
            FieldType::Date => "Date",
            FieldType::DateTime => "Date & Time",
            FieldType::File(_) => "File",
            FieldType::RichText => "Rich Text",
            FieldType::Markdown => "Markdown",
            FieldType::Code => "Code",
            FieldType::Array(_) => "Array",
            FieldType::Nested(_) => "Nested Form",
        }
    }
}

/// Number field configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberType {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
}

impl NumberType {
    /// Create a new number type with default values
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
            step: None,
        }
    }

    /// Set the minimum value
    pub fn with_min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// Set the maximum value
    pub fn with_max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// Set the step value
    pub fn with_step(mut self, step: f64) -> Self {
        self.step = Some(step);
        self
    }

    /// Set both min and max values
    pub fn with_range(mut self, min: f64, max: f64) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    /// Check if a value is within the valid range
    pub fn is_valid_value(&self, value: f64) -> bool {
        if let Some(min) = self.min {
            if value < min {
                return false;
            }
        }
        if let Some(max) = self.max {
            if value > max {
                return false;
            }
        }
        true
    }

    /// Get the range as a tuple
    pub fn range(&self) -> (Option<f64>, Option<f64>) {
        (self.min, self.max)
    }

    /// Check if this number type has constraints
    pub fn has_constraints(&self) -> bool {
        self.min.is_some() || self.max.is_some() || self.step.is_some()
    }
}

impl Default for NumberType {
    fn default() -> Self {
        Self::new()
    }
}

/// Select option configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectOption {
    /// Create a new select option
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    /// Create a disabled select option
    pub fn disabled(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: true,
        }
    }

    /// Check if this option is enabled
    pub fn is_enabled(&self) -> bool {
        !self.disabled
    }

    /// Check if this option is disabled
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Get the display label (fallback to value if label is empty)
    pub fn display_label(&self) -> &str {
        if self.label.is_empty() {
            &self.value
        } else {
            &self.label
        }
    }
}

/// File upload constraints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileConstraints {
    pub max_size: Option<usize>,
    pub accept: Vec<String>,
    pub multiple: bool,
}

impl FileConstraints {
    /// Create new file constraints
    pub fn new() -> Self {
        Self {
            max_size: None,
            accept: Vec::new(),
            multiple: false,
        }
    }

    /// Set the maximum file size in bytes
    pub fn with_max_size(mut self, max_size: usize) -> Self {
        self.max_size = Some(max_size);
        self
    }

    /// Add an accepted file type
    pub fn with_accept(mut self, accept: impl Into<String>) -> Self {
        self.accept.push(accept.into());
        self
    }

    /// Add multiple accepted file types
    pub fn with_accept_types(mut self, accept: Vec<String>) -> Self {
        self.accept.extend(accept);
        self
    }

    /// Enable multiple file selection
    pub fn with_multiple(mut self) -> Self {
        self.multiple = true;
        self
    }

    /// Check if a file type is accepted
    pub fn accepts_file_type(&self, file_type: &str) -> bool {
        if self.accept.is_empty() {
            return true; // No restrictions
        }
        self.accept.iter().any(|accept| {
            if accept.ends_with("/*") {
                let prefix = &accept[..accept.len() - 2];
                file_type.starts_with(prefix)
            } else {
                file_type == accept
            }
        })
    }

    /// Check if a file size is within limits
    pub fn is_valid_size(&self, size: usize) -> bool {
        if let Some(max_size) = self.max_size {
            size <= max_size
        } else {
            true
        }
    }

    /// Check if multiple files are allowed
    pub fn allows_multiple(&self) -> bool {
        self.multiple
    }

    /// Get the maximum file size in bytes
    pub fn max_size_bytes(&self) -> Option<usize> {
        self.max_size
    }

    /// Get the maximum file size in MB
    pub fn max_size_mb(&self) -> Option<f64> {
        self.max_size.map(|bytes| bytes as f64 / 1_048_576.0)
    }

    /// Get all accepted file types
    pub fn accepted_types(&self) -> &[String] {
        &self.accept
    }
}

impl Default for FileConstraints {
    fn default() -> Self {
        Self::new()
    }
}
