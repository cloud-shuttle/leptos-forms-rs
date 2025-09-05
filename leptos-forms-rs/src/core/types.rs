use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{NaiveDate, DateTime, Utc};
use crate::core::traits::Form;

/// Supported field types
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberType {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConstraints {
    pub max_size: Option<usize>,
    pub accept: Vec<String>,
    pub multiple: bool,
}

/// Dynamic field value representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldValue {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Date(NaiveDate),
    DateTime(DateTime<Utc>),
    Array(Vec<FieldValue>),
    Object(HashMap<String, FieldValue>),
    File(FileData),
    Null,
}

impl FieldValue {
    pub fn as_string(&self) -> Option<&String> {
        match self {
            FieldValue::String(s) => Some(s),
            _ => None,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            FieldValue::String(s) => s.clone(),
            FieldValue::Number(n) => n.to_string(),
            FieldValue::Integer(i) => i.to_string(),
            FieldValue::Boolean(b) => b.to_string(),
            FieldValue::Date(d) => d.to_string(),
            FieldValue::DateTime(dt) => dt.to_string(),
            FieldValue::Array(_) => "[]".to_string(),
            FieldValue::Object(_) => "{}".to_string(),
            FieldValue::File(_) => "file".to_string(),
            FieldValue::Null => "null".to_string(),
        }
    }
    
    pub fn as_number(&self) -> Option<f64> {
        match self {
            FieldValue::Number(n) => Some(*n),
            FieldValue::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }
    
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            FieldValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }
    
    pub fn as_array(&self) -> Option<&Vec<FieldValue>> {
        match self {
            FieldValue::Array(arr) => Some(arr),
            _ => None,
        }
    }
    
    pub fn as_object(&self) -> Option<&HashMap<String, FieldValue>> {
        match self {
            FieldValue::Object(obj) => Some(obj),
            _ => None,
        }
    }
    
    pub fn is_null(&self) -> bool {
        matches!(self, FieldValue::Null)
    }
    
    pub fn is_empty(&self) -> bool {
        match self {
            FieldValue::String(s) => s.is_empty(),
            FieldValue::Array(arr) => arr.is_empty(),
            FieldValue::Object(obj) => obj.is_empty(),
            FieldValue::Null => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileData {
    pub name: String,
    pub size: usize,
    pub mime_type: String,
    pub data: Vec<u8>, // In practice, this would be a URL or handle
}

/// Built-in validator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidatorConfig {
    Required,
    Email,
    Url,
    MinLength(usize),
    MaxLength(usize),
    Min(f64),
    Max(f64),
    Pattern(String),
    Custom(String), // Function name for code generation
}

/// Field-specific error
#[derive(Debug, Clone)]
pub struct FieldError {
    pub field: String,
    pub message: String,
    pub code: Option<String>,
}

impl FieldError {
    pub fn new(field: String, message: String) -> Self {
        Self {
            field,
            message,
            code: None,
        }
    }
    
    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }
}

impl std::fmt::Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

impl std::error::Error for FieldError {}

/// Form-level error
#[derive(Debug, Clone)]
pub struct FormError {
    pub message: String,
    pub code: Option<String>,
    pub field_errors: Vec<FieldError>,
}

impl FormError {
    pub fn new(message: String) -> Self {
        Self {
            message,
            code: None,
            field_errors: Vec::new(),
        }
    }
    
    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }
    
    pub fn with_field_errors(mut self, field_errors: Vec<FieldError>) -> Self {
        self.field_errors = field_errors;
        self
    }
}

impl std::fmt::Display for FormError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FormError {}

/// Form field configuration
#[derive(Debug, Clone)]
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
    
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }
    
    pub fn with_placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = Some(placeholder);
        self
    }
    
    pub fn with_help_text(mut self, help_text: String) -> Self {
        self.help_text = Some(help_text);
        self
    }
    
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
    
    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }
    
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }
    
    pub fn with_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }
}

/// Form submission result
#[derive(Debug, Clone)]
pub struct FormSubmissionResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub errors: Vec<FormError>,
    pub warnings: Vec<String>,
}

impl<T> FormSubmissionResult<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    pub fn failure(errors: Vec<FormError>) -> Self {
        Self {
            success: false,
            data: None,
            errors,
            warnings: Vec::new(),
        }
    }
    
    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings = warnings;
        self
    }
}

/// Form validation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationMode {
    /// Validate on field blur
    OnBlur,
    /// Validate on field change
    OnChange,
    /// Validate on form submission only
    OnSubmit,
    /// Validate on both blur and change
    OnBlurAndChange,
}

/// Form submission mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubmissionMode {
    /// Submit immediately when form is valid
    Immediate,
    /// Submit only when explicitly triggered
    Manual,
    /// Submit with debouncing
    Debounced(std::time::Duration),
}

/// Form state persistence options
#[derive(Debug, Clone)]
pub struct PersistenceOptions {
    pub enabled: bool,
    pub storage_key: Option<String>,
    pub auto_save: bool,
    pub auto_save_interval: Option<std::time::Duration>,
    pub clear_on_submit: bool,
}

impl Default for PersistenceOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            storage_key: None,
            auto_save: false,
            auto_save_interval: None,
            clear_on_submit: true,
        }
    }
}

/// Form analytics options
pub struct AnalyticsOptions {
    pub enabled: bool,
    pub track_field_interactions: bool,
    pub track_validation_errors: bool,
    pub track_submission_attempts: bool,
    pub custom_tracking: Option<Box<dyn Fn(&str, &str, &str) + Send + Sync>>,
}

impl Default for AnalyticsOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            track_field_interactions: true,
            track_validation_errors: true,
            track_submission_attempts: true,
            custom_tracking: None,
        }
    }
}

impl std::fmt::Debug for AnalyticsOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnalyticsOptions")
            .field("enabled", &self.enabled)
            .field("track_field_interactions", &self.track_field_interactions)
            .field("track_validation_errors", &self.track_validation_errors)
            .field("track_submission_attempts", &self.track_submission_attempts)
            .field("custom_tracking", &if self.custom_tracking.is_some() { "Some(Fn)" } else { "None" })
            .finish()
    }
}

impl Clone for AnalyticsOptions {
    fn clone(&self) -> Self {
        Self {
            enabled: self.enabled,
            track_field_interactions: self.track_field_interactions,
            track_validation_errors: self.track_validation_errors,
            track_submission_attempts: self.track_submission_attempts,
            custom_tracking: None, // Cannot clone the function, so we set it to None
        }
    }
}

/// Form state for internal management
#[derive(Debug, Clone, PartialEq)]
pub struct FormState<T: Form> {
    pub values: T,
    pub errors: crate::validation::ValidationErrors,
    pub is_dirty: bool,
    pub is_submitting: bool,
}

impl<T: Form> FormState<T> {
    pub fn new(values: T) -> Self {
        Self {
            values,
            errors: crate::validation::ValidationErrors::new(),
            is_dirty: false,
            is_submitting: false,
        }
    }
    
    pub fn with_errors(mut self, errors: crate::validation::ValidationErrors) -> Self {
        let is_empty = errors.is_empty();
        self.errors = errors;
        self.is_dirty = !is_empty;
        self
    }
    
    pub fn with_values(mut self, values: T) -> Self {
        self.values = values;
        self
    }
    
    pub fn mark_dirty(mut self) -> Self {
        self.is_dirty = true;
        self
    }
    
    pub fn mark_submitting(mut self) -> Self {
        self.is_submitting = true;
        self
    }
    
    pub fn mark_not_submitting(mut self) -> Self {
        self.is_submitting = false;
        self
    }
    
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}
