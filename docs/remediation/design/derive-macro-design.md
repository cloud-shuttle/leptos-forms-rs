# Derive Macro Design

## 🎯 **Overview**

The derive macro is currently missing entirely, forcing developers to manually implement all form functionality. This design document outlines how to implement a comprehensive derive macro that automatically generates form implementations.

## 🚨 **Current State**

### **What's Missing**

- No derive macro implementation
- Must manually implement `Form` trait for every form
- Must manually implement `get_field_value` and `set_field_value`
- Must manually define field metadata
- Terrible developer experience

### **Impact**

- Developers must write 50+ lines of boilerplate for each form
- Error-prone manual implementation
- No compile-time validation
- Poor developer experience

## 🎯 **Design Goals**

1. **Zero Boilerplate**: Generate all form code automatically
2. **Type Safety**: Maintain Rust's type safety guarantees
3. **Compile-time Validation**: Catch errors at compile time
4. **Flexible Configuration**: Support various field types and validators
5. **Performance**: Zero runtime overhead
6. **Developer Experience**: Intuitive and easy to use

## 🏗️ **Architecture**

### **Core Components**

1. **Derive Macro**: Main macro implementation
2. **Field Attributes**: Configuration attributes for fields
3. **Form Attributes**: Configuration attributes for forms
4. **Code Generation**: Automatic code generation
5. **Validation Generation**: Automatic validation rules

### **Data Flow**

```
Form Struct -> Derive Macro -> Generated Code -> Form Implementation
     ^                                                      |
     |                                                      v
Field Attributes -> Validation Rules -> Field Metadata -> Form Trait
```

## 📋 **Implementation Plan**

### **Phase 1: Basic Derive Macro (Week 1)**

#### **1.1 Macro Structure**

```rust
// leptos-forms-rs-macro/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, DataStruct, Fields};

#[proc_macro_derive(Form, attributes(field, form))]
pub fn derive_form(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;

    // Generate form implementation
    let expanded = quote! {
        impl #generics Form for #name {
            fn field_metadata() -> Vec<FieldMetadata> {
                vec![
                    // Generated field metadata
                ]
            }

            fn validate(&self) -> Result<(), ValidationErrors> {
                // Generated validation logic
            }

            fn default_values() -> Self {
                // Generated default values
            }

            fn get_field_value(&self, field_name: &str) -> FieldValue {
                // Generated field access
            }

            fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
                // Generated field updates
            }
        }
    };

    TokenStream::from(expanded)
}
```

#### **1.2 Field Attribute Parsing**

```rust
// leptos-forms-rs-macro/src/field_attributes.rs
use syn::{Attribute, Meta, NestedMeta, Lit, LitStr, LitInt};

#[derive(Debug, Clone)]
pub struct FieldAttributes {
    pub field_type: Option<FieldType>,
    pub required: bool,
    pub validators: Vec<Validator>,
    pub default_value: Option<FieldValue>,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
}

impl FieldAttributes {
    pub fn from_attributes(attrs: &[Attribute]) -> Self {
        let mut field_attrs = FieldAttributes {
            field_type: None,
            required: false,
            validators: Vec::new(),
            default_value: None,
            label: None,
            placeholder: None,
            help_text: None,
        };

        for attr in attrs {
            if attr.path.is_ident("field") {
                if let Ok(Meta::List(meta)) = attr.parse_args() {
                    for nested in meta.nested {
                        if let NestedMeta::Meta(Meta::NameValue(nv)) = nested {
                            match nv.path.get_ident().unwrap().to_string().as_str() {
                                "required" => {
                                    if let Lit::Bool(lit_bool) = &nv.lit {
                                        field_attrs.required = lit_bool.value;
                                    }
                                }
                                "email" => {
                                    if let Lit::Bool(lit_bool) = &nv.lit {
                                        if lit_bool.value {
                                            field_attrs.validators.push(Validator::Email);
                                        }
                                    }
                                }
                                "min_length" => {
                                    if let Lit::Int(lit_int) = &nv.lit {
                                        field_attrs.validators.push(Validator::MinLength(lit_int.base10_parse().unwrap()));
                                    }
                                }
                                "max_length" => {
                                    if let Lit::Int(lit_int) = &nv.lit {
                                        field_attrs.validators.push(Validator::MaxLength(lit_int.base10_parse().unwrap()));
                                    }
                                }
                                "pattern" => {
                                    if let Lit::Str(lit_str) = &nv.lit {
                                        field_attrs.validators.push(Validator::Pattern(lit_str.value()));
                                    }
                                }
                                "default" => {
                                    if let Lit::Str(lit_str) = &nv.lit {
                                        field_attrs.default_value = Some(FieldValue::String(lit_str.value()));
                                    }
                                }
                                "label" => {
                                    if let Lit::Str(lit_str) = &nv.lit {
                                        field_attrs.label = Some(lit_str.value());
                                    }
                                }
                                "placeholder" => {
                                    if let Lit::Str(lit_str) = &nv.lit {
                                        field_attrs.placeholder = Some(lit_str.value());
                                    }
                                }
                                "help_text" => {
                                    if let Lit::Str(lit_str) = &nv.lit {
                                        field_attrs.help_text = Some(lit_str.value());
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        field_attrs
    }
}
```

### **Phase 2: Code Generation (Week 2)**

#### **2.1 Field Metadata Generation**

```rust
// leptos-forms-rs-macro/src/field_metadata.rs
use quote::quote;
use syn::{Fields, Field, Type};

pub fn generate_field_metadata(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields_named) => {
            let field_metadata = fields_named.named.iter().map(|field| {
                let field_name = &field.ident;
                let field_type = &field.ty;
                let field_attrs = FieldAttributes::from_attributes(&field.attrs);

                // Determine field type from Rust type
                let leptos_field_type = determine_field_type(field_type);

                // Generate field metadata
                quote! {
                    FieldMetadata {
                        name: stringify!(#field_name).to_string(),
                        field_type: #leptos_field_type,
                        validators: vec![#(#field_attrs.validators),*],
                        is_required: #field_attrs.required,
                        default_value: #field_attrs.default_value,
                        dependencies: vec![],
                        attributes: std::collections::HashMap::new(),
                    }
                }
            });

            quote! {
                vec![
                    #(#field_metadata),*
                ]
            }
        }
        _ => quote! { vec![] }
    }
}

fn determine_field_type(ty: &Type) -> proc_macro2::TokenStream {
    // Match Rust types to Leptos field types
    match ty {
        Type::Path(type_path) => {
            if let Some(ident) = type_path.path.get_ident() {
                match ident.to_string().as_str() {
                    "String" => quote! { FieldType::Text },
                    "i32" | "i64" | "f32" | "f64" => quote! { FieldType::Number(NumberType::Integer) },
                    "bool" => quote! { FieldType::Boolean },
                    "Option" => {
                        // Handle Option<T> types
                        quote! { FieldType::Text }
                    }
                    _ => quote! { FieldType::Text }
                }
            } else {
                quote! { FieldType::Text }
            }
        }
        _ => quote! { FieldType::Text }
    }
}
```

#### **2.2 Field Access Generation**

```rust
// leptos-forms-rs-macro/src/field_access.rs
use quote::quote;
use syn::{Fields, Field};

pub fn generate_get_field_value(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields_named) => {
            let field_cases = fields_named.named.iter().map(|field| {
                let field_name = &field.ident;
                let field_type = &field.ty;

                quote! {
                    stringify!(#field_name) => {
                        let value = &self.#field_name;
                        #field_type_to_field_value(field_type, value)
                    }
                }
            });

            quote! {
                match field_name {
                    #(#field_cases),*
                    _ => FieldValue::String(String::new())
                }
            }
        }
        _ => quote! { FieldValue::String(String::new()) }
    }
}

pub fn generate_set_field_value(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields_named) => {
            let field_cases = fields_named.named.iter().map(|field| {
                let field_name = &field.ident;
                let field_type = &field.ty;

                quote! {
                    stringify!(#field_name) => {
                        if let Ok(converted_value) = #field_value_to_field_type(field_type, &value) {
                            self.#field_name = converted_value;
                        }
                    }
                }
            });

            quote! {
                match field_name {
                    #(#field_cases),*
                    _ => {}
                }
            }
        }
        _ => quote! {}
    }
}
```

### **Phase 3: Validation Generation (Week 3)**

#### **3.1 Validation Logic Generation**

```rust
// leptos-forms-rs-macro/src/validation.rs
use quote::quote;
use syn::{Fields, Field};

pub fn generate_validation(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields_named) => {
            let field_validations = fields_named.named.iter().map(|field| {
                let field_name = &field.ident;
                let field_attrs = FieldAttributes::from_attributes(&field.attrs);

                let mut validations = Vec::new();

                // Required validation
                if field_attrs.required {
                    validations.push(quote! {
                        if self.#field_name.is_empty() {
                            errors.add_field_error(stringify!(#field_name), "This field is required".to_string());
                        }
                    });
                }

                // Email validation
                if field_attrs.validators.contains(&Validator::Email) {
                    validations.push(quote! {
                        if !self.#field_name.is_empty() && !self.#field_name.contains('@') {
                            errors.add_field_error(stringify!(#field_name), "Invalid email format".to_string());
                        }
                    });
                }

                // Min length validation
                for validator in &field_attrs.validators {
                    if let Validator::MinLength(min_len) = validator {
                        validations.push(quote! {
                            if self.#field_name.len() < #min_len {
                                errors.add_field_error(stringify!(#field_name), format!("Minimum length is {} characters", #min_len));
                            }
                        });
                    }
                }

                // Max length validation
                for validator in &field_attrs.validators {
                    if let Validator::MaxLength(max_len) = validator {
                        validations.push(quote! {
                            if self.#field_name.len() > #max_len {
                                errors.add_field_error(stringify!(#field_name), format!("Maximum length is {} characters", #max_len));
                            }
                        });
                    }
                }

                quote! {
                    #(#validations)*
                }
            });

            quote! {
                let mut errors = ValidationErrors::new();

                #(#field_validations)*

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }
        }
        _ => quote! { Ok(()) }
    }
}
```

### **Phase 4: Default Values Generation (Week 4)**

#### **4.1 Default Values Implementation**

```rust
// leptos-forms-rs-macro/src/default_values.rs
use quote::quote;
use syn::{Fields, Field};

pub fn generate_default_values(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields_named) => {
            let field_defaults = fields_named.named.iter().map(|field| {
                let field_name = &field.ident;
                let field_type = &field.ty;
                let field_attrs = FieldAttributes::from_attributes(&field.attrs);

                // Use custom default if provided, otherwise use type default
                if let Some(default_value) = field_attrs.default_value {
                    match default_value {
                        FieldValue::String(s) => quote! { #field_name: #s.to_string() },
                        FieldValue::Number(n) => quote! { #field_name: #n },
                        FieldValue::Boolean(b) => quote! { #field_name: #b },
                        _ => quote! { #field_name: Default::default() }
                    }
                } else {
                    quote! { #field_name: Default::default() }
                }
            });

            quote! {
                Self {
                    #(#field_defaults),*
                }
            }
        }
        _ => quote! { Self {} }
    }
}
```

## 🎯 **Usage Examples**

### **Basic Form**

```rust
use leptos_forms_rs::*;

#[derive(Form, Clone, Serialize, Deserialize, PartialEq, Debug)]
struct LoginForm {
    #[field(required, email, label = "Email Address")]
    email: String,

    #[field(required, min_length = 8, label = "Password")]
    password: String,

    #[field(default = false, label = "Remember Me")]
    remember_me: bool,
}
```

### **Complex Form**

```rust
#[derive(Form, Clone, Serialize, Deserialize, PartialEq, Debug)]
struct UserRegistrationForm {
    #[field(required, label = "First Name")]
    first_name: String,

    #[field(required, label = "Last Name")]
    last_name: String,

    #[field(required, email, label = "Email Address")]
    email: String,

    #[field(required, min_length = 8, pattern = r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$", label = "Password")]
    password: String,

    #[field(required, min_length = 8, label = "Confirm Password")]
    confirm_password: String,

    #[field(default = false, label = "Subscribe to Newsletter")]
    newsletter: bool,
}
```

## 🎯 **Success Criteria**

### **Functional Requirements**

- ✅ Generate complete form implementations
- ✅ Support all field types
- ✅ Support all validation rules
- ✅ Support custom attributes
- ✅ Zero runtime overhead
- ✅ Compile-time validation

### **Performance Requirements**

- ✅ Compile time < 1 second
- ✅ Zero runtime overhead
- ✅ No memory allocations
- ✅ Type safety maintained

### **Quality Requirements**

- ✅ 100% test coverage
- ✅ No panics or crashes
- ✅ Proper error messages
- ✅ Documentation complete

## 📊 **Implementation Timeline**

- **Week 1**: Basic derive macro structure
- **Week 2**: Code generation for field access
- **Week 3**: Validation generation
- **Week 4**: Default values and testing

## 🚀 **Next Steps**

1. **Approve Design**: Review and approve this design
2. **Set Up Development**: Create feature branch
3. **Implement Phase 1**: Basic derive macro
4. **Implement Phase 2**: Code generation
5. **Implement Phase 3**: Validation generation
6. **Implement Phase 4**: Default values
7. **Testing**: Comprehensive testing
8. **Documentation**: Update docs and examples

This design will transform the developer experience from 50+ lines of boilerplate to a simple derive macro, making the library much more accessible and user-friendly.
