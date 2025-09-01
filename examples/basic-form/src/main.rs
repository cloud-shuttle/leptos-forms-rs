use leptos::prelude::*;
use leptos_forms_rs::*;
use leptos_forms_rs::core::types::{FieldType, FieldValue, ValidatorConfig};
use leptos_forms_rs::core::traits::{FieldMetadata, FormSchema};
use leptos_forms_rs::validation::ValidationErrors;
use leptos_forms_rs::core::types::FieldError;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Define a simple login form without the derive macro
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
struct LoginForm {
    email: String,
    password: String,
    remember_me: bool,
}

// Manual implementation of the Form trait
impl Form for LoginForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
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
                name: "remember_me".to_string(),
                field_type: FieldType::Boolean,
                validators: vec![],
                is_required: false,
                default_value: Some(FieldValue::Boolean(true)),
                dependencies: vec![],
                attributes: HashMap::new(),
            },
        ]
    }
    
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        
        // Validate email
        if self.email.is_empty() {
            errors.add_field_error("email".to_string(), "Email is required".to_string());
        } else if !self.email.contains('@') {
            errors.add_field_error("email".to_string(), "Invalid email format".to_string());
        }
        
        // Validate password
        if self.password.len() < 8 {
            errors.add_field_error("password".to_string(), "Password must be at least 8 characters".to_string());
        }
        
        if errors.is_empty() {
            Ok::<(), ValidationErrors>(())
        } else {
            Err(errors)
        }
    }
    
    fn get_field(&self, name: &str) -> Option<FieldValue> {
        match name {
            "email" => Some(FieldValue::String(self.email.clone())),
            "password" => Some(FieldValue::String(self.password.clone())),
            "remember_me" => Some(FieldValue::Boolean(self.remember_me)),
            _ => None,
        }
    }
    
    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), FieldError> {
        match name {
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
            "remember_me" => {
                if let FieldValue::Boolean(b) = value {
                    self.remember_me = b;
                    Ok(())
                } else {
                    Err(FieldError::new(name.to_string(), "Expected boolean value".to_string()))
                }
            }
            _ => Err(FieldError::new(name.to_string(), "Unknown field".to_string())),
        }
    }
    
    fn default_values() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            remember_me: true,
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
fn LoginPage() -> impl IntoView {
    let form = use_form::<LoginForm>();
    
    let form_clone = form.clone();
    let handle_submit = move |_| {
        let form_data = form_clone.get_values().get();
        log::info!("Form submitted: {:?}", form_data);
        // In a real app, you would send this to your backend
        if let Some(window) = web_sys::window() {
            let _ = window.alert_with_message(&format!("Login successful for: {}", form_data.email));
        }
    };
    
    let form_clone2 = form.clone();
    let form_clone3 = form.clone();
    
    view! {
        <div class="container">
            <h1>"Login Form Example"</h1>
            <p>"This demonstrates the basic functionality of Leptos Forms RS."</p>
            
            <div class="form">
                <div class="form-field">
                    <label for="email">"Email Address"</label>
                    <input 
                        type="email" 
                        id="email" 
                        placeholder="Enter your email"
                        required=true
                    />
                    <small>"We'll never share your email with anyone else."</small>
                </div>
                
                <div class="form-field">
                    <label for="password">"Password"</label>
                    <input 
                        type="password" 
                        id="password" 
                        placeholder="Enter your password"
                        required=true
                    />
                    <small>"Must be at least 8 characters long."</small>
                </div>
                
                <div class="form-field">
                    <label>
                        <input type="checkbox" id="remember_me" />
                        "Remember me"
                    </label>
                </div>
                
                <div class="form-actions">
                    <button class="btn btn-primary" on:click=move |_| {
                        handle_submit(());
                    }>
                        "Login"
                    </button>
                    
                    <button class="btn btn-secondary" on:click=move |_| {
                        // Reset form to default values
                        let default_values = LoginForm::default_values();
                        let _ = form_clone2.set_field_value("email", FieldValue::String(default_values.email));
                        let _ = form_clone2.set_field_value("password", FieldValue::String(default_values.password));
                        let _ = form_clone2.set_field_value("remember_me", FieldValue::Boolean(default_values.remember_me));
                    }>
                        "Reset"
                    </button>
                </div>
                
                <div class="form-debug">
                    <h3>"Form Debug Info"</h3>
                    <p>"Form is working! This demonstrates the current API."</p>
                    <p>"Form values: " {move || format!("{:?}", form_clone3.get_values().get())}</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app">
            <LoginPage />
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
