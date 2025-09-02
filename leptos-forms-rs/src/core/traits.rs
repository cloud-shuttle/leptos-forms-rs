use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::validation::ValidationErrors;
use crate::core::types::*;

/// Core trait that all forms must implement
pub trait Form: Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static {
    /// Get field metadata for runtime introspection
    fn field_metadata() -> Vec<FieldMetadata>;
    
    /// Validate the entire form
    fn validate(&self) -> Result<(), ValidationErrors>;
    
    /// Get a field value by name (for dynamic access)
    fn get_field(&self, name: &str) -> Option<FieldValue>;
    
    /// Set a field value by name (for dynamic updates)
    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), FieldError>;
    
    /// Get default values
    fn default_values() -> Self;
    
    /// Get the form schema (for validation and UI generation)
    fn schema() -> FormSchema;
}

/// Metadata about a form field
#[derive(Debug, Clone)]
pub struct FieldMetadata {
    pub name: String,
    pub field_type: FieldType,
    pub validators: Vec<ValidatorConfig>,
    pub is_required: bool,
    pub default_value: Option<FieldValue>,
    pub dependencies: Vec<String>,
    pub attributes: HashMap<String, String>,
}

/// Form schema for runtime introspection
#[derive(Debug, Clone)]
pub struct FormSchema {
    pub fields: Vec<FieldMetadata>,
    pub form_validators: Vec<ValidatorConfig>,
    pub attributes: HashMap<String, String>,
}

impl FormSchema {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            form_validators: Vec::new(),
            attributes: HashMap::new(),
        }
    }
    
    pub fn add_field(&mut self, field: FieldMetadata) {
        self.fields.push(field);
    }
    
    pub fn get_field(&self, name: &str) -> Option<&FieldMetadata> {
        self.fields.iter().find(|f| f.name == name)
    }
    
    pub fn required_fields(&self) -> Vec<&FieldMetadata> {
        self.fields.iter().filter(|f| f.is_required).collect()
    }
}

/// Trait for form field components
pub trait FormFieldComponent {
    /// Get the field name
    fn field_name(&self) -> &str;
    
    /// Get the field value
    fn field_value(&self) -> Option<FieldValue>;
    
    /// Set the field value
    fn set_field_value(&self, value: FieldValue);
    
    /// Get field errors
    fn field_errors(&self) -> Vec<String>;
    
    /// Check if field is valid
    fn is_valid(&self) -> bool;
    
    /// Check if field is dirty (has been modified)
    fn is_dirty(&self) -> bool;
    
    /// Reset field to default value
    fn reset(&self);
}

/// Trait for form submission handlers
pub trait FormSubmitHandler<T: Form> {
    /// Handle form submission
    fn handle_submit(&self, form: &T) -> Result<(), FormError>;
}

/// Trait for form validation
pub trait FormValidator<T: Form> {
    /// Validate a single field
    fn validate_field(&self, form: &T, field_name: &str) -> Result<(), FieldError>;
    
    /// Validate the entire form
    fn validate_form(&self, form: &T) -> Result<(), ValidationErrors>;
    
    /// Get validation rules for a field
    fn get_field_validators(&self, field_name: &str) -> Vec<ValidatorConfig>;
}

/// Trait for form state management
pub trait FormStateManager<T: Form> {
    /// Get current form state
    fn get_state(&self) -> FormState<T>;
    
    /// Update form state
    fn update_state(&self, state: FormState<T>);
    
    /// Subscribe to form state changes
    fn subscribe(&self, callback: Box<dyn Fn(FormState<T>) + Send + Sync + 'static>);
    
    /// Unsubscribe from form state changes
    fn unsubscribe(&self, id: SubscriptionId);
}

/// Form state representation
#[derive(Debug, Clone)]
pub struct FormState<T: Form> {
    pub values: T,
    pub errors: ValidationErrors,
    pub is_dirty: bool,
    pub is_submitting: bool,
    pub is_valid: bool,
    pub touched_fields: std::collections::HashSet<String>,
}

impl<T: Form> FormState<T> {
    pub fn new(values: T) -> Self {
        Self {
            values,
            errors: ValidationErrors::new(),
            is_dirty: false,
            is_submitting: false,
            is_valid: true,
            touched_fields: std::collections::HashSet::new(),
        }
    }
    
    pub fn with_errors(mut self, errors: ValidationErrors) -> Self {
        let is_empty = errors.is_empty();
        self.errors = errors;
        self.is_valid = is_empty;
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
    
    pub fn mark_touched(mut self, field_name: String) -> Self {
        self.touched_fields.insert(field_name);
        self
    }
}

/// Subscription ID for form state changes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubscriptionId(usize);

impl SubscriptionId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

/// Trait for form persistence
pub trait FormPersistence<T: Form> {
    /// Save form state to storage
    fn save(&self, form: &T) -> Result<(), FormError>;
    
    /// Load form state from storage
    fn load(&self) -> Result<Option<T>, FormError>;
    
    /// Clear saved form state
    fn clear(&self) -> Result<(), FormError>;
    
    /// Check if form state exists in storage
    fn exists(&self) -> bool;
}

/// Trait for form analytics and tracking
pub trait FormAnalytics<T: Form> {
    /// Track form view
    fn track_view(&self, form_name: &str);
    
    /// Track field interaction
    fn track_field_interaction(&self, form_name: &str, field_name: &str, action: &str);
    
    /// Track form submission
    fn track_submission(&self, form_name: &str, success: bool);
    
    /// Track validation errors
    fn track_validation_errors(&self, form_name: &str, errors: &ValidationErrors);
}
