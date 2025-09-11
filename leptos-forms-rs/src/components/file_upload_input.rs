use crate::core::types::{FieldValue, FileConstraints};
use leptos::prelude::*;

/// File upload input component with drag and drop support
#[component]
pub fn FileUploadInput(
    /// Field name for the input
    #[prop(into)]
    name: String,
    /// Current value of the field
    #[prop(into)]
    value: Signal<FieldValue>,
    /// Callback when the value changes
    #[prop(into)]
    _on_change: Callback<FieldValue>,
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
    _has_error: Option<bool>,
    /// File constraints (types, size limits, etc.)
    #[prop(optional)]
    constraints: Option<FileConstraints>,
    /// Maximum number of files allowed
    #[prop(optional)]
    max_files: Option<usize>,
    /// Whether to show file preview
    #[prop(optional)]
    show_preview: Option<bool>,
    /// Whether to show upload progress
    #[prop(optional)]
    show_progress: Option<bool>,
    /// Whether to allow multiple file selection
    #[prop(optional)]
    multiple: Option<bool>,
) -> impl IntoView {
    let max_files = max_files.unwrap_or(10);
    let show_preview = show_preview.unwrap_or(true);
    let _show_progress = show_progress.unwrap_or(true);
    let multiple = multiple.unwrap_or(true);

    let current_files = move || match value.get() {
        FieldValue::Array(files) => files,
        _ => Vec::new(),
    };

    let file_count = move || current_files().len();

    let constraints_clone = constraints.clone();
    let constraints_info = move || {
        if let Some(constraints) = constraints_clone.as_ref() {
            let mut info = Vec::new();

            if let Some(max_size) = constraints.max_size {
                info.push(format!("Max size: {}MB", max_size / (1024 * 1024)));
            }

            if !constraints.accept.is_empty() {
                info.push(format!("Types: {}", constraints.accept.join(", ")));
            }

            if constraints.multiple {
                info.push("Multiple files allowed".to_string());
            }

            info.join(" • ")
        } else {
            "No restrictions".to_string()
        }
    };

    let file_list_view = move || {
        if show_preview && file_count() > 0 {
            Some(view! {
                <div class="file-list">
                    <div class="file-list-header">
                        <h4>"Selected Files"</h4>
                    </div>
                    <div class="file-grid">
                        {current_files().iter().map(|file| {
                            match file {
                                FieldValue::String(file_name) => {
                                    view! {
                                        <div class="file-preview">
                                            <div class="file-icon">"📁"</div>
                                            <div class="file-info">
                                                <div class="file-name">{file_name.clone()}</div>
                                                <div class="file-size">"Unknown size"</div>
                                            </div>
                                        </div>
                                    }
                                }
                                _ => view! {
                                    <div class="file-preview hidden">
                                        <div class="file-icon">"📁"</div>
                                        <div class="file-info">
                                            <div class="file-name">{"Unknown file".to_string()}</div>
                                            <div class="file-size">"Unknown size"</div>
                                        </div>
                                    </div>
                                }
                            }
                        }).collect::<Vec<_>>()}
                    </div>
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
        <div class={format!("file-upload-input {}", class.unwrap_or_default())}>
            <div class="file-upload-area">
                <div class="drop-zone">
                    <div class="upload-icon">"📁"</div>
                    <div class="upload-text">
                        <p>"Drag and drop files here or"</p>
                        <label class="file-input-label">
                            <input
                                type="file"
                                name={name.clone()}
                                multiple={constraints.as_ref().map(|c| c.multiple).unwrap_or(multiple)}
                                accept={constraints.as_ref().map(|c| c.accept.join(",")).unwrap_or_default()}
                                required={required}
                                disabled={disabled}
                                class="file-input"
                            />
                            "Browse Files"
                        </label>
                    </div>
                    <div class="upload-constraints">{constraints_info()}</div>
                    <div class="upload-limit">"Files: " {file_count} "/" {max_files}</div>
                </div>
            </div>

            {file_list_view}
            {error_view}
        </div>
    }
}
