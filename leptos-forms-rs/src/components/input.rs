use leptos::*;
use crate::core::types::*;

/// Text input component
#[component]
pub fn TextInput(
    name: String,
    value: ReadSignal<Option<FieldValue>>,
    #[prop(optional)] input_type: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
) -> impl IntoView {
    let _ = value;
    let input_type = input_type.unwrap_or_else(|| "text".to_string());
    
    view! {
        <input
            type=input_type
            name=name
            placeholder=placeholder.unwrap_or_default()
            disabled=disabled
            required=required
            class="form-input"
        />
    }
}

/// Number input component
#[component]
pub fn NumberInput(
    name: String,
    value: ReadSignal<Option<FieldValue>>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
) -> impl IntoView {
    let _ = value;
    view! {
        <input
            type="number"
            name=name
            placeholder=placeholder.unwrap_or_default()
            disabled=disabled
            required=required
            class="form-input"
        />
    }
}

/// Checkbox input component
#[component]
pub fn CheckboxInput(
    name: String,
    value: ReadSignal<Option<FieldValue>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
) -> impl IntoView {
    let _ = value;
    view! {
        <input
            type="checkbox"
            name=name
            disabled=disabled
            required=required
            class="form-checkbox"
        />
    }
}

/// Select input component
#[component]
pub fn SelectInput(
    name: String,
    value: ReadSignal<Option<FieldValue>>,
    options: Vec<SelectOption>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
) -> impl IntoView {
    let _ = value;
    view! {
        <select
            name=name
            disabled=disabled
            required=required
            class="form-select"
        >
            {if let Some(placeholder_text) = placeholder {
                view! {
                    <option value="" disabled=true selected=true>
                        {placeholder_text}
                    </option>
                }
            } else {
                view! { <option style="display: none;"></option> }
            }}
            
            {options.iter().map(|option| {
                view! {
                    <option 
                        value=option.value.clone()
                        disabled=option.disabled
                    >
                        {option.label.clone()}
                    </option>
                }
            }).collect::<Vec<_>>()}
        </select>
    }
}
