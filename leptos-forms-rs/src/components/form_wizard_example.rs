use crate::components::{FormWizard, WizardStep};
use crate::core::{FieldValue, Form, FormHandle};
use crate::hooks::use_form;
use leptos::prelude::*;

#[component]
pub fn FormWizardExample() -> impl IntoView {
    let form_handle = use_form::<WizardForm>();

    // Current step state
    let (current_step, set_current_step) = signal(0);

    // Step validation state
    let (step_validation, set_step_validation) = signal::<Vec<bool>>(vec![false, false, false]);

    // Form data state
    let form_data = Signal::derive(move || WizardForm {
        personal_info: PersonalInfo {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
        },
        preferences: Preferences {
            theme: "dark".to_string(),
            notifications: true,
            newsletter: false,
        },
        confirmation: Confirmation {
            terms_accepted: false,
            privacy_accepted: false,
        },
    });

    // Step validation callback
    let on_step_validation = Callback::new(move |(step_index, is_valid): (usize, bool)| {
        set_step_validation.update(|validation| {
            if step_index < validation.len() {
                validation[step_index] = is_valid;
            }
        });
        log::info!("Step {} validation: {}", step_index, is_valid);
    });

    // Wizard completion callback
    let on_wizard_complete = Callback::new(move |form_data: WizardForm| {
        log::info!("Wizard completed successfully: {:?}", form_data);
        // Here you would typically submit the form data
    });

    // Step change handler
    let on_step_change = Callback::new(move |step_index: usize| {
        set_current_step.set(step_index);
        log::info!("Changed to step: {}", step_index);
    });

    // Create wizard steps
    let steps = vec![
        WizardStep {
            title: "Personal Info".to_string(),
            description: Some("Enter your basic information".to_string()),
            content: view! {
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
            },
        },
        WizardStep {
            title: "Preferences".to_string(),
            description: Some("Choose your preferences".to_string()),
            content: view! {
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
            },
        },
        WizardStep {
            title: "Confirmation".to_string(),
            description: Some("Review and confirm your choices".to_string()),
            content: view! {
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
            },
        },
    ];

    view! {
        <div class="form-wizard-example">
            <h1>"Form Wizard Example"</h1>
            <p>"This example demonstrates the enhanced Form Wizard component with validation, navigation, and step management."</p>

            <FormWizard
                steps=steps
                current_step=current_step.into()
                on_step_change=on_step_change
                form_handle=form_handle
                show_progress_bar=signal(true).0.into()
                show_step_numbers=signal(true).0.into()
                allow_back_navigation=signal(true).0.into()
                allow_skip_steps=signal(false).0.into()
                validate_on_step_change=signal(true).0.into()
                on_step_validation=Some(on_step_validation)
                on_wizard_complete=Some(on_wizard_complete)
            />

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

// Example form structure
#[derive(Clone, Debug, PartialEq)]
pub struct WizardForm {
    pub personal_info: PersonalInfo,
    pub preferences: Preferences,
    pub confirmation: Confirmation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PersonalInfo {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Preferences {
    pub theme: String,
    pub notifications: bool,
    pub newsletter: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Confirmation {
    pub terms_accepted: bool,
    pub privacy_accepted: bool,
}

// Example CSS for the demo
pub const FORM_WIZARD_EXAMPLE_CSS: &str = r#"
.form-wizard-example {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
}

.form-wizard-example h1 {
    text-align: center;
    color: #1e293b;
    margin-bottom: 1rem;
}

.form-wizard-example > p {
    text-align: center;
    color: #64748b;
    margin-bottom: 3rem;
    font-size: 1.1rem;
}

.example-info {
    margin-top: 3rem;
    padding: 2rem;
    background: #f8fafc;
    border-radius: 8px;
    border: 1px solid #e2e8f0;
}

.example-info h3 {
    color: #1e293b;
    margin-bottom: 1rem;
}

.example-info ul {
    margin-bottom: 2rem;
    padding-left: 1.5rem;
}

.example-info li {
    color: #475569;
    margin-bottom: 0.5rem;
    line-height: 1.5;
}

.current-step-info {
    background: #eff6ff;
    border: 1px solid #bfdbfe;
    border-radius: 6px;
    padding: 1rem;
}

.current-step-info h4 {
    color: #1e40af;
    margin: 0 0 0.5rem 0;
}

.current-step-info p {
    color: #1e40af;
    margin: 0;
    font-weight: 500;
}

/* Step-specific styles */
.step-personal-info,
.step-preferences,
.step-confirmation {
    padding: 1rem;
}

.step-personal-info h3,
.step-preferences h3,
.step-confirmation h3 {
    color: #1e293b;
    margin-bottom: 0.5rem;
}

.step-personal-info p,
.step-preferences p,
.step-confirmation p {
    color: #64748b;
    margin-bottom: 1.5rem;
}

.form-group {
    margin-bottom: 1.5rem;
}

.form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: #374151;
}

.form-input,
.form-select {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    font-size: 0.875rem;
    transition: border-color 0.2s ease;
}

.form-input:focus,
.form-select:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    font-weight: 500;
    color: #374151;
}

.checkbox-label input[type="checkbox"] {
    width: 1.25rem;
    height: 1.25rem;
    accent-color: #3b82f6;
}

.summary {
    background: #f9fafb;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    padding: 1rem;
    margin-bottom: 1.5rem;
}

.summary h4 {
    color: #1f2937;
    margin: 0 0 1rem 0;
    font-size: 1rem;
}

.summary-item {
    display: flex;
    justify-content: space-between;
    padding: 0.5rem 0;
    border-bottom: 1px solid #e5e7eb;
}

.summary-item:last-child {
    border-bottom: none;
}

.summary-item strong {
    color: #374151;
    font-weight: 600;
}

.summary-item span {
    color: #6b7280;
}

/* Responsive adjustments */
@media (max-width: 768px) {
    .form-wizard-example {
        padding: 1rem;
    }

    .example-info {
        margin-top: 2rem;
        padding: 1rem;
    }

    .summary-item {
        flex-direction: column;
        gap: 0.25rem;
    }
}
"#;
