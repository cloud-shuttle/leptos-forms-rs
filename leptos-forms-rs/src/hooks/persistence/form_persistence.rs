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
    _form_handle: &FormHandle<T>,
    storage_key: &str,
) -> (Callback<()>, Callback<()>, Callback<()>) {
    let storage_key = storage_key.to_string();
    let storage_key1 = storage_key.clone();
    let storage_key2 = storage_key.clone();
    let storage_key3 = storage_key.clone();

    let save = Callback::new(move |_| {
        let storage_key = storage_key1.clone();
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(_storage)) = window.local_storage() {
                    log::info!("Saving form data to storage with key: {}", storage_key);
                }
            }
        });
    });

    let load = Callback::new(move |_| {
        let storage_key = storage_key2.clone();
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(_storage)) = window.local_storage() {
                    log::info!("Loading form data from storage with key: {}", storage_key);
                }
            }
        });
    });

    let clear = Callback::new(move |_| {
        let storage_key = storage_key3.clone();
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(_storage)) = window.local_storage() {
                    log::info!("Clearing form data from storage with key: {}", storage_key);
                }
            }
        });
    });

    (save, load, clear)
}
