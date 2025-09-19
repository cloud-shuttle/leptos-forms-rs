//! FormDebug component
//!
//! This module provides the FormDebug component for development debugging.

use crate::core::*;
use leptos::prelude::*;

/// FormDebug component for development debugging
#[component]
pub fn FormDebug<T: Form + PartialEq + Clone + Send + Sync + std::fmt::Debug>(
    form: FormHandle<T>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let debug_class = class.unwrap_or_else(|| "form-debug".to_string());
    let values = form.values();
    let errors = form.errors();
    let is_valid = form.is_valid();
    let is_dirty = form.is_dirty();
    let is_submitting = form.is_submitting();

    view! {
        <details class=debug_class>
            <summary>"Form Debug Info"</summary>
            <div class="debug-content">
                <div class="debug-section">
                    <h4>"Form State"</h4>
                    <ul>
                        <li>"Valid: " {move || is_valid.get()}</li>
                        <li>"Dirty: " {move || is_dirty.get()}</li>
                        <li>"Submitting: " {move || is_submitting.get()}</li>
                    </ul>
                </div>

                <div class="debug-section">
                    <h4>"Form Values"</h4>
                    <pre>{move || format!("{:#?}", values.get())}</pre>
                </div>

                <div class="debug-section">
                    <h4>"Form Errors"</h4>
                    <pre>{move || format!("{:#?}", errors.get())}</pre>
                </div>
            </div>
        </details>
    }
}
