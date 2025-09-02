use leptos::prelude::*;
use crate::core::{Form, FormHandle, FieldValue};
use crate::hooks::use_field_array;

#[component]
pub fn FieldArray<T>(
    field_name: String,
    form_handle: FormHandle<T>,
    render_item: Callback<(usize, FieldValue), View<()>>,
    #[prop(optional)] min: Option<usize>,
    #[prop(optional)] max: Option<usize>,
    #[prop(optional)] can_add: Option<Signal<bool>>,
    #[prop(optional)] can_remove: Option<Signal<bool>>,
    #[prop(optional)] _show_validation: Option<Signal<bool>>,
    #[prop(optional)] _allow_reordering: Option<Signal<bool>>,
) -> impl IntoView 
where
    T: Form + Clone + PartialEq + 'static,
{
    // Items count for validation
    let items_count = Signal::derive({
        let form_handle = form_handle.clone();
        let field_name = field_name.clone();
        move || {
            let field_array_handle = use_field_array::<T, FieldValue>(&form_handle, &field_name);
            field_array_handle.len().get()
        }
    });
    
    // Can add/remove based on props and validation
    let can_add_item = Signal::derive(move || {
        let base_can_add = can_add.as_ref().map(|s| s.get()).unwrap_or(true);
        let max_limit = max.map(|m| items_count.get() < m).unwrap_or(true);
        base_can_add && max_limit
    });

    let can_remove_item = Signal::derive(move || {
        let base_can_remove = can_remove.as_ref().map(|s| s.get()).unwrap_or(true);
        let min_limit = min.map(|m| items_count.get() > m).unwrap_or(true);
        base_can_remove && min_limit
    });

    // Add item handler
    let add_item = {
        let mut form_handle = form_handle.clone();
        let field_name = field_name.clone();
        move |_| {
            let _ = form_handle.add_field_array_item(&field_name);
        }
    };

    view! {
        <div class="field-array">
            // Header with add button and count
            <div class="field-array-header">
                <div class="field-array-info">
                    <span class="field-array-label">{field_name.clone()}</span>
                    <span class="field-array-count">
                        {move || format!("({} items)", items_count.get())}
                    </span>
                </div>
                
                <button
                    type="button"
                    class="add-item-btn"
                    class:disabled=move || !can_add_item.get()
                    on:click=add_item
                >
                    "Add Item"
                </button>
            </div>

            // Items list
            <div class="field-array-items">
                {move || {
                    let form_handle = form_handle.clone();
                    let field_name = field_name.clone();
                    let field_array_handle = use_field_array::<T, FieldValue>(&form_handle, &field_name);
                    let items = field_array_handle.items().get();
                    
                    items.into_iter().enumerate().map(|(index, item)| {
                        let remove_item = {
                            let mut form_handle = form_handle.clone();
                            let field_name = field_name.clone();
                            move |_| {
                                let _ = form_handle.remove_field_array_item(&field_name, index);
                            }
                        };
                        
                        view! {
                            <div class="field-array-item">
                                // Item content
                                <div class="item-content">
                                    {render_item.run((index, item))}
                                </div>

                                // Item actions
                                <div class="item-actions">
                                    // Remove button
                                    <button
                                        type="button"
                                        class="remove-item-btn"
                                        class:disabled=move || !can_remove_item.get()
                                        title="Remove item"
                                        on:click=remove_item
                                    >
                                        "Remove"
                                    </button>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>

            // Empty state
            <div class="field-array-empty" style="display: none;">
                <div class="empty-icon">Note</div>
                <div class="empty-message">"No items yet. Click 'Add Item' to get started."</div>
            </div>

            // Footer with summary
            <div class="field-array-footer">
                <div class="field-array-summary">
                    {move || {
                        let count = items_count.get();
                        let min_msg = min.map(|m| format!("Min: {}", m));
                        let max_msg = max.map(|m| format!("Max: {}", m));
                        
                        let mut parts = vec![format!("Total: {}", count)];
                        if let Some(min) = min_msg {
                            parts.push(min);
                        }
                        if let Some(max) = max_msg {
                            parts.push(max);
                        }
                        
                        parts.join(" | ")
                    }}
                </div>
            </div>
        </div>
    }
}

