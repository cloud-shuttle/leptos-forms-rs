//! FormErrors component
//!
//! This module provides the FormErrors component for displaying form-level errors.

use crate::validation::ValidationErrors;
use leptos::prelude::*;

/// FormErrors component for displaying form-level errors
#[component]
pub fn FormErrors(
    errors: Memo<ValidationErrors>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let error_class = class.unwrap_or_else(|| "form-errors".to_string());

    let error_class_clone = error_class.clone();
    view! {
        {move || {
            let validation_errors = errors.get();
            if !validation_errors.form_errors.is_empty() {
                view! {
                    <div class=error_class_clone.clone()>
                        <ul class="error-list">
                            {validation_errors.form_errors.iter().map(|error| {
                                let error_text = error.clone();
                                view! {
                                    <li class="error-item">{error_text}</li>
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                    </div>
                }
            } else {
                view! {
                    <div class="hidden".to_string()>
                        <ul class="error-list">{Vec::<_>::new()}</ul>
                    </div>
                }
            }
        }}
    }
}
