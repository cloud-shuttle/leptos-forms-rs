use leptos::prelude::*;
use leptos_forms_rs::core::{Form, FormHandle, FieldMetadata, FormSchema, FieldValue, FieldType};
use leptos_forms_rs::validation::{ValidationErrors, Validator};
use leptos_forms_rs::hooks::use_form;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// Test form for multi-step functionality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiStepTestForm {
    // Step 1: Personal Information
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    
    // Step 2: Contact Information
    pub phone: String,
    pub address: String,
    pub city: String,
    
    // Step 3: Preferences
    pub newsletter: bool,
    pub notifications: bool,
    pub theme: String,
}

impl Default for MultiStepTestForm {
    fn default() -> Self {
        Self {
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            phone: String::new(),
            address: String::new(),
            city: String::new(),
            newsletter: false,
            notifications: false,
            theme: "light".to_string(),
        }
    }
}

impl Form for MultiStepTestForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        let mut metadata = Vec::new();
        
        // Step 1 fields
        metadata.push(FieldMetadata {
            name: "first_name".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::MinLength(2)],
        });
        
        metadata.push(FieldMetadata {
            name: "last_name".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::MinLength(2)],
        });
        
        metadata.push(FieldMetadata {
            name: "email".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::Email],
        });
        
        // Step 2 fields
        metadata.push(FieldMetadata {
            name: "phone".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::Pattern(r"^\d{10}$".to_string())],
        });
        
        metadata.push(FieldMetadata {
            name: "address".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::MinLength(10)],
        });
        
        metadata.push(FieldMetadata {
            name: "city".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::MinLength(2)],
        });
        
        // Step 3 fields
        metadata.push(FieldMetadata {
            name: "newsletter".to_string(),
            field_type: FieldType::Boolean,
            is_required: false,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![],
        });
        
        metadata.push(FieldMetadata {
            name: "notifications".to_string(),
            field_type: FieldType::Boolean,
            is_required: false,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![],
        });
        
        metadata.push(FieldMetadata {
            name: "theme".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required],
        });
        
        metadata
    }
    
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        
        // Step 1 validation
        if self.first_name.is_empty() {
            errors.add_field_error("first_name", "First name is required".to_string());
        } else if self.first_name.len() < 2 {
            errors.add_field_error("first_name", "First name must be at least 2 characters".to_string());
        }
        
        if self.last_name.is_empty() {
            errors.add_field_error("last_name", "Last name is required".to_string());
        } else if self.last_name.len() < 2 {
            errors.add_field_error("last_name", "Last name must be at least 2 characters".to_string());
        }
        
        if self.email.is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        } else if !self.email.contains('@') {
            errors.add_field_error("email", "Invalid email format".to_string());
        }
        
        // Step 2 validation
        if self.phone.is_empty() {
            errors.add_field_error("phone", "Phone is required".to_string());
        } else if !self.phone.chars().all(|c| c.is_ascii_digit()) || self.phone.len() != 10 {
            errors.add_field_error("phone", "Phone must be 10 digits".to_string());
        }
        
        if self.address.is_empty() {
            errors.add_field_error("address", "Address is required".to_string());
        } else if self.address.len() < 10 {
            errors.add_field_error("address", "Address must be at least 10 characters".to_string());
        }
        
        if self.city.is_empty() {
            errors.add_field_error("city", "City is required".to_string());
        } else if self.city.len() < 2 {
            errors.add_field_error("city", "City must be at least 2 characters".to_string());
        }
        
        // Step 3 validation
        if self.theme.is_empty() {
            errors.add_field_error("theme", "Theme is required".to_string());
        }
        
        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }
    
    fn default_values() -> Self {
        Self::default()
    }
    
    fn schema() -> FormSchema {
        FormSchema {
            name: "MultiStepTestForm".to_string(),
            field_metadata: Self::field_metadata(),
        }
    }
    
    fn get_field_value(&self, field_name: &str) -> FieldValue {
        match field_name {
            "first_name" => FieldValue::String(self.first_name.clone()),
            "last_name" => FieldValue::String(self.last_name.clone()),
            "email" => FieldValue::String(self.email.clone()),
            "phone" => FieldValue::String(self.phone.clone()),
            "address" => FieldValue::String(self.address.clone()),
            "city" => FieldValue::String(self.city.clone()),
            "newsletter" => FieldValue::Boolean(self.newsletter),
            "notifications" => FieldValue::Boolean(self.notifications),
            "theme" => FieldValue::String(self.theme.clone()),
            _ => FieldValue::Null,
        }
    }
    
    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "first_name" => {
                if let FieldValue::String(s) = value {
                    self.first_name = s;
                }
            }
            "last_name" => {
                if let FieldValue::String(s) = value {
                    self.last_name = s;
                }
            }
            "email" => {
                if let FieldValue::String(s) = value {
                    self.email = s;
                }
            }
            "phone" => {
                if let FieldValue::String(s) = value {
                    self.phone = s;
                }
            }
            "address" => {
                if let FieldValue::String(s) = value {
                    self.address = s;
                }
            }
            "city" => {
                if let FieldValue::String(s) = value {
                    self.city = s;
                }
            }
            "newsletter" => {
                if let FieldValue::Boolean(b) = value {
                    self.newsletter = b;
                }
            }
            "notifications" => {
                if let FieldValue::Boolean(b) = value {
                    self.notifications = b;
                }
            }
            "theme" => {
                if let FieldValue::String(s) = value {
                    self.theme = s;
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::GetUntracked;
    
    #[test]
    fn test_multi_step_form_initialization() {
        let form = MultiStepTestForm::default();
        assert_eq!(form.first_name, "");
        assert_eq!(form.last_name, "");
        assert_eq!(form.email, "");
        assert_eq!(form.phone, "");
        assert_eq!(form.address, "");
        assert_eq!(form.city, "");
        assert_eq!(form.newsletter, false);
        assert_eq!(form.notifications, false);
        assert_eq!(form.theme, "light");
    }
    
    #[test]
    fn test_multi_step_form_field_metadata() {
        let metadata = MultiStepTestForm::field_metadata();
        assert_eq!(metadata.len(), 9);
        
        // Check Step 1 fields
        assert!(metadata.iter().any(|f| f.name == "first_name"));
        assert!(metadata.iter().any(|f| f.name == "last_name"));
        assert!(metadata.iter().any(|f| f.name == "email"));
        
        // Check Step 2 fields
        assert!(metadata.iter().any(|f| f.name == "phone"));
        assert!(metadata.iter().any(|f| f.name == "address"));
        assert!(metadata.iter().any(|f| f.name == "city"));
        
        // Check Step 3 fields
        assert!(metadata.iter().any(|f| f.name == "newsletter"));
        assert!(metadata.iter().any(|f| f.name == "notifications"));
        assert!(metadata.iter().any(|f| f.name == "theme"));
    }
    
    #[test]
    fn test_multi_step_form_validation_step1() {
        let mut form = MultiStepTestForm::default();
        
        // Test empty form - should fail
        let result = form.validate();
        assert!(result.is_err());
        
        // Fill Step 1 fields
        form.first_name = "John".to_string();
        form.last_name = "Doe".to_string();
        form.email = "john@example.com".to_string();
        
        // Clear theme to make validation fail
        form.theme = String::new();
        
        // Should still fail because Step 2 and 3 fields are empty
        let result = form.validate();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_multi_step_form_validation_step2() {
        let mut form = MultiStepTestForm::default();
        
        // Fill Step 1 fields
        form.first_name = "John".to_string();
        form.last_name = "Doe".to_string();
        form.email = "john@example.com".to_string();
        
        // Fill Step 2 fields
        form.phone = "1234567890".to_string();
        form.address = "123 Main Street".to_string();
        form.city = "New York".to_string();
        
        // Clear theme to make validation fail
        form.theme = String::new();
        
        // Should still fail because Step 3 fields are empty
        let result = form.validate();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_multi_step_form_validation_complete() {
        let mut form = MultiStepTestForm::default();
        
        // Fill all fields
        form.first_name = "John".to_string();
        form.last_name = "Doe".to_string();
        form.email = "john@example.com".to_string();
        form.phone = "1234567890".to_string();
        form.address = "123 Main Street".to_string();
        form.city = "New York".to_string();
        form.newsletter = true;
        form.notifications = false;
        form.theme = "dark".to_string();
        
        // Should pass validation
        let result = form.validate();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_multi_step_form_field_values() {
        let mut form = MultiStepTestForm::default();
        form.first_name = "Jane".to_string();
        form.email = "jane@example.com".to_string();
        form.newsletter = true;
        
        assert_eq!(form.get_field_value("first_name"), FieldValue::String("Jane".to_string()));
        assert_eq!(form.get_field_value("email"), FieldValue::String("jane@example.com".to_string()));
        assert_eq!(form.get_field_value("newsletter"), FieldValue::Boolean(true));
        assert_eq!(form.get_field_value("nonexistent"), FieldValue::Null);
    }
    
    #[test]
    fn test_multi_step_form_set_field_values() {
        let mut form = MultiStepTestForm::default();
        
        form.set_field_value("first_name", FieldValue::String("Alice".to_string()));
        form.set_field_value("email", FieldValue::String("alice@example.com".to_string()));
        form.set_field_value("newsletter", FieldValue::Boolean(true));
        
        assert_eq!(form.first_name, "Alice");
        assert_eq!(form.email, "alice@example.com");
        assert_eq!(form.newsletter, true);
    }
    
    #[test]
    fn test_multi_step_form_handle_creation() {
        let (form_handle, _submit, _reset) = use_form(MultiStepTestForm::default());
        let form_data = form_handle.values().get_untracked();
        
        assert_eq!(form_data.first_name, "");
        assert_eq!(form_data.last_name, "");
        assert_eq!(form_data.email, "");
        assert_eq!(form_data.phone, "");
        assert_eq!(form_data.address, "");
        assert_eq!(form_data.city, "");
        assert_eq!(form_data.newsletter, false);
        assert_eq!(form_data.notifications, false);
        assert_eq!(form_data.theme, "light");
    }
    
    #[test]
    fn test_multi_step_form_handle_field_updates() {
        let (form_handle, _submit, _reset) = use_form(MultiStepTestForm::default());
        
        // Update Step 1 fields
        form_handle.set_field_value("first_name", FieldValue::String("Bob".to_string()));
        form_handle.set_field_value("last_name", FieldValue::String("Smith".to_string()));
        form_handle.set_field_value("email", FieldValue::String("bob@example.com".to_string()));
        
        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.first_name, "Bob");
        assert_eq!(form_data.last_name, "Smith");
        assert_eq!(form_data.email, "bob@example.com");
        
        // Update Step 2 fields
        form_handle.set_field_value("phone", FieldValue::String("9876543210".to_string()));
        form_handle.set_field_value("address", FieldValue::String("456 Oak Avenue".to_string()));
        form_handle.set_field_value("city", FieldValue::String("Los Angeles".to_string()));
        
        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.phone, "9876543210");
        assert_eq!(form_data.address, "456 Oak Avenue");
        assert_eq!(form_data.city, "Los Angeles");
        
        // Update Step 3 fields
        form_handle.set_field_value("newsletter", FieldValue::Boolean(true));
        form_handle.set_field_value("notifications", FieldValue::Boolean(true));
        form_handle.set_field_value("theme", FieldValue::String("auto".to_string()));
        
        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.newsletter, true);
        assert_eq!(form_data.notifications, true);
        assert_eq!(form_data.theme, "auto");
    }
    
    #[test]
    fn test_multi_step_form_validation_errors() {
        let (form_handle, _submit, _reset) = use_form(MultiStepTestForm::default());
        
        // Try to validate empty form
        let result = form_handle.validate();
        assert!(result.is_err());
        
        if let Err(_errors) = result {
            // The form validation failed as expected
            // We can't easily check specific field errors from FormError in this test
            // since FormError doesn't expose field_errors directly
        }
    }
    
    #[test]
    fn test_multi_step_form_step_validation() {
        let (form_handle, _submit, _reset) = use_form(MultiStepTestForm::default());
        
        // Fill Step 1 only
        form_handle.set_field_value("first_name", FieldValue::String("Charlie".to_string()));
        form_handle.set_field_value("last_name", FieldValue::String("Brown".to_string()));
        form_handle.set_field_value("email", FieldValue::String("charlie@example.com".to_string()));
        
        // Clear theme to make validation fail
        form_handle.set_field_value("theme", FieldValue::String(String::new()));
        
        // Should still fail validation
        let result = form_handle.validate();
        assert!(result.is_err());
        
        // Fill Step 2
        form_handle.set_field_value("phone", FieldValue::String("5555555555".to_string()));
        form_handle.set_field_value("address", FieldValue::String("789 Pine Street".to_string()));
        form_handle.set_field_value("city", FieldValue::String("Chicago".to_string()));
        
        // Theme is still empty, so should still fail validation
        let result = form_handle.validate();
        assert!(result.is_err());
        
        // Fill Step 3
        form_handle.set_field_value("newsletter", FieldValue::Boolean(false));
        form_handle.set_field_value("notifications", FieldValue::Boolean(true));
        form_handle.set_field_value("theme", FieldValue::String("light".to_string()));
        
        // Should now pass validation
        let result = form_handle.validate();
        assert!(result.is_ok());
    }
}
