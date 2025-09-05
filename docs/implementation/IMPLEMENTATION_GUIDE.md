# Leptos Forms - Implementation Guide

## Quick Start

### 1. Project Setup

**Create Cargo workspace:**

```toml
# Cargo.toml
[workspace]
members = [
    "leptos-forms",
    "leptos-forms-macro",
    "examples/basic-form",
    "examples/complex-form"
]

[workspace.dependencies]
leptos = "0.6"
serde = { version = "1.0", features = ["derive"] }
proc-macro2 = "1.0"
quote = "1.0"
syn = { version = "2.0", features = ["full"] }
```

**Core library setup:**

```toml
# leptos-forms/Cargo.toml
[package]
name = "leptos-forms"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = { workspace = true }
serde = { workspace = true }
leptos-forms-macro = { path = "../leptos-forms-macro" }
web-sys = "0.3"
wasm-bindgen = "0.2"
chrono = { version = "0.4", features = ["serde"] }
regex = "1.0"

[features]
default = ["csr"]
csr = ["leptos/csr"]
ssr = ["leptos/ssr"]
hydrate = ["leptos/hydrate"]
```

**Macro crate setup:**

```toml
# leptos-forms-macro/Cargo.toml
[package]
name = "leptos-forms-macro"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
proc-macro2 = { workspace = true }
quote = { workspace = true }
syn = { workspace = true }
```

### 2. Implementation Phases

## Phase 1: Foundation (Week 1-2)

### Step 1.1: Core Traits (`leptos-forms/src/core/traits.rs`)

```rust
use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core trait that all forms must implement
pub trait Form: Clone + Serialize + for<'de> Deserialize<'de> + 'static {
    /// Get field metadata for runtime introspection
    fn field_metadata() -> Vec<FieldMetadata>;

    /// Validate the entire form
    fn validate(&self) -> Result<(), ValidationErrors>;

    /// Get a field value by name (for dynamic access)
    fn get_field(&self, name: &str) -> Option<FieldValue>;

    /// Set a field value by name (for dynamic updates)
    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), FieldError>;

    /// Get default values
    fn default_values() -> Self;
}

/// Metadata about a form field
#[derive(Debug, Clone)]
pub struct FieldMetadata {
    pub name: String,
    pub field_type: FieldType,
    pub validators: Vec<ValidatorConfig>,
    pub is_required: bool,
    pub default_value: Option<FieldValue>,
    pub dependencies: Vec<String>,
    pub attributes: HashMap<String, String>,
}

/// Supported field types
#[derive(Debug, Clone)]
pub enum FieldType {
    Text,
    Email,
    Password,
    Number(NumberType),
    Boolean,
    Select(Vec<SelectOption>),
    MultiSelect(Vec<SelectOption>),
    Date,
    DateTime,
    File(FileConstraints),
    Array(Box<FieldType>),
    Nested(String), // Type name for nested forms
}

#[derive(Debug, Clone)]
pub struct NumberType {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

#[derive(Debug, Clone)]
pub struct FileConstraints {
    pub max_size: Option<usize>,
    pub accept: Vec<String>,
    pub multiple: bool,
}

/// Dynamic field value representation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldValue {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Date(chrono::NaiveDate),
    DateTime(chrono::DateTime<chrono::Utc>),
    Array(Vec<FieldValue>),
    Object(HashMap<String, FieldValue>),
    File(FileData),
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileData {
    pub name: String,
    pub size: usize,
    pub mime_type: String,
    pub data: Vec<u8>, // In practice, this would be a URL or handle
}

/// Field-specific error
#[derive(Debug, Clone)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

impl std::fmt::Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

impl std::error::Error for FieldError {}
```

### Step 1.2: Validation System (`leptos-forms/src/validation/mod.rs`)

```rust
use std::collections::HashMap;
use regex::Regex;

/// Container for all validation errors
#[derive(Debug, Clone, Default)]
pub struct ValidationErrors {
    pub field_errors: HashMap<String, String>,
    pub form_errors: Vec<String>,
}

impl ValidationErrors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.field_errors.is_empty() && self.form_errors.is_empty()
    }

    pub fn add_field_error(&mut self, field: String, message: String) {
        self.field_errors.insert(field, message);
    }

    pub fn add_form_error(&mut self, message: String) {
        self.form_errors.push(message);
    }

    pub fn clear_field(&mut self, field: &str) {
        self.field_errors.remove(field);
    }
}

/// Built-in validator configuration
#[derive(Debug, Clone)]
pub enum ValidatorConfig {
    Required,
    Email,
    Url,
    MinLength(usize),
    MaxLength(usize),
    Min(f64),
    Max(f64),
    Pattern(String),
    Custom(String), // Function name for code generation
}

/// Built-in validators implementation
pub struct Validators;

impl Validators {
    pub fn required(value: &FieldValue) -> Result<(), String> {
        match value {
            FieldValue::String(s) if s.trim().is_empty() => Err("This field is required".to_string()),
            FieldValue::Null => Err("This field is required".to_string()),
            FieldValue::Array(arr) if arr.is_empty() => Err("This field is required".to_string()),
            _ => Ok(()),
        }
    }

    pub fn email(value: &FieldValue) -> Result<(), String> {
        if let FieldValue::String(email) = value {
            let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
            if email_regex.is_match(email) {
                Ok(())
            } else {
                Err("Invalid email format".to_string())
            }
        } else {
            Err("Email must be a string".to_string())
        }
    }

    pub fn min_length(value: &FieldValue, min: usize) -> Result<(), String> {
        if let FieldValue::String(s) = value {
            if s.len() >= min {
                Ok(())
            } else {
                Err(format!("Minimum length is {} characters", min))
            }
        } else {
            Err("Value must be a string".to_string())
        }
    }

    pub fn max_length(value: &FieldValue, max: usize) -> Result<(), String> {
        if let FieldValue::String(s) = value {
            if s.len() <= max {
                Ok(())
            } else {
                Err(format!("Maximum length is {} characters", max))
            }
        } else {
            Err("Value must be a string".to_string())
        }
    }

    pub fn pattern(value: &FieldValue, pattern: &str) -> Result<(), String> {
        if let FieldValue::String(s) = value {
            let regex = Regex::new(pattern).map_err(|_| "Invalid pattern".to_string())?;
            if regex.is_match(s) {
                Ok(())
            } else {
                Err("Value doesn't match required pattern".to_string())
            }
        } else {
            Err("Value must be a string".to_string())
        }
    }
}

/// Validate field value against a specific validator
pub fn validate_field_value(value: &FieldValue, validator: &ValidatorConfig) -> Result<(), String> {
    match validator {
        ValidatorConfig::Required => Validators::required(value),
        ValidatorConfig::Email => Validators::email(value),
        ValidatorConfig::MinLength(min) => Validators::min_length(value, *min),
        ValidatorConfig::MaxLength(max) => Validators::max_length(value, *max),
        ValidatorConfig::Pattern(pattern) => Validators::pattern(value, pattern),
        // Add other validators...
        _ => Ok(()),
    }
}
```

### Step 1.3: Basic Form Hook (`leptos-forms/src/hooks/use_form.rs`)

```rust
use leptos::*;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::core::traits::*;
use crate::validation::*;

/// Main form hook that manages all form state
#[derive(Clone)]
pub struct FormHandle<T: Form> {
    // Core state
    pub values: ReadSignal<T>,
    pub errors: ReadSignal<ValidationErrors>,
    pub touched: ReadSignal<HashSet<String>>,
    pub dirty_fields: ReadSignal<HashSet<String>>,

    // Derived state
    pub is_valid: Signal<bool>,
    pub is_dirty: Signal<bool>,
    pub is_submitting: ReadSignal<bool>,
    pub submit_count: ReadSignal<u32>,

    // Actions
    pub set_values: WriteSignal<T>,
    pub set_field_value: Rc<dyn Fn(&str, FieldValue)>,
    pub set_field_error: Rc<dyn Fn(&str, String)>,
    pub clear_field_error: Rc<dyn Fn(&str)>,
    pub touch_field: Rc<dyn Fn(&str)>,
    pub reset: Rc<dyn Fn()>,

    // Form submission
    pub handle_submit: Rc<dyn Fn(web_sys::Event)>,
    pub submit_form: Rc<dyn Fn()>,

    // Field registration
    pub register: Rc<dyn Fn(&str) -> FieldRegistration>,
}

/// Form configuration options
#[derive(Clone)]
pub struct FormOptions<T: Form> {
    pub validation_mode: ValidationMode,
    pub on_submit: Option<Rc<dyn Fn(T) -> ()>>,
    pub on_submit_async: Option<Rc<dyn Fn(T) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), ValidationErrors>>>>>>,
}

impl<T: Form> Default for FormOptions<T> {
    fn default() -> Self {
        Self {
            validation_mode: ValidationMode::OnSubmit,
            on_submit: None,
            on_submit_async: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationMode {
    OnChange,
    OnBlur,
    OnSubmit,
}

/// Field registration object returned for each field
#[derive(Clone)]
pub struct FieldRegistration {
    pub name: String,
    pub value: Signal<FieldValue>,
    pub error: Signal<Option<String>>,
    pub is_touched: Signal<bool>,
    pub is_dirty: Signal<bool>,
    pub is_focused: ReadSignal<bool>,

    // Event handlers
    pub on_input: Callback<web_sys::Event>,
    pub on_change: Callback<web_sys::Event>,
    pub on_blur: Callback<web_sys::FocusEvent>,
    pub on_focus: Callback<web_sys::FocusEvent>,

    // DOM props
    pub props: FieldProps,
}

/// Properties ready for DOM binding
#[derive(Clone)]
pub struct FieldProps {
    pub id: String,
    pub name: String,
    pub value: Signal<String>,
    pub on_input: Callback<web_sys::Event>,
    pub on_blur: Callback<web_sys::FocusEvent>,
    pub on_focus: Callback<web_sys::FocusEvent>,
    pub aria_invalid: Signal<bool>,
    pub aria_describedby: Signal<Option<String>>,
}

/// Create a form with reactive state management
pub fn use_form<T: Form>(
    initial_values: Option<T>,
    options: FormOptions<T>,
) -> FormHandle<T> {
    let initial = initial_values.unwrap_or_else(T::default_values);

    // Core state signals
    let (values, set_values) = create_signal(initial.clone());
    let (errors, set_errors) = create_signal(ValidationErrors::new());
    let (touched, set_touched) = create_signal(HashSet::new());
    let (dirty_fields, set_dirty_fields) = create_signal(HashSet::new());
    let (is_submitting, set_submitting) = create_signal(false);
    let (submit_count, set_submit_count) = create_signal(0u32);
    let (focused_field, set_focused_field) = create_signal(None::<String>);

    // Derived state
    let is_valid = create_memo(move |_| errors.get().is_empty());
    let is_dirty = create_memo(move |_| !dirty_fields.get().is_empty());

    // Field value setter
    let set_field_value = {
        let set_values = set_values.clone();
        let set_dirty_fields = set_dirty_fields.clone();
        let validation_mode = options.validation_mode.clone();
        let set_errors = set_errors.clone();

        Rc::new(move |field_name: &str, field_value: FieldValue| {
            let field_name = field_name.to_string();

            set_values.update(|form| {
                if let Ok(()) = form.set_field(&field_name, field_value.clone()) {
                    // Mark field as dirty
                    set_dirty_fields.update(|dirty| {
                        dirty.insert(field_name.clone());
                    });

                    // Validate on change if configured
                    if validation_mode == ValidationMode::OnChange {
                        if let Err(validation_errors) = form.validate() {
                            set_errors.set(validation_errors);
                        } else {
                            set_errors.update(|errors| {
                                errors.clear_field(&field_name);
                            });
                        }
                    }
                }
            });
        })
    };

    // Field registration
    let register = {
        let values = values.clone();
        let errors = errors.clone();
        let touched = touched.clone();
        let dirty_fields = dirty_fields.clone();
        let focused_field = focused_field.clone();
        let set_field_value = set_field_value.clone();
        let set_touched = set_touched.clone();
        let set_focused_field = set_focused_field.clone();

        Rc::new(move |field_name: &str| -> FieldRegistration {
            let field_name_owned = field_name.to_string();

            // Create field-specific signals
            let field_value = create_memo({
                let field_name = field_name_owned.clone();
                let values = values.clone();
                move |_| values.get().get_field(&field_name).unwrap_or(FieldValue::Null)
            });

            let field_error = create_memo({
                let field_name = field_name_owned.clone();
                let errors = errors.clone();
                move |_| errors.get().field_errors.get(&field_name).cloned()
            });

            let is_touched = create_memo({
                let field_name = field_name_owned.clone();
                let touched = touched.clone();
                move |_| touched.get().contains(&field_name)
            });

            let is_dirty = create_memo({
                let field_name = field_name_owned.clone();
                let dirty_fields = dirty_fields.clone();
                move |_| dirty_fields.get().contains(&field_name)
            });

            let is_focused = create_memo({
                let field_name = field_name_owned.clone();
                let focused_field = focused_field.clone();
                move |_| focused_field.get().as_ref() == Some(&field_name)
            });

            // Event handlers
            let on_input = {
                let field_name = field_name_owned.clone();
                let set_field_value = set_field_value.clone();

                Callback::new(move |ev: web_sys::Event| {
                    if let Some(target) = ev.target() {
                        if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                            let value = FieldValue::String(input.value());
                            set_field_value(&field_name, value);
                        }
                    }
                })
            };

            let on_blur = {
                let field_name = field_name_owned.clone();
                let set_touched = set_touched.clone();
                let set_focused_field = set_focused_field.clone();

                Callback::new(move |_: web_sys::FocusEvent| {
                    set_touched.update(|touched| {
                        touched.insert(field_name.clone());
                    });
                    set_focused_field.set(None);
                })
            };

            let on_focus = {
                let field_name = field_name_owned.clone();
                let set_focused_field = set_focused_field.clone();

                Callback::new(move |_: web_sys::FocusEvent| {
                    set_focused_field.set(Some(field_name.clone()));
                })
            };

            // Create props for DOM binding
            let props = FieldProps {
                id: format!("field-{}", field_name_owned),
                name: field_name_owned.clone(),
                value: create_memo(move |_| {
                    match field_value.get() {
                        FieldValue::String(s) => s,
                        FieldValue::Number(n) => n.to_string(),
                        FieldValue::Integer(i) => i.to_string(),
                        FieldValue::Boolean(b) => b.to_string(),
                        _ => String::new(),
                    }
                }).into(),
                on_input: on_input.clone(),
                on_blur: on_blur.clone(),
                on_focus: on_focus.clone(),
                aria_invalid: field_error.clone().into(),
                aria_describedby: create_memo(move |_| {
                    field_error.get().is_some().then(|| format!("field-{}-error", field_name_owned))
                }).into(),
            };

            FieldRegistration {
                name: field_name_owned,
                value: field_value.into(),
                error: field_error.into(),
                is_touched: is_touched.into(),
                is_dirty: is_dirty.into(),
                is_focused: is_focused.into(),
                on_input,
                on_change: on_input.clone(),
                on_blur,
                on_focus,
                props,
            }
        })
    };

    // Form submission handler
    let handle_submit = {
        let values = values.clone();
        let set_errors = set_errors.clone();
        let set_submitting = set_submitting.clone();
        let set_submit_count = set_submit_count.clone();
        let on_submit = options.on_submit.clone();
        let on_submit_async = options.on_submit_async.clone();

        Rc::new(move |ev: web_sys::Event| {
            ev.prevent_default();

            set_submit_count.update(|count| *count += 1);

            let current_values = values.get();
            match current_values.validate() {
                Ok(()) => {
                    set_errors.set(ValidationErrors::new());
                    set_submitting.set(true);

                    if let Some(handler) = &on_submit {
                        handler(current_values);
                        set_submitting.set(false);
                    } else if let Some(async_handler) = &on_submit_async {
                        let handler = async_handler.clone();
                        let values = current_values.clone();
                        let set_submitting = set_submitting.clone();
                        let set_errors = set_errors.clone();

                        spawn_local(async move {
                            match handler(values).await {
                                Ok(()) => {
                                    // Success - maybe redirect or show success message
                                }
                                Err(server_errors) => {
                                    set_errors.set(server_errors);
                                }
                            }
                            set_submitting.set(false);
                        });
                    }
                }
                Err(validation_errors) => {
                    set_errors.set(validation_errors);

                    // Focus first error field
                    // Implementation depends on DOM manipulation needs
                }
            }
        })
    };

    FormHandle {
        values: values.into(),
        errors: errors.into(),
        touched: touched.into(),
        dirty_fields: dirty_fields.into(),
        is_valid: is_valid.into(),
        is_dirty: is_dirty.into(),
        is_submitting: is_submitting.into(),
        submit_count: submit_count.into(),
        set_values,
        set_field_value,
        set_field_error: Rc::new(move |field: &str, message: String| {
            set_errors.update(|errors| {
                errors.add_field_error(field.to_string(), message);
            });
        }),
        clear_field_error: Rc::new(move |field: &str| {
            set_errors.update(|errors| {
                errors.clear_field(field);
            });
        }),
        touch_field: Rc::new(move |field: &str| {
            set_touched.update(|touched| {
                touched.insert(field.to_string());
            });
        }),
        reset: Rc::new(move || {
            set_values.set(initial.clone());
            set_errors.set(ValidationErrors::new());
            set_touched.set(HashSet::new());
            set_dirty_fields.set(HashSet::new());
            set_submit_count.set(0);
        }),
        handle_submit,
        submit_form: Rc::new(|| {
            // Programmatic form submission
        }),
        register,
    }
}
```

## Phase 2: Derive Macro (Week 3-4)

### Step 2.1: Macro Implementation (`leptos-forms-macro/src/lib.rs`)

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Field, Attribute, Meta, MetaList};

#[proc_macro_derive(Form, attributes(form))]
pub fn derive_form(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = match input.data {
        Data::Struct(data_struct) => {
            match data_struct.fields {
                Fields::Named(fields) => {
                    let form_impl = generate_form_impl(&input.ident, &fields.named);
                    quote! {
                        #form_impl
                    }
                }
                _ => {
                    return syn::Error::new_spanned(
                        input,
                        "Form can only be derived for structs with named fields"
                    ).to_compile_error().into();
                }
            }
        }
        _ => {
            return syn::Error::new_spanned(
                input,
                "Form can only be derived for structs"
            ).to_compile_error().into();
        }
    };

    TokenStream::from(expanded)
}

fn generate_form_impl(
    struct_name: &syn::Ident,
    fields: &syn::punctuated::Punctuated<Field, syn::Token![,]>
) -> proc_macro2::TokenStream {
    let field_metadata = generate_field_metadata(fields);
    let validate_impl = generate_validate_impl(fields);
    let get_field_impl = generate_get_field_impl(fields);
    let set_field_impl = generate_set_field_impl(fields);
    let default_values_impl = generate_default_values_impl(struct_name, fields);

    quote! {
        impl Form for #struct_name {
            fn field_metadata() -> Vec<FieldMetadata> {
                vec![#(#field_metadata),*]
            }

            fn validate(&self) -> Result<(), ValidationErrors> {
                let mut errors = ValidationErrors::new();
                #validate_impl

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }

            fn get_field(&self, name: &str) -> Option<FieldValue> {
                match name {
                    #get_field_impl
                    _ => None,
                }
            }

            fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), FieldError> {
                match name {
                    #set_field_impl
                    _ => Err(FieldError {
                        field: name.to_string(),
                        message: format!("Unknown field: {}", name),
                    }),
                }
            }

            fn default_values() -> Self {
                #default_values_impl
            }
        }
    }
}

fn generate_field_metadata(fields: &syn::punctuated::Punctuated<Field, syn::Token![,]>) -> Vec<proc_macro2::TokenStream> {
    fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref()?.to_string();
        let field_type = determine_field_type(&field.ty);
        let validators = extract_validators(&field.attrs);
        let is_required = validators.iter().any(|v| matches!(v, ValidatorConfig::Required));
        let dependencies = extract_dependencies(&field.attrs);
        let attributes = extract_attributes(&field.attrs);

        Some(quote! {
            FieldMetadata {
                name: #field_name.to_string(),
                field_type: #field_type,
                validators: vec![#(#validators),*],
                is_required: #is_required,
                default_value: None, // TODO: Extract from attributes
                dependencies: vec![#(#dependencies.to_string()),*],
                attributes: {
                    let mut attrs = std::collections::HashMap::new();
                    #(attrs.insert(#attributes.0.to_string(), #attributes.1.to_string());)*
                    attrs
                },
            }
        })
    }).collect()
}

fn determine_field_type(ty: &syn::Type) -> proc_macro2::TokenStream {
    // This is a simplified implementation
    // In practice, you'd need comprehensive type analysis
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            match segment.ident.to_string().as_str() {
                "String" => quote! { FieldType::Text },
                "bool" => quote! { FieldType::Boolean },
                "i32" | "i64" | "f32" | "f64" => quote! {
                    FieldType::Number(NumberType { min: None, max: None, step: None })
                },
                _ => quote! { FieldType::Text }, // Default fallback
            }
        } else {
            quote! { FieldType::Text }
        }
    } else {
        quote! { FieldType::Text }
    }
}

fn extract_validators(attrs: &[Attribute]) -> Vec<proc_macro2::TokenStream> {
    let mut validators = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("form") {
            if let Ok(Meta::List(meta_list)) = attr.meta {
                // Parse nested attributes like validators(required, email)
                // This is a simplified implementation
                let content = meta_list.tokens.to_string();
                if content.contains("required") {
                    validators.push(quote! { ValidatorConfig::Required });
                }
                if content.contains("email") {
                    validators.push(quote! { ValidatorConfig::Email });
                }
                // Add more validator parsing logic...
            }
        }
    }

    validators
}

fn extract_dependencies(attrs: &[Attribute]) -> Vec<&str> {
    // Extract depends_on attribute values
    // Simplified implementation
    Vec::new()
}

fn extract_attributes(attrs: &[Attribute]) -> Vec<(String, String)> {
    // Extract other form attributes like placeholder, autocomplete, etc.
    // Simplified implementation
    Vec::new()
}

fn generate_validate_impl(fields: &syn::punctuated::Punctuated<Field, syn::Token![,]>) -> proc_macro2::TokenStream {
    let validations: Vec<_> = fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref()?;
        let field_name_str = field_name.to_string();
        let validators = extract_validators(&field.attrs);

        if validators.is_empty() {
            return None;
        }

        Some(quote! {
            {
                let field_value = match self.#field_name {
                    // Convert field value to FieldValue enum
                    // This needs type-specific logic
                    ref val => FieldValue::String(val.to_string()), // Simplified
                };

                #(
                    if let Err(error) = validate_field_value(&field_value, &#validators) {
                        errors.add_field_error(#field_name_str.to_string(), error);
                    }
                )*
            }
        })
    }).collect();

    quote! {
        #(#validations)*
    }
}

fn generate_get_field_impl(fields: &syn::punctuated::Punctuated<Field, syn::Token![,]>) -> proc_macro2::TokenStream {
    let cases: Vec<_> = fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref()?;
        let field_name_str = field_name.to_string();

        Some(quote! {
            #field_name_str => Some({
                // Convert field to FieldValue
                // This needs type-specific conversion logic
                FieldValue::String(self.#field_name.to_string()) // Simplified
            }),
        })
    }).collect();

    quote! {
        #(#cases)*
    }
}

fn generate_set_field_impl(fields: &syn::punctuated::Punctuated<Field, syn::Token![,]>) -> proc_macro2::TokenStream {
    let cases: Vec<_> = fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref()?;
        let field_name_str = field_name.to_string();

        Some(quote! {
            #field_name_str => {
                // Convert FieldValue back to field type
                // This needs type-specific conversion logic
                match value {
                    FieldValue::String(s) => {
                        self.#field_name = s; // Simplified - needs proper conversion
                        Ok(())
                    }
                    _ => Err(FieldError {
                        field: name.to_string(),
                        message: "Invalid value type".to_string(),
                    }),
                }
            },
        })
    }).collect();

    quote! {
        #(#cases)*
    }
}

fn generate_default_values_impl(struct_name: &syn::Ident, fields: &syn::punctuated::Punctuated<Field, syn::Token![,]>) -> proc_macro2::TokenStream {
    // Generate default initialization
    quote! {
        Self::default() // Requires Default trait
    }
}
```

## Phase 3: Basic Components (Week 5-6)

### Step 3.1: Form Provider (`leptos-forms/src/components/provider.rs`)

```rust
use leptos::*;
use crate::hooks::*;
use crate::core::traits::*;

#[derive(Clone)]
pub struct FormContext<T: Form> {
    pub form_handle: FormHandle<T>,
}

/// Form provider component that distributes form context
#[component]
pub fn FormProvider<T: Form>(
    form_handle: FormHandle<T>,
    children: Children,
) -> impl IntoView {
    let context = FormContext { form_handle };

    view! {
        <Provider value={context}>
            {children()}
        </Provider>
    }
}

/// Hook to access form context
pub fn use_form_context<T: Form>() -> FormContext<T> {
    use_context::<FormContext<T>>()
        .expect("use_form_context must be used within FormProvider")
}

/// Hook to register a field within form context
pub fn use_field_registration(name: &str) -> FieldRegistration {
    let context = use_context::<FormContext<impl Form>>()
        .expect("use_field_registration must be used within FormProvider");

    (context.form_handle.register)(name)
}
```

### Step 3.2: Form Root (`leptos-forms/src/components/form.rs`)

```rust
use leptos::*;
use leptos::html::*;
use crate::components::provider::*;
use crate::core::traits::*;

/// Root form element with built-in submission handling
#[component]
pub fn Form<T: Form>(
    form_handle: FormHandle<T>,
    #[prop(optional, into)] class: Option<AttributeValue>,
    #[prop(optional)] prevent_default: bool,
    children: Children,
) -> impl IntoView {
    let prevent_default = prevent_default.unwrap_or(true);
    let handle_submit = form_handle.handle_submit.clone();

    let on_submit = move |ev: web_sys::Event| {
        if prevent_default {
            ev.prevent_default();
        }
        handle_submit(ev);
    };

    view! {
        <FormProvider form_handle={form_handle}>
            <form on:submit=on_submit class=class>
                {children()}
            </form>
        </FormProvider>
    }
}
```

### Step 3.3: Field Components (`leptos-forms/src/components/field.rs`)

```rust
use leptos::*;
use crate::components::provider::*;
use crate::core::traits::*;

/// Wrapper component providing field labeling and error display
#[component]
pub fn FormField(
    #[prop(into)] name: String,
    #[prop(into)] label: String,
    #[prop(optional)] description: Option<String>,
    #[prop(optional)] required: bool,
    #[prop(optional, into)] class: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    let field = use_field_registration(&name);
    let required = required.unwrap_or(false);

    view! {
        <div class=class>
            <label
                for={&field.props.id}
                class="form-label"
            >
                {label}
                {required.then(|| view! {
                    <span class="required-indicator" aria-label="required">"*"</span>
                })}
            </label>

            {description.map(|desc| view! {
                <p class="form-description" id={format!("{}-description", field.props.id)}>
                    {desc}
                </p>
            })}

            <div class="form-field-content">
                {children()}
            </div>

            <Show when=move || field.error.get().is_some()>
                <p
                    class="form-error"
                    id={format!("{}-error", field.props.id)}
                    role="alert"
                    aria-live="polite"
                >
                    {move || field.error.get()}
                </p>
            </Show>
        </div>
    }
}

/// Basic text input component
#[component]
pub fn TextInput(
    #[prop(into)] name: String,
    #[prop(optional)] input_type: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] autocomplete: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView {
    let field = use_field_registration(&name);
    let input_type = input_type.unwrap_or_else(|| "text".to_string());
    let disabled = disabled.unwrap_or(false);

    view! {
        <input
            type={input_type}
            id={&field.props.id}
            name={&field.props.name}
            value={move || field.value.get().to_string()}
            on:input={field.props.on_input}
            on:blur={field.props.on_blur}
            on:focus={field.props.on_focus}
            placeholder={placeholder}
            autocomplete={autocomplete}
            disabled={disabled}
            class={class}
            aria-invalid={move || field.error.get().is_some()}
            aria-describedby={move || field.props.aria_describedby.get()}
        />
    }
}

/// Textarea component
#[component]
pub fn TextArea(
    #[prop(into)] name: String,
    #[prop(optional)] rows: Option<u32>,
    #[prop(optional)] cols: Option<u32>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView {
    let field = use_field_registration(&name);
    let rows = rows.unwrap_or(4);
    let disabled = disabled.unwrap_or(false);

    view! {
        <textarea
            id={&field.props.id}
            name={&field.props.name}
            rows={rows}
            cols={cols}
            placeholder={placeholder}
            disabled={disabled}
            class={class}
            aria-invalid={move || field.error.get().is_some()}
            aria-describedby={move || field.props.aria_describedby.get()}
            on:input={field.props.on_input}
            on:blur={field.props.on_blur}
            on:focus={field.props.on_focus}
        >
            {move || field.value.get().to_string()}
        </textarea>
    }
}

/// Checkbox component
#[component]
pub fn Checkbox(
    #[prop(into)] name: String,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView {
    let field = use_field_registration(&name);
    let disabled = disabled.unwrap_or(false);

    // Custom handler for checkbox
    let on_change = {
        let field_name = name.clone();
        let context = use_form_context::<impl Form>();

        Callback::new(move |ev: web_sys::Event| {
            if let Some(target) = ev.target() {
                if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                    let value = FieldValue::Boolean(input.checked());
                    (context.form_handle.set_field_value)(&field_name, value);
                }
            }
        })
    };

    view! {
        <input
            type="checkbox"
            id={&field.props.id}
            name={&field.props.name}
            checked={move || {
                match field.value.get() {
                    FieldValue::Boolean(b) => b,
                    _ => false,
                }
            }}
            disabled={disabled}
            class={class}
            aria-invalid={move || field.error.get().is_some()}
            aria-describedby={move || field.props.aria_describedby.get()}
            on:change={on_change}
            on:blur={field.props.on_blur}
            on:focus={field.props.on_focus}
        />
    }
}
```

## Testing Implementation

### Step 4.1: Testing Utilities (`leptos-forms/src/testing/mod.rs`)

```rust
use leptos::*;
use leptos_dom::helpers::TestEvent;
use crate::core::traits::*;
use crate::hooks::*;

/// Test utilities for form testing
pub mod test_utils {
    use super::*;

    /// Create a test form handle for testing
    pub fn create_test_form<T: Form>(initial: T) -> FormHandle<T> {
        use_form(Some(initial), FormOptions::default())
    }

    /// Simulate user input on a field
    pub fn simulate_input<T: Form>(
        form: &FormHandle<T>,
        field_name: &str,
        value: impl Into<FieldValue>,
    ) {
        (form.set_field_value)(field_name, value.into());
    }

    /// Simulate form submission
    pub fn simulate_submit<T: Form>(form: &FormHandle<T>) {
        // Create a mock event
        let event = web_sys::Event::new("submit").unwrap();
        (form.handle_submit)(event);
    }

    /// Assert that a field has a specific error
    pub fn assert_field_error<T: Form>(form: &FormHandle<T>, field: &str, expected: &str) {
        let errors = form.errors.get();
        assert!(
            errors.field_errors.get(field).map(|msg| msg.as_str()) == Some(expected),
            "Expected field '{}' to have error '{}', but got: {:?}",
            field,
            expected,
            errors.field_errors.get(field)
        );
    }

    /// Assert that form is valid
    pub fn assert_form_valid<T: Form>(form: &FormHandle<T>) {
        assert!(
            form.is_valid.get(),
            "Expected form to be valid, but found errors: {:?}",
            form.errors.get()
        );
    }

    /// Assert that form is invalid
    pub fn assert_form_invalid<T: Form>(form: &FormHandle<T>) {
        assert!(
            !form.is_valid.get(),
            "Expected form to be invalid"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;
    use serde::{Serialize, Deserialize};

    #[derive(Form, Clone, Debug, Serialize, Deserialize, Default)]
    struct TestForm {
        #[form(validators(required, min_length = 2))]
        name: String,

        #[form(validators(required, email))]
        email: String,

        age: Option<u32>,
    }

    #[test]
    fn test_form_validation() {
        let form = create_test_form(TestForm::default());

        // Test invalid form
        simulate_input(&form, "name", "a");
        simulate_input(&form, "email", "invalid-email");

        assert_form_invalid(&form);
        assert_field_error(&form, "name", "Minimum length is 2 characters");
        assert_field_error(&form, "email", "Invalid email format");

        // Test valid form
        simulate_input(&form, "name", "John Doe");
        simulate_input(&form, "email", "john@example.com");

        assert_form_valid(&form);
    }

    #[test]
    fn test_field_state() {
        let form = create_test_form(TestForm::default());

        // Initially pristine
        assert!(!form.is_dirty.get());

        // Becomes dirty after input
        simulate_input(&form, "name", "test");
        assert!(form.is_dirty.get());

        // Reset makes it pristine again
        (form.reset)();
        assert!(!form.is_dirty.get());
    }
}
```

## Example Usage

### Basic Form Example (`examples/basic-form/src/main.rs`)

```rust
use leptos::*;
use leptos_forms::*;
use serde::{Serialize, Deserialize};

#[derive(Form, Clone, Debug, Serialize, Deserialize, Default)]
struct ContactForm {
    #[form(validators(required, min_length = 2))]
    name: String,

    #[form(validators(required, email))]
    email: String,

    #[form(validators(required, min_length = 10))]
    message: String,
}

#[component]
fn App() -> impl IntoView {
    let form = use_form::<ContactForm>(
        None,
        FormOptions {
            validation_mode: ValidationMode::OnBlur,
            on_submit_async: Some(Rc::new(|data| {
                Box::pin(async move {
                    // Simulate API call
                    logging::log!("Submitting: {:?}", data);

                    // Simulate success/error
                    Ok(())
                })
            })),
            ..Default::default()
        }
    );

    view! {
        <div class="container">
            <h1>"Contact Form"</h1>

            <Form form_handle={form.clone()}>
                <FormField name="name" label="Your Name" required=true>
                    <TextInput name="name" placeholder="John Doe" />
                </FormField>

                <FormField name="email" label="Email Address" required=true>
                    <TextInput
                        name="email"
                        input_type="email".to_string()
                        placeholder="john@example.com"
                    />
                </FormField>

                <FormField name="message" label="Message" required=true>
                    <TextArea name="message" rows=5 placeholder="Your message here..." />
                </FormField>

                <button
                    type="submit"
                    disabled=move || form.is_submitting.get() || !form.is_valid.get()
                    class="submit-button"
                >
                    {move || if form.is_submitting.get() { "Sending..." } else { "Send Message" }}
                </button>
            </Form>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
```

## Next Steps

### Week 7-8: Advanced Features

1. Implement FieldArray component
2. Add ConditionalField component
3. Create FileInput with upload progress
4. Build FormWizard for multi-step forms

### Week 9-10: UI Integrations

1. Create shadcn-ui adapters
2. Build radix-leptos integration
3. Add theme support
4. Implement CSS-in-JS compatibility

### Week 11-12: Production Ready

1. Comprehensive test suite
2. Documentation website
3. Performance optimizations
4. Accessibility audit

This implementation guide provides a solid foundation for building the leptos-forms library incrementally, with clear milestones and testable components at each phase.
