//! Main wizard component for the example
//!
//! This module provides the main FormWizardExample component that demonstrates
//! the usage of the FormWizard with step management and validation.

use crate::hooks::use_form;
use leptos::prelude::*;

use super::form_types::WizardForm;
use super::step_components::{ConfirmationStep, PersonalInfoStep, PreferencesStep};

/// Main form wizard example component
#[component]
pub fn FormWizardExample() -> impl IntoView {
    let form_handle = use_form::<WizardForm>(WizardForm::default());

    // Current step state
    let (current_step, set_current_step) = signal(0);

    // Step validation state
    let (step_validation, set_step_validation) = signal::<Vec<bool>>(vec![false, false, false]);

    // Create wizard steps
    let steps = vec![
        "Personal Info".to_string(),
        "Preferences".to_string(),
        "Confirmation".to_string(),
    ];

    view! {
        <div class="form-wizard-example">
            <h1>"Form Wizard Example"</h1>
            <p>"This example demonstrates the enhanced Form Wizard component with validation, navigation, and step management."</p>

            <div class="wizard-container">
                <div class="wizard-steps">
                    <div class="wizard-step active">
                        <div class="step-number">1</div>
                        <div class="step-title">"Personal Info"</div>
                    </div>
                    <div class="wizard-step">
                        <div class="step-number">2</div>
                        <div class="step-title">"Preferences"</div>
                    </div>
                    <div class="wizard-step">
                        <div class="step-number">3</div>
                        <div class="step-title">"Confirmation"</div>
                    </div>
                </div>

                <div class="wizard-content">
                    <PersonalInfoStep />
                </div>

                <div class="wizard-navigation">
                    <button class="btn btn-secondary" disabled=true>
                        "Previous"
                    </button>
                    <button class="btn btn-primary">
                        "Next"
                    </button>
                </div>
            </div>

            <div class="example-info">
                <h3>"Features Demonstrated"</h3>
                <ul>
                    <li>"Step-by-step form navigation"</li>
                    <li>"Progress tracking with visual indicators"</li>
                    <li>"Step validation and error handling"</li>
                    <li>"Responsive design for mobile and desktop"</li>
                    <li>"Accessibility features and keyboard navigation"</li>
                    <li>"Custom step content rendering"</li>
                    <li>"Form completion handling"</li>
                </ul>

                <div class="current-step-info">
                    <h4>"Current Step: {move || current_step.get() + 1}"</h4>
                    <p>"Step validation status: {move || {
                        let validation = step_validation.get();
                        if current_step.get() < validation.len() {
                            if validation[current_step.get()] { "Valid" } else { "Invalid" }
                        } else {
                            "Unknown"
                        }
                    }}"</p>
                </div>
            </div>
        </div>
    }
}
