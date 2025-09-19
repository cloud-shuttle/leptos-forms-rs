//! Step components for the wizard example
//!
//! This module provides individual step components for the form wizard.

use leptos::prelude::*;

/// Personal information step component
#[component]
pub fn PersonalInfoStep() -> impl IntoView {
    view! {
        <div class="step-personal-info">
            <h3>"Personal Information"</h3>
            <p>"Please provide your basic contact information."</p>

            <div class="form-group">
                <label for="first-name">"First Name"</label>
                <input
                    type="text"
                    id="first-name"
                    value="John"
                    placeholder="Enter your first name"
                    class="form-input"
                />
            </div>

            <div class="form-group">
                <label for="last-name">"Last Name"</label>
                <input
                    type="text"
                    id="last-name"
                    value="Doe"
                    placeholder="Enter your last name"
                    class="form-input"
                />
            </div>

            <div class="form-group">
                <label for="email">"Email Address"</label>
                <input
                    type="email"
                    id="email"
                    value="john.doe@example.com"
                    placeholder="Enter your email"
                    class="form-input"
                />
            </div>
        </div>
    }
}

/// Preferences step component
#[component]
pub fn PreferencesStep() -> impl IntoView {
    view! {
        <div class="step-preferences">
            <h3>"Preferences"</h3>
            <p>"Customize your experience with these settings."</p>

            <div class="form-group">
                <label for="theme">"Theme"</label>
                <select id="theme" class="form-select">
                    <option value="light">"Light"</option>
                    <option value="dark" selected="true">"Dark"</option>
                    <option value="auto">"Auto"</option>
                </select>
            </div>

            <div class="form-group">
                <label class="checkbox-label">
                    <input type="checkbox" checked="true" />
                    <span>"Enable notifications"</span>
                </label>
            </div>

            <div class="form-group">
                <label class="checkbox-label">
                    <input type="checkbox" />
                    <span>"Subscribe to newsletter"</span>
                </label>
            </div>
        </div>
    }
}

/// Confirmation step component
#[component]
pub fn ConfirmationStep() -> impl IntoView {
    view! {
        <div class="step-confirmation">
            <h3>"Confirmation"</h3>
            <p>"Please review your information and accept the terms."</p>

            <div class="summary">
                <h4>"Summary"</h4>
                <div class="summary-item">
                    <strong>"Name:"</strong>
                    <span>"John Doe"</span>
                </div>
                <div class="summary-item">
                    <strong>"Email:"</strong>
                    <span>"john.doe@example.com"</span>
                </div>
                <div class="summary-item">
                    <strong>"Theme:"</strong>
                    <span>"Dark"</span>
                </div>
                <div class="summary-item">
                    <strong>"Notifications:"</strong>
                    <span>"Enabled"</span>
                </div>
            </div>

            <div class="form-group">
                <label class="checkbox-label">
                    <input type="checkbox" />
                    <span>"I accept the terms and conditions"</span>
                </label>
            </div>

            <div class="form-group">
                <label class="checkbox-label">
                    <input type="checkbox" />
                    <span>"I agree to the privacy policy"</span>
                </label>
            </div>
        </div>
    }
}
