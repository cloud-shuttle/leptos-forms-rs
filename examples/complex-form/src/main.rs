use leptos::prelude::*;
use leptos_forms_rs::*;
use leptos_forms_rs::core::types::{FieldType, FieldValue, ValidatorConfig};
use leptos_forms_rs::core::traits::{FieldMetadata, FormSchema};
use leptos_forms_rs::validation::ValidationErrors;
use leptos_forms_rs::core::types::FieldError;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Define a complex user registration form using manual implementation
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
struct UserRegistrationForm {
    // Personal Information
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    confirm_password: String,
    
    // Contact Information
    phone: Option<String>,
    website: Option<String>,
    
    // Address Information
    street_address: String,
    city: String,
    state: String,
    postal_code: String,
    country: String,
    
    // Preferences
    newsletter: bool,
    marketing_emails: bool,
    language: String,
}

// Manual implementation of the Form trait
impl Form for UserRegistrationForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "first_name".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "last_name".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "email".to_string(),
                field_type: FieldType::Email,
                validators: vec![ValidatorConfig::Required, ValidatorConfig::Email],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "password".to_string(),
                field_type: FieldType::Password,
                validators: vec![ValidatorConfig::Required, ValidatorConfig::MinLength(8)],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "confirm_password".to_string(),
                field_type: FieldType::Password,
                validators: vec![ValidatorConfig::Required, ValidatorConfig::MinLength(8)],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "phone".to_string(),
                field_type: FieldType::Text,
                validators: vec![],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "website".to_string(),
                field_type: FieldType::Text,
                validators: vec![],
                is_required: false,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "street_address".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "city".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "state".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "postal_code".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "country".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "newsletter".to_string(),
                field_type: FieldType::Boolean,
                validators: vec![],
                is_required: false,
                default_value: Some(FieldValue::Boolean(false)),
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "marketing_emails".to_string(),
                field_type: FieldType::Boolean,
                validators: vec![],
                is_required: false,
                default_value: Some(FieldValue::Boolean(false)),
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "language".to_string(),
                field_type: FieldType::Text,
                validators: vec![],
                is_required: false,
                default_value: Some(FieldValue::String("en".to_string())),
                dependencies: vec![],
                attributes: HashMap::new(),
            },
        ]
    }
    
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        
        // Basic validation
        if self.first_name.is_empty() {
            errors.add_field_error("first_name".to_string(), "First name is required".to_string());
        }
        
        if self.last_name.is_empty() {
            errors.add_field_error("last_name".to_string(), "Last name is required".to_string());
        }
        
        if self.email.is_empty() {
            errors.add_field_error("email".to_string(), "Email is required".to_string());
        } else if !self.email.contains('@') {
            errors.add_field_error("email".to_string(), "Invalid email format".to_string());
        }
        
        if self.password.len() < 8 {
            errors.add_field_error("password".to_string(), "Password must be at least 8 characters".to_string());
        }
        
        if self.password != self.confirm_password {
            errors.add_field_error("confirm_password".to_string(), "Passwords do not match".to_string());
        }
        
        if self.street_address.is_empty() {
            errors.add_field_error("street_address".to_string(), "Street address is required".to_string());
        }
        
        if self.city.is_empty() {
            errors.add_field_error("city".to_string(), "City is required".to_string());
        }
        
        if self.state.is_empty() {
            errors.add_field_error("state".to_string(), "State is required".to_string());
        }
        
        if self.postal_code.is_empty() {
            errors.add_field_error("postal_code".to_string(), "Postal code is required".to_string());
        }
        
        if self.country.is_empty() {
            errors.add_field_error("country".to_string(), "Country is required".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    fn get_field(&self, name: &str) -> Option<FieldValue> {
        match name {
            "first_name" => Some(FieldValue::String(self.first_name.clone())),
            "last_name" => Some(FieldValue::String(self.last_name.clone())),
            "email" => Some(FieldValue::String(self.email.clone())),
            "password" => Some(FieldValue::String(self.password.clone())),
            "confirm_password" => Some(FieldValue::String(self.confirm_password.clone())),
            "phone" => self.phone.as_ref().map(|p| FieldValue::String(p.clone())),
            "website" => self.website.as_ref().map(|w| FieldValue::String(w.clone())),
            "street_address" => Some(FieldValue::String(self.street_address.clone())),
            "city" => Some(FieldValue::String(self.city.clone())),
            "state" => Some(FieldValue::String(self.state.clone())),
            "postal_code" => Some(FieldValue::String(self.postal_code.clone())),
            "country" => Some(FieldValue::String(self.country.clone())),
            "newsletter" => Some(FieldValue::Boolean(self.newsletter)),
            "marketing_emails" => Some(FieldValue::Boolean(self.marketing_emails)),
            "language" => Some(FieldValue::String(self.language.clone())),
            _ => None,
        }
    }
    
    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), FieldError> {
        match name {
            "first_name" => {
                if let FieldValue::String(s) = value {
                    self.first_name = s;
                    Ok(())
                } else {
                    Err(FieldError::new(name.to_string(), "Expected string value".to_string()))
                }
            }
            "last_name" => {
                if let FieldValue::String(s) = value {
                    self.last_name = s;
                    Ok(())
                } else {
                    Err(FieldError::new(name.to_string(), "Expected string value".to_string()))
                }
            }
            "email" => {
                if let FieldValue::String(s) = value {
                    self.email = s;
                    Ok(())
                } else {
                    Err(FieldError::new(name.to_string(), "Expected string value".to_string()))
                }
            }
            "password" => {
                if let FieldValue::String(s) = value {
                    self.password = s;
                    Ok(())
                } else {
                    Err(FieldError::new(name.to_string(), "Expected string value".to_string()))
                }
            }
            "confirm_password" => {
                if let FieldValue::String(s) = value {
                    self.confirm_password = s;
                    Ok(())
                } else {
                    Err(FieldError::new(name.to_string(), "Expected string value".to_string()))
                }
            }
            "phone" => {
                if let FieldValue::String(s) = value {
                    self.phone = Some(s);
                    Ok(())
                } else {
                    Err(FieldError::new(name.to_string(), "Expected string value".to_string()))
                }
            }
            "website" => {
                if let FieldValue::String(s) = value {
                    self.website = Some(s);
                    Ok(())
                } else {
                    Err(FieldError::new(name.to_string(), "Expected string value".to_string()))
                }
            }
            "street_address" => {
                if let FieldValue::String(s) = value {
                    self.street_address = s;
                    Ok(())
                } else {
                    Err(FieldError::new(name.to_string(), "Expected string value".to_string()))
                }
            }
            "city" => {
                if let FieldValue::String(s) = value {
                    self.city = s;
                    Ok(())
                } else {
                    Err(FieldError::new(name.to_string(), "Expected string value".to_string()))
                }
            }
            "state" => {
                if let FieldValue::String(s) = value {
                    self.state = s;
                    Ok(())
                } else {
                    Err(FieldError::new(name.to_string(), "Expected string value".to_string()))
                }
            }
            "postal_code" => {
                if let FieldValue::String(s) = value {
                    self.postal_code = s;
                    Ok(())
                } else {
                    Err(FieldError::new(name.to_string(), "Expected string value".to_string()))
                }
            }
            "country" => {
                if let FieldValue::String(s) = value {
                    self.country = s;
                    Ok(())
                } else {
                    Err(FieldError::new(name.to_string(), "Expected string value".to_string()))
                }
            }
            "newsletter" => {
                if let FieldValue::Boolean(b) = value {
                    self.newsletter = b;
                    Ok(())
                } else {
                    Err(FieldError::new(name.to_string(), "Expected boolean value".to_string()))
                }
            }
            "marketing_emails" => {
                if let FieldValue::Boolean(b) = value {
                    self.marketing_emails = b;
                    Ok(())
                } else {
                    Err(FieldError::new(name.to_string(), "Expected boolean value".to_string()))
                }
            }
            "language" => {
                if let FieldValue::String(s) = value {
                    self.language = s;
                    Ok(())
                } else {
                    Err(FieldError::new(name.to_string(), "Expected string value".to_string()))
                }
            }
            _ => Err(FieldError::new(name.to_string(), "Unknown field".to_string())),
        }
    }
    
    fn default_values() -> Self {
        Self {
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            phone: None,
            website: None,
            street_address: String::new(),
            city: String::new(),
            state: String::new(),
            postal_code: String::new(),
            country: String::new(),
            newsletter: false,
            marketing_emails: false,
            language: "en".to_string(),
        }
    }
    
    fn schema() -> FormSchema {
        let mut schema = FormSchema::new();
        for field in Self::field_metadata() {
            schema.add_field(field);
        }
        schema
    }
}

#[component]
fn UserRegistrationPage() -> impl IntoView {
    let form = use_form::<UserRegistrationForm>();
    
    let form_clone = form.clone();
    let handle_submit = move |_| {
        let form_data = form_clone.get_values().get();
        log::info!("Form submitted: {:?}", form_data);
        // In a real app, you would send this to your backend
        if let Some(window) = web_sys::window() {
            let _ = window.alert_with_message(&format!("Registration successful for: {} {}", form_data.first_name, form_data.last_name));
        }
    };
    
    view! {
        <div class="container">
            <h1>"User Registration Form Example"</h1>
            <p>"This demonstrates the complex functionality of Leptos Forms RS with Leptos 0.8."</p>
            
            <div class="form">
                <div class="form-section">
                    <h2>"Personal Information"</h2>
                    
                    <div class="form-row">
                        <div class="form-field">
                            <label for="first_name">"First Name *"</label>
                            <input type="text" id="first_name" required=true />
                        </div>
                        
                        <div class="form-field">
                            <label for="last_name">"Last Name *"</label>
                            <input type="text" id="last_name" required=true />
                        </div>
                    </div>
                    
                    <div class="form-field">
                        <label for="email">"Email Address *"</label>
                        <input type="email" id="email" required=true />
                    </div>
                    
                    <div class="form-row">
                        <div class="form-field">
                            <label for="password">"Password *"</label>
                            <input type="password" id="password" required=true />
                            <small>"Must be at least 8 characters long."</small>
                        </div>
                        
                        <div class="form-field">
                            <label for="confirm_password">"Confirm Password *"</label>
                            <input type="password" id="confirm_password" required=true />
                        </div>
                    </div>
                </div>
                
                <div class="form-section">
                    <h2>"Contact Information"</h2>
                    
                    <div class="form-row">
                        <div class="form-field">
                            <label for="phone">"Phone Number"</label>
                            <input type="tel" id="phone" />
                        </div>
                        
                        <div class="form-field">
                            <label for="website">"Website"</label>
                            <input type="url" id="website" />
                        </div>
                    </div>
                </div>
                
                <div class="form-section">
                    <h2>"Address Information"</h2>
                    
                    <div class="form-field">
                        <label for="street_address">"Street Address *"</label>
                        <input type="text" id="street_address" required=true />
                    </div>
                    
                    <div class="form-row">
                        <div class="form-field">
                            <label for="city">"City *"</label>
                            <input type="text" id="city" required=true />
                        </div>
                        
                        <div class="form-field">
                            <label for="state">"State/Province *"</label>
                            <input type="text" id="state" required=true />
                        </div>
                    </div>
                    
                    <div class="form-row">
                        <div class="form-field">
                            <label for="postal_code">"Postal Code *"</label>
                            <input type="text" id="postal_code" required=true />
                        </div>
                        
                        <div class="form-field">
                            <label for="country">"Country *"</label>
                            <input type="text" id="country" required=true />
                        </div>
                    </div>
                </div>
                
                <div class="form-section">
                    <h2>"Preferences"</h2>
                    
                    <div class="form-field">
                        <label>
                            <input type="checkbox" id="newsletter" />
                            "Subscribe to newsletter"
                        </label>
                    </div>
                    
                    <div class="form-field">
                        <label>
                            <input type="checkbox" id="marketing_emails" />
                            "Receive marketing emails"
                        </label>
                    </div>
                    
                    <div class="form-field">
                        <label for="language">"Preferred Language"</label>
                        <select id="language">
                            <option value="en" selected=true>"English"</option>
                            <option value="es">"Spanish"</option>
                            <option value="fr">"French"</option>
                            <option value="de">"German"</option>
                        </select>
                    </div>
                </div>
                
                <div class="form-actions">
                    <button class="btn btn-primary" on:click=move |_| {
                        handle_submit(());
                    }>
                        "Register"
                    </button>
                    
                    <button class="btn btn-secondary" on:click=move |_| {
                        // Reset form to default values
                        let _default_values = UserRegistrationForm::default_values();
                        // In a real implementation, you would reset all form fields
                        log::info!("Form reset to default values");
                    }>
                        "Reset"
                    </button>
                </div>
                
                <div class="form-debug">
                    <h3>"Form Debug Info"</h3>
                    <p>"Form is working! This demonstrates the current API."</p>
                    <p>"Form values: " {move || format!("{:?}", form.get_values().get())}</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app">
            <UserRegistrationPage />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");
    
    mount_to_body(|| view! { <App /> });
}

#[wasm_bindgen]
pub fn run_app() {
    main();
}
