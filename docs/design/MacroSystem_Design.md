# Macro System Design

**File**: `leptos-forms-rs-macro/src/lib.rs` (544 lines → split into <300 line files)
**Priority**: CRITICAL (Currently broken - missing helper functions)
**Dependencies**: Core helper functions, FormHandle

## Problem Statement

The current derive macro system is completely broken:

- Generates calls to undefined functions (`convert_to_field_value`, `convert_from_field_value`, `get_type_default`)
- 544-line monolithic file that's impossible to maintain
- No compile-time validation of generated code
- Poor error messages for users

## Solution Architecture

### File Structure (Split monolithic macro)

```
leptos-forms-rs-macro/src/
├── lib.rs (< 100 lines) - Main proc_macro entry points
├── form_derive/
│   ├── mod.rs (< 50 lines) - Form derive exports
│   ├── parser.rs (< 200 lines) - Parse struct definitions
│   ├── generator.rs (< 250 lines) - Generate Form trait impl
│   └── validation.rs (< 150 lines) - Generate validation code
├── field_derive/
│   ├── mod.rs (< 50 lines) - Field-level derives
│   ├── metadata.rs (< 200 lines) - Generate field metadata
│   └── conversion.rs (< 150 lines) - Type conversion helpers
├── utils/
│   ├── mod.rs (< 50 lines)
│   ├── diagnostics.rs (< 150 lines) - Error reporting
│   └── codegen.rs (< 100 lines) - Code generation helpers
└── tests.rs (< 200 lines) - Macro expansion tests
```

## Core Design Principles

### 1. Fail Fast with Clear Errors

- Validate struct definitions at compile time
- Provide helpful error messages with span information
- Guide users toward correct usage

### 2. Generate Minimal, Efficient Code

- No runtime overhead for type conversions
- Inline simple operations
- Use const/static where possible

### 3. Extensibility

- Support custom field attributes
- Allow validation rule customization
- Enable third-party integrations

## Form Derive Implementation

### Main Entry Point (`lib.rs`)

```rust
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod form_derive;
mod field_derive;
mod utils;

/// Derive macro for implementing the Form trait
#[proc_macro_derive(Form, attributes(field, form))]
pub fn derive_form(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match form_derive::generate_form_impl(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => {
            // Convert syn::Error to compile error with proper spans
            err.to_compile_error().into()
        }
    }
}

/// Derive macro for field-level metadata
#[proc_macro_derive(FieldMetadata, attributes(validate, field_type))]
pub fn derive_field_metadata(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match field_derive::generate_field_metadata(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into()
    }
}
```

### Struct Parser (`form_derive/parser.rs`)

```rust
use syn::{Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident, Type};
use quote::quote;

pub struct FormStruct {
    pub name: Ident,
    pub fields: Vec<FormField>,
    pub form_attributes: FormAttributes,
}

pub struct FormField {
    pub name: Ident,
    pub ty: Type,
    pub field_type: Option<FieldType>,
    pub validators: Vec<Validator>,
    pub optional: bool,
    pub default_value: Option<proc_macro2::TokenStream>,
}

#[derive(Default)]
pub struct FormAttributes {
    pub form_name: Option<String>,
    pub validate_on_submit: bool,
    pub validate_on_change: bool,
    pub persist: Option<String>, // localStorage key
}

pub fn parse_form_struct(input: &DeriveInput) -> syn::Result<FormStruct> {
    let name = input.ident.clone();
    let form_attributes = parse_form_attributes(&input.attrs)?;

    let data = match &input.data {
        Data::Struct(data) => data,
        Data::Enum(_) => return Err(syn::Error::new_spanned(
            input,
            "Form derive can only be used on structs"
        )),
        Data::Union(_) => return Err(syn::Error::new_spanned(
            input,
            "Form derive cannot be used on unions"
        )),
    };

    let fields = parse_struct_fields(data)?;

    Ok(FormStruct {
        name,
        fields,
        form_attributes,
    })
}

fn parse_struct_fields(data: &DataStruct) -> syn::Result<Vec<FormField>> {
    let fields = match &data.fields {
        Fields::Named(FieldsNamed { named, .. }) => named,
        _ => return Err(syn::Error::new_spanned(
            data.struct_token,
            "Form derive requires named fields"
        )),
    };

    let mut form_fields = Vec::new();

    for field in fields {
        let form_field = parse_single_field(field)?;
        form_fields.push(form_field);
    }

    Ok(form_fields)
}

fn parse_single_field(field: &Field) -> syn::Result<FormField> {
    let name = field.ident.clone().ok_or_else(|| {
        syn::Error::new_spanned(field, "Field must have a name")
    })?;

    let ty = field.ty.clone();
    let (field_type, validators, optional, default_value) = parse_field_attributes(&field.attrs)?;

    Ok(FormField {
        name,
        ty,
        field_type,
        validators,
        optional,
        default_value,
    })
}
```

### Code Generator (`form_derive/generator.rs`)

```rust
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

pub fn generate_form_impl(form_struct: &FormStruct) -> syn::Result<TokenStream> {
    let struct_name = &form_struct.name;
    let default_impl = generate_default_values(form_struct)?;
    let field_metadata_impl = generate_field_metadata(form_struct)?;
    let validation_impl = generate_validation(form_struct)?;
    let conversion_impl = generate_conversions(form_struct)?;

    let form_name = form_struct.form_attributes.form_name
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or_else(|| struct_name.to_string().as_str());

    Ok(quote! {
        impl leptos_forms_rs::Form for #struct_name {
            fn default_values() -> Self {
                #default_impl
            }

            fn field_metadata() -> Vec<leptos_forms_rs::FieldMetadata> {
                #field_metadata_impl
            }

            fn form_name() -> &'static str {
                #form_name
            }

            fn validate(&self) -> Result<(), leptos_forms_rs::ValidationErrors> {
                #validation_impl
            }

            fn to_field_map(&self) -> std::collections::HashMap<String, leptos_forms_rs::FieldValue> {
                #conversion_impl
            }
        }
    })
}

fn generate_default_values(form_struct: &FormStruct) -> syn::Result<TokenStream> {
    let struct_name = &form_struct.name;
    let field_defaults = form_struct.fields.iter().map(|field| {
        let field_name = &field.name;

        if let Some(default_value) = &field.default_value {
            // User provided explicit default
            quote! { #field_name: #default_value }
        } else if field.optional {
            // Optional field defaults to None
            quote! { #field_name: None }
        } else {
            // Use type's Default implementation
            quote! { #field_name: Default::default() }
        }
    });

    Ok(quote! {
        #struct_name {
            #(#field_defaults,)*
        }
    })
}

fn generate_field_metadata(form_struct: &FormStruct) -> syn::Result<TokenStream> {
    let field_metadata = form_struct.fields.iter().map(|field| {
        let field_name = field.name.to_string();
        let field_type = field.field_type.as_ref()
            .map(|ft| quote! { leptos_forms_rs::FieldType::#ft })
            .unwrap_or_else(|| infer_field_type(&field.ty));

        let validators = field.validators.iter().map(|validator| {
            generate_validator_code(validator)
        });

        quote! {
            leptos_forms_rs::FieldMetadata {
                name: #field_name.to_string(),
                field_type: #field_type,
                validators: vec![#(#validators,)*],
                required: !#field_optional,
                ..Default::default()
            }
        }
    });

    Ok(quote! {
        vec![#(#field_metadata,)*]
    })
}

fn generate_conversions(form_struct: &FormStruct) -> syn::Result<TokenStream> {
    let field_conversions = form_struct.fields.iter().map(|field| {
        let field_name = &field.name;
        let field_name_str = field_name.to_string();

        quote! {
            map.insert(
                #field_name_str.to_string(),
                leptos_forms_rs::core::convert_to_field_value(&self.#field_name)
            );
        }
    });

    Ok(quote! {
        let mut map = std::collections::HashMap::new();
        #(#field_conversions)*
        map
    })
}
```

## Helper Function Implementation

### Core Conversion Functions (`leptos-forms-rs/src/core/conversions.rs`)

```rust
use serde::Serialize;
use crate::core::FieldValue;

/// Convert any serializable value to FieldValue
pub fn convert_to_field_value<T: Serialize>(value: &T) -> FieldValue {
    // Try direct conversion for common types first (fast path)
    if let Ok(string_val) = serde_json::to_string(value) {
        match string_val.as_str() {
            "true" => return FieldValue::Boolean(true),
            "false" => return FieldValue::Boolean(false),
            "null" => return FieldValue::Empty,
            _ => {}
        }

        // Try parsing as number
        if let Ok(num) = string_val.parse::<f64>() {
            return FieldValue::Number(num);
        }

        // Default to string representation
        FieldValue::String(string_val.trim_matches('"').to_string())
    } else {
        FieldValue::Empty
    }
}

/// Convert FieldValue back to typed value
pub fn convert_from_field_value<T>(value: &FieldValue) -> Result<T, ValidationError>
where
    T: serde::de::DeserializeOwned + Default,
{
    match value {
        FieldValue::Empty => Ok(T::default()),
        FieldValue::String(s) => {
            serde_json::from_str(&format!("\"{}\"", s))
                .or_else(|_| serde_json::from_str(s))
                .map_err(|e| ValidationError::Conversion(e.to_string()))
        },
        FieldValue::Number(n) => {
            serde_json::from_value(serde_json::Value::Number(
                serde_json::Number::from_f64(*n).unwrap()
            )).map_err(|e| ValidationError::Conversion(e.to_string()))
        },
        FieldValue::Boolean(b) => {
            serde_json::from_value(serde_json::Value::Bool(*b))
                .map_err(|e| ValidationError::Conversion(e.to_string()))
        },
    }
}

/// Get default value for any type
pub fn get_type_default<T: Default>() -> T {
    T::default()
}
```

## Attribute Parsing

### Field Attributes (`field_derive/metadata.rs`)

```rust
use syn::{Attribute, Lit, Meta, NestedMeta};

#[derive(Default)]
pub struct FieldAttributes {
    pub field_type: Option<FieldType>,
    pub validators: Vec<Validator>,
    pub default: Option<proc_macro2::TokenStream>,
    pub optional: bool,
}

pub fn parse_field_attributes(attrs: &[Attribute]) -> syn::Result<FieldAttributes> {
    let mut field_attrs = FieldAttributes::default();

    for attr in attrs {
        if attr.path.is_ident("field") {
            parse_field_attribute(attr, &mut field_attrs)?;
        } else if attr.path.is_ident("validate") {
            parse_validate_attribute(attr, &mut field_attrs)?;
        }
    }

    Ok(field_attrs)
}

fn parse_validate_attribute(
    attr: &Attribute,
    field_attrs: &mut FieldAttributes
) -> syn::Result<()> {
    match attr.parse_meta()? {
        Meta::List(meta_list) => {
            for nested in meta_list.nested {
                match nested {
                    NestedMeta::Meta(Meta::Path(path)) if path.is_ident("required") => {
                        field_attrs.validators.push(Validator::Required);
                    },
                    NestedMeta::Meta(Meta::Path(path)) if path.is_ident("email") => {
                        field_attrs.validators.push(Validator::Email);
                    },
                    NestedMeta::Meta(Meta::NameValue(nv)) if nv.path.is_ident("min_length") => {
                        if let Lit::Int(lit_int) = nv.lit {
                            let value: usize = lit_int.base10_parse()?;
                            field_attrs.validators.push(Validator::MinLength(value));
                        }
                    },
                    // ... more validators
                    _ => return Err(syn::Error::new_spanned(
                        nested,
                        "Unknown validation attribute"
                    )),
                }
            }
        },
        _ => return Err(syn::Error::new_spanned(
            attr,
            "Invalid validate attribute format"
        )),
    }

    Ok(())
}
```

## Error Handling & Diagnostics

### Helpful Error Messages (`utils/diagnostics.rs`)

```rust
use proc_macro2::Span;
use syn::Error;

pub struct DiagnosticBuilder {
    span: Span,
    message: String,
    suggestions: Vec<String>,
}

impl DiagnosticBuilder {
    pub fn new(span: Span, message: &str) -> Self {
        Self {
            span,
            message: message.to_string(),
            suggestions: Vec::new(),
        }
    }

    pub fn suggestion(mut self, suggestion: &str) -> Self {
        self.suggestions.push(suggestion.to_string());
        self
    }

    pub fn help(mut self, help: &str) -> Self {
        self.suggestions.push(format!("help: {}", help));
        self
    }

    pub fn build(self) -> Error {
        let mut message = self.message;

        if !self.suggestions.is_empty() {
            message.push_str("\n\n");
            for suggestion in self.suggestions {
                message.push_str(&format!("  {}\n", suggestion));
            }
        }

        Error::new(self.span, message)
    }
}

// Usage example:
pub fn unsupported_field_type_error(span: Span, type_name: &str) -> Error {
    DiagnosticBuilder::new(
        span,
        &format!("Unsupported field type: {}", type_name)
    )
    .suggestion("Try using a supported type like String, bool, or i32")
    .suggestion("Or implement custom conversion logic")
    .help("See documentation for supported field types")
    .build()
}
```

## Testing Strategy

### Macro Expansion Tests (`tests.rs`)

```rust
use quote::quote;
use syn::parse_quote;

#[test]
fn test_basic_form_derive() {
    let input = parse_quote! {
        #[derive(Form)]
        struct LoginForm {
            #[validate(required, email)]
            email: String,

            #[validate(required, min_length = 8)]
            password: String,
        }
    };

    let output = form_derive::generate_form_impl(&input).unwrap();

    // Verify generated code compiles
    let expected = quote! {
        impl leptos_forms_rs::Form for LoginForm {
            // ... expected implementation
        }
    };

    // Use prettyplease or similar to compare formatted output
    assert_tokens_eq!(output, expected);
}

#[test]
fn test_invalid_field_type_error() {
    let input = parse_quote! {
        #[derive(Form)]
        struct BadForm {
            unsupported_field: std::collections::HashMap<String, String>,
        }
    };

    let result = form_derive::generate_form_impl(&input);
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert!(error.to_string().contains("Unsupported field type"));
}
```

## Usage Examples

### Basic Form

```rust
use leptos_forms_rs::prelude::*;

#[derive(Form, Clone, Debug)]
struct ContactForm {
    #[validate(required)]
    #[field(type = "text")]
    name: String,

    #[validate(required, email)]
    email: String,

    #[validate(min_length = 10)]
    #[field(type = "textarea")]
    message: String,

    #[field(default = false)]
    subscribe: bool,
}
```

### Advanced Form with Custom Validation

```rust
#[derive(Form)]
#[form(name = "UserRegistration", validate_on_change = true)]
struct UserForm {
    #[validate(required, min_length = 3, max_length = 20)]
    #[field(placeholder = "Enter username")]
    username: String,

    #[validate(required, custom = "validate_strong_password")]
    password: String,

    #[validate(required, custom = "validate_password_match")]
    password_confirm: String,

    #[field(optional)]
    age: Option<u32>,
}

fn validate_strong_password(value: &FieldValue) -> ValidationResult {
    let password = value.as_string();
    if password.len() < 8 {
        return Err("Password must be at least 8 characters".into());
    }
    // ... more validation
    Ok(())
}
```

## Success Criteria

### Functionality

- [ ] Derive macro compiles without errors
- [ ] Generated code works with FormHandle
- [ ] All field types properly converted
- [ ] Validation attributes work correctly
- [ ] Helpful error messages for invalid usage

### Code Quality

- [ ] All files under 300 lines
- [ ] Clear separation of parsing/generation
- [ ] Comprehensive error handling
- [ ] Extensive test coverage (>90%)

### Performance

- [ ] Fast compilation (macro expansion <500ms)
- [ ] Generated code has minimal runtime overhead
- [ ] No unnecessary allocations in generated code

### Developer Experience

- [ ] Clear documentation with examples
- [ ] IDE autocomplete works correctly
- [ ] Error messages guide users to solutions
- [ ] Supports common field types out of the box
