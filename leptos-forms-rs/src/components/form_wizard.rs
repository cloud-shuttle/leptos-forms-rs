use leptos::prelude::*;
use crate::core::{Form, FormHandle};
use crate::validation::ValidationErrors;
use leptos::html::{Div, HtmlElement};

#[component]
pub fn FormWizard<T: Form + PartialEq + Clone + 'static>(
    steps: Vec<WizardStep>,
    current_step: Signal<usize>,
    on_step_change: Callback<usize>,
    form_handle: FormHandle<T>,
    #[prop(optional)] show_progress_bar: Option<Signal<bool>>,
    #[prop(optional)] show_step_numbers: Option<Signal<bool>>,
    #[prop(optional)] allow_back_navigation: Option<Signal<bool>>,
    #[prop(optional)] allow_skip_steps: Option<Signal<bool>>,
    #[prop(optional)] validate_on_step_change: Option<Signal<bool>>,
    #[prop(optional)] on_step_validation: Option<Callback<(usize, bool)>>,
    #[prop(optional)] on_wizard_complete: Option<Callback<T>>,
) -> impl IntoView {
    let total_steps = steps.len();
    
    // Default values for optional props
    let _show_progress_bar = show_progress_bar.unwrap_or_else(|| signal(true).0.into());
    let _show_step_numbers = show_step_numbers.unwrap_or_else(|| signal(true).0.into());
    let allow_back_navigation = allow_back_navigation.unwrap_or_else(|| signal(true).0.into());
    let allow_skip_steps = allow_skip_steps.unwrap_or_else(|| signal(false).0.into());
    let validate_on_step_change = validate_on_step_change.unwrap_or_else(|| signal(true).0.into());
    
    // Current step value
    let current_step_value = current_step;
    
    // Step validation errors
    let (_step_validation_errors, set_step_validation_errors) = signal::<Vec<Option<ValidationErrors>>>(vec![None; total_steps]);
    
    // Progress percentage
    let progress_percentage = Signal::derive(move || {
        if total_steps == 0 {
            0.0
        } else {
            ((current_step_value.get() + 1) as f64 / total_steps as f64) * 100.0
        }
    });
    
    // Navigation functions
    let go_to_step = {
        let on_step_change = on_step_change.clone();
        let validate_on_step_change = validate_on_step_change.clone();
        let on_step_validation = on_step_validation.clone();
        let form_handle = form_handle.clone();
        let set_step_validation_errors = set_step_validation_errors.clone();
        
        move |step_index: usize| {
            if step_index < total_steps {
                // Validate current step if enabled
                if validate_on_step_change.get() {
                    let is_valid = validate_current_step(&form_handle, step_index);
                    
                    // Update validation errors
                    set_step_validation_errors.update(|errors| {
                        if step_index < errors.len() {
                            errors[step_index] = if is_valid { None } else { Some(ValidationErrors::new()) };
                        }
                    });
                    
                    // Call validation callback if provided
                    if let Some(callback) = on_step_validation.as_ref() {
                        callback.run((step_index, is_valid));
                    }
                }
                
                // Navigate to step
                on_step_change.run(step_index);
            }
        }
    };
    
    let mut go_to_next_step = {
        let go_to_step = go_to_step.clone();
        move |_| {
            let current = current_step_value.get();
            if current < total_steps - 1 {
                go_to_step(current + 1);
            }
        }
    };
    
    let go_to_previous_step = {
        let go_to_step = go_to_step.clone();
        move |_| {
            let current = current_step_value.get();
            if current > 0 {
                go_to_step(current - 1);
            }
        }
    };
    
    // Submit handler
    let mut submit_form = {
        let on_wizard_complete = on_wizard_complete.clone();
        let form_handle = form_handle.clone();
        move |_| {
            // Validate all steps before submission
            let all_steps_valid = (0..total_steps).all(|step_index| {
                validate_current_step(&form_handle, step_index)
            });
            
            if all_steps_valid {
                if let Some(callback) = on_wizard_complete.as_ref() {
                    let form_data = form_handle.get_values().get();
                    callback.run(form_data);
                }
            }
        }
    };
    
    // Check if current step is the last step
    let is_last_step = Signal::derive(move || current_step_value.get() == total_steps - 1);
    
    // Check if current step is the first step
    let _is_first_step = Signal::derive(move || current_step_value.get() == 0);
    
    // Current step validation status
    let current_step_is_valid = Signal::derive(move || {
        let current = current_step_value.get();
        if current < total_steps {
            validate_current_step(&form_handle, current)
        } else {
            false
        }
    });

    view! {
        <div class="form-wizard">
            // Progress bar
            <div class="wizard-progress-bar" style="display: none;">
                <div class="progress-track">
                    <div 
                        class="progress-fill"
                        style=move || format!("width: {}%", progress_percentage.get())
                    ></div>
                </div>
                <div class="progress-text">
                    "Step {move || current_step_value.get() + 1} of {total_steps}"
                </div>
            </div>

            // Step indicators
            <div class="wizard-steps">
                <div class="step-indicator">
                    <p>"Step {move || current_step_value.get() + 1} of {total_steps}"</p>
                </div>
            </div>

            // Step content
            <div class="wizard-content">
                <div class="step-content">
                    <div class="current-step">
                        <h3>"Step Content"</h3>
                        <p>"Current step: {move || current_step_value.get() + 1}"</p>
                    </div>
                </div>

                // Step validation errors
                <div class="step-validation-errors" style="display: none;">
                    <h4>"Please fix the following errors:"</h4>
                    <ul>
                        <li class="validation-error">
                            <strong>"Field"</strong>: "Error message"
                        </li>
                    </ul>
                </div>

                // Navigation buttons
                <div class="wizard-navigation">
                    <button
                        type="button"
                        class="btn btn-secondary"
                        class:hidden=move || !allow_back_navigation.get() || current_step_value.get() == 0
                        on:click=go_to_previous_step
                    >
                        "Previous"
                    </button>

                    <button
                        type="button"
                        class="btn btn-primary"
                        class:disabled=move || !current_step_is_valid.get()
                        on:click=move |_| {
                            if is_last_step.get() {
                                submit_form.run(Some(()))
                            } else {
                                go_to_next_step.run(Some(()))
                            }
                        }
                    >
                        {move || if is_last_step.get() { "Submit" } else { "Next" }}
                    </button>

                    <button
                        type="button"
                        class="btn btn-outline"
                        class:hidden=move || !allow_skip_steps.get() || (current_step_value.get() >= total_steps - 1)
                        on:click=move |_| {
                            let current = current_step_value.get();
                            if current < total_steps - 1 {
                                on_step_change.run(current + 1);
                            }
                        }
                    >
                        "Skip"
                    </button>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone)]
pub struct WizardStep {
    pub title: String,
    pub description: Option<String>,
    pub content: View<HtmlElement<Div, (), ()>>,
}

// Helper function to validate current step
fn validate_current_step<T: Form + PartialEq>(form_handle: &FormHandle<T>, _step_index: usize) -> bool {
    // For now, we'll do basic validation
    // In a real implementation, this would check step-specific validation rules
    form_handle.is_valid().get()
}
