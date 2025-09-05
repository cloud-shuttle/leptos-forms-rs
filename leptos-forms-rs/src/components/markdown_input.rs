use leptos::prelude::*;
use crate::core::types::FieldValue;

/// Markdown input component with live preview
#[component]
pub fn MarkdownInput(
    /// Field name for the input
    #[prop(into)] name: String,
    /// Current value of the field
    #[prop(into)] value: Signal<FieldValue>,
    /// Callback when the value changes
    #[prop(into)] on_change: Callback<FieldValue>,
    /// Placeholder text
    #[prop(optional, into)] placeholder: Option<String>,
    /// Whether the field is required
    #[prop(optional)] required: Option<bool>,
    /// Whether the field is disabled
    #[prop(optional)] disabled: Option<bool>,
    /// CSS classes
    #[prop(optional, into)] class: Option<String>,
    /// Error message to display
    #[prop(optional, into)] error: Option<String>,
    /// Whether the field has an error
    #[prop(optional)] has_error: Option<bool>,
    /// Whether to show preview
    #[prop(optional)] show_preview: Option<bool>,
    /// Whether to show toolbar
    #[prop(optional)] show_toolbar: Option<bool>,
    /// Minimum height in pixels
    #[prop(optional)] min_height: Option<u32>,
    /// Maximum height in pixels
    #[prop(optional)] max_height: Option<u32>,
) -> impl IntoView {
    let show_preview = show_preview.unwrap_or(true);
    let show_toolbar = show_toolbar.unwrap_or(true);
    let min_height = min_height.unwrap_or(200);
    let max_height = max_height.unwrap_or(600);
    
    let current_value = move || {
        match value.get() {
            FieldValue::String(s) => s,
            _ => String::new(),
        }
    };
    
    let toolbar_view = move || {
        if show_toolbar {
            Some(view! {
                <div class="markdown-toolbar">
                    <button type="button" class="toolbar-btn">"H1"</button>
                    <button type="button" class="toolbar-btn">"H2"</button>
                    <button type="button" class="toolbar-btn">"H3"</button>
                    <div class="toolbar-separator"></div>
                    <button type="button" class="toolbar-btn">"B"</button>
                    <button type="button" class="toolbar-btn">"I"</button>
                    <div class="toolbar-separator"></div>
                    <button type="button" class="toolbar-btn">"•"</button>
                    <button type="button" class="toolbar-btn">"1."</button>
                    <div class="toolbar-separator"></div>
                    <button type="button" class="toolbar-btn">"🔗"</button>
                    <button type="button" class="toolbar-btn">"🖼️"</button>
                    <button type="button" class="toolbar-btn">"```"</button>
                    <button type="button" class="toolbar-btn">">"</button>
                    <div class="toolbar-separator"></div>
                    <button type="button" class="toolbar-btn">"👁️"</button>
                    <button type="button" class="toolbar-btn">"⫴"</button>
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
        <div class={format!("markdown-input {}", class.unwrap_or_default())}>
            {toolbar_view}
            
            <div class="markdown-editor-container">
                <textarea
                    name={name.clone()}
                    placeholder={placeholder}
                    required={required}
                    disabled={disabled}
                    class={move || {
                        let mut classes = vec!["markdown-textarea"];
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
            </div>
            
            {error_view}
        </div>
    }
}
