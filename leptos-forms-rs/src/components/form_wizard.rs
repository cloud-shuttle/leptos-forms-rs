use leptos::prelude::*;

/// Form wizard component for multi-step forms
#[component]
pub fn FormWizard(
    #[prop(into)] steps: Vec<String>,
    #[prop(into)] children: Children,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let (current_step_index, set_current_step_index) = signal(0);
    let steps1 = steps.clone();
    let steps2 = steps.clone();
    let steps3 = steps.clone();
    let steps4 = steps.clone();
    let steps5 = steps.clone();
    
    let next_step = move |_| {
        let current = current_step_index.get();
        if current < steps1.len() - 1 {
            set_current_step_index.set(current + 1);
        }
    };
    
    let prev_step = move |_| {
        let current = current_step_index.get();
        if current > 0 {
            set_current_step_index.set(current - 1);
        }
    };
    
    let _go_to_step = Callback::new(move |step: usize| {
        if step < steps2.len() {
            set_current_step_index.set(step);
        }
    });
    
    let complete = move |_| {
        log::info!("Form wizard completed");
    };
    
    let validate_current_step = move || {
        // For now, always return true
        // In a real implementation, you'd validate the current step
        true
    };
    
    let can_go_next = Memo::new(move |_| {
        let current = current_step_index.get();
        current < steps3.len() - 1 && validate_current_step()
    });
    
    let can_go_prev = Memo::new(move |_| {
        let current = current_step_index.get();
        current > 0
    });
    
    let is_last_step = Memo::new(move |_| {
        let current = current_step_index.get();
        current == steps4.len() - 1
    });
    
    let wizard_class = class.unwrap_or_else(|| "form-wizard".to_string());
    
    view! {
        <div class={wizard_class}>
            // Progress bar
            <div class="wizard-progress">
                {steps5.iter().enumerate().map(|(step_index, step_name)| {
                    let is_active = Memo::new(move |_| current_step_index.get() == step_index);
                    let is_completed = Memo::new(move |_| current_step_index.get() > step_index);
                    
                    view! {
                        <div class="wizard-step" class:active=is_active class:completed=is_completed>
                            <div class="step-indicator">
                                {if is_completed.get() { "✓".to_string() } else { (step_index + 1).to_string() }}
                            </div>
                            <div class="step-label">
                                {step_name.clone()}
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
            
            // Step content
            <div class="wizard-content">
                {children().into_any()}
            </div>
            
            // Navigation buttons
            <div class="wizard-navigation">
                <button 
                    type="button" 
                    class="nav-button prev-button"
                    disabled=move || !can_go_prev.get()
                    on:click=prev_step
                >
                    "Previous"
                </button>
                
                {if is_last_step.get() {
                    view! {
                        <button 
                            type="button" 
                            class="nav-button complete-button"
                            on:click=complete
                        >
                            "Complete"
                        </button>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
                
                <button 
                    type="button" 
                    class="nav-button next-button"
                    disabled=move || !can_go_next.get()
                    on:click=next_step
                >
                    "Next"
                </button>
            </div>
        </div>
    }
}
