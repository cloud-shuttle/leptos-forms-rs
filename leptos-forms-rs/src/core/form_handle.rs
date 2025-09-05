
use leptos::prelude::*;
use leptos::prelude::GetUntracked;
use crate::core::traits::Form;
use crate::core::types::FieldValue;
use crate::core::traits::FormState;
use crate::validation::{ValidationErrors, Validator};
use crate::error::FormError;

/// Form handle for managing form state and operations
pub struct FormHandle<T: Form> where T: Send, T: std::marker::Sync {
    state: RwSignal<FormState<T>>,
}

impl<T: Form + Send + Sync + PartialEq> FormHandle<T> {
    /// Create a new form handle
    pub fn new(form: T) -> Self {
        let initial_state = FormState::new(form);
        let state = RwSignal::new(initial_state);
        Self { state }
    }

    /// Helper function to update array field and set form state
    fn update_array_field<F>(&self, field_name: &str, mutator: F) 
    where 
        F: FnOnce(&mut Vec<FieldValue>)
    {
        let current_state = self.state.get_untracked();
        let mut new_values = current_state.values.clone();
        
        let current_array = new_values.get_field_value(field_name);
        if let FieldValue::Array(mut array) = current_array {
            mutator(&mut array);
            
            new_values.set_field_value(field_name, FieldValue::Array(array));
            
            let new_state = FormState {
                values: new_values,
                is_dirty: true,
                is_submitting: current_state.is_submitting,
                errors: current_state.errors,
            };
            
            self.state.set(new_state);
        }
    }

    /// Get the form state signal
    pub fn state(&self) -> ReadSignal<FormState<T>> {
        self.state.read_only()
    }

    /// Get the form values signal
    pub fn values(&self) -> Memo<T> {
        let state = self.state.clone();
        Memo::new(move |_| state.get().values.clone())
    }

    /// Get the form errors signal
    pub fn errors(&self) -> Memo<ValidationErrors> {
        let state = self.state.clone();
        Memo::new(move |_| state.get().errors.clone())
    }

    /// Get the form validity signal
    pub fn is_valid(&self) -> Memo<bool> {
        let state = self.state.clone();
        Memo::new(move |_| state.get().is_valid())
    }

    /// Get the form dirty state signal
    pub fn is_dirty(&self) -> Memo<bool> {
        let state = self.state.clone();
        Memo::new(move |_| state.get().is_dirty)
    }

    /// Get the form submitting state signal
    pub fn is_submitting(&self) -> Memo<bool> {
        let state = self.state.clone();
        Memo::new(move |_| state.get().is_submitting)
    }

    /// Get a field value
    pub fn get_field_value(&self, field_name: &str) -> Option<FieldValue> {
        let state = self.state.get_untracked();
        let schema = T::schema();
        
        // Check if the field exists in the schema
        if schema.get_field(field_name).is_some() {
            Some(state.values.get_field_value(field_name))
        } else {
            None
        }
    }
    
    /// Set a field value
    pub fn set_field_value(&self, field_name: &str, value: FieldValue) {
        let current_state = self.state.get_untracked();
        
        // Create a new form instance with the updated field
        let mut new_form = current_state.values.clone();
        
        // Update the field using the form's set_field_value method
        new_form.set_field_value(field_name, value);
        
        // Create new state with updated form and mark as dirty
        let new_state = FormState {
            values: new_form,
            is_dirty: true,
            is_submitting: current_state.is_submitting,
            errors: current_state.errors,
        };
        
        self.state.set(new_state);
    }

    /// Validate a specific field
    pub fn validate_field(&self, field_name: &str) -> Result<(), ValidationErrors> {
        let state = self.state.get();
        let form_data = &state.values;
        let binding = T::schema();
        let metadata = binding.get_field(field_name);
        
        if let Some(field_meta) = metadata {
            let _default_value = FieldValue::String(String::new());
            let field_value = form_data.get_field_value(field_name);
            
            // Validate field
            for validator in &field_meta.validators {
                if let Err(error) = self.validate_field_value(field_value.clone(), validator) {
                    let mut errors = ValidationErrors::new();
                    errors.add_field_error(field_name, error);
                    return Err(errors);
                }
            }
        }
        
        Ok(())
    }
    
    /// Validate the entire form
    pub fn validate(&self) -> Result<(), FormError> {
        let state = self.state.get_untracked();
        let form_data = &state.values;
        let metadata = T::schema().field_metadata.clone();
        let mut errors = ValidationErrors::new();
        
        for field_meta in metadata {
            let field_name = &field_meta.name;
            let field_value = form_data.get_field_value(field_name);
            
            // Validate field
            for validator in &field_meta.validators {
                if let Err(error) = self.validate_field_value(field_value.clone(), validator) {
                    errors.add_field_error(field_name, error);
                }
            }
        }
        
        // Also call the form's own validate method for custom validation logic
        if let Err(form_errors) = form_data.validate() {
            for (field_name, error_msgs) in form_errors.field_errors {
                for error_msg in error_msgs {
                    errors.add_field_error(&field_name, error_msg);
                }
            }
        }
        
        if errors.has_errors() {
            // Update state with errors
            let new_state = state.with_errors(errors.clone());
            self.state.set(new_state);
            
            Err(FormError::validation_error("Form validation failed".to_string(), errors.to_field_errors()))
        } else {
            // Clear any existing errors
            let new_state = state.with_errors(ValidationErrors::new());
            self.state.set(new_state);
            Ok(())
        }
    }
    
    /// Submit the form
    pub fn submit(&self) -> Result<T, FormError> {
        // Validate first
        self.validate()?;
        
        let state = self.state.get_untracked();
        let new_state = state.mark_submitting();
        self.state.set(new_state.clone());
        
        // In a real implementation, you would send the data here
        // For now, just return the values
        Ok(new_state.values)
    }

    /// Reset the form to initial values
    pub fn reset(&self) {
        let _state = self.state.get_untracked();
        let initial_values = T::default_values();
        let new_state = FormState::new(initial_values);
        self.state.set(new_state);
    }

    /// Clear all validation errors
    pub fn clear_errors(&self) {
        let state = self.state.get_untracked();
        let new_state = state.with_errors(ValidationErrors::new());
        self.state.set(new_state);
    }

    /// Clear errors for a specific field
    pub fn clear_field_errors(&self, field_name: &str) {
        let state = self.state.get_untracked();
        let mut new_errors = state.errors.clone();
        new_errors.remove_field_error(field_name);
        let new_state = state.with_errors(new_errors);
        self.state.set(new_state);
    }

    /// Add an item to a field array
    pub fn add_array_item(&self, field_name: &str, value: FieldValue) {
        self.update_array_field(field_name, |array| {
            array.push(value);
        });
    }

    /// Remove an item from a field array
    pub fn remove_array_item(&self, field_name: &str, index: usize) {
        self.update_array_field(field_name, |array| {
            if index < array.len() {
                array.remove(index);
            }
        });
    }

    /// Move an item in a field array
    pub fn move_array_item(&self, field_name: &str, from_index: usize, to_index: usize) {
        self.update_array_field(field_name, |array| {
            if from_index < array.len() && to_index < array.len() {
                let item = array.remove(from_index);
                array.insert(to_index, item);
            }
        });
    }

    /// Clear all items in a field array
    pub fn clear_array(&self, field_name: &str) {
        self.update_array_field(field_name, |array| {
            array.clear();
        });
    }

    /// Insert an item at a specific index in a field array
    pub fn insert_array_item(&self, field_name: &str, index: usize, value: FieldValue) {
        self.update_array_field(field_name, |array| {
            if index <= array.len() {
                array.insert(index, value);
            }
        });
    }

    /// Duplicate an item at a specific index in a field array
    pub fn duplicate_array_item(&self, field_name: &str, index: usize) {
        self.update_array_field(field_name, |array| {
            if index < array.len() {
                let item_to_duplicate = array[index].clone();
                array.insert(index + 1, item_to_duplicate);
            }
        });
    }

    /// Get the length of a field array
    pub fn get_array_length(&self, field_name: &str) -> Option<usize> {
        let current_state = self.state.get_untracked();
        let field_value = current_state.values.get_field_value(field_name);
        
        if let FieldValue::Array(array) = field_value {
            Some(array.len())
        } else {
            None
        }
    }

    /// Get an item from a field array by index
    pub fn get_array_item(&self, field_name: &str, index: usize) -> Option<FieldValue> {
        let current_state = self.state.get_untracked();
        let field_value = current_state.values.get_field_value(field_name);
        
        if let FieldValue::Array(array) = field_value {
            array.get(index).cloned()
        } else {
            None
        }
    }
    
    /// Set an item in a field array at a specific index
    pub fn set_array_item(&self, field_name: &str, index: usize, value: FieldValue) {
        self.update_array_field(field_name, |array| {
            if index < array.len() {
                array[index] = value;
            }
        });
    }

    /// Swap two items in a field array
    pub fn swap_array_items(&self, field_name: &str, index1: usize, index2: usize) {
        self.update_array_field(field_name, |array| {
            if index1 < array.len() && index2 < array.len() && index1 != index2 {
                array.swap(index1, index2);
            }
        });
    }

    /// Batch add multiple items to a field array
    pub fn batch_add_array_items(&self, field_name: &str, items: Vec<FieldValue>) {
        self.update_array_field(field_name, |array| {
            array.extend(items);
        });
    }

    /// Get the form schema
    pub fn schema(&self) -> crate::core::traits::FormSchema {
        T::schema()
    }

    /// Get field metadata
    pub fn get_field_metadata(&self, field_name: &str) -> Option<crate::core::traits::FieldMetadata> {
        let binding = T::schema();
        binding.get_field(field_name).cloned()
    }

    /// Check if a field is required
    pub fn is_field_required(&self, field_name: &str) -> bool {
        self.get_field_metadata(field_name)
            .map(|meta| meta.is_required)
            .unwrap_or(false)
    }

    /// Get the field type
    pub fn get_field_type(&self, field_name: &str) -> Option<crate::core::types::FieldType> {
        self.get_field_metadata(field_name)
            .map(|meta| meta.field_type.clone())
    }

    /// Private helper to validate a field value against a validator
    fn validate_field_value(&self, value: FieldValue, validator: &Validator) -> Result<(), String> {
        use crate::validation::ValidationRuleEngine;
        
        let mut engine = ValidationRuleEngine::new();
        engine.add_validator(validator.clone());
        
        match engine.validate_value(value) {
            Ok(_) => Ok(()),
            Err(errors) => {
                if let Some(error) = errors.get_field_error("").and_then(|v| v.first()) {
                    Err(error.clone())
                } else {
                    Err("Validation failed".to_string())
                }
            }
        }
    }
}

impl<T: Form + Send + Sync> Clone for FormHandle<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

impl<T: Form + Send + Sync + PartialEq> PartialEq for FormHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.state.get_untracked() == other.state.get_untracked()
    }
}
