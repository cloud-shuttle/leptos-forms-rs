use crate::core::types::FieldValue;
use leptos::prelude::*;

/// Rich text input component with advanced formatting capabilities
#[component]
pub fn RichTextInput(
    /// Field name for the input
    #[prop(into)]
    name: String,
    /// Current value of the field
    #[prop(into)]
    value: Signal<FieldValue>,
    /// Callback when the value changes
    #[prop(into)]
    _on_change: Callback<FieldValue>,
    /// Placeholder text
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Whether the field is required
    #[prop(optional)]
    required: Option<bool>,
    /// Whether the field is disabled
    #[prop(optional)]
    disabled: Option<bool>,
    /// CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Error message to display
    #[prop(optional, into)]
    error: Option<String>,
    /// Whether the field has an error
    #[prop(optional)]
    has_error: Option<bool>,
    /// Toolbar configuration
    #[prop(optional)]
    show_toolbar: Option<bool>,
    /// Minimum height in pixels
    #[prop(optional)]
    min_height: Option<u32>,
    /// Maximum height in pixels
    #[prop(optional)]
    max_height: Option<u32>,
) -> impl IntoView {
    let show_toolbar = show_toolbar.unwrap_or(true);
    let min_height = min_height.unwrap_or(200);
    let max_height = max_height.unwrap_or(600);

    let current_value = move || match value.get() {
        FieldValue::String(s) => s,
        _ => String::new(),
    };

    let toolbar_view = move || {
        if show_toolbar {
            Some(view! {
                <div class="rich-text-toolbar">
                    <button type="button" class="toolbar-btn">"B"</button>
                    <button type="button" class="toolbar-btn">"I"</button>
                    <button type="button" class="toolbar-btn">"U"</button>
                    <div class="toolbar-separator"></div>
                    <button type="button" class="toolbar-btn">"•"</button>
                    <button type="button" class="toolbar-btn">"1."</button>
                    <div class="toolbar-separator"></div>
                    <button type="button" class="toolbar-btn">"🔗"</button>
                    <button type="button" class="toolbar-btn">"🖼️"</button>
                    <div class="toolbar-separator"></div>
                    <button type="button" class="toolbar-btn">"🧹"</button>
                </div>
            })
        } else {
            None
        }
    };

    let error_view = move || {
        error.as_ref().map(|error_msg| {
            view! {
                <div class="error-message">
                    {error_msg.clone()}
                </div>
            }
        })
    };

    view! {
        <div class={format!("rich-text-input {}", class.unwrap_or_default())}>
            {toolbar_view}

            <textarea
                name={name.clone()}
                placeholder={placeholder}
                required={required}
                disabled={disabled}
                class={move || {
                    let mut classes = vec!["rich-text-editor"];
                    if has_error.unwrap_or(false) {
                        classes.push("error");
                    }
                    if disabled.unwrap_or(false) {
                        classes.push("disabled");
                    }
                    classes.join(" ")
                }}
                style={format!("min-height: {}px; max-height: {}px;", min_height, max_height)}
            >{current_value}</textarea>

            {error_view}
        </div>
    }
}
