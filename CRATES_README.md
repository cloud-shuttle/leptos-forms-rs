# Leptos Forms RS

[![Crates.io](https://img.shields.io/crates/v/leptos-forms-rs)](https://crates.io/crates/leptos-forms-rs)
[![Crates.io](https://img.shields.io/crates/d/leptos-forms-rs)](https://crates.io/crates/leptos-forms-rs)
[![License](https://img.shields.io/crates/l/leptos-forms-rs)](https://github.com/cloud-shuttle/leptos-forms-rs/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.89+-blue.svg)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/leptos-0.8-blue.svg)](https://leptos.dev/)

**Type-safe, reactive form handling library for Leptos 0.8 applications**

## 🚀 **Quick Start**

### **Installation**

Add to your `Cargo.toml`:

```toml
[dependencies]
leptos-forms-rs = "0.2.0"
leptos-forms-rs-macro = "0.2.0"  # For enhanced form functionality
leptos = "0.8"
serde = { version = "1.0", features = ["derive"] }
```

### **Basic Usage**

```rust
use leptos::*;
use leptos_forms_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct UserForm {
    name: String,
    email: String,
    age: u32,
}

impl Form for UserForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "name".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "email".to_string(),
                field_type: FieldType::Email,
                validators: vec![ValidatorConfig::Required, ValidatorConfig::Email],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "age".to_string(),
                field_type: FieldType::Number(NumberType {
                    min: Some(18.0),
                    max: Some(120.0),
                    step: Some(1.0),
                }),
                validators: vec![ValidatorConfig::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
        ]
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.name.is_empty() {
            errors.add_field_error("name".to_string(), "Name is required".to_string());
        }

        if self.email.is_empty() {
            errors.add_field_error("email".to_string(), "Email is required".to_string());
        } else if !self.email.contains('@') {
            errors.add_field_error("email".to_string(), "Invalid email format".to_string());
        }

        if self.age < 18 {
            errors.add_field_error("age".to_string(), "Must be at least 18 years old".to_string());
        }

        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn get_field(&self, name: &str) -> Option<FieldValue> {
        match name {
            "name" => Some(FieldValue::String(self.name.clone())),
            "email" => Some(FieldValue::String(self.email.clone())),
            "age" => Some(FieldValue::Integer(self.age as i64)),
            _ => None,
        }
    }

    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), FieldError> {
        match name {
            "name" => {
                if let FieldValue::String(s) = value {
                    self.name = s;
                    Ok(())
                } else {
                    Err(FieldError::new("name".to_string(), "Expected string value".to_string()))
                }
            },
            "email" => {
                if let FieldValue::String(s) = value {
                    self.email = s;
                    Ok(())
                } else {
                    Err(FieldError::new("email".to_string(), "Expected string value".to_string()))
                }
            },
            "age" => {
                if let FieldValue::Integer(i) = value {
                    self.age = i as u32;
                    Ok(())
                } else {
                    Err(FieldError::new("age".to_string(), "Expected integer value".to_string()))
                }
            },
            _ => Err(FieldError::new(name.to_string(), "Unknown field".to_string())),
        }
    }

    fn default_values() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            age: 18,
        }
    }

    fn schema() -> FormSchema {
        let mut schema = FormSchema::new();
        for field in Self::field_metadata() {
            schema.add_field(field);
        }
        schema
    }
}

#[component]
pub fn UserFormComponent() -> impl IntoView {
    let form = use_form::<UserForm>();

    view! {
        <form on:submit=form.handle_submit>
            <div>
                <label for="name">Name:</label>
                <input
                    type="text"
                    id="name"
                    name="name"
                    on:input=form.handle_input
                    required
                />
                {move || form.get_field_error("name").map(|error| view! {
                    <span class="error">{error}</span>
                })}
            </div>

            <div>
                <label for="email">Email:</label>
                <input
                    type="email"
                    id="email"
                    name="email"
                    on:input=form.handle_input
                    required
                />
                {move || form.get_field_error("email").map(|error| view! {
                    <span class="error">{error}</span>
                })}
            </div>

            <div>
                <label for="age">Age:</label>
                <input
                    type="number"
                    id="age"
                    name="age"
                    min="18"
                    max="120"
                    on:input=form.handle_input
                    required
                />
                {move || form.get_field_error("age").map(|error| view! {
                    <span class="error">{error}</span>
                })}
            </div>

            <button type="submit">Submit</button>
        </form>
    }
}
```

## ✨ **Features**

### **Core Capabilities**

- **Type-safe forms** with compile-time validation
- **Reactive state management** using Leptos signals
- **WASM-powered** for high performance
- **Field arrays and dynamic forms** support
- **Conditional field rendering** based on form state
- **Form persistence** with localStorage support
- **Accessibility-first** design with ARIA support

### **Field Types**

- **Text**: Basic text input with validation
- **Email**: Email-specific validation
- **Password**: Secure password handling
- **Number**: Numeric input with min/max/step
- **Boolean**: Checkbox and radio buttons
- **Select**: Dropdown and multi-select
- **File**: File upload handling
- **Custom**: Extensible field types

### **Validation System**

- **Built-in validators**: Required, Email, MinLength, MaxLength, Pattern
- **Custom validators**: Write your own validation logic
- **Real-time validation**: Validate as users type
- **Error handling**: Comprehensive error management
- **Field dependencies**: Conditional validation rules

### **State Management**

- **Reactive updates**: Automatic UI updates on form changes
- **Form persistence**: Save and restore form state
- **Field tracking**: Monitor individual field changes
- **Submit handling**: Process form submissions
- **Reset functionality**: Clear form data

## 🧪 **Testing & Quality**

- **265 tests passing** (100% success rate)
- **245 E2E tests** across all major browsers
- **20 unit tests** for core functionality
- **Cross-browser compatibility** verified
- **Mobile responsiveness** testing included

## 📚 **Documentation & Examples**

- **Complete API documentation**
- **Working examples** for all features
- **Migration guide** from Leptos 0.6
- **Best practices** and patterns
- **Performance optimization** tips

## 🔧 **Advanced Usage**

### **Dynamic Forms**

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
struct DynamicForm {
    fields: Vec<DynamicField>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DynamicField {
    name: String,
    field_type: FieldType,
    value: FieldValue,
}

impl Form for DynamicForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        // Generate metadata dynamically based on form state
        vec![]
    }

    // ... implement other required methods
}
```

### **Custom Validation**

```rust
fn custom_email_validator(value: &str) -> Result<(), String> {
    if value.contains("@") && value.contains(".") {
        Ok(())
    } else {
        Err("Invalid email format".to_string())
    }
}

// Use in field metadata
FieldMetadata {
    name: "email".to_string(),
    field_type: FieldType::Email,
    validators: vec![
        ValidatorConfig::Required,
        ValidatorConfig::Custom(Box::new(custom_email_validator)),
    ],
    // ... other fields
}
```

### **Form Arrays**

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ArrayForm {
    items: Vec<String>,
}

impl Form for ArrayForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "items".to_string(),
                field_type: FieldType::Array(ArrayType {
                    item_type: Box::new(FieldType::Text),
                    min_items: Some(1),
                    max_items: Some(10),
                }),
                // ... other fields
            }
        ]
    }

    // ... implement other required methods
}
```

## 🚀 **Performance Features**

- **WASM compilation** for optimal performance
- **Lazy validation** - only validate when needed
- **Efficient state updates** using Leptos signals
- **Minimal re-renders** with smart change detection
- **Memory efficient** field storage

## 🔒 **Security**

- **Input sanitization** built-in
- **XSS protection** through proper escaping
- **CSRF protection** ready
- **Secure validation** on both client and server
- **Audit trail** for form submissions

## 🌐 **Browser Support**

- **Chrome** 90+
- **Firefox** 88+
- **Safari** 14+
- **Edge** 90+
- **Mobile browsers** supported

## 📦 **Installation Options**

### **Minimal Setup**

```toml
[dependencies]
leptos-forms-rs = "0.2.0"
```

### **Full Features**

```toml
[dependencies]
leptos-forms-rs = { version = "0.2.0", features = ["full"] }
leptos-forms-rs-macro = "0.2.0"
```

### **Development Dependencies**

```toml
[dev-dependencies]
leptos-forms-rs = { version = "0.2.0", features = ["dev"] }
```

## 🤝 **Contributing**

We welcome contributions! Please see our [Contributing Guide](https://github.com/cloud-shuttle/leptos-forms-rs/blob/main/docs/guides/contributing.md) for details.

## 📄 **License**

This project is licensed under the MIT License - see the [LICENSE](https://github.com/cloud-shuttle/leptos-forms-rs/blob/main/LICENSE) file for details.

## 🔗 **Links**

- **Repository**: [https://github.com/cloud-shuttle/leptos-forms-rs](https://github.com/cloud-shuttle/leptos-forms-rs)
- **Documentation**: [https://github.com/cloud-shuttle/leptos-forms-rs/tree/main/docs](https://github.com/cloud-shuttle/leptos-forms-rs/tree/main/docs)
- **Examples**: [https://github.com/cloud-shuttle/leptos-forms-rs/tree/main/examples](https://github.com/cloud-shuttle/leptos-forms-rs/tree/main/examples)
- **Issues**: [https://github.com/cloud-shuttle/leptos-forms-rs/issues](https://github.com/cloud-shuttle/leptos-forms-rs/issues)

---

**Built with ❤️ in Rust for the Leptos ecosystem**
