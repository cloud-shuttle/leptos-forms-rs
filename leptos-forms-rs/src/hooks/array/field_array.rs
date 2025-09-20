//! Field array hook
//!
//! This module provides the `use_field_array` hook for managing field arrays.

use crate::core::traits::Form;
use crate::core::types::{ArrayState, FieldValue};
use crate::core::FormHandle;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Hook for managing field arrays
pub fn use_field_array<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
    field_name: &str,
) -> FieldArrayHandle {
    let form_handle1 = form_handle.clone();
    let form_handle2 = form_handle.clone();
    let form_handle3 = form_handle.clone();
    let form_handle4 = form_handle.clone();
    let field_name1 = field_name.to_string();
    let field_name2 = field_name.to_string();
    let field_name3 = field_name.to_string();
    let field_name4 = field_name.to_string();

    let add_item = Callback::new(move |_| {
        let form_handle = form_handle1.clone();
        let field_name = field_name1.clone();

        // Get current array value
        let current_value = form_handle
            .get_field_value(&field_name)
            .and_then(|v| v.as_array().cloned())
            .unwrap_or_default();

        // Add new empty item
        let mut new_array = current_value;
        new_array.push(FieldValue::String(String::new()));

        // Update form
        form_handle.set_field_value(&field_name, FieldValue::Array(new_array));
    });

    let remove_item = Callback::new(move |index: usize| {
        let form_handle = form_handle2.clone();
        let field_name = field_name2.clone();

        // Get current array value
        let current_value = form_handle
            .get_field_value(&field_name)
            .and_then(|v| v.as_array().cloned())
            .unwrap_or_default();

        // Remove item at index
        if index < current_value.len() {
            let mut new_array = current_value;
            new_array.remove(index);
            form_handle.set_field_value(&field_name, FieldValue::Array(new_array));
        }
    });

    let move_item = Callback::new(move |(from, to): (usize, usize)| {
        let form_handle = form_handle3.clone();
        let field_name = field_name3.clone();

        // Get current array value
        let current_value = form_handle
            .get_field_value(&field_name)
            .and_then(|v| v.as_array().cloned())
            .unwrap_or_default();

        // Move item
        if from < current_value.len() && to < current_value.len() && from != to {
            let mut new_array = current_value;
            let item = new_array.remove(from);
            new_array.insert(to, item);
            form_handle.set_field_value(&field_name, FieldValue::Array(new_array));
        }
    });

    let clear_array = Callback::new(move |_| {
        let form_handle = form_handle4.clone();
        let field_name = field_name4.clone();

        form_handle.set_field_value(&field_name, FieldValue::Array(Vec::new()));
    });

    FieldArrayHandle {
        add_item,
        remove_item,
        move_item,
        clear_array,
    }
}

/// Hook for getting field array values
pub fn use_field_array_values<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
    field_name: &str,
) -> Memo<Vec<FieldValue>> {
    let form_handle = form_handle.clone();
    let field_name = field_name.to_string();

    Memo::new(move |_| {
        form_handle
            .get_field_value(&field_name)
            .and_then(|v| v.as_array().cloned())
            .unwrap_or_default()
    })
}

/// Hook for getting field array state
pub fn use_field_array_state<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
    field_name: &str,
) -> Memo<ArrayState<FieldValue>> {
    let form_handle = form_handle.clone();
    let field_name = field_name.to_string();

    Memo::new(move |_| {
        let items = form_handle
            .get_field_value(&field_name)
            .and_then(|v| v.as_array().cloned())
            .unwrap_or_default();

        ArrayState::new(items)
    })
}

/// Handle for field array operations
#[derive(Clone)]
pub struct FieldArrayHandle {
    pub add_item: Callback<()>,
    pub remove_item: Callback<usize>,
    pub move_item: Callback<(usize, usize)>,
    pub clear_array: Callback<()>,
}

// FieldArrayHandle methods are implemented as direct field access
// The callbacks can be called directly: array_handle.add_item(())
