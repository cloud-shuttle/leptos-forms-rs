use leptos::prelude::*;
use leptos_forms_rs::core::{Form, FormHandle, FieldMetadata, FormSchema, FieldValue, FieldType, NumberType};
use leptos_forms_rs::validation::{ValidationErrors, Validator};
use leptos_forms_rs::hooks::{use_form, use_form_wizard};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// Test form for wizard functionality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WizardTestForm {
    // Step 1: Basic Info
    pub name: String,
    pub age: i64,
    
    // Step 2: Contact
    pub email: String,
    pub phone: String,
    
    // Step 3: Preferences
    pub newsletter: bool,
    pub theme: String,
    
    // Step 4: Review
    pub terms_accepted: bool,
}

impl Default for WizardTestForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            age: 0,
            email: String::new(),
            phone: String::new(),
            newsletter: false,
            theme: "light".to_string(),
            terms_accepted: false,
        }
    }
}

impl Form for WizardTestForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        let mut metadata = Vec::new();
        
        // Step 1 fields
        metadata.push(FieldMetadata {
            name: "name".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::MinLength(2)],
        });
        
        metadata.push(FieldMetadata {
            name: "age".to_string(),
            field_type: FieldType::Number(NumberType { min: Some(18.0), max: Some(120.0), step: Some(1.0) }),
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::Min(18.0), Validator::Max(120.0)],
        });
        
        // Step 2 fields
        metadata.push(FieldMetadata {
            name: "email".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::Email],
        });
        
        metadata.push(FieldMetadata {
            name: "phone".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required, Validator::Pattern(r"^\d{10}$".to_string())],
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
            name: "theme".to_string(),
            field_type: FieldType::Text,
            is_required: true,
            default_value: None,
            dependencies: Vec::new(),
            attributes: HashMap::new(),
            validators: vec![Validator::Required],
        });
        
        // Step 4 fields
        metadata.push(FieldMetadata {
            name: "terms_accepted".to_string(),
            field_type: FieldType::Boolean,
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
        if self.name.is_empty() {
            errors.add_field_error("name", "Name is required".to_string());
        } else if self.name.len() < 2 {
            errors.add_field_error("name", "Name must be at least 2 characters".to_string());
        }
        
        if self.age < 18 {
            errors.add_field_error("age", "Age must be at least 18".to_string());
        } else if self.age > 120 {
            errors.add_field_error("age", "Age must be at most 120".to_string());
        }
        
        // Step 2 validation
        if self.email.is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        } else if !self.email.contains('@') {
            errors.add_field_error("email", "Invalid email format".to_string());
        }
        
        if self.phone.is_empty() {
            errors.add_field_error("phone", "Phone is required".to_string());
        } else if !self.phone.chars().all(|c| c.is_ascii_digit()) || self.phone.len() != 10 {
            errors.add_field_error("phone", "Phone must be 10 digits".to_string());
        }
        
        // Step 3 validation
        if self.theme.is_empty() {
            errors.add_field_error("theme", "Theme is required".to_string());
        }
        
        // Step 4 validation
        if !self.terms_accepted {
            errors.add_field_error("terms_accepted", "Terms must be accepted".to_string());
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
            name: "WizardTestForm".to_string(),
            field_metadata: Self::field_metadata(),
        }
    }
    
    fn get_field_value(&self, field_name: &str) -> FieldValue {
        match field_name {
            "name" => FieldValue::String(self.name.clone()),
            "age" => FieldValue::Integer(self.age),
            "email" => FieldValue::String(self.email.clone()),
            "phone" => FieldValue::String(self.phone.clone()),
            "newsletter" => FieldValue::Boolean(self.newsletter),
            "theme" => FieldValue::String(self.theme.clone()),
            "terms_accepted" => FieldValue::Boolean(self.terms_accepted),
            _ => FieldValue::Null,
        }
    }
    
    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "name" => {
                if let FieldValue::String(s) = value {
                    self.name = s;
                }
            }
            "age" => {
                if let FieldValue::Integer(i) = value {
                    self.age = i;
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
            "newsletter" => {
                if let FieldValue::Boolean(b) = value {
                    self.newsletter = b;
                }
            }
            "theme" => {
                if let FieldValue::String(s) = value {
                    self.theme = s;
                }
            }
            "terms_accepted" => {
                if let FieldValue::Boolean(b) = value {
                    self.terms_accepted = b;
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
    fn test_wizard_form_initialization() {
        let form = WizardTestForm::default();
        assert_eq!(form.name, "");
        assert_eq!(form.age, 0);
        assert_eq!(form.email, "");
        assert_eq!(form.phone, "");
        assert_eq!(form.newsletter, false);
        assert_eq!(form.theme, "light");
        assert_eq!(form.terms_accepted, false);
    }
    
    #[test]
    fn test_wizard_form_field_metadata() {
        let metadata = WizardTestForm::field_metadata();
        assert_eq!(metadata.len(), 7);
        
        // Check all fields exist
        assert!(metadata.iter().any(|f| f.name == "name"));
        assert!(metadata.iter().any(|f| f.name == "age"));
        assert!(metadata.iter().any(|f| f.name == "email"));
        assert!(metadata.iter().any(|f| f.name == "phone"));
        assert!(metadata.iter().any(|f| f.name == "newsletter"));
        assert!(metadata.iter().any(|f| f.name == "theme"));
        assert!(metadata.iter().any(|f| f.name == "terms_accepted"));
    }
    
    #[test]
    fn test_wizard_form_validation_step1() {
        let mut form = WizardTestForm::default();
        
        // Test empty form - should fail
        let result = form.validate();
        assert!(result.is_err());
        
        // Fill Step 1 fields
        form.name = "John Doe".to_string();
        form.age = 25;
        
        // Should still fail because other steps are empty
        let result = form.validate();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_wizard_form_validation_step2() {
        let mut form = WizardTestForm::default();
        
        // Fill Step 1 fields
        form.name = "John Doe".to_string();
        form.age = 25;
        
        // Fill Step 2 fields
        form.email = "john@example.com".to_string();
        form.phone = "1234567890".to_string();
        
        // Should still fail because other steps are empty
        let result = form.validate();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_wizard_form_validation_step3() {
        let mut form = WizardTestForm::default();
        
        // Fill Step 1 fields
        form.name = "John Doe".to_string();
        form.age = 25;
        
        // Fill Step 2 fields
        form.email = "john@example.com".to_string();
        form.phone = "1234567890".to_string();
        
        // Fill Step 3 fields
        form.newsletter = true;
        form.theme = "dark".to_string();
        
        // Should still fail because Step 4 is empty
        let result = form.validate();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_wizard_form_validation_complete() {
        let mut form = WizardTestForm::default();
        
        // Fill all fields
        form.name = "John Doe".to_string();
        form.age = 25;
        form.email = "john@example.com".to_string();
        form.phone = "1234567890".to_string();
        form.newsletter = true;
        form.theme = "dark".to_string();
        form.terms_accepted = true;
        
        // Should pass validation
        let result = form.validate();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_wizard_form_handle_creation() {
        let (form_handle, _submit, _reset) = use_form(WizardTestForm::default());
        let form_data = form_handle.values().get_untracked();
        
        assert_eq!(form_data.name, "");
        assert_eq!(form_data.age, 0);
        assert_eq!(form_data.email, "");
        assert_eq!(form_data.phone, "");
        assert_eq!(form_data.newsletter, false);
        assert_eq!(form_data.theme, "light");
        assert_eq!(form_data.terms_accepted, false);
    }
    
    #[test]
    fn test_wizard_form_handle_field_updates() {
        let (form_handle, _submit, _reset) = use_form(WizardTestForm::default());
        
        // Update Step 1 fields
        form_handle.set_field_value("name", FieldValue::String("Jane Smith".to_string()));
        form_handle.set_field_value("age", FieldValue::Integer(30));
        
        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.name, "Jane Smith");
        assert_eq!(form_data.age, 30);
        
        // Update Step 2 fields
        form_handle.set_field_value("email", FieldValue::String("jane@example.com".to_string()));
        form_handle.set_field_value("phone", FieldValue::String("9876543210".to_string()));
        
        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.email, "jane@example.com");
        assert_eq!(form_data.phone, "9876543210");
        
        // Update Step 3 fields
        form_handle.set_field_value("newsletter", FieldValue::Boolean(true));
        form_handle.set_field_value("theme", FieldValue::String("auto".to_string()));
        
        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.newsletter, true);
        assert_eq!(form_data.theme, "auto");
        
        // Update Step 4 fields
        form_handle.set_field_value("terms_accepted", FieldValue::Boolean(true));
        
        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.terms_accepted, true);
    }
    
    #[test]
    fn test_wizard_form_validation_errors() {
        let (form_handle, _submit, _reset) = use_form(WizardTestForm::default());
        
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
    fn test_wizard_form_step_validation() {
        let (form_handle, _submit, _reset) = use_form(WizardTestForm::default());
        
        // Fill Step 1 only
        form_handle.set_field_value("name", FieldValue::String("Charlie Brown".to_string()));
        form_handle.set_field_value("age", FieldValue::Integer(35));
        
        // Should still fail validation
        let result = form_handle.validate();
        assert!(result.is_err());
        
        // Fill Step 2
        form_handle.set_field_value("email", FieldValue::String("charlie@example.com".to_string()));
        form_handle.set_field_value("phone", FieldValue::String("5555555555".to_string()));
        
        // Should still fail validation
        let result = form_handle.validate();
        assert!(result.is_err());
        
        // Fill Step 3
        form_handle.set_field_value("newsletter", FieldValue::Boolean(false));
        form_handle.set_field_value("theme", FieldValue::String("light".to_string()));
        
        // Should still fail validation
        let result = form_handle.validate();
        assert!(result.is_err());
        
        // Fill Step 4
        form_handle.set_field_value("terms_accepted", FieldValue::Boolean(true));
        
        // Should now pass validation
        let result = form_handle.validate();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_wizard_form_handle_validation() {
        let (form_handle, _submit, _reset) = use_form(WizardTestForm::default());
        
        // Fill all fields with valid data
        form_handle.set_field_value("name", FieldValue::String("Alice Johnson".to_string()));
        form_handle.set_field_value("age", FieldValue::Integer(28));
        form_handle.set_field_value("email", FieldValue::String("alice@example.com".to_string()));
        form_handle.set_field_value("phone", FieldValue::String("1111111111".to_string()));
        form_handle.set_field_value("newsletter", FieldValue::Boolean(true));
        form_handle.set_field_value("theme", FieldValue::String("dark".to_string()));
        form_handle.set_field_value("terms_accepted", FieldValue::Boolean(true));
        
        // Should pass validation
        let result = form_handle.validate();
        assert!(result.is_ok());
        
        // Verify all fields are set correctly
        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.name, "Alice Johnson");
        assert_eq!(form_data.age, 28);
        assert_eq!(form_data.email, "alice@example.com");
        assert_eq!(form_data.phone, "1111111111");
        assert_eq!(form_data.newsletter, true);
        assert_eq!(form_data.theme, "dark");
        assert_eq!(form_data.terms_accepted, true);
    }
    
    #[test]
    fn test_wizard_form_handle_invalid_data() {
        let (form_handle, _submit, _reset) = use_form(WizardTestForm::default());
        
        // Fill with invalid data
        form_handle.set_field_value("name", FieldValue::String("A".to_string())); // Too short
        form_handle.set_field_value("age", FieldValue::Integer(15)); // Too young
        form_handle.set_field_value("email", FieldValue::String("invalid-email".to_string())); // Invalid email
        form_handle.set_field_value("phone", FieldValue::String("123".to_string())); // Too short
        form_handle.set_field_value("newsletter", FieldValue::Boolean(false));
        form_handle.set_field_value("theme", FieldValue::String("".to_string())); // Empty
        form_handle.set_field_value("terms_accepted", FieldValue::Boolean(false)); // Not accepted
        
        // Should fail validation
        let result = form_handle.validate();
        assert!(result.is_err());
        
        if let Err(_errors) = result {
            // The form validation failed as expected
            // We can't easily check specific field errors from FormError in this test
            // since FormError doesn't expose field_errors directly
        }
    }
}
