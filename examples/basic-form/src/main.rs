use leptos::prelude::*;
use leptos_forms_rs::core::traits::{FieldMetadata, FormSchema};
// use leptos_forms_rs::core::types::FieldError; // Not needed for this example
use leptos_forms_rs::components::{FormField, FormSubmit}; // Now using the actual components!
use leptos_forms_rs::core::types::{FieldType, FieldValue};
use leptos_forms_rs::validation::ValidationErrors;
use leptos_forms_rs::validation::Validator;
use leptos_forms_rs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
                validators: vec![Validator::Required, Validator::Email],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "password".to_string(),
                field_type: FieldType::Password,
                validators: vec![Validator::Required, Validator::MinLength(8)],
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
            errors.add_field_error("email", "Email is required".to_string());
        } else if !self.email.contains('@') {
            errors.add_field_error("email", "Invalid email format".to_string());
        }

        // Validate password
        if self.password.len() < 8 {
            errors.add_field_error(
                "password",
                "Password must be at least 8 characters".to_string(),
            );
        }

        if errors.is_empty() {
            Ok::<(), ValidationErrors>(())
        } else {
            Err(errors)
        }
    }

    fn get_field_value(&self, name: &str) -> FieldValue {
        match name {
            "email" => FieldValue::String(self.email.clone()),
            "password" => FieldValue::String(self.password.clone()),
            "remember_me" => FieldValue::Boolean(self.remember_me),
            _ => FieldValue::String(String::new()),
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
        FormSchema {
            name: "LoginForm".to_string(),
            field_metadata: Self::field_metadata(),
        }
    }
}

#[component]
fn LoginPage() -> impl IntoView {
    let (form, _submit_callback, _reset_callback) = use_form(LoginForm::default_values());

    // Create clones for different closures
    let form_clone1 = form.clone();
    let form_clone2 = form.clone();
    let form_clone3 = form.clone();
    let form_clone4 = form.clone();

    view! {
        <div class="container">
            <h1>"Login Form Example"</h1>
            <p>"This demonstrates the basic functionality of Leptos Forms RS with Leptos 0.8 using actual library components!"</p>

            <div class="form">
                <FormField
                    form=form.clone()
                    name="email".to_string()
                    label="Email Address".to_string()
                    placeholder="Enter your email".to_string()
                    help_text="We'll never share your email with anyone else.".to_string()
                    required=true
                    field_type=FieldType::Email
                />

                <FormField
                    form=form.clone()
                    name="password".to_string()
                    label="Password".to_string()
                    placeholder="Enter your password".to_string()
                    help_text="Must be at least 8 characters long.".to_string()
                    required=true
                    field_type=FieldType::Password
                />

                <FormField
                    form=form.clone()
                    name="remember_me".to_string()
                    label="Remember me".to_string()
                    field_type=FieldType::Boolean
                />

                <div class="form-actions">
                    <FormSubmit
                        form=form.clone()
                        on_submit=move |form_data| {
                            log::info!("Form submitted: {:?}", form_data);
                            if let Some(window) = web_sys::window() {
                                let _ = window.alert_with_message(&format!("Login successful for: {}", form_data.email));
                            }
                            Ok(())
                        }
                    />

                    <button
                        class="btn btn-secondary"
                        on:click=move |_| {
                            form_clone1.reset();
                            log::info!("Form reset to default values");
                        }
                    >
                        "Reset"
                    </button>
                </div>

                <div class="form-debug">
                    <h3>"Form Debug Info"</h3>
                    <p>"Form is working! This demonstrates the actual library API with proper components."</p>
                    <p>"Form values: " {move || format!("{:?}", form_clone2.values().get())}</p>
                    <p>"Form is valid: " {move || format!("{}", form_clone3.is_valid().get())}</p>
                    <p>"Form is dirty: " {move || format!("{}", form_clone4.is_dirty().get())}</p>
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
