use crate::core::*;
use crate::hooks::*;
use crate::validation::ValidationErrors;
use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys;

pub mod checkbox;
pub mod code_input;
pub mod field;
pub mod field_array;
pub mod file_input;
pub mod file_upload_input;
pub mod form;
pub mod form_wizard;
pub mod input;
pub mod markdown_input;
pub mod radio;
pub mod rich_text_input;
pub mod select;
pub mod textarea;

pub use code_input::CodeInput;
pub use field_array::FieldArray;
pub use file_upload_input::FileUploadInput;
pub use input::*;
pub use markdown_input::MarkdownInput;
pub use rich_text_input::RichTextInput;

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

/// FormField component for rendering form fields
#[component]
pub fn FormField<T: Form + PartialEq + Clone + Send + Sync>(
    mut form: FormHandle<T>,
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

    let _field_value = use_field_value(&mut form, &name);
    let field_error = use_field_error(&mut form, &name);
    let field_dirty = use_field_dirty(&mut form);
    let _ = &field_dirty;
    let field_touched = use_field_touched(&mut form);

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

/// FormSubmit component for form submission
#[component]
pub fn FormSubmit<T: Form + PartialEq, F>(
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
