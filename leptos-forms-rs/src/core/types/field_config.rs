//! Field configuration module - Field configuration and metadata
//!
//! This module provides FieldConfig for field configuration, FieldMetadata for
//! field metadata, and related configuration types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::field_types::FieldType;
use super::validator_config::ValidatorConfig;

/// Form field configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldConfig {
    pub name: String,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
    pub disabled: bool,
    pub readonly: bool,
    pub hidden: bool,
    pub attributes: HashMap<String, String>,
}

impl FieldConfig {
    /// Create a new field configuration
    pub fn new(name: String) -> Self {
        Self {
            name,
            label: None,
            placeholder: None,
            help_text: None,
            disabled: false,
            readonly: false,
            hidden: false,
            attributes: HashMap::new(),
        }
    }

    /// Set the field label
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the field placeholder
    pub fn with_placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Set the help text
    pub fn with_help_text(mut self, help_text: String) -> Self {
        self.help_text = Some(help_text);
        self
    }

    /// Mark the field as disabled
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Mark the field as readonly
    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }

    /// Mark the field as hidden
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    /// Add a custom attribute
    pub fn with_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }

    /// Get the field name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the field label
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Get the field placeholder
    pub fn placeholder(&self) -> Option<&str> {
        self.placeholder.as_deref()
    }

    /// Get the help text
    pub fn help_text(&self) -> Option<&str> {
        self.help_text.as_deref()
    }

    /// Check if the field is disabled
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Check if the field is readonly
    pub fn is_readonly(&self) -> bool {
        self.readonly
    }

    /// Check if the field is hidden
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    /// Check if the field is visible
    pub fn is_visible(&self) -> bool {
        !self.hidden
    }

    /// Check if the field is editable
    pub fn is_editable(&self) -> bool {
        !self.disabled && !self.readonly
    }

    /// Get a custom attribute
    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    /// Get all custom attributes
    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }

    /// Check if the field has a specific attribute
    pub fn has_attribute(&self, key: &str) -> bool {
        self.attributes.contains_key(key)
    }

    /// Get the display label (fallback to name if label is not set)
    pub fn display_label(&self) -> &str {
        self.label.as_deref().unwrap_or(&self.name)
    }
}
