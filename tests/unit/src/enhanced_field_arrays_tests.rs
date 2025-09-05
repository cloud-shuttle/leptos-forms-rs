use leptos::prelude::*;
use leptos_forms_rs::*;
use leptos_forms_rs::core::{FieldValue, FieldMetadata, FieldType};
use leptos_forms_rs::validation::Validator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Test form with various field array types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnhancedFieldArrayForm {
    pub tags: Vec<String>,
    pub items: Vec<TestItem>,
    pub nested_arrays: Vec<Vec<String>>,
    pub contacts: Vec<Contact>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestItem {
    pub name: String,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub email: String,
    pub phones: Vec<String>, // Nested array within array
}

impl Form for EnhancedFieldArrayForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "tags".to_string(),
                field_type: FieldType::Array(Box::new(FieldType::Text)),
                validators: vec![Validator::MinLength(1), Validator::MaxLength(10)],
                is_required: false,
                default_value: Some(FieldValue::Array(vec![])),
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "items".to_string(),
                field_type: FieldType::Array(Box::new(FieldType::Nested("TestItem".to_string()))),
                validators: vec![Validator::MinLength(1), Validator::MaxLength(5)],
                is_required: true,
                default_value: Some(FieldValue::Array(vec![])),
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "nested_arrays".to_string(),
                field_type: FieldType::Array(Box::new(FieldType::Array(Box::new(FieldType::Text)))),
                validators: vec![],
                is_required: false,
                default_value: Some(FieldValue::Array(vec![])),
                dependencies: vec![],
                attributes: HashMap::new(),
            },
            FieldMetadata {
                name: "contacts".to_string(),
                field_type: FieldType::Array(Box::new(FieldType::Nested("Contact".to_string()))),
                validators: vec![Validator::MinLength(1)],
                is_required: true,
                default_value: Some(FieldValue::Array(vec![])),
                dependencies: vec![],
                attributes: HashMap::new(),
            },
        ]
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        
        // Validate tags array
        if self.tags.len() > 10 {
            errors.add_field_error("tags", "Maximum 10 tags allowed".to_string());
        }
        
        // Validate items array
        if self.items.is_empty() {
            errors.add_field_error("items", "At least one item is required".to_string());
        }
        
        // Validate contacts array
        if self.contacts.is_empty() {
            errors.add_field_error("contacts", "At least one contact is required".to_string());
        }
        
        // Validate individual items
        for (index, item) in self.items.iter().enumerate() {
            if item.name.is_empty() {
                errors.add_field_error(&format!("items[{}].name", index), "Item name is required".to_string());
            }
            if item.quantity <= 0 {
                errors.add_field_error(&format!("items[{}].quantity", index), "Quantity must be positive".to_string());
            }
        }
        
        // Validate contacts
        for (index, contact) in self.contacts.iter().enumerate() {
            if contact.name.is_empty() {
                errors.add_field_error(&format!("contacts[{}].name", index), "Contact name is required".to_string());
            }
            if !contact.email.contains('@') {
                errors.add_field_error(&format!("contacts[{}].email", index), "Valid email is required".to_string());
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
            tags: vec!["default".to_string()],
            items: vec![TestItem {
                name: "Default Item".to_string(),
                quantity: 1,
                price: 10.0,
            }],
            nested_arrays: vec![vec!["nested1".to_string(), "nested2".to_string()]],
            contacts: vec![Contact {
                name: "Default Contact".to_string(),
                email: "default@example.com".to_string(),
                phones: vec!["123-456-7890".to_string()],
            }],
        }
    }

    fn get_field_value(&self, field_name: &str) -> FieldValue {
        match field_name {
            "tags" => FieldValue::Array(self.tags.iter().map(|t| FieldValue::String(t.clone())).collect()),
            "items" => FieldValue::Array(self.items.iter().map(|item| {
                let mut obj = HashMap::new();
                obj.insert("name".to_string(), FieldValue::String(item.name.clone()));
                obj.insert("quantity".to_string(), FieldValue::Integer(item.quantity as i64));
                obj.insert("price".to_string(), FieldValue::Number(item.price));
                FieldValue::Object(obj)
            }).collect()),
            "nested_arrays" => FieldValue::Array(self.nested_arrays.iter().map(|arr| {
                FieldValue::Array(arr.iter().map(|s| FieldValue::String(s.clone())).collect())
            }).collect()),
            "contacts" => FieldValue::Array(self.contacts.iter().map(|contact| {
                let mut obj = HashMap::new();
                obj.insert("name".to_string(), FieldValue::String(contact.name.clone()));
                obj.insert("email".to_string(), FieldValue::String(contact.email.clone()));
                obj.insert("phones".to_string(), FieldValue::Array(contact.phones.iter().map(|p| FieldValue::String(p.clone())).collect()));
                FieldValue::Object(obj)
            }).collect()),
            _ => FieldValue::String(String::new()),
        }
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "tags" => {
                if let FieldValue::Array(arr) = value {
                    self.tags = arr.into_iter()
                        .filter_map(|v| if let FieldValue::String(s) = v { Some(s) } else { None })
                        .collect();
                }
            },
            "items" => {
                if let FieldValue::Array(arr) = value {
                    self.items = arr.into_iter()
                        .filter_map(|v| {
                            if let FieldValue::Object(obj) = v {
                                let name = obj.get("name").and_then(|v| if let FieldValue::String(s) = v { Some(s.clone()) } else { None }).unwrap_or_default();
                                let quantity = obj.get("quantity").and_then(|v| if let FieldValue::Integer(i) = v { Some(*i as i32) } else { None }).unwrap_or(0);
                                let price = obj.get("price").and_then(|v| if let FieldValue::Number(n) = v { Some(*n) } else { None }).unwrap_or(0.0);
                                Some(TestItem { name, quantity, price })
                            } else { None }
                        })
                        .collect();
                }
            },
            "nested_arrays" => {
                if let FieldValue::Array(arr) = value {
                    self.nested_arrays = arr.into_iter()
                        .filter_map(|v| {
                            if let FieldValue::Array(inner_arr) = v {
                                Some(inner_arr.into_iter()
                                    .filter_map(|v| if let FieldValue::String(s) = v { Some(s) } else { None })
                                    .collect())
                            } else { None }
                        })
                        .collect();
                }
            },
            "contacts" => {
                if let FieldValue::Array(arr) = value {
                    self.contacts = arr.into_iter()
                        .filter_map(|v| {
                            if let FieldValue::Object(obj) = v {
                                let name = obj.get("name").and_then(|v| if let FieldValue::String(s) = v { Some(s.clone()) } else { None }).unwrap_or_default();
                                let email = obj.get("email").and_then(|v| if let FieldValue::String(s) = v { Some(s.clone()) } else { None }).unwrap_or_default();
                                let phones = obj.get("phones").and_then(|v| {
                                    if let FieldValue::Array(arr) = v {
                                        Some(arr.iter().filter_map(|v| if let FieldValue::String(s) = v { Some(s.clone()) } else { None }).collect())
                                    } else { None }
                                }).unwrap_or_default();
                                Some(Contact { name, email, phones })
                            } else { None }
                        })
                        .collect();
                }
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_array_initialization() {
        let form = EnhancedFieldArrayForm::default_values();
        
        // Test initial values
        assert_eq!(form.tags.len(), 1);
        assert_eq!(form.tags[0], "default");
        assert_eq!(form.items.len(), 1);
        assert_eq!(form.items[0].name, "Default Item");
        assert_eq!(form.nested_arrays.len(), 1);
        assert_eq!(form.nested_arrays[0].len(), 2);
        assert_eq!(form.contacts.len(), 1);
        assert_eq!(form.contacts[0].name, "Default Contact");
    }

    #[test]
    fn test_field_array_validation() {
        let form = EnhancedFieldArrayForm::default_values();
        
        // Should pass validation with default values
        assert!(form.validate().is_ok());
        
        // Test empty items array
        let mut empty_form = form.clone();
        empty_form.items.clear();
        assert!(empty_form.validate().is_err());
        
        // Test too many tags
        let mut many_tags_form = form.clone();
        many_tags_form.tags = (0..15).map(|i| format!("tag{}", i)).collect();
        assert!(many_tags_form.validate().is_err());
        
        // Test invalid item
        let mut invalid_item_form = form.clone();
        invalid_item_form.items[0].name = String::new();
        assert!(invalid_item_form.validate().is_err());
        
        // Test invalid contact
        let mut invalid_contact_form = form.clone();
        invalid_contact_form.contacts[0].email = "invalid-email".to_string();
        assert!(invalid_contact_form.validate().is_err());
    }

    #[test]
    fn test_field_array_get_set_values() {
        let mut form = EnhancedFieldArrayForm::default_values();
        
        // Test getting field values
        let tags_value = form.get_field_value("tags");
        if let FieldValue::Array(arr) = tags_value {
            assert_eq!(arr.len(), 1);
            if let FieldValue::String(s) = &arr[0] {
                assert_eq!(s, "default");
            }
        }
        
        // Test setting field values
        let new_tags = FieldValue::Array(vec![
            FieldValue::String("tag1".to_string()),
            FieldValue::String("tag2".to_string()),
        ]);
        form.set_field_value("tags", new_tags);
        assert_eq!(form.tags.len(), 2);
        assert_eq!(form.tags[0], "tag1");
        assert_eq!(form.tags[1], "tag2");
    }

    #[test]
    fn test_nested_field_arrays() {
        let mut form = EnhancedFieldArrayForm::default_values();
        
        // Test nested arrays
        let nested_value = form.get_field_value("nested_arrays");
        if let FieldValue::Array(arr) = nested_value {
            assert_eq!(arr.len(), 1);
            if let FieldValue::Array(inner_arr) = &arr[0] {
                assert_eq!(inner_arr.len(), 2);
            }
        }
        
        // Test setting nested arrays
        let new_nested = FieldValue::Array(vec![
            FieldValue::Array(vec![
                FieldValue::String("a".to_string()),
                FieldValue::String("b".to_string()),
            ]),
            FieldValue::Array(vec![
                FieldValue::String("c".to_string()),
            ]),
        ]);
        form.set_field_value("nested_arrays", new_nested);
        assert_eq!(form.nested_arrays.len(), 2);
        assert_eq!(form.nested_arrays[0].len(), 2);
        assert_eq!(form.nested_arrays[1].len(), 1);
    }

    #[test]
    fn test_complex_nested_objects() {
        let mut form = EnhancedFieldArrayForm::default_values();
        
        // Test contacts with nested phones array
        let contact_value = form.get_field_value("contacts");
        if let FieldValue::Array(arr) = contact_value {
            assert_eq!(arr.len(), 1);
            if let FieldValue::Object(obj) = &arr[0] {
                if let Some(FieldValue::Array(phones)) = obj.get("phones") {
                    assert_eq!(phones.len(), 1);
                }
            }
        }
        
        // Test setting complex nested objects
        let new_contact = FieldValue::Object({
            let mut obj = HashMap::new();
            obj.insert("name".to_string(), FieldValue::String("John Doe".to_string()));
            obj.insert("email".to_string(), FieldValue::String("john@example.com".to_string()));
            obj.insert("phones".to_string(), FieldValue::Array(vec![
                FieldValue::String("123-456-7890".to_string()),
                FieldValue::String("098-765-4321".to_string()),
            ]));
            obj
        });
        
        let new_contacts = FieldValue::Array(vec![new_contact]);
        form.set_field_value("contacts", new_contacts);
        
        assert_eq!(form.contacts.len(), 1);
        assert_eq!(form.contacts[0].name, "John Doe");
        assert_eq!(form.contacts[0].email, "john@example.com");
        assert_eq!(form.contacts[0].phones.len(), 2);
        assert_eq!(form.contacts[0].phones[0], "123-456-7890");
        assert_eq!(form.contacts[0].phones[1], "098-765-4321");
    }
}
