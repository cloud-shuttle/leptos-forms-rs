use crate::core::traits::Form;
use crate::core::types::FieldValue;
use crate::core::FormHandle;
use crate::hooks::{use_field_array, use_field_array_values, FieldArrayHandle};
use leptos::prelude::*;

/// Field array component for managing dynamic lists of fields
#[component]
pub fn FieldArray<T: Form + PartialEq + Clone + Send + Sync>(
    form: FormHandle<T>,
    #[prop(into)] field_name: String,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let array_handle = use_field_array(&form, &field_name);
    let array_values = use_field_array_values(&form, &field_name);

    let array_class = class.unwrap_or_else(|| "field-array".to_string());

    let add_item = move |_| {
        array_handle.add_item(());
    };

    let remove_item = move |index: usize| {
        array_handle.remove_item(index);
    };

    let move_item = move |(from, to): (usize, usize)| {
        array_handle.move_item((from, to));
    };

    let clear_all = move |_| {
        array_handle.clear_array(());
    };
    view! {
        <div class=array_class>
            <div class="field-array-header">
                <h4>{field_name.clone()}</h4>
                <div class="field-array-controls">
                    <button
                        type="button"
                        class="add-item-btn"
                        on:click=add_item
                    >
                        "Add Item"
                    </button>
                    <button
                        type="button"
                        class="clear-all-btn"
                        on:click=clear_all
                    >
                        "Clear All"
                    </button>
                </div>
            </div>

            <div class="field-array-items">
                {move || {
                    let items = array_values.get();
                    if items.is_empty() {
                        view! {
                            <div class="empty-array">
                                <p>"No items in array. Click 'Add Item' to get started."</p>
                            </div>
                        }
                    } else {
                        items.iter().enumerate().map(|(index, item)| {
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
                        }).collect::<Vec<_>>()
                    }
                }}
            </div>
        </div>
    }
}
