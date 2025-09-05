use crate::components::FieldArray;
use crate::core::{FieldValue, Form, FormHandle};
use crate::hooks::use_form;
use leptos::prelude::*;

#[component]
pub fn FieldArrayExample() -> impl IntoView {
    let form_handle = use_form::<ExampleForm>();

    // Example form data
    let form_data = Signal::derive(move || ExampleForm {
        name: "Example Form".to_string(),
        tags: vec![
            "rust".to_string(),
            "leptos".to_string(),
            "forms".to_string(),
        ],
        items: vec![
            ExampleItem {
                title: "Item 1".to_string(),
                description: "First example item".to_string(),
                active: true,
            },
            ExampleItem {
                title: "Item 2".to_string(),
                description: "Second example item".to_string(),
                active: false,
            },
        ],
    });

    // Render function for tags
    let render_tag = Callback::new(|(index, value): (usize, FieldValue)| {
        if let FieldValue::String(tag) = value {
            view! {
                <div class="tag-item">
                    <span class="tag-text">{tag}</span>
                </div>
            }
        } else {
            view! { <div>"Invalid tag"</div> }
        }
    });

    // Render function for items
    let render_item = Callback::new(|(index, value): (usize, FieldValue)| {
        if let FieldValue::Object(obj) = value {
            let title = obj
                .get("title")
                .and_then(|v| v.as_string())
                .unwrap_or("No title");
            let description = obj
                .get("description")
                .and_then(|v| v.as_string())
                .unwrap_or("No description");
            let active = obj
                .get("active")
                .and_then(|v| v.as_boolean())
                .unwrap_or(false);

            view! {
                <div class="example-item">
                    <div class="item-header">
                        <h4 class="item-title">{title}</h4>
                        <span class="item-status" class:active=move || active>
                            {move || if active { "Active" } else { "Inactive" }}
                        </span>
                    </div>
                    <p class="item-description">{description}</p>
                </div>
            }
        } else {
            view! { <div>"Invalid item"</div> }
        }
    });

    view! {
        <div class="field-array-example">
            <h2>"Field Array Examples"</h2>

            <div class="example-section">
                <h3>"Tags Array"</h3>
                <p>"A simple array of string tags with add/remove functionality."</p>

                <FieldArray
                    field_name="tags".to_string()
                    form_handle=form_handle.clone()
                    render_item=render_tag
                    min=Some(1)
                    max=Some(10)
                />
            </div>

            <div class="example-section">
                <h3>"Items Array"</h3>
                <p>"A complex array of objects with custom rendering and validation."</p>

                <FieldArray
                    field_name="items".to_string()
                    form_handle=form_handle.clone()
                    render_item=render_item
                    min=Some(1)
                    max=Some(5)
                />
            </div>

            <div class="form-actions">
                <button
                    type="button"
                    class="btn btn-primary"
                    on:click=move |_| {
                        log::info!("Form submitted: {:?}", form_data.get());
                    }
                >
                    "Submit Form"
                </button>

                <button
                    type="button"
                    class="btn btn-secondary"
                    on:click=move |_| {
                        // Reset form logic would go here
                        log::info!("Form reset");
                    }
                >
                    "Reset Form"
                </button>
            </div>
        </div>
    }
}

// Example form structure
#[derive(Clone, Debug, PartialEq)]
pub struct ExampleForm {
    pub name: String,
    pub tags: Vec<String>,
    pub items: Vec<ExampleItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExampleItem {
    pub title: String,
    pub description: String,
    pub active: bool,
}

// Example CSS for the demo
pub const FIELD_ARRAY_EXAMPLE_CSS: &str = r#"
.field-array-example {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
}

.field-array-example h2 {
    text-align: center;
    color: #1e293b;
    margin-bottom: 2rem;
}

.example-section {
    margin-bottom: 3rem;
    padding: 1.5rem;
    border: 1px solid #e2e8f0;
    border-radius: 8px;
    background: #f8fafc;
}

.example-section h3 {
    color: #1e293b;
    margin-bottom: 0.5rem;
}

.example-section p {
    color: #64748b;
    margin-bottom: 1.5rem;
    font-size: 0.875rem;
}

.tag-item {
    background: #3b82f6;
    color: white;
    padding: 0.25rem 0.75rem;
    border-radius: 9999px;
    font-size: 0.875rem;
    font-weight: 500;
}

.example-item {
    padding: 0.5rem;
}

.item-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
}

.item-title {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: #1e293b;
}

.item-status {
    font-size: 0.75rem;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    background: #f1f5f9;
    color: #64748b;
}

.item-status.active {
    background: #dcfce7;
    color: #166534;
}

.item-description {
    margin: 0;
    font-size: 0.875rem;
    color: #64748b;
    line-height: 1.4;
}

.form-actions {
    display: flex;
    gap: 1rem;
    justify-content: center;
    margin-top: 2rem;
}

.btn {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
}

.btn-primary {
    background: #3b82f6;
    color: white;
}

.btn-primary:hover {
    background: #2563eb;
}

.btn-secondary {
    background: #f1f5f9;
    color: #475569;
    border: 1px solid #e2e8f0;
}

.btn-secondary:hover {
    background: #e2e8f0;
    border-color: #cbd5e1;
}
"#;
