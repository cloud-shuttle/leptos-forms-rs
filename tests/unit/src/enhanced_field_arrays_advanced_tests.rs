use leptos::prelude::GetUntracked;
use leptos::prelude::*;
use leptos_forms_rs::core::{FieldMetadata, FieldType, FieldValue, FormHandle};
use leptos_forms_rs::validation::Validator;
use leptos_forms_rs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Test form for advanced field array features
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdvancedFieldArrayForm {
    pub items: Vec<TestItem>,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestItem {
    pub name: String,
    pub quantity: i32,
    pub price: f64,
}

impl Form for AdvancedFieldArrayForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "items".to_string(),
                field_type: FieldType::Array(Box::new(FieldType::Nested("TestItem".to_string()))),
                validators: vec![Validator::MinLength(1), Validator::MaxLength(10)],
                is_required: true,
                default_value: Some(FieldValue::Array(vec![])),
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "tags".to_string(),
                field_type: FieldType::Array(Box::new(FieldType::Text)),
                validators: vec![Validator::MinLength(1), Validator::MaxLength(5)],
                is_required: false,
                default_value: Some(FieldValue::Array(vec![])),
                dependencies: vec![],
                attributes: HashMap::new(),
            },
        ]
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.items.is_empty() {
            errors.add_field_error("items", "At least one item is required".to_string());
        }

        for (index, item) in self.items.iter().enumerate() {
            if item.name.is_empty() {
                errors.add_field_error(
                    &format!("items[{}].name", index),
                    "Item name is required".to_string(),
                );
            }
            if item.quantity <= 0 {
                errors.add_field_error(
                    &format!("items[{}].quantity", index),
                    "Quantity must be positive".to_string(),
                );
            }
        }

        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn default_values() -> Self {
        Self {
            items: vec![],
            tags: vec![],
        }
    }

    fn get_field_value(&self, field_name: &str) -> FieldValue {
        match field_name {
            "items" => FieldValue::Array(
                self.items
                    .iter()
                    .map(|item| {
                        let mut obj = HashMap::new();
                        obj.insert("name".to_string(), FieldValue::String(item.name.clone()));
                        obj.insert(
                            "quantity".to_string(),
                            FieldValue::Integer(item.quantity as i64),
                        );
                        obj.insert("price".to_string(), FieldValue::Number(item.price));
                        FieldValue::Object(obj)
                    })
                    .collect(),
            ),
            "tags" => FieldValue::Array(
                self.tags
                    .iter()
                    .map(|t| FieldValue::String(t.clone()))
                    .collect(),
            ),
            _ => FieldValue::String(String::new()),
        }
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "items" => {
                if let FieldValue::Array(arr) = value {
                    self.items = arr
                        .into_iter()
                        .filter_map(|v| {
                            if let FieldValue::Object(obj) = v {
                                let name = obj
                                    .get("name")
                                    .and_then(|v| {
                                        if let FieldValue::String(s) = v {
                                            Some(s.clone())
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap_or_default();
                                let quantity = obj
                                    .get("quantity")
                                    .and_then(|v| {
                                        if let FieldValue::Integer(i) = v {
                                            Some(*i as i32)
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap_or(0);
                                let price = obj
                                    .get("price")
                                    .and_then(|v| {
                                        if let FieldValue::Number(n) = v {
                                            Some(*n)
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap_or(0.0);
                                Some(TestItem {
                                    name,
                                    quantity,
                                    price,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                }
            }
            "tags" => {
                if let FieldValue::Array(arr) = value {
                    self.tags = arr
                        .into_iter()
                        .filter_map(|v| {
                            if let FieldValue::String(s) = v {
                                Some(s)
                            } else {
                                None
                            }
                        })
                        .collect();
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_field_array_add_item() {
        let form_handle = FormHandle::new(AdvancedFieldArrayForm::default_values());

        // Test adding a simple string to tags array
        let new_tag = FieldValue::String("new-tag".to_string());
        form_handle.add_array_item("tags", new_tag);

        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.tags.len(), 1);
        assert_eq!(form_data.tags[0], "new-tag");
    }

    #[test]
    fn test_enhanced_field_array_remove_item() {
        let mut form = AdvancedFieldArrayForm::default_values();
        form.tags = vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()];
        let form_handle = FormHandle::new(form);

        // Remove middle item
        form_handle.remove_array_item("tags", 1);

        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.tags.len(), 2);
        assert_eq!(form_data.tags[0], "tag1");
        assert_eq!(form_data.tags[1], "tag3");
    }

    #[test]
    fn test_enhanced_field_array_move_item() {
        let mut form = AdvancedFieldArrayForm::default_values();
        form.tags = vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()];
        let form_handle = FormHandle::new(form);

        // Move item from index 0 to index 2
        form_handle.move_array_item("tags", 0, 2);

        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.tags.len(), 3);
        assert_eq!(form_data.tags[0], "tag2");
        assert_eq!(form_data.tags[1], "tag3");
        assert_eq!(form_data.tags[2], "tag1");
    }

    #[test]
    fn test_enhanced_field_array_clear_array() {
        let mut form = AdvancedFieldArrayForm::default_values();
        form.tags = vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()];
        let form_handle = FormHandle::new(form);

        // Clear the array
        form_handle.clear_array("tags");

        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.tags.len(), 0);
    }

    #[test]
    fn test_enhanced_field_array_add_complex_item() {
        let form_handle = FormHandle::new(AdvancedFieldArrayForm::default_values());

        // Test adding a complex object to items array
        let new_item = FieldValue::Object({
            let mut obj = HashMap::new();
            obj.insert(
                "name".to_string(),
                FieldValue::String("Test Item".to_string()),
            );
            obj.insert("quantity".to_string(), FieldValue::Integer(5));
            obj.insert("price".to_string(), FieldValue::Number(19.99));
            obj
        });

        form_handle.add_array_item("items", new_item);

        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.items.len(), 1);
        assert_eq!(form_data.items[0].name, "Test Item");
        assert_eq!(form_data.items[0].quantity, 5);
        assert_eq!(form_data.items[0].price, 19.99);
    }

    #[test]
    fn test_enhanced_field_array_validation_with_arrays() {
        let mut form = AdvancedFieldArrayForm::default_values();

        // Test empty items array validation
        assert!(form.validate().is_err());

        // Add valid item
        form.items.push(TestItem {
            name: "Valid Item".to_string(),
            quantity: 1,
            price: 10.0,
        });

        // Should pass validation
        assert!(form.validate().is_ok());

        // Add invalid item
        form.items.push(TestItem {
            name: "".to_string(), // Invalid: empty name
            quantity: 0,          // Invalid: zero quantity
            price: 5.0,
        });

        // Should fail validation
        let result = form.validate();
        assert!(result.is_err());

        if let Err(errors) = result {
            assert!(errors.field_errors.contains_key("items[1].name"));
            assert!(errors.field_errors.contains_key("items[1].quantity"));
        }
    }

    #[test]
    fn test_enhanced_field_array_insert_at_index() {
        let mut form = AdvancedFieldArrayForm::default_values();
        form.tags = vec!["tag1".to_string(), "tag3".to_string()];
        let form_handle = FormHandle::new(form);

        // Insert "tag2" at index 1
        let new_tag = FieldValue::String("tag2".to_string());
        form_handle.insert_array_item("tags", 1, new_tag);

        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.tags.len(), 3);
        assert_eq!(form_data.tags[0], "tag1");
        assert_eq!(form_data.tags[1], "tag2");
        assert_eq!(form_data.tags[2], "tag3");
    }

    #[test]
    fn test_enhanced_field_array_duplicate_item() {
        let mut form = AdvancedFieldArrayForm::default_values();
        form.tags = vec!["tag1".to_string(), "tag2".to_string()];
        let form_handle = FormHandle::new(form);

        // Duplicate item at index 0
        form_handle.duplicate_array_item("tags", 0);

        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.tags.len(), 3);
        assert_eq!(form_data.tags[0], "tag1");
        assert_eq!(form_data.tags[1], "tag1"); // Duplicated
        assert_eq!(form_data.tags[2], "tag2");
    }

    #[test]
    fn test_enhanced_field_array_get_array_length() {
        let mut form = AdvancedFieldArrayForm::default_values();
        form.tags = vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()];
        let form_handle = FormHandle::new(form);

        let length = form_handle.get_array_length("tags");
        assert_eq!(length, Some(3));

        let length_nonexistent = form_handle.get_array_length("nonexistent");
        assert_eq!(length_nonexistent, None);
    }

    #[test]
    fn test_enhanced_field_array_get_array_item() {
        let mut form = AdvancedFieldArrayForm::default_values();
        form.tags = vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()];
        let form_handle = FormHandle::new(form);

        let item = form_handle.get_array_item("tags", 1);
        assert_eq!(item, Some(FieldValue::String("tag2".to_string())));

        let item_out_of_bounds = form_handle.get_array_item("tags", 10);
        assert_eq!(item_out_of_bounds, None);

        let item_nonexistent_field = form_handle.get_array_item("nonexistent", 0);
        assert_eq!(item_nonexistent_field, None);
    }

    #[test]
    fn test_enhanced_field_array_set_array_item() {
        let mut form = AdvancedFieldArrayForm::default_values();
        form.tags = vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()];
        let form_handle = FormHandle::new(form);

        // Set item at index 1
        let new_value = FieldValue::String("updated-tag2".to_string());
        form_handle.set_array_item("tags", 1, new_value);

        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.tags[1], "updated-tag2");

        // Try to set item out of bounds (should not panic)
        let out_of_bounds_value = FieldValue::String("should-not-work".to_string());
        form_handle.set_array_item("tags", 10, out_of_bounds_value);

        // Array should remain unchanged
        let form_data_after = form_handle.values().get_untracked();
        assert_eq!(form_data_after.tags.len(), 3);
        assert_eq!(form_data_after.tags[1], "updated-tag2");
    }

    #[test]
    fn test_enhanced_field_array_swap_items() {
        let mut form = AdvancedFieldArrayForm::default_values();
        form.tags = vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()];
        let form_handle = FormHandle::new(form);

        // Swap items at indices 0 and 2
        form_handle.swap_array_items("tags", 0, 2);

        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.tags[0], "tag3");
        assert_eq!(form_data.tags[1], "tag2");
        assert_eq!(form_data.tags[2], "tag1");
    }

    #[test]
    fn test_enhanced_field_array_batch_operations() {
        let form_handle = FormHandle::new(AdvancedFieldArrayForm::default_values());

        // Batch add multiple items
        let items_to_add = vec![
            FieldValue::String("tag1".to_string()),
            FieldValue::String("tag2".to_string()),
            FieldValue::String("tag3".to_string()),
        ];

        form_handle.batch_add_array_items("tags", items_to_add);

        let form_data = form_handle.values().get_untracked();
        assert_eq!(form_data.tags.len(), 3);
        assert_eq!(form_data.tags[0], "tag1");
        assert_eq!(form_data.tags[1], "tag2");
        assert_eq!(form_data.tags[2], "tag3");
    }
}
