//! Array state management module
//!
//! This module provides array state management for the FieldArray component including
//! add, remove, move, and clear operations with proper state tracking.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Array state for managing dynamic lists of fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayState<T> {
    pub items: Vec<T>,
    pub is_dirty: bool,
    pub errors: Vec<String>,
    pub touched: Vec<bool>,
    pub field_errors: HashMap<usize, Vec<String>>,
}

impl<T: Clone + PartialEq> ArrayState<T> {
    /// Create a new array state
    pub fn new(items: Vec<T>) -> Self {
        let len = items.len();
        Self {
            items,
            is_dirty: false,
            errors: Vec::new(),
            touched: vec![false; len],
            field_errors: HashMap::new(),
        }
    }

    /// Add an item to the array
    pub fn add_item(&mut self, item: T) {
        self.items.push(item);
        self.touched.push(false);
        self.is_dirty = true;
    }

    /// Remove an item from the array at the specified index
    pub fn remove_item(&mut self, index: usize) -> Option<T> {
        if index < self.items.len() {
            self.touched.remove(index);
            self.field_errors.remove(&index);
            // Update field error indices for items after the removed item
            let mut new_field_errors = HashMap::new();
            for (old_index, errors) in &self.field_errors {
                if *old_index > index {
                    new_field_errors.insert(old_index - 1, errors.clone());
                } else if *old_index < index {
                    new_field_errors.insert(*old_index, errors.clone());
                }
            }
            self.field_errors = new_field_errors;
            self.is_dirty = true;
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    /// Move an item from one position to another
    pub fn move_item(&mut self, from: usize, to: usize) -> Result<(), String> {
        if from >= self.items.len() || to >= self.items.len() {
            return Err("Index out of bounds".to_string());
        }

        let item = self.items.remove(from);
        self.items.insert(to, item);

        let touched = self.touched.remove(from);
        self.touched.insert(to, touched);

        // Update field error indices
        let mut new_field_errors = HashMap::new();
        for (old_index, errors) in &self.field_errors {
            let new_index = if *old_index == from {
                to
            } else if *old_index > from && *old_index <= to {
                old_index - 1
            } else if *old_index < from && *old_index >= to {
                old_index + 1
            } else {
                *old_index
            };
            new_field_errors.insert(new_index, errors.clone());
        }
        self.field_errors = new_field_errors;

        self.is_dirty = true;
        Ok(())
    }

    /// Clear all items from the array
    pub fn clear(&mut self) {
        self.items.clear();
        self.touched.clear();
        self.field_errors.clear();
        self.is_dirty = true;
    }

    /// Update an item at the specified index
    pub fn update_item(&mut self, index: usize, item: T) -> Result<(), String> {
        if index < self.items.len() {
            self.items[index] = item;
            self.touched[index] = true;
            self.is_dirty = true;
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    }

    /// Get an item at the specified index
    pub fn get_item(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    /// Get a mutable reference to an item at the specified index
    pub fn get_item_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index)
    }

    /// Check if the array is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the length of the array
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Mark an item as touched
    pub fn mark_touched(&mut self, index: usize) {
        if index < self.touched.len() {
            self.touched[index] = true;
        }
    }

    /// Check if an item is touched
    pub fn is_touched(&self, index: usize) -> bool {
        self.touched.get(index).copied().unwrap_or(false)
    }

    /// Add field errors for a specific item
    pub fn add_field_error(&mut self, index: usize, error: String) {
        self.field_errors
            .entry(index)
            .or_insert_with(Vec::new)
            .push(error);
    }

    /// Remove field errors for a specific item
    pub fn remove_field_error(&mut self, index: usize) {
        self.field_errors.remove(&index);
    }

    /// Get field errors for a specific item
    pub fn get_field_errors(&self, index: usize) -> Vec<String> {
        self.field_errors.get(&index).cloned().unwrap_or_default()
    }

    /// Check if an item has errors
    pub fn has_field_errors(&self, index: usize) -> bool {
        self.field_errors.contains_key(&index)
    }

    /// Check if the array has any errors
    pub fn has_errors(&self) -> bool {
        !self.field_errors.is_empty() || !self.errors.is_empty()
    }

    /// Get all field errors
    pub fn get_all_field_errors(&self) -> HashMap<usize, Vec<String>> {
        self.field_errors.clone()
    }

    /// Clear all errors
    pub fn clear_errors(&mut self) {
        self.errors.clear();
        self.field_errors.clear();
    }

    /// Reset the array to its initial state
    pub fn reset(&mut self, initial_items: Vec<T>) {
        self.items = initial_items;
        self.touched = vec![false; self.items.len()];
        self.field_errors.clear();
        self.errors.clear();
        self.is_dirty = false;
    }

    /// Get array statistics
    pub fn get_stats(&self) -> ArrayStats {
        ArrayStats {
            total_items: self.items.len(),
            touched_items: self.touched.iter().filter(|&&t| t).count(),
            items_with_errors: self.field_errors.len(),
            is_dirty: self.is_dirty,
            has_errors: self.has_errors(),
        }
    }
}

/// Statistics about the array state
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayStats {
    pub total_items: usize,
    pub touched_items: usize,
    pub items_with_errors: usize,
    pub is_dirty: bool,
    pub has_errors: bool,
}

impl ArrayStats {
    /// Get the percentage of touched items
    pub fn touched_percentage(&self) -> f64 {
        if self.total_items == 0 {
            0.0
        } else {
            (self.touched_items as f64 / self.total_items as f64) * 100.0
        }
    }

    /// Get the percentage of items with errors
    pub fn error_percentage(&self) -> f64 {
        if self.total_items == 0 {
            0.0
        } else {
            (self.items_with_errors as f64 / self.total_items as f64) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_state_creation() {
        let items = vec!["item1", "item2", "item3"];
        let state = ArrayState::new(items.clone());

        assert_eq!(state.items, items);
        assert_eq!(state.len(), 3);
        assert!(!state.is_dirty);
        assert!(!state.has_errors());
    }

    #[test]
    fn test_add_item() {
        let mut state = ArrayState::new(vec!["item1"]);
        state.add_item("item2");

        assert_eq!(state.len(), 2);
        assert!(state.is_dirty);
        assert_eq!(state.items, vec!["item1", "item2"]);
    }

    #[test]
    fn test_remove_item() {
        let mut state = ArrayState::new(vec!["item1", "item2", "item3"]);
        let removed = state.remove_item(1);

        assert_eq!(removed, Some("item2"));
        assert_eq!(state.len(), 2);
        assert!(state.is_dirty);
        assert_eq!(state.items, vec!["item1", "item3"]);
    }

    #[test]
    fn test_move_item() {
        let mut state = ArrayState::new(vec!["item1", "item2", "item3"]);
        state.move_item(0, 2).unwrap();

        assert_eq!(state.items, vec!["item2", "item3", "item1"]);
        assert!(state.is_dirty);
    }

    #[test]
    fn test_clear() {
        let mut state = ArrayState::new(vec!["item1", "item2", "item3"]);
        state.clear();

        assert_eq!(state.len(), 0);
        assert!(state.is_dirty);
        assert!(state.is_empty());
    }

    #[test]
    fn test_field_errors() {
        let mut state = ArrayState::new(vec!["item1", "item2"]);

        state.add_field_error(0, "Error 1".to_string());
        state.add_field_error(0, "Error 2".to_string());
        state.add_field_error(1, "Error 3".to_string());

        assert!(state.has_field_errors(0));
        assert!(state.has_field_errors(1));
        assert!(state.has_errors());

        let errors_0 = state.get_field_errors(0);
        assert_eq!(errors_0, vec!["Error 1", "Error 2"]);

        state.remove_field_error(0);
        assert!(!state.has_field_errors(0));
        assert!(state.has_field_errors(1));
    }

    #[test]
    fn test_stats() {
        let mut state = ArrayState::new(vec!["item1", "item2", "item3"]);
        state.mark_touched(0);
        state.mark_touched(2);
        state.add_field_error(1, "Error".to_string());

        let stats = state.get_stats();
        assert_eq!(stats.total_items, 3);
        assert_eq!(stats.touched_items, 2);
        assert_eq!(stats.items_with_errors, 1);
        assert!(!stats.is_dirty);
        assert!(stats.has_errors);
        assert!((stats.touched_percentage() - 66.66666666666667).abs() < 0.0001);
        assert!((stats.error_percentage() - 33.333333333333336).abs() < 0.0001);
    }
}
