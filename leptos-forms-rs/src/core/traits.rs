use crate::core::types::{FieldType, FieldValue};
use crate::validation::Validator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata for a form field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldMetadata {
    pub name: String,
    pub field_type: FieldType,
    pub is_required: bool,
    pub default_value: Option<FieldValue>,
    pub dependencies: Vec<String>,
    pub attributes: HashMap<String, String>,
    pub validators: Vec<Validator>,
}

impl Default for FieldMetadata {
    fn default() -> Self {
        Self {
            name: String::new(),
            field_type: FieldType::Text,
            is_required: false,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: Vec::new(),
        }
    }
}

/// Schema for a form
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormSchema {
    pub name: String,
    pub field_metadata: Vec<FieldMetadata>,
}

impl Default for FormSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl FormSchema {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            field_metadata: Vec::new(),
        }
    }

    pub fn get_field(&self, name: &str) -> Option<&FieldMetadata> {
        self.field_metadata.iter().find(|f| f.name == name)
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

    pub fn reset(mut self) -> Self {
        self.values = T::default_values();
        self.errors = crate::validation::ValidationErrors::new();
        self.is_dirty = false;
        self.is_submitting = false;
        self
    }
}

/// Core form trait that all forms must implement
pub trait Form: for<'de> Deserialize<'de> + Serialize + Clone + 'static {
    /// Get metadata for all fields in the form
    fn field_metadata() -> Vec<FieldMetadata>;

    /// Validate the form data
    fn validate(&self) -> Result<(), crate::validation::ValidationErrors>;

    /// Get default values for the form
    fn default_values() -> Self;

    /// Get the form schema
    fn schema() -> FormSchema {
        FormSchema {
            name: std::any::type_name::<Self>()
                .split("::")
                .last()
                .unwrap_or("unknown")
                .to_string(),
            field_metadata: Self::field_metadata(),
        }
    }

    /// Get form data as a HashMap
    fn get_form_data(&self) -> HashMap<String, FieldValue> {
        let mut data = HashMap::new();
        let metadata = Self::field_metadata();

        for field_meta in metadata {
            let field_name = field_meta.name.clone();
            let field_value = self.get_field_value(&field_name);
            data.insert(field_name, field_value);
        }

        data
    }

    /// Get a field value by name
    fn get_field_value(&self, _field_name: &str) -> FieldValue {
        // This is a default implementation that should be overridden
        // by forms that need custom field access logic
        FieldValue::String(String::new())
    }

    /// Set a field value by name
    fn set_field_value(&mut self, _field_name: &str, _value: FieldValue) {
        // This is a default implementation that should be overridden
        // by forms that need custom field update logic
        // For now, this is a no-op
    }
}

/// Trait for form field components
pub trait FormFieldComponent {
    /// Get the field name
    fn field_name(&self) -> &str;

    /// Get the field type
    fn field_type(&self) -> FieldType;

    /// Check if the field is required
    fn is_required(&self) -> bool;
}

/// Trait for form submission handling
pub trait FormSubmitHandler<T: Form> {
    /// Handle form submission
    fn handle_submit(&self, form: &T) -> Result<(), String>;
}

/// Trait for form validation
pub trait FormValidator<T: Form> {
    /// Validate a single field
    fn validate_field(&self, form: &T, field_name: &str) -> Result<(), String>;

    /// Validate the entire form
    fn validate_form(&self, form: &T) -> Result<(), crate::validation::ValidationErrors>;

    /// Get validation rules for a field
    fn get_field_validators(&self, field_name: &str) -> Vec<Validator>;
}

/// Trait for form persistence
pub trait FormPersistence<T: Form> {
    /// Save form state to storage
    fn save(&self, form: &T) -> Result<(), String>;

    /// Load form state from storage
    fn load(&self) -> Result<Option<T>, String>;

    /// Clear saved form state
    fn clear(&self) -> Result<(), String>;

    /// Check if form state exists in storage
    fn exists(&self) -> bool;
}

/// Trait for form analytics
pub trait FormAnalytics<T: Form> {
    /// Track form view
    fn track_view(&self, form_name: &str);

    /// Track field interaction
    fn track_field_interaction(&self, form_name: &str, field_name: &str, action: &str);

    /// Track form submission
    fn track_submission(&self, form_name: &str, success: bool);

    /// Track validation errors
    fn track_validation_errors(
        &self,
        form_name: &str,
        errors: &crate::validation::ValidationErrors,
    );
}
