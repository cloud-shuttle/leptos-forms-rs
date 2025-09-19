//! Main Form component
//!
//! This module provides the main Form component for rendering form containers.

use crate::core::*;
use leptos::prelude::*;

/// Main Form component
#[component]
pub fn Form<T: Form + PartialEq + Clone + Send + Sync>(
    form: FormHandle<T>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] novalidate: Option<bool>,
    #[prop(optional)] _children: Option<Children>,
) -> impl IntoView {
    let _ = &form;
    let form_id = id.unwrap_or_else(|| format!("form-{}", std::any::type_name::<T>()));
    let form_class = class.unwrap_or_else(|| "leptos-form".to_string());
    let novalidate_attr = if novalidate.unwrap_or(true) {
        "novalidate"
    } else {
        ""
    };

    view! {
        <form
            id=form_id
            class=form_class
            novalidate=novalidate_attr
            on:submit=move |ev| {
                ev.prevent_default();
                // Handle form submission
            }
        >
            {match _children {
                Some(children) => children(),
                None => view! { <div class="hidden">{String::new()}</div> }.into_any()
            }}
        </form>
    }
}
