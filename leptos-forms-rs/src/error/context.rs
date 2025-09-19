//! Error context module - Error context and debugging information
//!
//! This module provides ErrorContext for collecting debugging information,
//! context building methods, debugging information collection,
//! and timestamp and metadata handling.

use std::collections::HashMap;

/// Error context for additional debugging information
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub form_name: Option<String>,
    pub field_name: Option<String>,
    pub operation: Option<String>,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub user_agent: Option<String>,
    pub additional_data: HashMap<String, String>,
}

impl ErrorContext {
    /// Create a new error context
    pub fn new() -> Self {
        Self {
            form_name: None,
            field_name: None,
            operation: None,
            timestamp: Some(chrono::Utc::now()),
            user_agent: None,
            additional_data: HashMap::new(),
        }
    }

    /// Set the form name
    pub fn with_form_name(mut self, form_name: impl Into<String>) -> Self {
        self.form_name = Some(form_name.into());
        self
    }

    /// Set the field name
    pub fn with_field_name(mut self, field_name: impl Into<String>) -> Self {
        self.field_name = Some(field_name.into());
        self
    }

    /// Set the operation
    pub fn with_operation(mut self, operation: impl Into<String>) -> Self {
        self.operation = Some(operation.into());
        self
    }

    /// Set the user agent
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Add additional data
    pub fn with_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.additional_data.insert(key.into(), value.into());
        self
    }

    /// Get the form name
    pub fn form_name(&self) -> Option<&str> {
        self.form_name.as_deref()
    }

    /// Get the field name
    pub fn field_name(&self) -> Option<&str> {
        self.field_name.as_deref()
    }

    /// Get the operation
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Get the timestamp
    pub fn timestamp(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.timestamp
    }

    /// Get the user agent
    pub fn user_agent(&self) -> Option<&str> {
        self.user_agent.as_deref()
    }

    /// Get additional data by key
    pub fn get_data(&self, key: &str) -> Option<&String> {
        self.additional_data.get(key)
    }

    /// Get all additional data
    pub fn all_data(&self) -> &HashMap<String, String> {
        &self.additional_data
    }

    /// Check if context has form name
    pub fn has_form_name(&self) -> bool {
        self.form_name.is_some()
    }

    /// Check if context has field name
    pub fn has_field_name(&self) -> bool {
        self.field_name.is_some()
    }

    /// Check if context has operation
    pub fn has_operation(&self) -> bool {
        self.operation.is_some()
    }

    /// Check if context has timestamp
    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    /// Check if context has user agent
    pub fn has_user_agent(&self) -> bool {
        self.user_agent.is_some()
    }

    /// Check if context has additional data
    pub fn has_additional_data(&self) -> bool {
        !self.additional_data.is_empty()
    }

    /// Get the number of additional data entries
    pub fn additional_data_count(&self) -> usize {
        self.additional_data.len()
    }

    /// Clear all additional data
    pub fn clear_additional_data(&mut self) {
        self.additional_data.clear();
    }

    /// Remove a specific data entry
    pub fn remove_data(&mut self, key: &str) -> Option<String> {
        self.additional_data.remove(key)
    }

    /// Update timestamp to current time
    pub fn update_timestamp(&mut self) {
        self.timestamp = Some(chrono::Utc::now());
    }

    /// Create a context for a specific form
    pub fn for_form(form_name: impl Into<String>) -> Self {
        Self::new().with_form_name(form_name)
    }

    /// Create a context for a specific field
    pub fn for_field(field_name: impl Into<String>) -> Self {
        Self::new().with_field_name(field_name)
    }

    /// Create a context for a specific operation
    pub fn for_operation(operation: impl Into<String>) -> Self {
        Self::new().with_operation(operation)
    }
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrorContext(")?;

        let mut parts = Vec::new();

        if let Some(form_name) = &self.form_name {
            parts.push(format!("form: {}", form_name));
        }

        if let Some(field_name) = &self.field_name {
            parts.push(format!("field: {}", field_name));
        }

        if let Some(operation) = &self.operation {
            parts.push(format!("operation: {}", operation));
        }

        if let Some(timestamp) = &self.timestamp {
            parts.push(format!(
                "timestamp: {}",
                timestamp.format("%Y-%m-%d %H:%M:%S UTC")
            ));
        }

        if let Some(user_agent) = &self.user_agent {
            parts.push(format!("user_agent: {}", user_agent));
        }

        if !self.additional_data.is_empty() {
            parts.push(format!("data: {} entries", self.additional_data.len()));
        }

        write!(f, "{}", parts.join(", "))?;
        write!(f, ")")
    }
}
