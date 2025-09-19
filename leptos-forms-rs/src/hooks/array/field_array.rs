//! Field array hook
//!
//! This module provides the `use_field_array` hook for managing field arrays.

use crate::core::traits::Form;
use crate::core::types::FieldValue;
use crate::core::FormHandle;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Hook for managing field arrays
pub fn use_field_array<
    T: Form + PartialEq + Clone + Send + Sync,
    U: std::fmt::Debug + Clone + Send + Sync + 'static,
>(
    form_handle: &FormHandle<T>,
    field_name: &str,
) -> FieldArrayHandle<U> {
    let form_clone = form_handle.clone();
    let field_name = field_name.to_string();
    let field_name_clone1 = field_name.clone();
    let add_item = Callback::new(move |value: U| {
        let form_clone = form_clone.clone();
        let field_name = field_name_clone1.clone();
        spawn_local(async move {
            let field_value = FieldValue::String(format!("{:?}", value));
            form_clone.add_array_item(&field_name, field_value);
        });
    });

    let form_clone2 = form_handle.clone();
    let field_name_clone2 = field_name.clone();
    let remove_item = Callback::new(move |index: usize| {
        let form_clone = form_clone2.clone();
        let field_name = field_name_clone2.clone();
        spawn_local(async move {
            form_clone.remove_array_item(&field_name, index);
        });
    });

    let form_clone3 = form_handle.clone();
    let field_name3 = field_name.clone();
    let move_item = Callback::new(move |(from_index, to_index): (usize, usize)| {
        let form_clone = form_clone3.clone();
        let field_name = field_name3.clone();
        spawn_local(async move {
            form_clone.move_array_item(&field_name, from_index, to_index);
        });
    });

    let form_clone4 = form_handle.clone();
    let field_name4 = field_name.clone();
    let clear_array = Callback::new(move |_| {
        let form_clone = form_clone4.clone();
        let field_name = field_name4.clone();
        spawn_local(async move {
            form_clone.clear_array(&field_name);
        });
    });

    FieldArrayHandle {
        add_item,
        remove_item,
        move_item,
        clear_array,
    }
}

/// Handle for field array operations
#[derive(Clone)]
pub struct FieldArrayHandle<U: 'static> {
    pub add_item: Callback<U>,
    pub remove_item: Callback<usize>,
    pub move_item: Callback<(usize, usize)>,
    pub clear_array: Callback<()>,
}
