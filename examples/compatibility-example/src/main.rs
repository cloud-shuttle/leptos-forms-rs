//! Compatibility Example - Demonstrates Leptos 0.6 and 0.8 compatibility
//! 
//! This example shows how the compatibility layer works across both versions.

use leptos_forms_rs::compat::*;
use serde::{Deserialize, Serialize};

// Define a simple form using the compatibility layer
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
struct CompatForm {
    name: String,
    email: String,
    age: u32,
}

// Use the compatibility layer for component definition
compat_component! {
    fn CompatFormComponent() -> impl IntoView {
        // Use compatibility layer signals
        let (name, set_name) = signal(String::new());
        let (email, set_email) = signal(String::new());
        let (age, set_age) = signal(0u32);
        
        // Use compatibility layer effects
        let effect = EffectCompat::new(move |_| {
            log::info!("Form values changed: name={}, email={}, age={}", 
                      name.get(), email.get(), age.get());
        });
        
        // Use compatibility layer memos
        let is_valid = memo(move || {
            !name.get().is_empty() && 
            email.get().contains('@') && 
            age.get() >= 18
        });
        
        view! {
            <div class="compat-form">
                <h1>"Leptos Compatibility Example"</h1>
                <p>"This form works with both Leptos 0.6 and 0.8!"</p>
                
                <div class="form-field">
                    <label for="name">"Name"</label>
                    <input 
                        type="text" 
                        id="name" 
                        placeholder="Enter your name"
                        on:input=move |ev| {
                            if let Some(target) = event_target::<web_sys::HtmlInputElement>(&ev) {
                                set_name.set(target.value());
                            }
                        }
                    />
                </div>
                
                <div class="form-field">
                    <label for="email">"Email"</label>
                    <input 
                        type="email" 
                        id="email" 
                        placeholder="Enter your email"
                        on:input=move |ev| {
                            if let Some(target) = event_target::<web_sys::HtmlInputElement>(&ev) {
                                set_email.set(target.value());
                            }
                        }
                    />
                </div>
                
                <div class="form-field">
                    <label for="age">"Age"</label>
                    <input 
                        type="number" 
                        id="age" 
                        placeholder="Enter your age"
                        on:input=move |ev| {
                            if let Some(target) = event_target::<web_sys::HtmlInputElement>(&ev) {
                                if let Ok(age_val) = target.value().parse::<u32>() {
                                    set_age.set(age_val);
                                }
                            }
                        }
                    />
                </div>
                
                <div class="form-status">
                    <p>"Form is valid: " {move || if is_valid.get() { "Yes" } else { "No" }}</p>
                    <p>"Current values:"</p>
                    <ul>
                        <li>"Name: " {move || name.get()}</li>
                        <li>"Email: " {move || email.get()}</li>
                        <li>"Age: " {move || age.get()}</li>
                    </ul>
                </div>
                
                <div class="version-info">
                    <p>"Running on Leptos version: " {version::LEPTOS_VERSION}</p>
                    <p>"Supports Tachys: " {version::supports_tachys()}</p>
                    <p>"Requires Scope: " {version::requires_scope()}</p>
                </div>
            </div>
        }
    }
}

// Main app component
compat_component! {
    fn App() -> impl IntoView {
        view! {
            <div class="app">
                <header>
                    <h1>"Leptos Forms RS - Compatibility Demo"</h1>
                    <p>"Demonstrating dual-version support"</p>
                </header>
                
                <main>
                    <CompatFormComponent />
                </main>
                
                <footer>
                    <p>"Built with the compatibility layer"</p>
                </footer>
            </div>
        }
    }
}

fn main() {
    // Initialize logging
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
    
    // Log version information
    log::info!("Starting compatibility example");
    log::info!("Leptos version: {}", version::LEPTOS_VERSION);
    log::info!("Supports Tachys: {}", version::supports_tachys());
    log::info!("Requires Scope: {}", version::requires_scope());
    
    // Mount the app using compatibility layer
    ().mount_to_body(|| view! { <App /> });
}

#[wasm_bindgen]
pub fn run_app() {
    main();
}
