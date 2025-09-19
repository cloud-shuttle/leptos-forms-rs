//! Array operations module
//!
//! This module provides array field operations for the FormHandle including
//! adding, removing, moving, and manipulating items in field arrays.

use crate::core::traits::{Form, FormState};
use crate::core::types::FieldValue;
use leptos::prelude::{GetUntracked, Set};

use super::core::FormHandle;

impl<T: Form + Send + Sync + PartialEq> FormHandle<T> {
    /// Helper function to update array field and set form state with validation
    fn update_array_field<F>(&self, field_name: &str, mutator: F)
    where
        F: FnOnce(&mut Vec<FieldValue>),
    {
        let current_state = self.state.get_untracked();
        let mut new_values = current_state.values.clone();

        let current_array = new_values.get_field_value(field_name);
        if let FieldValue::Array(mut array) = current_array {
            mutator(&mut array);

            new_values.set_field_value(field_name, FieldValue::Array(array));

            // Validate the array field after updating
            let mut new_errors = current_state.errors.clone();
            if let Err(field_errors) = self.validate_field_internal(&new_values, field_name) {
                // Update errors for this field
                for (error_field, error_messages) in field_errors.field_errors {
                    for error_message in error_messages {
                        new_errors.add_field_error(&error_field, error_message);
                    }
                }
            } else {
                // Clear errors for this field if validation passes
                new_errors.remove_field_error(field_name);
            }

            let new_state = FormState {
                values: new_values,
                is_dirty: true,
                is_submitting: current_state.is_submitting,
                errors: new_errors,
            };

            self.state.set(new_state);
        }
    }

    /// Add an item to a field array
    pub fn add_array_item(&self, field_name: &str, value: FieldValue) {
        self.update_array_field(field_name, |array| {
            array.push(value);
        });
    }

    /// Remove an item from a field array
    pub fn remove_array_item(&self, field_name: &str, index: usize) {
        self.update_array_field(field_name, |array| {
            if index < array.len() {
                array.remove(index);
            }
        });
    }

    /// Move an item in a field array
    pub fn move_array_item(&self, field_name: &str, from_index: usize, to_index: usize) {
        self.update_array_field(field_name, |array| {
            if from_index < array.len() && to_index < array.len() {
                let item = array.remove(from_index);
                array.insert(to_index, item);
            }
        });
    }

    /// Clear all items in a field array
    pub fn clear_array(&self, field_name: &str) {
        self.update_array_field(field_name, |array| {
            array.clear();
        });
    }

    /// Insert an item at a specific index in a field array
    pub fn insert_array_item(&self, field_name: &str, index: usize, value: FieldValue) {
        self.update_array_field(field_name, |array| {
            if index <= array.len() {
                array.insert(index, value);
            }
        });
    }

    /// Duplicate an item at a specific index in a field array
    pub fn duplicate_array_item(&self, field_name: &str, index: usize) {
        self.update_array_field(field_name, |array| {
            if index < array.len() {
                let item_to_duplicate = array[index].clone();
                array.insert(index + 1, item_to_duplicate);
            }
        });
    }

    /// Get the length of a field array
    pub fn get_array_length(&self, field_name: &str) -> Option<usize> {
        let current_state = self.state.get_untracked();
        let field_value = current_state.values.get_field_value(field_name);

        if let FieldValue::Array(array) = field_value {
            Some(array.len())
        } else {
            None
        }
    }

    /// Get an item from a field array by index
    pub fn get_array_item(&self, field_name: &str, index: usize) -> Option<FieldValue> {
        let current_state = self.state.get_untracked();
        let field_value = current_state.values.get_field_value(field_name);

        if let FieldValue::Array(array) = field_value {
            array.get(index).cloned()
        } else {
            None
        }
    }

    /// Set an item in a field array at a specific index
    pub fn set_array_item(&self, field_name: &str, index: usize, value: FieldValue) {
        self.update_array_field(field_name, |array| {
            if index < array.len() {
                array[index] = value;
            }
        });
    }

    /// Swap two items in a field array
    pub fn swap_array_items(&self, field_name: &str, index1: usize, index2: usize) {
        self.update_array_field(field_name, |array| {
            if index1 < array.len() && index2 < array.len() && index1 != index2 {
                array.swap(index1, index2);
            }
        });
    }

    /// Batch add multiple items to a field array
    pub fn batch_add_array_items(&self, field_name: &str, items: Vec<FieldValue>) {
        self.update_array_field(field_name, |array| {
            array.extend(items);
        });
    }

    /// Check if a field is an array field
    pub fn is_array_field(&self, field_name: &str) -> bool {
        self.get_field_type(field_name)
            .map(|field_type| field_type.is_array_type())
            .unwrap_or(false)
    }

    /// Get all items from a field array
    pub fn get_array_items(&self, field_name: &str) -> Option<Vec<FieldValue>> {
        self.get_field_value_as_array(field_name)
    }

    /// Check if a field array is empty
    pub fn is_array_empty(&self, field_name: &str) -> bool {
        self.get_array_length(field_name)
            .map(|len| len == 0)
            .unwrap_or(true)
    }

    /// Check if a field array has items
    pub fn has_array_items(&self, field_name: &str) -> bool {
        !self.is_array_empty(field_name)
    }

    /// Find an item in a field array by value
    pub fn find_array_item(&self, field_name: &str, value: &FieldValue) -> Option<usize> {
        if let Some(array) = self.get_array_items(field_name) {
            array.iter().position(|item| item == value)
        } else {
            None
        }
    }

    /// Check if a field array contains a specific value
    pub fn array_contains(&self, field_name: &str, value: &FieldValue) -> bool {
        self.find_array_item(field_name, value).is_some()
    }

    /// Remove all occurrences of a value from a field array
    pub fn remove_array_items_by_value(&self, field_name: &str, value: &FieldValue) {
        self.update_array_field(field_name, |array| {
            array.retain(|item| item != value);
        });
    }

    /// Get array items as strings
    pub fn get_array_items_as_strings(&self, field_name: &str) -> Option<Vec<String>> {
        self.get_array_items(field_name).map(|items| {
            items
                .into_iter()
                .filter_map(|item| item.as_string().cloned())
                .collect()
        })
    }

    /// Get array items as numbers
    pub fn get_array_items_as_numbers(&self, field_name: &str) -> Option<Vec<f64>> {
        self.get_array_items(field_name).map(|items| {
            items
                .into_iter()
                .filter_map(|item| item.as_number())
                .collect()
        })
    }

    /// Get array items as booleans
    pub fn get_array_items_as_booleans(&self, field_name: &str) -> Option<Vec<bool>> {
        self.get_array_items(field_name).map(|items| {
            items
                .into_iter()
                .filter_map(|item| item.as_boolean())
                .collect()
        })
    }

    /// Sort array items (for comparable types)
    pub fn sort_array_items(&self, field_name: &str) {
        self.update_array_field(field_name, |array| {
            array.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
        });
    }

    /// Reverse array items
    pub fn reverse_array_items(&self, field_name: &str) {
        self.update_array_field(field_name, |array| {
            array.reverse();
        });
    }

    /// Get unique items from a field array (removes duplicates)
    pub fn get_unique_array_items(&self, field_name: &str) -> Option<Vec<FieldValue>> {
        self.get_array_items(field_name).map(|items| {
            let mut unique_items = Vec::new();
            for item in items {
                if !unique_items.contains(&item) {
                    unique_items.push(item);
                }
            }
            unique_items
        })
    }

    /// Remove duplicate items from a field array
    pub fn remove_duplicate_array_items(&self, field_name: &str) {
        if let Some(unique_items) = self.get_unique_array_items(field_name) {
            self.update_array_field(field_name, |array| {
                *array = unique_items;
            });
        }
    }
}
