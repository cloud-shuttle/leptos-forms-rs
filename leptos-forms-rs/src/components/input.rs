use crate::core::FieldType;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

/// Unified Input component that handles different field types
#[component]
pub fn Input(
    name: String,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] field_type: Option<FieldType>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] _error: Option<String>,
    #[prop(optional)] _has_error: Option<bool>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let input_type = field_type
        .map(|ft| match ft {
            FieldType::Text => "text",
            FieldType::Email => "email",
            FieldType::Password => "password",
            FieldType::Number(_) => "number",
            FieldType::Boolean => "checkbox",
            FieldType::Select(_) => "select",
            FieldType::MultiSelect(_) => "select",
            FieldType::Date => "date",
            FieldType::DateTime => "datetime-local",
            FieldType::File(_) => "file",
            FieldType::Array(_) => "text", // Default to text for arrays
            FieldType::Nested(_) => "text",
            FieldType::RichText | FieldType::Markdown | FieldType::Code => "text", // Default to text for nested forms
        })
        .unwrap_or("text");

    let is_required = required.unwrap_or(false);
    let is_disabled = disabled.unwrap_or(false);
    let input_class = class.unwrap_or_else(|| "form-input".to_string());

    view! {
        <div class="input-wrapper">
            <input
                type=input_type
                name=name.clone()
                value=value
                placeholder=placeholder
                required=is_required
                disabled=is_disabled
                class=input_class
                on:input=move |ev| {
                    if let Some(on_change_callback) = on_change {
                        if let Some(target) = ev.target() {
                            if let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>() {
                                let value = input.value();
                                on_change_callback.run(value);
                            }
                        }
                    }
                }
            />
            // Error display will be added later when we fix the view macro issue
        </div>
    }
}
