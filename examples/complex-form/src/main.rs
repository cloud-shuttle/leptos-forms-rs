use leptos::prelude::*;
use leptos_forms_rs::core::traits::{FieldMetadata, FormSchema};
// use leptos_forms_rs::core::types::FieldError; // Not needed for this example
use leptos_forms_rs::components::{FormField, FormSubmit}; // Using actual library components!
use leptos_forms_rs::core::types::{FieldType, FieldValue};
use leptos_forms_rs::validation::ValidationErrors;
use leptos_forms_rs::validation::Validator;
use leptos_forms_rs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
                validators: vec![Validator::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "last_name".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
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
                name: "confirm_password".to_string(),
                field_type: FieldType::Password,
                validators: vec![Validator::Required, Validator::MinLength(8)],
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
                validators: vec![Validator::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "city".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "state".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "postal_code".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "country".to_string(),
                field_type: FieldType::Text,
                validators: vec![Validator::Required],
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
            errors.add_field_error("first_name", "First name is required".to_string());
        }

        if self.last_name.is_empty() {
            errors.add_field_error("last_name", "Last name is required".to_string());
        }

        if self.email.is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        } else if !self.email.contains('@') {
            errors.add_field_error("email", "Invalid email format".to_string());
        }

        if self.password.len() < 8 {
            errors.add_field_error(
                "password",
                "Password must be at least 8 characters".to_string(),
            );
        }

        if self.password != self.confirm_password {
            errors.add_field_error("confirm_password", "Passwords do not match".to_string());
        }

        if self.street_address.is_empty() {
            errors.add_field_error("street_address", "Street address is required".to_string());
        }

        if self.city.is_empty() {
            errors.add_field_error("city", "City is required".to_string());
        }

        if self.state.is_empty() {
            errors.add_field_error("state", "State is required".to_string());
        }

        if self.postal_code.is_empty() {
            errors.add_field_error("postal_code", "Postal code is required".to_string());
        }

        if self.country.is_empty() {
            errors.add_field_error("country", "Country is required".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn get_field_value(&self, name: &str) -> FieldValue {
        match name {
            "first_name" => FieldValue::String(self.first_name.clone()),
            "last_name" => FieldValue::String(self.last_name.clone()),
            "email" => FieldValue::String(self.email.clone()),
            "password" => FieldValue::String(self.password.clone()),
            "confirm_password" => FieldValue::String(self.confirm_password.clone()),
            "phone" => self
                .phone
                .as_ref()
                .map(|p| FieldValue::String(p.clone()))
                .unwrap_or(FieldValue::String(String::new())),
            "website" => self
                .website
                .as_ref()
                .map(|w| FieldValue::String(w.clone()))
                .unwrap_or(FieldValue::String(String::new())),
            "street_address" => FieldValue::String(self.street_address.clone()),
            "city" => FieldValue::String(self.city.clone()),
            "state" => FieldValue::String(self.state.clone()),
            "postal_code" => FieldValue::String(self.postal_code.clone()),
            "country" => FieldValue::String(self.country.clone()),
            "newsletter" => FieldValue::Boolean(self.newsletter),
            "marketing_emails" => FieldValue::Boolean(self.marketing_emails),
            "language" => FieldValue::String(self.language.clone()),
            _ => FieldValue::String(String::new()),
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
        FormSchema {
            name: "UserRegistrationForm".to_string(),
            field_metadata: Self::field_metadata(),
        }
    }
}

#[component]
fn UserRegistrationPage() -> impl IntoView {
    let (form, _submit_callback, _reset_callback) =
        use_form(UserRegistrationForm::default_values());

    // Create clones for different closures
    let form_clone1 = form.clone();
    let form_clone2 = form.clone();
    let form_clone3 = form.clone();
    let form_clone4 = form.clone();

    view! {
        <div class="container">
            <h1>"User Registration Form Example"</h1>
            <p>"This demonstrates the complex functionality of Leptos Forms RS with Leptos 0.8 using actual library components!"</p>

            <div class="form">
                <div class="form-section">
                    <h2>"Personal Information"</h2>

                    <div class="form-row">
                        <FormField
                            form=form.clone()
                            name="first_name".to_string()
                            label="First Name".to_string()
                            required=true
                            field_type=FieldType::Text
                        />

                        <FormField
                            form=form.clone()
                            name="last_name".to_string()
                            label="Last Name".to_string()
                            required=true
                            field_type=FieldType::Text
                        />
                    </div>

                    <FormField
                        form=form.clone()
                        name="email".to_string()
                        label="Email Address".to_string()
                        required=true
                        field_type=FieldType::Email
                    />

                    <div class="form-row">
                        <FormField
                            form=form.clone()
                            name="password".to_string()
                            label="Password".to_string()
                            help_text="Must be at least 8 characters long.".to_string()
                            required=true
                            field_type=FieldType::Password
                        />

                        <FormField
                            form=form.clone()
                            name="confirm_password".to_string()
                            label="Confirm Password".to_string()
                            required=true
                            field_type=FieldType::Password
                        />
                    </div>
                </div>

                <div class="form-section">
                    <h2>"Contact Information"</h2>

                    <div class="form-row">
                        <FormField
                            form=form.clone()
                            name="phone".to_string()
                            label="Phone Number".to_string()
                            field_type=FieldType::Text
                        />

                        <FormField
                            form=form.clone()
                            name="website".to_string()
                            label="Website".to_string()
                            field_type=FieldType::Text
                        />
                    </div>
                </div>

                <div class="form-section">
                    <h2>"Address Information"</h2>

                    <FormField
                        form=form.clone()
                        name="street_address".to_string()
                        label="Street Address".to_string()
                        required=true
                        field_type=FieldType::Text
                    />

                    <div class="form-row">
                        <FormField
                            form=form.clone()
                            name="city".to_string()
                            label="City".to_string()
                            required=true
                            field_type=FieldType::Text
                        />

                        <FormField
                            form=form.clone()
                            name="state".to_string()
                            label="State/Province".to_string()
                            required=true
                            field_type=FieldType::Text
                        />
                    </div>

                    <div class="form-row">
                        <FormField
                            form=form.clone()
                            name="postal_code".to_string()
                            label="Postal Code".to_string()
                            required=true
                            field_type=FieldType::Text
                        />

                        <FormField
                            form=form.clone()
                            name="country".to_string()
                            label="Country".to_string()
                            required=true
                            field_type=FieldType::Text
                        />
                    </div>
                </div>

                <div class="form-section">
                    <h2>"Preferences"</h2>

                    <FormField
                        form=form.clone()
                        name="newsletter".to_string()
                        label="Subscribe to newsletter".to_string()
                        field_type=FieldType::Boolean
                    />

                    <FormField
                        form=form.clone()
                        name="marketing_emails".to_string()
                        label="Receive marketing emails".to_string()
                        field_type=FieldType::Boolean
                    />

                    <FormField
                        form=form.clone()
                        name="language".to_string()
                        label="Preferred Language".to_string()
                        field_type=FieldType::Text
                    />
                </div>

                <div class="form-actions">
                    <FormSubmit
                        form=form.clone()
                        on_submit=move |form_data| {
                            log::info!("Form submitted: {:?}", form_data);
                            if let Some(window) = web_sys::window() {
                                let _ = window.alert_with_message(&format!(
                                    "Registration successful for: {} {}",
                                    form_data.first_name, form_data.last_name
                                ));
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
