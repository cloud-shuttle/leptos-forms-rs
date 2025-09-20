# Field Array Component Design

## 🎯 **Overview**

The Field Array component is currently completely broken with all methods being no-ops. This design document outlines how to implement a fully functional field array component for managing dynamic lists of fields.

## 🚨 **Current State**

### **What's Broken**

```rust
// For now, we'll create a dummy array handle
// In a real implementation, this would get the form handle from context
let _array_handle = FieldArrayHandle {
    add_item: Callback::new(|_: U| {}),  // NO-OP!
    remove_item: Callback::new(|_: usize| {}),  // NO-OP!
    move_item: Callback::new(|_: (usize, usize)| {}),  // NO-OP!
    clear_array: Callback::new(|_: ()| {}),  // NO-OP!
};
```

### **Impact**

- Cannot build dynamic forms
- Major feature completely missing
- All array operations just log to console

## 🎯 **Design Goals**

1. **Real Form Integration**: Connect to actual form handle
2. **Dynamic Operations**: Add, remove, move, clear items
3. **State Synchronization**: Keep form state in sync with array operations
4. **Validation Support**: Validate array items and array length
5. **Performance**: Efficient updates for large arrays
6. **Type Safety**: Maintain Rust's type safety guarantees

## 🏗️ **Architecture**

### **Core Components**

1. **FieldArray Component**: Main UI component
2. **FieldArrayHandle**: Hook for array operations
3. **ArrayState**: Internal state management
4. **ArrayValidation**: Validation for array items

### **Data Flow**

```
FormHandle -> FieldArrayHandle -> ArrayState -> FieldArray Component
     ^                                                      |
     |                                                      v
FormState <-- ArrayOperations <-- User Interactions <-- UI Events
```

## 📋 **Implementation Plan**

### **Phase 1: Core Infrastructure (Week 1)**

#### **1.1 ArrayState Management**

```rust
// leptos-forms-rs/src/core/types/array_state.rs
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayState<T> {
    pub items: Vec<T>,
    pub is_dirty: bool,
    pub errors: Vec<String>,
    pub touched: Vec<bool>,
}

impl<T: Clone + PartialEq> ArrayState<T> {
    pub fn new(items: Vec<T>) -> Self {
        let len = items.len();
        Self {
            items,
            is_dirty: false,
            errors: Vec::new(),
            touched: vec![false; len],
        }
    }

    pub fn add_item(&mut self, item: T) {
        self.items.push(item);
        self.touched.push(false);
        self.is_dirty = true;
    }

    pub fn remove_item(&mut self, index: usize) -> Option<T> {
        if index < self.items.len() {
            self.touched.remove(index);
            self.is_dirty = true;
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn move_item(&mut self, from: usize, to: usize) -> Result<(), String> {
        if from >= self.items.len() || to >= self.items.len() {
            return Err("Index out of bounds".to_string());
        }

        let item = self.items.remove(from);
        self.items.insert(to, item);

        let touched = self.touched.remove(from);
        self.touched.insert(to, touched);

        self.is_dirty = true;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.touched.clear();
        self.is_dirty = true;
    }
}
```

#### **1.2 FieldArrayHandle Hook**

```rust
// leptos-forms-rs/src/hooks/array/field_array.rs
pub fn use_field_array<T: Form + Clone + PartialEq + Send + Sync>(
    form_handle: &FormHandle<T>,
    field_name: &str,
) -> FieldArrayHandle<T> {
    let form_handle = form_handle.clone();
    let field_name = field_name.to_string();

    let add_item = Callback::new(move |item: T| {
        let form_handle = form_handle.clone();
        let field_name = field_name.clone();

        // Get current array value
        let current_value = form_handle.get_field_value(&field_name)
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        // Add new item
        let mut new_array = current_value;
        new_array.push(FieldValue::from(item));

        // Update form
        form_handle.set_field_value(&field_name, FieldValue::Array(new_array));
    });

    let remove_item = Callback::new(move |index: usize| {
        let form_handle = form_handle.clone();
        let field_name = field_name.clone();

        // Get current array value
        let current_value = form_handle.get_field_value(&field_name)
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        // Remove item at index
        if index < current_value.len() {
            let mut new_array = current_value;
            new_array.remove(index);
            form_handle.set_field_value(&field_name, FieldValue::Array(new_array));
        }
    });

    let move_item = Callback::new(move |(from, to): (usize, usize)| {
        let form_handle = form_handle.clone();
        let field_name = field_name.clone();

        // Get current array value
        let current_value = form_handle.get_field_value(&field_name)
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        // Move item
        if from < current_value.len() && to < current_value.len() {
            let mut new_array = current_value;
            let item = new_array.remove(from);
            new_array.insert(to, item);
            form_handle.set_field_value(&field_name, FieldValue::Array(new_array));
        }
    });

    let clear_array = Callback::new(move |_| {
        let form_handle = form_handle.clone();
        let field_name = field_name.clone();

        form_handle.set_field_value(&field_name, FieldValue::Array(Vec::new()));
    });

    FieldArrayHandle {
        add_item,
        remove_item,
        move_item,
        clear_array,
    }
}
```

### **Phase 2: Component Implementation (Week 2)**

#### **2.1 FieldArray Component**

```rust
// leptos-forms-rs/src/components/field_array.rs
#[component]
pub fn FieldArray<T: Form + Clone + PartialEq + Send + Sync>(
    form: FormHandle<T>,
    name: String,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let array_handle = use_field_array(&form, &name);
    let array_value = use_field_value(&form, &name);

    let array_class = class.unwrap_or_else(|| "field-array".to_string());

    view! {
        <div class=array_class>
            <div class="field-array-header">
                <h4>{name.clone()}</h4>
                <div class="field-array-controls">
                    <button
                        type="button"
                        class="add-item-btn"
                        on:click=move |_| {
                            // Add default item based on field type
                            let default_item = get_default_item_for_field(&form, &name);
                            array_handle.add_item.call(default_item);
                        }
                    >
                        "Add Item"
                    </button>
                    <button
                        type="button"
                        class="clear-all-btn"
                        on:click=move |_| {
                            array_handle.clear_array.call(());
                        }
                    >
                        "Clear All"
                    </button>
                </div>
            </div>

            <div class="field-array-items">
                {move || {
                    let array = array_value.get();
                    if let Some(FieldValue::Array(items)) = array.as_array() {
                        items.iter().enumerate().map(|(index, item)| {
                            let item_index = index;
                            let array_handle = array_handle.clone();

                            view! {
                                <div class="field-array-item">
                                    <div class="item-content">
                                        {if let Some(children) = children {
                                            children.call((item_index, item.clone()))
                                        } else {
                                            view! {
                                                <div class="item-placeholder">
                                                    {format!("Item {}", item_index + 1)}
                                                </div>
                                            }
                                        }}
                                    </div>

                                    <div class="item-controls">
                                        <button
                                            type="button"
                                            class="remove-item-btn"
                                            on:click=move |_| {
                                                array_handle.remove_item.call(item_index);
                                            }
                                        >
                                            "Remove"
                                        </button>

                                        {if item_index > 0 {
                                            view! {
                                                <button
                                                    type="button"
                                                    class="move-up-btn"
                                                    on:click=move |_| {
                                                        array_handle.move_item.call((item_index, item_index - 1));
                                                    }
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
                                                    on:click=move |_| {
                                                        array_handle.move_item.call((item_index, item_index + 1));
                                                    }
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
                    } else {
                        vec![]
                    }
                }}
            </div>
        </div>
    }
}
```

### **Phase 3: Validation and Testing (Week 3)**

#### **3.1 Array Validation**

```rust
// leptos-forms-rs/src/validation/array_validators.rs
pub struct ArrayValidators;

impl ArrayValidators {
    pub fn min_length(value: &FieldValue, min: usize) -> Result<(), String> {
        if let FieldValue::Array(arr) = value {
            if arr.len() >= min {
                Ok(())
            } else {
                Err(format!("Array must have at least {} items", min))
            }
        } else {
            Err("Value must be an array".to_string())
        }
    }

    pub fn max_length(value: &FieldValue, max: usize) -> Result<(), String> {
        if let FieldValue::Array(arr) = value {
            if arr.len() <= max {
                Ok(())
            } else {
                Err(format!("Array must have at most {} items", max))
            }
        } else {
            Err("Value must be an array".to_string())
        }
    }

    pub fn exact_length(value: &FieldValue, exact: usize) -> Result<(), String> {
        if let FieldValue::Array(arr) = value {
            if arr.len() == exact {
                Ok(())
            } else {
                Err(format!("Array must have exactly {} items", exact))
            }
        } else {
            Err("Value must be an array".to_string())
        }
    }
}
```

#### **3.2 Comprehensive Testing**

```rust
// tests/unit/src/field_array_tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_item() {
        // Test adding items to array
    }

    #[test]
    fn test_remove_item() {
        // Test removing items from array
    }

    #[test]
    fn test_move_item() {
        // Test moving items within array
    }

    #[test]
    fn test_clear_array() {
        // Test clearing entire array
    }

    #[test]
    fn test_array_validation() {
        // Test array validation rules
    }

    #[test]
    fn test_form_integration() {
        // Test integration with form state
    }
}
```

## 🎯 **Success Criteria**

### **Functional Requirements**

- ✅ Add items to array
- ✅ Remove items from array
- ✅ Move items within array
- ✅ Clear entire array
- ✅ Validate array length
- ✅ Integrate with form state
- ✅ Support custom item rendering

### **Performance Requirements**

- ✅ Handle arrays up to 1000 items
- ✅ Updates in < 100ms
- ✅ Memory efficient
- ✅ No memory leaks

### **Quality Requirements**

- ✅ 100% test coverage
- ✅ No panics or crashes
- ✅ Proper error handling
- ✅ Type safety maintained

## 📊 **Implementation Timeline**

- **Week 1**: Core infrastructure and state management
- **Week 2**: Component implementation and UI
- **Week 3**: Validation, testing, and documentation

## 🚀 **Next Steps**

1. **Approve Design**: Review and approve this design
2. **Set Up Development**: Create feature branch
3. **Implement Phase 1**: Core infrastructure
4. **Implement Phase 2**: Component implementation
5. **Implement Phase 3**: Validation and testing
6. **Integration Testing**: Test with real forms
7. **Documentation**: Update docs and examples

This design will transform the field array component from a broken stub into a fully functional, production-ready feature.
