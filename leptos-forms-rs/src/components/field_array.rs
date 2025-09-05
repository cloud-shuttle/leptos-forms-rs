use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::hooks::FieldArrayHandle;

/// Field array component for managing dynamic lists of fields
#[component]
pub fn FieldArray<U: Clone + Send + Sync + Default + 'static>(
    #[prop(into)] field_name: String,
    #[prop(into)] items: Vec<U>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // For now, we'll create a dummy array handle
    // In a real implementation, this would get the form handle from context
    let array_handle = FieldArrayHandle {
        add_item: Callback::new(|_: U| {}),
        remove_item: Callback::new(|_: usize| {}),
        move_item: Callback::new(|_: (usize, usize)| {}),
        clear_array: Callback::new(|_: ()| {}),
    };
    
    let add_item = move |_| {
        // For now, just log - this will be implemented when we have a real form handle
        log::info!("Add item clicked");
    };
    
    let remove_item = move |index: usize| {
        // For now, just log - this will be implemented when we have a real form handle
        log::info!("Remove item clicked at index: {}", index);
    };
    
    let move_item = move |(from, to): (usize, usize)| {
        // For now, just log - this will be implemented when we have a real form handle
        log::info!("Move item from {} to {}", from, to);
    };
    
    let clear_all = move |_| {
        // For now, just log - this will be implemented when we have a real form handle
        log::info!("Clear all clicked");
    };
    
    let array_class = class.unwrap_or_else(|| "field-array".to_string());
    
    view! {
        <div class={array_class}>
            <div class="field-array-header">
                <h4>{field_name.clone()}</h4>
                <div class="field-array-controls">
                    <button type="button" class="add-item-btn" on:click=add_item>
                        "Add Item"
                    </button>
                    <button type="button" class="clear-all-btn" on:click=clear_all>
                        "Clear All"
                    </button>
                </div>
            </div>
            
            <div class="field-array-items">
                {items.iter().enumerate().map(|(index, item)| {
                    let item_index = index;
                    let item_value = item.clone();
                    
                    view! {
                        <div class="field-array-item">
                            <div class="item-content">
                                <div class="item-placeholder">
                                    {format!("Item {}", item_index + 1)}
                                </div>
                            </div>
                            
                            <div class="item-controls">
                                <button 
                                    type="button" 
                                    class="remove-item-btn"
                                    on:click=move |_| remove_item(item_index)
                                >
                                    "Remove"
                                </button>
                                
                                {if item_index > 0 {
                                    view! {
                                        <button 
                                            type="button" 
                                            class="move-up-btn"
                                            on:click=move |_| move_item((item_index, item_index - 1))
                                        >
                                            "↑"
                                        </button>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }}
                                
                                {if item_index < items.len() - 1 {
                                    view! {
                                        <button 
                                            type="button" 
                                            class="move-down-btn"
                                            on:click=move |_| move_item((item_index, item_index + 1))
                                        >
                                            "↓"
                                        </button>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }}
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

