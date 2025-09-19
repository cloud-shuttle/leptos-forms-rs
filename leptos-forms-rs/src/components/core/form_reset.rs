//! FormReset component
//!
//! This module provides the FormReset component for resetting forms.

use crate::core::*;
use leptos::prelude::*;
use web_sys;

/// FormReset component for resetting forms
#[component]
pub fn FormReset<T: Form + PartialEq + Clone + Send + Sync>(
    form: FormHandle<T>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] confirm_message: Option<String>,
    children: Option<Children>,
) -> impl IntoView {
    let button_class = class.unwrap_or_else(|| "form-reset".to_string());
    let confirm_message =
        confirm_message.unwrap_or_else(|| "Are you sure you want to reset the form?".to_string());

    let _reset_handler = move |_: leptos::ev::MouseEvent| {
        if let Some(window) = web_sys::window() {
            if window
                .confirm_with_message(&confirm_message)
                .unwrap_or(false)
            {
                form.reset();
            }
        }
    };

    view! {
        <button
            type="button"
            class=button_class
            // TODO: Fix event handler for Leptos 0.8
            // on:click=reset_handler
        >
            {if let Some(children) = children { children() } else { view! { <span></span> }.into_any() }}
        </button>
    }
}
