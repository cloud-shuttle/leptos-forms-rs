use leptos::*;
use crate::core::*;
use crate::hooks::*;
use crate::validation::ValidationErrors;

pub mod form;
pub mod field;
pub mod input;
pub mod select;
pub mod checkbox;
pub mod radio;
pub mod textarea;
pub mod file_input;
pub mod field_array;
pub mod form_wizard;

pub use input::*;

/// Main Form component
#[component]
pub fn Form<T: Form>(
    form: FormHandle<T>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] novalidate: Option<bool>,
    #[prop(optional)] _children: Option<Children>,
) -> impl IntoView {
    let _ = &form;
    let form_id = id.unwrap_or_else(|| format!("form-{}", std::any::type_name::<T>()));
    let form_class = class.unwrap_or_else(|| "leptos-form".to_string());
    let novalidate_attr = if novalidate.unwrap_or(true) { "novalidate" } else { "" };
    
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
            {if let Some(children) = _children { children() } else { Fragment::new(vec![]) }}
        </form>
    }
}

/// FormField component for rendering form fields
#[component]
pub fn FormField<T: Form + PartialEq>(
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
    
    let field_value = use_field_value(&mut form, &name);
    let field_error = use_field_error(&mut form, &name);
    let field_dirty = use_field_dirty(&mut form, &name);
    let _ = &field_dirty;
    let field_touched = use_field_touched(&mut form, &name);
    
    let field_class = class.unwrap_or_else(|| "form-field".to_string());
    let is_required = required.unwrap_or(false);
    let is_disabled = disabled.unwrap_or(false);
    
    let show_error = move || {
        field_error.get().is_some() && field_touched.get()
    };
    
    view! {
        <div class=field_class>
            {move || {
                if let Some(label_text) = label_clone.clone() {
                    view! {
                        <label for=name_clone1.clone() class="form-label">
                            {label_text}
                            {if is_required {
                                view! { <span class="required">" *"</span> }
                            } else {
                                view! { <span style="display: none;"></span> }
                            }}
                        </label>
                    }
                } else {
                    view! { <label style="display: none;"></label> }
                }
            }}
            
            <div class="form-input-wrapper">
                {move || {
                    // Clone variables for use inside this closure
                    let name_clone = name_clone2.clone();
                    let field_type_clone = field_type_clone.clone();
                    let placeholder_clone = placeholder_clone.clone();
                    
                    // Default input rendering based on field type
                    match field_type_clone {
                        Some(FieldType::Text) => view! {
                            <TextInput
                                name=name_clone.clone()
                                value=field_value
                                placeholder=placeholder_clone.unwrap_or_default()
                                disabled=is_disabled
                                required=is_required
                            />
                        },
                        Some(FieldType::Email) => view! {
                            <TextInput
                                name=name_clone.clone()
                                value=field_value
                                input_type="email".to_string()
                                placeholder=placeholder_clone.unwrap_or_default()
                                disabled=is_disabled
                                required=is_required
                            />
                        },
                        Some(FieldType::Password) => view! {
                            <TextInput
                                name=name_clone.clone()
                                value=field_value
                                input_type="password".to_string()
                                placeholder=placeholder_clone.unwrap_or_default()
                                disabled=is_disabled
                                required=is_required
                            />
                        },
                        Some(FieldType::Number(_)) => view! {
                            <NumberInput
                                name=name_clone.clone()
                                value=field_value
                                placeholder=placeholder_clone.unwrap_or_default()
                                disabled=is_disabled
                                required=is_required
                            />
                        },
                        Some(FieldType::Boolean) => view! {
                            <CheckboxInput
                                name=name_clone.clone()
                                value=field_value
                                disabled=is_disabled
                                required=is_required
                            />
                        },
                        Some(FieldType::Select(options)) => view! {
                            <SelectInput
                                name=name_clone.clone()
                                value=field_value
                                options=options
                                placeholder=placeholder_clone.unwrap_or_default()
                                disabled=is_disabled
                                required=is_required
                            />
                        },
                        _ => view! {
                            <TextInput
                                name=name_clone.clone()
                                value=field_value
                                placeholder=placeholder_clone.unwrap_or_default()
                                disabled=is_disabled
                                required=is_required
                            />
                        }
                    }.into_view()
                }}
            </div>
            
            {if show_error() {
                view! {
                    <div class="form-error">
                        {field_error.get().unwrap_or_default()}
                    </div>
                }
            } else {
                view! { <div style="display: none;"></div> }
            }}
            
            {if let Some(help) = help_text_clone {
                view! {
                    <div class="form-help">
                        {help}
                    </div>
                }
            } else {
                view! { <div style="display: none;"></div> }
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
                                view! {
                                    <li class="error-item">{error}</li>
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                    </div>
                }
            } else {
                view! { <div style="display: none;"></div> }
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
    F: Fn(&T) -> Result<(), crate::error::FormError> + 'static + Clone,
{
    let is_submitting = form.is_submitting();
    let is_valid = form.is_valid();
    
    let button_class = class.unwrap_or_else(|| "form-submit".to_string());
    let disabled_class = disabled_class.unwrap_or_else(|| "disabled".to_string());
    let loading_text = loading_text.unwrap_or_else(|| "Submitting...".to_string());
    
    let submit_handler = move |_| {
        let form_clone = form.clone();
        let on_submit_clone = on_submit.clone();
        
        spawn_local(async move {
            let form_data = form_clone.get_values().get();
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
            on:click=submit_handler
        >
            {move || {
                if is_submitting.get() {
                    view! { <span>{loading_text.clone()}</span> }
                } else {
                    view! { <span>"Submit"</span> }
                }
            }}
        </button>
    }
}

/// FormReset component for resetting forms
#[component]
pub fn FormReset<T: Form + PartialEq>(
    form: FormHandle<T>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] confirm_message: Option<String>,
    children: Option<Children>,
) -> impl IntoView {
    let button_class = class.unwrap_or_else(|| "form-reset".to_string());
    let confirm_message = confirm_message.unwrap_or_else(|| "Are you sure you want to reset the form?".to_string());
    
    let reset_handler = move |_| {
        if let Some(window) = web_sys::window() {
            if window.confirm_with_message(&confirm_message).unwrap_or(false) {
                form.reset();
            }
        }
    };
    
    view! {
        <button
            type="button"
            class=button_class
            on:click=reset_handler
        >
            {if let Some(children) = children { children() } else { Fragment::new(vec![]) }}
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
pub fn FormDebug<T: Form + PartialEq + std::fmt::Debug>(
    form: FormHandle<T>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let debug_class = class.unwrap_or_else(|| "form-debug".to_string());
    let values = form.get_values();
    let errors = form.get_errors();
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


