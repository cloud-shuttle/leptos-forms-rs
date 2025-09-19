//! FormSubmit component
//!
//! This module provides the FormSubmit component for form submission handling.

use crate::core::*;
use crate::events::FormSubmitHandler as EventFormSubmitHandler;
use leptos::prelude::*;

/// FormSubmit component for form submission
#[component]
pub fn FormSubmit<T, F>(
    form: FormHandle<T>,
    on_submit: F,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] disabled_class: Option<String>,
    #[prop(optional)] loading_text: Option<String>,
    #[prop(optional)] _children: Option<Children>,
) -> impl IntoView
where
    T: Form + PartialEq + Clone + Send + Sync,
    F: Fn(&T) -> Result<(), crate::error::FormError> + 'static + Clone + Send + Sync,
{
    let is_submitting = form.is_submitting();
    let is_valid = form.is_valid();

    let button_class = class.unwrap_or_else(|| "form-submit".to_string());
    let disabled_class = disabled_class.unwrap_or_else(|| "disabled".to_string());
    let loading_text = loading_text.unwrap_or_else(|| "Submitting...".to_string());

    // Create the submit handler using the event system
    let submit_handler = EventFormSubmitHandler::new(form.clone());

    view! {
        <button
            type="submit"
            class=move || {
                let mut classes = vec![button_class.clone()];
                if !is_valid.get() {
                    classes.push(disabled_class.clone());
                }
                if is_submitting.get() {
                    classes.push("loading".to_string());
                }
                classes.join(" ")
            }
            disabled=move || !is_valid.get() || is_submitting.get()
            on:click=submit_handler.handle_submit()
        >
            {move || {
                if is_submitting.get() {
                    view! { <span>{loading_text.clone()}</span> }
                } else {
                    view! { <span>{String::from("Submit")}</span> }
                }
            }}
        </button>
    }
}
