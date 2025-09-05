use leptos::prelude::*;
use leptos_forms_rs::core::types::FieldValue;
use leptos_forms_rs::core::FormHandle;
use leptos_forms_rs::core::Form;
use leptos_forms_rs::validation::ValidationErrors;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ConditionalForm {
    account_type: String,           // "personal" or "business"
    company_name: Option<String>,   // Required only if account_type is "business"
    tax_id: Option<String>,         // Required only if account_type is "business"
    personal_id: Option<String>,    // Required only if account_type is "personal"
    email: String,                  // Always required
}

impl Form for ConditionalForm {
    fn field_metadata() -> Vec<leptos_forms_rs::core::FieldMetadata> {
        vec![
            leptos_forms_rs::core::FieldMetadata {
                name: "account_type".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Select(
                    vec![
                        leptos_forms_rs::core::types::SelectOption {
                            value: "personal".to_string(),
                            label: "Personal Account".to_string(),
                            disabled: false,
                        },
                        leptos_forms_rs::core::types::SelectOption {
                            value: "business".to_string(),
                            label: "Business Account".to_string(),
                            disabled: false,
                        },
                    ]
                ),
                validators: vec![],
                is_required: true,
                default_value: Some(FieldValue::String("personal".to_string())),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "company_name".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Text,
                validators: vec![],
                is_required: false, // Will be conditionally required
                default_value: Some(FieldValue::String("".to_string())),
                dependencies: vec!["account_type".to_string()],
                attributes: std::collections::HashMap::new(),
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "tax_id".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Text,
                validators: vec![],
                is_required: false, // Will be conditionally required
                default_value: Some(FieldValue::String("".to_string())),
                dependencies: vec!["account_type".to_string()],
                attributes: std::collections::HashMap::new(),
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "personal_id".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Text,
                validators: vec![],
                is_required: false, // Will be conditionally required
                default_value: Some(FieldValue::String("".to_string())),
                dependencies: vec!["account_type".to_string()],
                attributes: std::collections::HashMap::new(),
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "email".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Email,
                validators: vec![],
                is_required: true,
                default_value: Some(FieldValue::String("".to_string())),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
        ]
    }
    
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        
        // Always required fields
        if self.email.trim().is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        }
        
        if self.account_type.trim().is_empty() {
            errors.add_field_error("account_type", "Account type is required".to_string());
        }
        
        // Conditional validation based on account type
        match self.account_type.as_str() {
            "business" => {
                if self.company_name.as_ref().map_or(true, |s| s.trim().is_empty()) {
                    errors.add_field_error("company_name", "Company name is required for business accounts".to_string());
                }
                if self.tax_id.as_ref().map_or(true, |s| s.trim().is_empty()) {
                    errors.add_field_error("tax_id", "Tax ID is required for business accounts".to_string());
                }
            }
            "personal" => {
                if self.personal_id.as_ref().map_or(true, |s| s.trim().is_empty()) {
                    errors.add_field_error("personal_id", "Personal ID is required for personal accounts".to_string());
                }
            }
            _ => {
                errors.add_field_error("account_type", "Invalid account type".to_string());
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    fn default_values() -> Self {
        Self {
            account_type: "personal".to_string(),
            company_name: None,
            tax_id: None,
            personal_id: None,
            email: "".to_string(),
        }
    }
    
    fn schema() -> leptos_forms_rs::core::FormSchema {
        leptos_forms_rs::core::FormSchema {
            name: "ConditionalForm".to_string(),
            field_metadata: Self::field_metadata(),
        }
    }
}

#[test]
fn test_conditional_validation_personal_account() {
    // Test personal account validation
    let form = ConditionalForm {
        account_type: "personal".to_string(),
        company_name: None,
        tax_id: None,
        personal_id: Some("12345".to_string()),
        email: "test@example.com".to_string(),
    };
    
    let result = form.validate();
    assert!(result.is_ok(), "Personal account should be valid with required fields");
}

#[test]
fn test_conditional_validation_business_account() {
    // Test business account validation
    let form = ConditionalForm {
        account_type: "business".to_string(),
        company_name: Some("Acme Corp".to_string()),
        tax_id: Some("TAX123456".to_string()),
        personal_id: None,
        email: "business@example.com".to_string(),
    };
    
    let result = form.validate();
    assert!(result.is_ok(), "Business account should be valid with required fields");
}

#[test]
fn test_conditional_validation_business_missing_fields() {
    // Test business account with missing required fields
    let form = ConditionalForm {
        account_type: "business".to_string(),
        company_name: None, // Missing required field
        tax_id: Some("TAX123456".to_string()),
        personal_id: None,
        email: "business@example.com".to_string(),
    };
    
    let result = form.validate();
    assert!(result.is_err(), "Business account should fail validation without company name");
    
    if let Err(errors) = result {
        assert!(errors.has_field_error("company_name"), "Should have company_name error");
        assert!(!errors.has_field_error("personal_id"), "Should not have personal_id error for business account");
    }
}

#[test]
fn test_conditional_validation_personal_missing_fields() {
    // Test personal account with missing required fields
    let form = ConditionalForm {
        account_type: "personal".to_string(),
        company_name: None,
        tax_id: None,
        personal_id: None, // Missing required field
        email: "personal@example.com".to_string(),
    };
    
    let result = form.validate();
    assert!(result.is_err(), "Personal account should fail validation without personal ID");
    
    if let Err(errors) = result {
        assert!(errors.has_field_error("personal_id"), "Should have personal_id error");
        assert!(!errors.has_field_error("company_name"), "Should not have company_name error for personal account");
    }
}

#[test]
fn test_conditional_validation_invalid_account_type() {
    // Test with invalid account type
    let form = ConditionalForm {
        account_type: "invalid".to_string(),
        company_name: None,
        tax_id: None,
        personal_id: None,
        email: "test@example.com".to_string(),
    };
    
    let result = form.validate();
    assert!(result.is_err(), "Invalid account type should fail validation");
    
    if let Err(errors) = result {
        assert!(errors.has_field_error("account_type"), "Should have account_type error");
    }
}

#[test]
fn test_field_dependencies_metadata() {
    // Test that field dependencies are correctly defined
    let metadata = ConditionalForm::field_metadata();
    
    let company_name_field = metadata.iter().find(|f| f.name == "company_name").unwrap();
    assert!(company_name_field.dependencies.contains(&"account_type".to_string()));
    
    let tax_id_field = metadata.iter().find(|f| f.name == "tax_id").unwrap();
    assert!(tax_id_field.dependencies.contains(&"account_type".to_string()));
    
    let personal_id_field = metadata.iter().find(|f| f.name == "personal_id").unwrap();
    assert!(personal_id_field.dependencies.contains(&"account_type".to_string()));
    
    let email_field = metadata.iter().find(|f| f.name == "email").unwrap();
    assert!(email_field.dependencies.is_empty(), "Email should have no dependencies");
}
