//! FormSubmit component
//!
//! This module provides the FormSubmit component for form submission handling.

use crate::core::*;
use leptos::prelude::*;
use leptos::task::spawn_local;

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
    F: Fn(&T) -> Result<(), crate::error::FormError> + 'static + Clone,
{
    let is_submitting = form.is_submitting();
    let is_valid = form.is_valid();

    let button_class = class.unwrap_or_else(|| "form-submit".to_string());
    let disabled_class = disabled_class.unwrap_or_else(|| "disabled".to_string());
    let loading_text = loading_text.unwrap_or_else(|| "Submitting...".to_string());

    let _submit_handler = move |_: leptos::ev::MouseEvent| {
        let form_clone = form.clone();
        let on_submit_clone = on_submit.clone();

        spawn_local(async move {
            let form_data = form_clone.values().get();
            if let Err(error) = on_submit_clone(&form_data) {
                log::error!("Form submission error: {}", error);
            }
        });
    };

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
            // TODO: Fix event handler for Leptos 0.8
            // on:click=submit_handler
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
