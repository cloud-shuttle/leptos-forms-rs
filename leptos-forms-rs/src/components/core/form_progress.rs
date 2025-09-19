//! FormProgress component
//!
//! This module provides the FormProgress component for multi-step forms.

use leptos::prelude::*;

/// FormProgress component for multi-step forms
#[component]
pub fn FormProgress(
    current_step: ReadSignal<usize>,
    total_steps: usize,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let progress_class = class.unwrap_or_else(|| "form-progress".to_string());

    let progress_percentage = move || {
        if total_steps == 0 {
            0.0
        } else {
            ((current_step.get() + 1) as f64 / total_steps as f64) * 100.0
        }
    };

    view! {
        <div class=progress_class>
            <div class="progress-bar">
                <div
                    class="progress-fill"
                    style=move || format!("width: {}%", progress_percentage())
                ></div>
            </div>
            <div class="progress-text">
                {move || format!("Step {} of {}", current_step.get() + 1, total_steps)}
            </div>
        </div>
    }
}
