//! FormField component
//!
//! This module provides the FormField component for rendering individual form fields.

use crate::core::*;
use crate::hooks::*;
use leptos::prelude::*;

/// FormField component for rendering form fields
#[component]
pub fn FormField<T: Form + PartialEq + Clone + Send + Sync>(
    form: FormHandle<T>,
    name: String,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] help_text: Option<String>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] field_type: Option<FieldType>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] _children: Option<Children>,
) -> impl IntoView {
    // Clone props that will be used in multiple closures
    let name_clone1 = name.clone();
    let name_clone2 = name.clone();
    let label_clone = label.clone();
    let placeholder_clone = placeholder.clone();
    let help_text_clone = help_text.clone();
    let field_type_clone = field_type.clone();

    let _field_value = use_field_value(&form, &name);
    let field_error = use_field_error(&form, &name);
    let field_dirty = use_field_dirty(&form);
    let _ = &field_dirty;
    let field_touched = use_field_touched(&form);

    let field_class = class.unwrap_or_else(|| "form-field".to_string());
    let is_required = required.unwrap_or(false);
    let is_disabled = disabled.unwrap_or(false);

    let show_error = move || !field_error.get().is_empty() && field_touched.get();

    view! {
        <div class=field_class>
            {move || {
                if let Some(label_text) = label_clone.clone() {
                    view! {
                        <label for=name_clone1.clone() class="form-label">
                            {label_text}
                            {if is_required {
                                view! { <span class="required">{String::from(" *")}</span> }
                            } else {
                                view! { <span class="hidden">{String::new()}</span> }
                            }}
                        </label>
                    }
                } else {
                    view! {
                        <label for=String::new() class="hidden">
                            {String::new()}
                            {view! { <span class="hidden">{String::new()}</span> }}
                        </label>
                    }
                }
            }}

            <div class="form-input-wrapper">
                {move || {
                    // Clone variables for use inside this closure
                    let name_clone = name_clone2.clone();
                    let field_type_clone = field_type_clone.clone();
                    let placeholder_clone = placeholder_clone.clone();

                    // Render the appropriate input component based on field type
                    let input_type = if let Some(FieldType::Email) = field_type_clone {
                        "email"
                    } else if let Some(FieldType::Password) = field_type_clone {
                        "password"
                    } else if let Some(FieldType::Number(_)) = field_type_clone {
                        "number"
                    } else if let Some(FieldType::Boolean) = field_type_clone {
                        "checkbox"
                    } else {
                        "text"
                    };

                    view! {
                        <input
                            type=input_type
                            name=name_clone
                            placeholder=placeholder_clone.unwrap_or_default()
                            disabled=is_disabled
                            required=is_required
                            class="form-input"
                        />
                    }
                }}
            </div>

            {if show_error() {
                view! {
                    <div class="form-error">
                        {field_error.get().join(", ")}
                    </div>
                }
            } else {
                view! { <div class="hidden">{String::new()}</div> }
            }}

            {if let Some(help) = help_text_clone {
                view! {
                    <div class="form-help">
                        {help}
                    </div>
                }
            } else {
                view! { <div class="hidden">{String::new()}</div> }
            }}
        </div>
    }
}
