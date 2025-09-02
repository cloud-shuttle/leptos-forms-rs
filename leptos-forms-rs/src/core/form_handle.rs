
use leptos::prelude::{ReadSignal, WriteSignal, signal, Memo, Get, Set};
use std::collections::HashMap;
use crate::core::traits::*;
use crate::core::types::*;
use crate::validation::ValidationErrors;
use crate::error::FormError;

/// Main form handle for managing form state and operations
pub struct FormHandle<T: Form> {
    /// Current form state (read signal)
    state: ReadSignal<FormState<T>>,
    /// Current form state (write signal)
    write_state: WriteSignal<FormState<T>>,
    /// Form schema
    schema: FormSchema,
    /// Validation mode
    validation_mode: ValidationMode,
    /// Submission mode
    submission_mode: SubmissionMode,
    /// Persistence options
    persistence: PersistenceOptions,
    /// Analytics options
    analytics: AnalyticsOptions,
    /// Field-specific signals
    field_signals: HashMap<String, FieldSignal>,
    /// Subscribers for state changes
    subscribers: Vec<Box<dyn Fn(FormState<T>) + Send + Sync + 'static>>,
}

/// Signal wrapper for individual field state
#[derive(Clone)]
pub struct FieldSignal {
    pub value: ReadSignal<Option<FieldValue>>,
    pub error: ReadSignal<Option<String>>,
    pub is_dirty: ReadSignal<bool>,
    pub is_touched: ReadSignal<bool>,
}

impl<T: Form + PartialEq + Clone + Send + Sync> FormHandle<T> {
    /// Create a new form handle with default values
    pub fn new() -> Self {
        let default_values = T::default_values();
        let schema = T::schema();
        
        let (read, write) = signal(FormState::new(default_values));
        Self {
            state: read,
            write_state: write,
            schema,
            validation_mode: ValidationMode::OnBlur,
            submission_mode: SubmissionMode::Manual,
            persistence: PersistenceOptions::default(),
            analytics: AnalyticsOptions::default(),
            field_signals: HashMap::new(),
            subscribers: Vec::new(),
        }
    }
    
    /// Create a new form handle with custom initial values
    pub fn with_values(values: T) -> Self {
        let schema = T::schema();
        
        let (read, write) = signal(FormState::new(values));
        Self {
            state: read,
            write_state: write,
            schema,
            validation_mode: ValidationMode::OnBlur,
            submission_mode: SubmissionMode::Manual,
            persistence: PersistenceOptions::default(),
            analytics: AnalyticsOptions::default(),
            field_signals: HashMap::new(),
            subscribers: Vec::new(),
        }
    }
    
    /// Get the current form state
    pub fn get_state(&self) -> ReadSignal<FormState<T>> {
        self.state
    }
    
    /// Get the form values
    pub fn get_values(&self) -> Memo<T> {
        let state_signal = self.state;
        Memo::new(move |_| {
            let state = state_signal.get();
            state.values.clone()
        })
    }
    
    /// Get form errors
    pub fn get_errors(&self) -> Memo<ValidationErrors> {
        let state_signal = self.state;
        Memo::new(move |_| {
            let state = state_signal.get();
            state.errors.clone()
        })
    }
    
    /// Check if form is valid
    pub fn is_valid(&self) -> Memo<bool> {
        let state_signal = self.state;
        Memo::new(move |_| {
            let state = state_signal.get();
            state.is_valid
        })
    }
    
    /// Check if form is dirty (has been modified)
    pub fn is_dirty(&self) -> Memo<bool> {
        let state_signal = self.state;
        Memo::new(move |_| {
            let state = state_signal.get();
            state.is_dirty
        })
    }
    
    /// Check if form is submitting
    pub fn is_submitting(&self) -> Memo<bool> {
        let state_signal = self.state;
        Memo::new(move |_| {
            state_signal.get().is_submitting
        })
    }
    
    /// Get a field signal by name
    pub fn get_field_signal(&mut self, field_name: &str) -> Option<&FieldSignal> {
        if !self.field_signals.contains_key(field_name) {
            self.create_field_signal(field_name);
        }
        self.field_signals.get(field_name)
    }
    
    /// Create a field signal for a specific field
    fn create_field_signal(&mut self, field_name: &str) {
        // For now, we'll create simple signals
        // In a real implementation, you'd want to create derived signals
        let (value_signal, _) = signal(None::<FieldValue>);
        let (error_signal, _) = signal(None::<String>);
        let (is_dirty_signal, _) = signal(false);
        let (is_touched_signal, _) = signal(false);
        
        let field_signal = FieldSignal {
            value: value_signal,
            error: error_signal,
            is_dirty: is_dirty_signal,
            is_touched: is_touched_signal,
        };
        
        self.field_signals.insert(field_name.to_string(), field_signal);
    }
    
    /// Set a field value
    pub fn set_field_value(&self, field_name: &str, value: FieldValue) -> Result<(), FormError> {
        let current_state = self.state.get();
        let mut values = current_state.values.clone();
        
        values.set_field(field_name, value).map_err(FormError::from_field_error)?;
        
        let new_state = current_state
            .with_errors(ValidationErrors::new())
            .mark_dirty()
            .mark_touched(field_name.to_string());
        
        // Validate field if needed
        let validation_errors = if self.should_validate_field() {
            self.validate_field(&values, field_name)
        } else {
            ValidationErrors::new()
        };
        
        let final_state = new_state.with_errors(validation_errors);
        self.write_state.set(final_state);
        
        // Track analytics
        if self.analytics.track_field_interactions {
            self.track_field_interaction(field_name, "value_change");
        }
        
        Ok(())
    }
    
    /// Get a field value
    pub fn get_field_value(&self, field_name: &str) -> Option<FieldValue> {
        let state = self.state.get();
        state.values.get_field(field_name)
    }
    
    /// Validate a single field
    pub fn validate_field(&self, form: &T, field_name: &str) -> ValidationErrors {
        let mut errors = ValidationErrors::new();
        
        if let Some(field_metadata) = self.schema.get_field(field_name) {
            if let Some(value) = form.get_field(field_name) {
                for validator in &field_metadata.validators {
                    if let Err(error) = self.validate_field_value(&value, validator) {
                        errors.add_field_error(field_name.to_string(), error);
                    }
                }
            } else if field_metadata.is_required {
                errors.add_field_error(field_name.to_string(), "This field is required".to_string());
            }
        }
        
        errors
    }
    
    /// Validate the entire form
    pub fn validate_form(&self) -> Result<(), ValidationErrors> {
        let state = self.state.get();
        let validation_result = state.values.validate();
        
        match validation_result {
            Ok(()) => {
                let new_state = state.with_errors(ValidationErrors::new());
                self.write_state.set(new_state);
                Ok(())
            }
            Err(errors) => {
                let new_state = state.with_errors(errors.clone());
                self.write_state.set(new_state);
                Err(errors)
            }
        }
    }
    
    /// Validate field value against a specific validator
    fn validate_field_value(&self, value: &FieldValue, validator: &ValidatorConfig) -> Result<(), String> {
        use crate::validation::Validators;
        
        match validator {
            ValidatorConfig::Required => Validators::required(value),
            ValidatorConfig::Email => Validators::email(value),
            ValidatorConfig::MinLength(min) => Validators::min_length(value, *min),
            ValidatorConfig::MaxLength(max) => Validators::max_length(value, *max),
            ValidatorConfig::Pattern(pattern) => Validators::pattern(value, pattern),
            ValidatorConfig::Min(min) => Validators::min(value, *min),
            ValidatorConfig::Max(max) => Validators::max(value, *max),
            ValidatorConfig::Custom(_) => Ok(()), // Custom validators handled separately
            _ => Ok(()),
        }
    }
    
    /// Reset the form to default values
    pub fn reset(&self) {
        let default_values = T::default_values();
        let new_state = FormState::new(default_values);
        self.write_state.set(new_state);
        
        // Track analytics
        if self.analytics.track_field_interactions {
            self.track_field_interaction("form", "reset");
        }
    }
    
    /// Submit the form
    pub async fn submit(&self, handler: Box<dyn FormSubmitHandler<T>>) -> Result<(), crate::error::FormError> {
        // Validate form first
        if let Err(errors) = self.validate_form() {
            if self.analytics.track_validation_errors {
                self.track_validation_errors(&errors);
            }
            return Err(crate::error::FormError::validation_error("Form validation failed", 
                errors.field_errors.iter()
                    .map(|(field, msg)| crate::error::FieldError::new(field.clone(), msg.clone()))
                    .collect()));
        }
        
        // Mark as submitting
        let current_state = self.state.get();
        let submitting_state = current_state.mark_submitting();
        self.write_state.set(submitting_state);
        
        // Track submission attempt
        if self.analytics.track_submission_attempts {
            self.track_submission_attempt(true);
        }
        
        // Handle submission
        let result = handler.handle_submit(&self.state.get().values);
        
        // Update state based on result
        let mut final_state = self.state.get();
        final_state.is_submitting = false;
        
        match result {
            Ok(()) => {
                // Clear form if configured
                if self.persistence.clear_on_submit {
                    self.reset();
                }
                
                if self.analytics.track_submission_attempts {
                    self.track_submission_attempt(false);
                }
            }
            Err(ref error) => {
                final_state.errors.form_errors.push(error.message.clone());
                if self.analytics.track_submission_attempts {
                    self.track_submission_attempt(false);
                }
            }
        }
        
        self.write_state.set(final_state);
        result.map_err(|e| crate::error::FormError::submission_error(e.message, None, None))
    }
    
    /// Set validation mode
    pub fn set_validation_mode(&mut self, mode: ValidationMode) {
        self.validation_mode = mode;
    }
    
    /// Set submission mode
    pub fn set_submission_mode(&mut self, mode: SubmissionMode) {
        self.submission_mode = mode;
    }
    
    /// Set persistence options
    pub fn set_persistence(&mut self, options: PersistenceOptions) {
        self.persistence = options;
    }
    
    /// Set analytics options
    pub fn set_analytics(&mut self, options: AnalyticsOptions) {
        self.analytics = options;
    }
    
    /// Subscribe to form state changes
    pub fn subscribe(&mut self, callback: Box<dyn Fn(FormState<T>) + Send + Sync + 'static>) {
        self.subscribers.push(callback);
    }
    
    /// Check if field should be validated based on current mode
    fn should_validate_field(&self) -> bool {
        matches!(self.validation_mode, 
            ValidationMode::OnChange | ValidationMode::OnBlurAndChange)
    }
    
    /// Track field interaction for analytics
    fn track_field_interaction(&self, field_name: &str, action: &str) {
        if let Some(custom_tracking) = &self.analytics.custom_tracking {
            custom_tracking("field_interaction", field_name, action);
        }
    }
    
    /// Track validation errors for analytics
    fn track_validation_errors(&self, errors: &ValidationErrors) {
        if let Some(custom_tracking) = &self.analytics.custom_tracking {
            let error_count = errors.field_errors.len() + errors.form_errors.len();
            custom_tracking("validation_errors", "form", &error_count.to_string());
        }
    }
    
    /// Track submission attempt for analytics
    fn track_submission_attempt(&self, success: bool) {
        if let Some(custom_tracking) = &self.analytics.custom_tracking {
            custom_tracking("submission_attempt", "form", if success { "success" } else { "failure" });
        }
    }
    
    /// Field Array Management Methods
    
    /// Add a new item to a field array
    pub fn add_field_array_item(&mut self, field_name: &str) -> Result<(), FormError> {
        let mut current_state = self.state.get();
        
        // Get the field metadata to understand the array structure
        if let Some(field_meta) = self.schema.get_field(field_name) {
            if let FieldType::Array(item_type) = &field_meta.field_type {
                // Create a default value for the new item
                let default_value = self.create_default_value_for_type(&item_type);
                
                // Add to the array field
                if let Some(FieldValue::Array(ref mut array)) = current_state.values.get_field(field_name) {
                    array.push(default_value);
                } else {
                    // Initialize array if it doesn't exist
                    let mut new_array = Vec::new();
                    new_array.push(default_value);
                    current_state.values.set_field(field_name, FieldValue::Array(new_array))
                        .map_err(|e| FormError::field_error(field_name.to_string(), e.message))?;
                }
                
                // Clear any existing errors for this field
                current_state.errors.field_errors.remove(field_name);
                
                // Update state
                self.write_state.set(current_state);
                
                // Notify subscribers
                self.notify_subscribers();
                
                Ok(())
            } else {
                Err(FormError::field_error(
                    format!("Field '{}' is not an array type", field_name),
                    field_name.to_string(),
                ))
            }
        } else {
            Err(FormError::field_error(
                format!("Field '{}' not found in schema", field_name),
                field_name.to_string(),
            ))
        }
    }
    
    /// Remove an item from a field array
    pub fn remove_field_array_item(&mut self, field_name: &str, index: usize) -> Result<(), FormError> {
        let mut current_state = self.state.get();
        
        if let Some(FieldValue::Array(ref mut array)) = current_state.values.get_field(field_name) {
            if index < array.len() {
                array.remove(index);
                
                // Clear any existing errors for this field
                current_state.errors.field_errors.remove(field_name);
                
                // Update state
                self.write_state.set(current_state);
                
                // Notify subscribers
                self.notify_subscribers();
                
                Ok(())
            } else {
                Err(FormError::field_error(
                    format!("Index {} out of bounds for array field '{}'", index, field_name),
                    field_name.to_string(),
                ))
            }
        } else {
            Err(FormError::field_error(
                format!("Field '{}' is not an array or doesn't exist", field_name),
                field_name.to_string(),
            ))
        }
    }
    
    /// Get field array items
    pub fn field_array(&self, field_name: &str) -> Option<Vec<FieldValue>> {
        let current_state = self.state.get();
        if let Some(FieldValue::Array(array)) = current_state.values.get_field(field_name) {
            Some(array.clone())
        } else {
            None
        }
    }
    
    /// Set field array item value
    pub fn set_field_array_item_value(&mut self, field_name: &str, index: usize, value: FieldValue) -> Result<(), FormError> {
        let current_state = self.state.get();
        if let Some(FieldValue::Array(ref mut array)) = current_state.values.get_field(field_name) {
            if index < array.len() {
                array[index] = value;
                self.write_state.set(current_state);
                self.notify_subscribers();
                Ok(())
            } else {
                Err(FormError::field_error(format!("Index {} out of bounds for array field '{}'", index, field_name), field_name.to_string()))
            }
        } else {
            Err(FormError::field_error(format!("Field '{}' is not an array or doesn't exist", field_name), field_name.to_string()))
        }
    }
    
    /// Helper method to create default values for different field types
    fn create_default_value_for_type(&self, field_type: &FieldType) -> FieldValue {
        match field_type {
            FieldType::Text => FieldValue::String(String::new()),
            FieldType::Email => FieldValue::String(String::new()),
            FieldType::Password => FieldValue::String(String::new()),
            FieldType::Number(_) => FieldValue::Number(0.0),
            FieldType::Boolean => FieldValue::Boolean(false),
            FieldType::Select(_) => FieldValue::String(String::new()),
            FieldType::MultiSelect(_) => FieldValue::Array(Vec::new()),
            FieldType::Date => FieldValue::Date(chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            FieldType::DateTime => FieldValue::DateTime(chrono::Utc::now()),
            FieldType::File(_) => FieldValue::Null,
            FieldType::Array(_) => FieldValue::Array(Vec::new()),
            FieldType::Nested(_) => FieldValue::Object(std::collections::HashMap::new()),
        }
    }
    
    /// Notify all subscribers of state changes
    fn notify_subscribers(&self) {
        let current_state = self.state.get();
        for subscriber in &self.subscribers {
            subscriber(current_state.clone());
        }
    }
}

impl<T: Form + PartialEq + Clone + Send + Sync> Clone for FormHandle<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state,
            write_state: self.write_state,
            schema: self.schema.clone(),
            validation_mode: self.validation_mode,
            submission_mode: self.submission_mode,
            persistence: self.persistence.clone(),
            analytics: self.analytics.clone(),
            field_signals: self.field_signals.clone(),
            subscribers: Vec::new(), // Don't clone subscribers
        }
    }
}

impl<T: Form + PartialEq + Clone + Send + Sync> Default for FormHandle<T> {
    fn default() -> Self {
        Self::new()
    }
}
