//! Form persistence hook
//!
//! This module provides the `use_form_persistence` hook for
//! persisting form data to browser storage.

use crate::core::traits::Form;
use crate::core::FormHandle;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Hook for form persistence
pub fn use_form_persistence<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
    storage_key: &str,
) -> (Callback<()>, Callback<()>, Callback<()>) {
    let form_handle1 = form_handle.clone();
    let form_handle2 = form_handle.clone();
    let form_handle3 = form_handle.clone();
    let storage_key = storage_key.to_string();
    let storage_key1 = storage_key.clone();
    let storage_key2 = storage_key.clone();
    let storage_key3 = storage_key.clone();

    let save = Callback::new(move |_| {
        let form_handle = form_handle1.clone();
        let storage_key = storage_key1.clone();
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let form_data = form_handle.values().get();
                    if let Ok(json_data) = serde_json::to_string(&form_data) {
                        if let Err(e) = storage.set_item(&storage_key, &json_data) {
                            log::error!("Failed to save form data: {:?}", e);
                        } else {
                            log::info!("Form data saved to storage with key: {}", storage_key);
                        }
                    }
                }
            }
        });
    });

    let load = Callback::new(move |_| {
        let form_handle = form_handle2.clone();
        let storage_key = storage_key2.clone();
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(json_data)) = storage.get_item(&storage_key) {
                        if let Ok(form_data) = serde_json::from_str::<T>(&json_data) {
                            // Load the form data back into the form by updating the state
                            form_handle.update_state(|mut state| {
                                state.values = form_data;
                                state
                            });
                            log::info!("Form data loaded from storage with key: {}", storage_key);
                        } else {
                            log::error!("Failed to parse form data from storage");
                        }
                    } else {
                        log::info!("No form data found in storage with key: {}", storage_key);
                    }
                }
            }
        });
    });

    let clear = Callback::new(move |_| {
        let storage_key = storage_key3.clone();
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Err(e) = storage.remove_item(&storage_key) {
                        log::error!("Failed to clear form data: {:?}", e);
                    } else {
                        log::info!("Form data cleared from storage with key: {}", storage_key);
                    }
                }
            }
        });
    });

    (save, load, clear)
}
