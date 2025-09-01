use leptos::prelude::*;
use leptos_forms_rs::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Define a complex user registration form using the Form derive macro
#[derive(Form, Clone, Serialize, Deserialize, PartialEq, Debug)]
struct UserRegistrationForm {
    // Personal Information
    #[form(required)]
    first_name: String,
    
    #[form(required)]
    last_name: String,
    
    #[form(required, email)]
    email: String,
    
    #[form(required, min_length = 8)]
    password: String,
    
    #[form(required, min_length = 8)]
    confirm_password: String,
    
    // Contact Information
    #[form(optional)]
    phone: Option<String>,
    
    #[form(optional, url)]
    website: Option<String>,
    
    // Address Information
    #[form(required)]
    street_address: String,
    
    #[form(required)]
    city: String,
    
    #[form(required)]
    state: String,
    
    #[form(required)]
    postal_code: String,
    
    #[form(required)]
    country: String,
    
    // Preferences
    #[form(default = false)]
    newsletter: bool,
    
    #[form(default = false)]
    marketing_emails: bool,
    
    #[form(default = "en")]
    language: String,
    
    // Skills (Array)
    #[form(array)]
    skills: Vec<String>,
    
    // Work Experience (Array)
    #[form(array)]
    work_experience: Vec<WorkExperience>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct WorkExperience {
    company: String,
    position: String,
    start_date: String,
    end_date: String,
    description: String,
}

#[component]
fn UserRegistrationPage() -> impl IntoView {
    let form = use_form::<UserRegistrationForm>();
    
    let handle_submit = move |_| {
        if let Some(form_data) = form.form_data().get() {
            log::info!("Form submitted: {:?}", form_data);
            // In a real app, you would send this to your backend
            if let Some(window) = web_sys::window() {
                let _ = window.alert_with_message(&format!("Registration successful for: {} {}", form_data.first_name, form_data.last_name));
            }
        }
    };
    
    view! {
        <div class="container">
            <h1>"User Registration Form Example"</h1>
            <p>"This demonstrates the complex functionality of Leptos Forms RS."</p>
            
            <form on:submit=form.handle_submit>
                <div class="form-section">
                    <h2>"Personal Information"</h2>
                    
                    <div class="form-row">
                        <div class="form-field">
                            <label for="first_name">"First Name"</label>
                            <input 
                                type="text" 
                                id="first_name" 
                                name="first_name"
                                placeholder="Enter your first name"
                                on:input=form.handle_input
                                required=true
                            />
                            {move || form.field_error("first_name").map(|error| 
                                view! { <span class="error">{error}</span> }
                            )}
                        </div>
                        
                        <div class="form-field">
                            <label for="last_name">"Last Name"</label>
                            <input 
                                type="text" 
                                id="last_name" 
                                name="last_name"
                                placeholder="Enter your last name"
                                on:input=form.handle_input
                                required=true
                            />
                            {move || form.field_error("last_name").map(|error| 
                                view! { <span class="error">{error}</span> }
                            )}
                        </div>
                    </div>
                    
                    <div class="form-field">
                        <label for="email">"Email Address"</label>
                        <input 
                            type="email" 
                            id="email" 
                            name="email"
                            placeholder="Enter your email"
                            on:input=form.handle_input
                            required=true
                        />
                        {move || form.field_error("email").map(|error| 
                            view! { <span class="error">{error}</span> }
                        )}
                    </div>
                    
                    <div class="form-row">
                        <div class="form-field">
                            <label for="password">"Password"</label>
                            <input 
                                type="password" 
                                id="password" 
                                name="password"
                                placeholder="Enter your password"
                                on:input=form.handle_input
                                required=true
                            />
                            {move || form.field_error("password").map(|error| 
                                view! { <span class="error">{error}</span> }
                            )}
                        </div>
                        
                        <div class="form-field">
                            <label for="confirm_password">"Confirm Password"</label>
                            <input 
                                type="password" 
                                id="confirm_password" 
                                name="confirm_password"
                                placeholder="Confirm your password"
                                on:input=form.handle_input
                                required=true
                            />
                            {move || form.field_error("confirm_password").map(|error| 
                                view! { <span class="error">{error}</span> }
                            )}
                        </div>
                    </div>
                </div>
                
                <div class="form-section">
                    <h2>"Contact Information"</h2>
                    
                    <div class="form-row">
                        <div class="form-field">
                            <label for="phone">"Phone Number"</label>
                            <input 
                                type="tel" 
                                id="phone" 
                                name="phone"
                                placeholder="Enter your phone number"
                                on:input=form.handle_input
                            />
                        </div>
                        
                        <div class="form-field">
                            <label for="website">"Website"</label>
                            <input 
                                type="url" 
                                id="website" 
                                name="website"
                                placeholder="Enter your website"
                                on:input=form.handle_input
                            />
                        </div>
                    </div>
                </div>
                
                <div class="form-section">
                    <h2>"Address Information"</h2>
                    
                    <div class="form-field">
                        <label for="street_address">"Street Address"</label>
                        <input 
                            type="text" 
                            id="street_address" 
                            name="street_address"
                            placeholder="Enter your street address"
                            on:input=form.handle_input
                            required=true
                        />
                        {move || form.field_error("street_address").map(|error| 
                            view! { <span class="error">{error}</span> }
                        )}
                    </div>
                    
                    <div class="form-row">
                        <div class="form-field">
                            <label for="city">"City"</label>
                            <input 
                                type="text" 
                                id="city" 
                                name="city"
                                placeholder="Enter your city"
                                on:input=form.handle_input
                                required=true
                            />
                            {move || form.field_error("city").map(|error| 
                                view! { <span class="error">{error}</span> }
                            )}
                        </div>
                        
                        <div class="form-field">
                            <label for="state">"State/Province"</label>
                            <input 
                                type="text" 
                                id="state" 
                                name="state"
                                placeholder="Enter your state"
                                on:input=form.handle_input
                                required=true
                            />
                            {move || form.field_error("state").map(|error| 
                                view! { <span class="error">{error}</span> }
                            )}
                        </div>
                    </div>
                    
                    <div class="form-row">
                        <div class="form-field">
                            <label for="postal_code">"Postal Code"</label>
                            <input 
                                type="text" 
                                id="postal_code" 
                                name="postal_code"
                                placeholder="Enter your postal code"
                                on:input=form.handle_input
                                required=true
                            />
                            {move || form.field_error("postal_code").map(|error| 
                                view! { <span class="error">{error}</span> }
                            )}
                        </div>
                        
                        <div class="form-field">
                            <label for="country">"Country"</label>
                            <select 
                                id="country" 
                                name="country"
                                on:change=form.handle_input
                                required=true
                            >
                                <option value="">"Select a country"</option>
                                <option value="us">"United States"</option>
                                <option value="ca">"Canada"</option>
                                <option value="uk">"United Kingdom"</option>
                                <option value="de">"Germany"</option>
                                <option value="fr">"France"</option>
                                <option value="other">"Other"</option>
                            </select>
                            {move || form.field_error("country").map(|error| 
                                view! { <span class="error">{error}</span> }
                            )}
                        </div>
                    </div>
                </div>
                
                <div class="form-section">
                    <h2>"Preferences"</h2>
                    
                    <div class="form-field">
                        <label>
                            <input 
                                type="checkbox" 
                                id="newsletter" 
                                name="newsletter"
                                on:change=form.handle_input
                            />
                            "Subscribe to newsletter"
                        </label>
                    </div>
                    
                    <div class="form-field">
                        <label>
                            <input 
                                type="checkbox" 
                                id="marketing_emails" 
                                name="marketing_emails"
                                on:change=form.handle_input
                            />
                            "Receive marketing emails"
                        </label>
                    </div>
                    
                    <div class="form-field">
                        <label for="language">"Preferred Language"</label>
                        <select 
                            id="language" 
                            name="language"
                            on:change=form.handle_input
                        >
                            <option value="en">"English"</option>
                            <option value="es">"Spanish"</option>
                            <option value="fr">"French"</option>
                            <option value="de">"German"</option>
                        </select>
                    </div>
                </div>
                
                <div class="form-actions">
                    <button 
                        type="submit" 
                        class="btn btn-primary"
                        disabled=move || !form.is_valid()
                    >
                        "Register"
                    </button>
                    
                    <button 
                        type="button" 
                        class="btn btn-secondary"
                        on:click=move |_| form.reset()
                    >
                        "Reset"
                    </button>
                </div>
                
                <div class="form-debug">
                    <h3>"Form Debug Info"</h3>
                    <p>"Complex form is working! This demonstrates the current API."</p>
                    <p>"Form valid: " {move || form.is_valid().to_string()}</p>
                    <p>"Form dirty: " {move || form.is_dirty().to_string()}</p>
                    <p>"Form touched: " {move || form.is_touched().to_string()}</p>
                    {move || form.form_data().map(|data| 
                        view! { 
                            <p>"Form data: " {format!("{:?}", data)}</p>
                        }
                    )}
                    {move || form.validation_errors().map(|errors| 
                        view! { 
                            <p>"Validation errors: " {format!("{:?}", errors)}</p>
                        }
                    )}
                </div>
            </form>
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
