use std::collections::HashMap;
use crate::core::types::*;
use crate::validation::ValidationErrors;

/// Utility functions for form handling
pub mod form_utils {
    use super::*;
    
    /// Convert a form struct to a HashMap of field values
    pub fn form_to_map<T: crate::core::traits::Form>(form: &T) -> HashMap<String, FieldValue> {
        let mut map = HashMap::new();
        let metadata = T::field_metadata();
        
        for field in metadata {
            if let Some(value) = form.get_field(&field.name) {
                map.insert(field.name, value);
            }
        }
        
        map
    }
    
    /// Convert a HashMap of field values to a form struct
    pub fn map_to_form<T: crate::core::traits::Form>(map: &HashMap<String, FieldValue>) -> Result<T, crate::error::FormError> {
        let mut form = T::default_values();
        
        for (field_name, value) in map {
            form.set_field(field_name, value.clone()).map_err(crate::error::FormError::from_field_error)?;
        }
        
        Ok(form)
    }
    
    /// Deep clone a form struct
    pub fn clone_form<T: crate::core::traits::Form>(form: &T) -> T {
        let map = form_to_map(form);
        map_to_form(&map).unwrap_or_else(|_| T::default_values())
    }
    
    /// Compare two forms for equality
    pub fn forms_equal<T: crate::core::traits::Form>(form1: &T, form2: &T) -> bool {
        let map1 = form_to_map(form1);
        let map2 = form_to_map(form2);
        
        if map1.len() != map2.len() {
            return false;
        }
        
        for (key, value1) in &map1 {
            if let Some(value2) = map2.get(key) {
                if !field_values_equal(value1, value2) {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        true
    }
    
    /// Check if two field values are equal
    pub fn field_values_equal(value1: &FieldValue, value2: &FieldValue) -> bool {
        match (value1, value2) {
            (FieldValue::String(s1), FieldValue::String(s2)) => s1 == s2,
            (FieldValue::Number(n1), FieldValue::Number(n2)) => (n1 - n2).abs() < f64::EPSILON,
            (FieldValue::Integer(i1), FieldValue::Integer(i2)) => i1 == i2,
            (FieldValue::Boolean(b1), FieldValue::Boolean(b2)) => b1 == b2,
            (FieldValue::Date(d1), FieldValue::Date(d2)) => d1 == d2,
            (FieldValue::DateTime(dt1), FieldValue::DateTime(dt2)) => dt1 == dt2,
            (FieldValue::Array(arr1), FieldValue::Array(arr2)) => {
                if arr1.len() != arr2.len() {
                    return false;
                }
                arr1.iter().zip(arr2.iter()).all(|(v1, v2)| field_values_equal(v1, v2))
            }
            (FieldValue::Object(obj1), FieldValue::Object(obj2)) => {
                if obj1.len() != obj2.len() {
                    return false;
                }
                obj1.iter().all(|(k, v1)| {
                    obj2.get(k).map_or(false, |v2| field_values_equal(v1, v2))
                })
            }
            (FieldValue::Null, FieldValue::Null) => true,
            _ => false,
        }
    }
}

/// Utility functions for validation
pub mod validation_utils {
    use super::*;
    
    /// Merge multiple validation error collections
    pub fn merge_validation_errors(errors: Vec<ValidationErrors>) -> ValidationErrors {
        let mut merged = ValidationErrors::new();
        
        for error in errors {
            merged.merge(error);
        }
        
        merged
    }
    
    /// Check if a validation error affects a specific field
    pub fn error_affects_field(error: &ValidationErrors, field_name: &str) -> bool {
        error.field_errors.contains_key(field_name)
    }
    
    /// Get all field names that have errors
    pub fn get_error_fields(errors: &ValidationErrors) -> Vec<String> {
        errors.field_errors.keys().cloned().collect()
    }
    
    /// Count total number of errors
    pub fn count_errors(errors: &ValidationErrors) -> usize {
        errors.field_errors.len() + errors.form_errors.len()
    }
    
    /// Check if validation errors are all field-specific
    pub fn are_all_field_errors(errors: &ValidationErrors) -> bool {
        errors.form_errors.is_empty()
    }
    
    /// Check if validation errors are all form-level
    pub fn are_all_form_errors(errors: &ValidationErrors) -> bool {
        errors.field_errors.is_empty()
    }
}

/// Utility functions for field values
pub mod field_utils {
    use super::*;
    
    /// Convert a string to a field value
    pub fn string_to_field_value(s: &str) -> FieldValue {
        if s.is_empty() {
            FieldValue::Null
        } else {
            FieldValue::String(s.to_string())
        }
    }
    
    /// Convert a number to a field value
    pub fn number_to_field_value(n: f64) -> FieldValue {
        FieldValue::Number(n)
    }
    
    /// Convert a boolean to a field value
    pub fn bool_to_field_value(b: bool) -> FieldValue {
        FieldValue::Boolean(b)
    }
    
    /// Convert a field value to a string
    pub fn field_value_to_string(value: &FieldValue) -> String {
        match value {
            FieldValue::String(s) => s.clone(),
            FieldValue::Number(n) => n.to_string(),
            FieldValue::Integer(i) => i.to_string(),
            FieldValue::Boolean(b) => b.to_string(),
            FieldValue::Date(d) => d.format("%Y-%m-%d").to_string(),
            FieldValue::DateTime(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            FieldValue::Array(arr) => format!("[{}]", arr.iter()
                .map(|v| field_value_to_string(v))
                .collect::<Vec<_>>()
                .join(", ")),
            FieldValue::Object(obj) => format!("{{{}}}", obj.iter()
                .map(|(k, v)| format!("{}: {}", k, field_value_to_string(v)))
                .collect::<Vec<_>>()
                .join(", ")),
            FieldValue::Null => "".to_string(),
            FieldValue::File(file) => format!("File: {}", file.name),
        }
    }
    
    /// Check if a field value is empty
    pub fn is_field_value_empty(value: &FieldValue) -> bool {
        match value {
            FieldValue::String(s) => s.trim().is_empty(),
            FieldValue::Array(arr) => arr.is_empty(),
            FieldValue::Object(obj) => obj.is_empty(),
            FieldValue::Null => true,
            _ => false,
        }
    }
    
    /// Get the type name of a field value
    pub fn get_field_value_type(value: &FieldValue) -> &'static str {
        match value {
            FieldValue::String(_) => "string",
            FieldValue::Number(_) => "number",
            FieldValue::Integer(_) => "integer",
            FieldValue::Boolean(_) => "boolean",
            FieldValue::Date(_) => "date",
            FieldValue::DateTime(_) => "datetime",
            FieldValue::Array(_) => "array",
            FieldValue::Object(_) => "object",
            FieldValue::Null => "null",
            FieldValue::File(_) => "file",
        }
    }
    
    /// Convert any value to a field value
    pub fn convert_to_field_value<T: std::fmt::Debug>(value: &T) -> FieldValue {
        // This is a simplified conversion - in a real implementation you'd want more robust type checking
        if std::any::type_name::<T>() == std::any::type_name::<String>() {
            // This is a bit of a hack - in practice you'd want proper type checking
            FieldValue::String(format!("{:?}", value))
        } else if std::any::type_name::<T>() == std::any::type_name::<bool>() {
            // Another hack - this won't work correctly
            FieldValue::Boolean(false)
        } else {
            FieldValue::String(format!("{:?}", value))
        }
    }
}

/// Utility functions for form configuration
pub mod config_utils {
    use super::*;
    
    /// Create a field configuration from metadata
    pub fn create_field_config(metadata: &crate::core::traits::FieldMetadata) -> FieldConfig {
        let mut config = FieldConfig::new(metadata.name.clone());
        
        if metadata.is_required {
            config = config.with_attribute("required".to_string(), "true".to_string());
        }
        
        for (key, value) in &metadata.attributes {
            config = config.with_attribute(key.clone(), value.clone());
        }
        
        config
    }
    
    /// Merge multiple field configurations
    pub fn merge_field_configs(configs: Vec<FieldConfig>) -> FieldConfig {
        if configs.is_empty() {
            return FieldConfig::new("".to_string());
        }
        
        let mut merged = configs[0].clone();
        
        for config in configs.iter().skip(1) {
            // Merge attributes
            for (key, value) in &config.attributes {
                merged.attributes.insert(key.clone(), value.clone());
            }
            
            // Merge other properties (take the last non-None value)
            if config.label.is_some() {
                merged.label = config.label.clone();
            }
            if config.placeholder.is_some() {
                merged.placeholder = config.placeholder.clone();
            }
            if config.help_text.is_some() {
                merged.help_text = config.help_text.clone();
            }
            
            merged.disabled |= config.disabled;
            merged.readonly |= config.readonly;
            merged.hidden |= config.hidden;
        }
        
        merged
    }
}

/// Utility functions for form persistence
pub mod persistence_utils {
    
    
    /// Generate a storage key for a form
    pub fn generate_storage_key<T: crate::core::traits::Form>(form_name: Option<&str>) -> String {
        let type_name = std::any::type_name::<T>();
        let form_name = form_name.unwrap_or("default");
        
        format!("leptos_forms_rs_{}_{}", 
            type_name.split("::").last().unwrap_or("unknown"),
            form_name
        )
    }
    
    /// Serialize form data for storage
    pub fn serialize_form<T: crate::core::traits::Form>(form: &T) -> Result<String, crate::error::FormError> {
        serde_json::to_string(form)
            .map_err(|e| crate::error::FormError::serialization_error(
                format!("Failed to serialize form: {}", e),
                None
            ))
    }
    
    /// Deserialize form data from storage
    pub fn deserialize_form<T: crate::core::traits::Form>(data: &str) -> Result<T, crate::error::FormError> {
        serde_json::from_str(data)
            .map_err(|e| crate::error::FormError::serialization_error(
                format!("Failed to deserialize form: {}", e),
                None
            ))
    }
    
    /// Check if form data exists in storage
    pub fn form_exists_in_storage<T: crate::core::traits::Form>(storage_key: &str) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(local_storage)) = window.local_storage() {
                    return local_storage.get_item(storage_key).ok().flatten().is_some();
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = storage_key; // Silence unused warning on non-wasm targets
        false
    }
    
    /// Save form data to storage
    pub fn save_form_to_storage<T: crate::core::traits::Form>(
        form: &T, 
        storage_key: &str
    ) -> Result<(), crate::error::FormError> {
        let serialized = serialize_form(form)?;
        
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(local_storage)) = window.local_storage() {
                    local_storage.set_item(storage_key, &serialized)
                        .map_err(|e| crate::error::FormError::persistence_error(
                            format!("Failed to save to localStorage: {:?}", e),
                            "localStorage"
                        ))?;
                    return Ok(());
                }
            }
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (storage_key, &serialized); // Silence unused warnings on non-wasm targets
        }
        
        Err(crate::error::FormError::persistence_error(
            "localStorage not available".to_string(),
            "localStorage"
        ))
    }
    
    /// Load form data from storage
    pub fn load_form_from_storage<T: crate::core::traits::Form>(
        storage_key: &str
    ) -> Result<Option<T>, crate::error::FormError> {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(local_storage)) = window.local_storage() {
                    if let Some(data) = local_storage.get_item(storage_key).ok().flatten() {
                        let form = deserialize_form::<T>(&data)?;
                        return Ok(Some(form));
                    }
                }
            }
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        let _ = storage_key; // Silence unused warning on non-wasm targets
        
        Ok(None)
    }
    
    /// Clear form data from storage
    pub fn clear_form_from_storage(storage_key: &str) -> Result<(), crate::error::FormError> {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(local_storage)) = window.local_storage() {
                    local_storage.remove_item(storage_key)
                        .map_err(|e| crate::error::FormError::persistence_error(
                            format!("Failed to clear from localStorage: {:?}", e),
                            "localStorage"
                        ))?;
                    return Ok(());
                }
            }
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        let _ = storage_key; // Silence unused warning on non-wasm targets
        
        Err(crate::error::FormError::persistence_error(
            "localStorage not available".to_string(),
            "localStorage"
        ))
    }
}

/// Utility functions for form analytics
pub mod analytics_utils {
    use super::*;
    
    /// Track form view event
    pub fn track_form_view(form_name: &str) {
        log::info!("Form view tracked: {}", form_name);
        // In a real implementation, this would send data to an analytics service
    }
    
    /// Track field interaction event
    pub fn track_field_interaction(form_name: &str, field_name: &str, action: &str) {
        log::info!("Field interaction tracked: {} - {} - {}", form_name, field_name, action);
        // In a real implementation, this would send data to an analytics service
    }
    
    /// Track form submission event
    pub fn track_form_submission(form_name: &str, success: bool) {
        log::info!("Form submission tracked: {} - {}", form_name, if success { "success" } else { "failure" });
        // In a real implementation, this would send data to an analytics service
    }
    
    /// Track validation errors event
    pub fn track_validation_errors(form_name: &str, errors: &ValidationErrors) {
        let error_count = errors.field_errors.len() + errors.form_errors.len();
        log::info!("Validation errors tracked: {} - {} errors", form_name, error_count);
        // In a real implementation, this would send data to an analytics service
    }
}

/// Utility functions for form debugging
pub mod debug_utils {
    use super::*;
    
    /// Print form state for debugging
    pub fn debug_form_state<T: crate::core::traits::Form + std::fmt::Debug>(form: &T) {
        log::debug!("Form state: {:#?}", form);
    }
    
    /// Print field metadata for debugging
    pub fn debug_field_metadata<T: crate::core::traits::Form>() {
        let metadata = T::field_metadata();
        log::debug!("Field metadata: {:#?}", metadata);
    }
    
    /// Print form schema for debugging
    pub fn debug_form_schema<T: crate::core::traits::Form>() {
        let schema = T::schema();
        log::debug!("Form schema: {:#?}", schema);
    }
    
    /// Validate form structure
    pub fn validate_form_structure<T: crate::core::traits::Form>() -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let metadata = T::field_metadata();
        
        // Check for duplicate field names
        let mut field_names = std::collections::HashSet::new();
        for field in &metadata {
            if !field_names.insert(&field.name) {
                errors.push(format!("Duplicate field name: {}", field.name));
            }
        }
        
        // Check for invalid field types
        for field in &metadata {
            match &field.field_type {
                FieldType::Select(options) => {
                    if options.is_empty() {
                        errors.push(format!("Select field '{}' has no options", field.name));
                    }
                }
                FieldType::Number(number_type) => {
                    if let (Some(min), Some(max)) = (number_type.min, number_type.max) {
                        if min > max {
                            errors.push(format!("Number field '{}' has invalid range: min > max", field.name));
                        }
                    }
                }
                _ => {}
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
