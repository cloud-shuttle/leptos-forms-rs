use crate::core::types::FieldValue;
use leptos::prelude::*;

/// Code input component with syntax highlighting support
#[component]
pub fn CodeInput(
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
    /// Programming language for syntax highlighting
    #[prop(optional, into)]
    _language: Option<String>,
    /// Whether to show line numbers
    #[prop(optional)]
    show_line_numbers: Option<bool>,
    /// Whether to show language selector
    #[prop(optional)]
    show_language_selector: Option<bool>,
    /// Whether to show fullscreen toggle
    #[prop(optional)]
    show_fullscreen: Option<bool>,
    /// Minimum height in pixels
    #[prop(optional)]
    min_height: Option<u32>,
    /// Maximum height in pixels
    #[prop(optional)]
    max_height: Option<u32>,
) -> impl IntoView {
    let show_line_numbers = show_line_numbers.unwrap_or(true);
    let show_language_selector = show_language_selector.unwrap_or(true);
    let _show_fullscreen = show_fullscreen.unwrap_or(true);
    let min_height = min_height.unwrap_or(200);
    let max_height = max_height.unwrap_or(600);

    let current_value = move || match value.get() {
        FieldValue::String(s) => s,
        _ => String::new(),
    };

    let supported_languages = vec![
        ("rust", "Rust"),
        ("javascript", "JavaScript"),
        ("typescript", "TypeScript"),
        ("python", "Python"),
        ("java", "Java"),
        ("cpp", "C++"),
        ("c", "C"),
        ("go", "Go"),
        ("html", "HTML"),
        ("css", "CSS"),
        ("sql", "SQL"),
        ("json", "JSON"),
        ("yaml", "YAML"),
        ("toml", "TOML"),
        ("markdown", "Markdown"),
    ];

    let language_selector_view = move || {
        if show_language_selector {
            Some(view! {
                <select class="language-selector">
                    {supported_languages.iter().map(|(value, label)| {
                        view! {
                            <option value={*value}>{*label}</option>
                        }
                    }).collect::<Vec<_>>()}
                </select>
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
        <div class={format!("code-input {}", class.unwrap_or_default())}>
            <div class="code-toolbar">
                {language_selector_view}

                <div class="toolbar-actions">
                    <button type="button" class="toolbar-btn">"📋"</button>
                    <button type="button" class="toolbar-btn">"💾"</button>
                    <button type="button" class="toolbar-btn">"📝"</button>
                    <button type="button" class="toolbar-btn">"🔍"</button>
                    <button type="button" class="toolbar-btn">"⚙️"</button>
                    <button type="button" class="toolbar-btn">"⛶"</button>
                </div>
            </div>

            <div class="code-editor-container">
                <textarea
                    name={name.clone()}
                    placeholder={placeholder}
                    required={required}
                    disabled={disabled}
                    class={move || {
                        let mut classes = vec!["code-textarea"];
                        if has_error.unwrap_or(false) {
                            classes.push("error");
                        }
                        if disabled.unwrap_or(false) {
                            classes.push("disabled");
                        }
                        if show_line_numbers {
                            classes.push("with-line-numbers");
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
