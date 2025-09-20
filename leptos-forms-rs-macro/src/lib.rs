use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Type};

/// Derive macro for implementing the Form trait
#[proc_macro_derive(Form, attributes(field, form))]
pub fn derive_form(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    // Parse the struct fields and their attributes
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("Form derive macro only supports structs"),
    };

    let field_metadata = generate_field_metadata(fields);
    let field_accessors = generate_field_accessors(fields);
    let field_setters = generate_field_setters(fields);
    let default_values = generate_default_values(fields);
    let validation_impl = generate_validation_impl(fields);

    let expanded = quote! {
        impl #generics crate::core::traits::Form for #name #generics {
            fn field_metadata() -> Vec<crate::core::traits::FieldMetadata> {
                vec![#field_metadata]
            }

            fn get_field_value(&self, field_name: &str) -> crate::core::types::FieldValue {
                match field_name {
                    #field_accessors
                    _ => crate::core::types::FieldValue::String(String::new()),
                }
            }

            fn set_field_value(&mut self, field_name: &str, value: crate::core::types::FieldValue) {
                match field_name {
                    #field_setters
                    _ => {}
                }
            }

            fn default_values() -> Self {
                Self {
                    #default_values
                }
            }

            fn validate(&self) -> Result<(), crate::validation::ValidationErrors> {
                let mut errors = crate::validation::ValidationErrors::new();

                #validation_impl

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Generate field metadata from struct fields
fn generate_field_metadata(fields: &Fields) -> proc_macro2::TokenStream {
    let field_metadata: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_type = determine_field_type(&field.ty);
            let field_attrs = FieldAttributes::from_attributes(&field.attrs);

            let validators: Vec<proc_macro2::TokenStream> = field_attrs
                .validators
                .iter()
                .map(|validator| match validator {
                    ValidatorType::Required => quote! { crate::validation::Validator::Required },
                    ValidatorType::Email => quote! { crate::validation::Validator::Email },
                    ValidatorType::Url => quote! { crate::validation::Validator::Url },
                    ValidatorType::MinLength(len) => {
                        quote! { crate::validation::Validator::MinLength(#len) }
                    }
                    ValidatorType::MaxLength(len) => {
                        quote! { crate::validation::Validator::MaxLength(#len) }
                    }
                    ValidatorType::Pattern(pattern) => {
                        quote! { crate::validation::Validator::Pattern(#pattern.to_string()) }
                    }
                })
                .collect();

            let default_value = if let Some(default) = &field_attrs.default_value {
                match default {
                    DefaultValue::String(s) => {
                        quote! { Some(crate::core::types::FieldValue::String(#s.to_string())) }
                    }
                    DefaultValue::Number(n) => {
                        quote! { Some(crate::core::types::FieldValue::Number(*#n)) }
                    }
                    DefaultValue::Boolean(b) => {
                        quote! { Some(crate::core::types::FieldValue::Boolean(#b)) }
                    }
                }
            } else {
                quote! { None }
            };

            let is_required = field_attrs.required;
            quote! {
                crate::core::traits::FieldMetadata {
                    name: stringify!(#field_name).to_string(),
                    field_type: #field_type,
                    validators: vec![#(#validators),*],
                    is_required: #is_required,
                    default_value: #default_value,
                    dependencies: vec![],
                    attributes: std::collections::HashMap::new(),
                }
            }
        })
        .collect();

    quote! {
        #(#field_metadata),*
    }
}

/// Generate field accessors for get_field_value method
fn generate_field_accessors(fields: &Fields) -> proc_macro2::TokenStream {
    let accessors: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;

            let conversion = convert_to_field_value(field_type, &quote! { self.#field_name });
            quote! {
                stringify!(#field_name) => {
                    #conversion
                }
            }
        })
        .collect();

    quote! {
        #(#accessors),*
    }
}

/// Generate field setters for set_field_value method
fn generate_field_setters(fields: &Fields) -> proc_macro2::TokenStream {
    let setters: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;

            let conversion =
                convert_from_field_value(field_type, &quote! { #field_name }, &quote! { value });
            quote! {
                stringify!(#field_name) => {
                    #conversion
                }
            }
        })
        .collect();

    quote! {
        #(#setters),*
    }
}

/// Generate default values for the struct
fn generate_default_values(fields: &Fields) -> proc_macro2::TokenStream {
    let defaults: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_attrs = FieldAttributes::from_attributes(&field.attrs);
            let field_type = &field.ty;

            let default_value = if let Some(default) = &field_attrs.default_value {
                match default {
                    DefaultValue::String(s) => quote! { #s.to_string() },
                    DefaultValue::Number(n) => quote! { *#n },
                    DefaultValue::Boolean(b) => quote! { #b },
                }
            } else {
                get_type_default(field_type)
            };

            quote! {
                #field_name: #default_value
            }
        })
        .collect();

    quote! {
        #(#defaults),*
    }
}

/// Generate validation implementation
fn generate_validation_impl(fields: &Fields) -> proc_macro2::TokenStream {
    let validations: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_attrs = FieldAttributes::from_attributes(&field.attrs);
            let field_type = &field.ty;

            let mut field_validations = Vec::new();

            // Generate type-specific validation
            match field_type {
                syn::Type::Path(type_path) => {
                    if let Some(ident) = type_path.path.get_ident() {
                        match ident.to_string().as_str() {
                            "String" => {
                                // Required validation for strings
                                if field_attrs.required {
                                    field_validations.push(quote! {
                                        if self.#field_name.is_empty() {
                                            errors.add_field_error(stringify!(#field_name), "This field is required".to_string());
                                        }
                                    });
                                }

                                // Email validation
                                if field_attrs.validators.contains(&ValidatorType::Email) {
                                    field_validations.push(quote! {
                                        if !self.#field_name.is_empty() && !self.#field_name.contains('@') {
                                            errors.add_field_error(stringify!(#field_name), "Invalid email format".to_string());
                                        }
                                    });
                                }

                                // URL validation
                                if field_attrs.validators.contains(&ValidatorType::Url) {
                                    field_validations.push(quote! {
                                        if !self.#field_name.is_empty() && !self.#field_name.starts_with("http") {
                                            errors.add_field_error(stringify!(#field_name), "Invalid URL format".to_string());
                                        }
                                    });
                                }

                                // Min length validation
                                for validator in &field_attrs.validators {
                                    if let ValidatorType::MinLength(min_len) = validator {
                                        field_validations.push(quote! {
                                            if self.#field_name.len() < #min_len {
                                                errors.add_field_error(stringify!(#field_name), format!("Minimum length is {} characters", #min_len));
                                            }
                                        });
                                    }
                                }

                                // Max length validation
                                for validator in &field_attrs.validators {
                                    if let ValidatorType::MaxLength(max_len) = validator {
                                        field_validations.push(quote! {
                                            if self.#field_name.len() > #max_len {
                                                errors.add_field_error(stringify!(#field_name), format!("Maximum length is {} characters", #max_len));
                                            }
                                        });
                                    }
                                }

                                // Pattern validation
                                for validator in &field_attrs.validators {
                                    if let ValidatorType::Pattern(pattern) = validator {
                                        field_validations.push(quote! {
                                            if !self.#field_name.is_empty() {
                                                if let Ok(regex) = regex::Regex::new(#pattern) {
                                                    if !regex.is_match(&self.#field_name) {
                                                        errors.add_field_error(stringify!(#field_name), "Pattern validation failed".to_string());
                                                    }
                                                }
                                            }
                                        });
                                    }
                                }
                            }
                            "bool" => {
                                // Boolean fields don't need validation typically
                            }
                            "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => {
                                // Required validation for numbers (check if 0)
                                if field_attrs.required {
                                    field_validations.push(quote! {
                                        if self.#field_name == 0 {
                                            errors.add_field_error(stringify!(#field_name), "This field is required".to_string());
                                        }
                                    });
                                }
                            }
                            _ => {
                                // For other types, just do basic required validation if needed
                                if field_attrs.required {
                                    field_validations.push(quote! {
                                        // Basic required validation for unknown types
                                        // This is a placeholder - specific validation should be added per type
                                    });
                                }
                            }
                        }
                    }
                }
                _ => {
                    // For non-path types, skip validation for now
                }
            }

            quote! {
                #(#field_validations)*
            }
        })
        .collect();

    quote! {
        #(#validations)*
    }
}

/// Determine the field type from the Rust type
fn determine_field_type(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) => {
            if let Some(ident) = type_path.path.get_ident() {
                match ident.to_string().as_str() {
                    "String" => quote! { crate::core::types::FieldType::Text },
                    "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => {
                        quote! { crate::core::types::FieldType::Number(crate::core::types::NumberType::new()) }
                    }
                    "bool" => quote! { crate::core::types::FieldType::Boolean },
                    _ => quote! { crate::core::types::FieldType::Text },
                }
            } else {
                quote! { crate::core::types::FieldType::Text }
            }
        }
        _ => quote! { crate::core::types::FieldType::Text },
    }
}

/// Convert Rust value to FieldValue
fn convert_to_field_value(ty: &Type, value: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) => {
            if let Some(ident) = type_path.path.get_ident() {
                match ident.to_string().as_str() {
                    "String" => quote! { crate::core::types::FieldValue::String(#value.clone()) },
                    "i32" => quote! { crate::core::types::FieldValue::Number(#value as f64) },
                    "i64" => quote! { crate::core::types::FieldValue::Number(#value as f64) },
                    "u32" => quote! { crate::core::types::FieldValue::Number(#value as f64) },
                    "u64" => quote! { crate::core::types::FieldValue::Number(#value as f64) },
                    "f32" => quote! { crate::core::types::FieldValue::Number(#value as f64) },
                    "f64" => quote! { crate::core::types::FieldValue::Number(#value) },
                    "bool" => quote! { crate::core::types::FieldValue::Boolean(#value) },
                    _ => quote! { crate::core::types::FieldValue::String(#value.to_string()) },
                }
            } else {
                quote! { crate::core::types::FieldValue::String(#value.to_string()) }
            }
        }
        _ => quote! { crate::core::types::FieldValue::String(#value.to_string()) },
    }
}

/// Convert FieldValue to Rust value
fn convert_from_field_value(
    ty: &Type,
    field_name: &proc_macro2::TokenStream,
    value: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) => {
            if let Some(ident) = type_path.path.get_ident() {
                match ident.to_string().as_str() {
                    "String" => quote! {
                        if let crate::core::types::FieldValue::String(s) = #value {
                            self.#field_name = s.clone();
                        }
                    },
                    "i32" => quote! {
                        if let crate::core::types::FieldValue::Number(n) = #value {
                            self.#field_name = n as i32;
                        }
                    },
                    "i64" => quote! {
                        if let crate::core::types::FieldValue::Number(n) = #value {
                            self.#field_name = n as i64;
                        }
                    },
                    "u32" => quote! {
                        if let crate::core::types::FieldValue::Number(n) = #value {
                            self.#field_name = n as u32;
                        }
                    },
                    "u64" => quote! {
                        if let crate::core::types::FieldValue::Number(n) = #value {
                            self.#field_name = n as u64;
                        }
                    },
                    "f32" => quote! {
                        if let crate::core::types::FieldValue::Number(n) = #value {
                            self.#field_name = n as f32;
                        }
                    },
                    "f64" => quote! {
                        if let crate::core::types::FieldValue::Number(n) = #value {
                            self.#field_name = n;
                        }
                    },
                    "bool" => quote! {
                        if let crate::core::types::FieldValue::Boolean(b) = #value {
                            self.#field_name = b;
                        }
                    },
                    _ => quote! {
                        if let crate::core::types::FieldValue::String(s) = #value {
                            self.#field_name = s.clone();
                        }
                    },
                }
            } else {
                quote! {
                    if let crate::core::types::FieldValue::String(s) = #value {
                        self.#field_name = s.clone();
                    }
                }
            }
        }
        _ => quote! {
            if let crate::core::types::FieldValue::String(s) = #value {
                self.#field_name = s.clone();
            }
        },
    }
}

/// Get default value for a type
fn get_type_default(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) => {
            if let Some(ident) = type_path.path.get_ident() {
                match ident.to_string().as_str() {
                    "String" => quote! { String::new() },
                    "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => quote! { 0 },
                    "bool" => quote! { false },
                    _ => quote! { Default::default() },
                }
            } else {
                quote! { Default::default() }
            }
        }
        _ => quote! { Default::default() },
    }
}

/// Field attributes parsed from the field attributes
#[derive(Debug, Clone)]
struct FieldAttributes {
    pub required: bool,
    pub validators: Vec<ValidatorType>,
    pub default_value: Option<DefaultValue>,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
}

impl FieldAttributes {
    fn from_attributes(attrs: &[Attribute]) -> Self {
        let mut field_attrs = FieldAttributes {
            required: false,
            validators: Vec::new(),
            default_value: None,
            label: None,
            placeholder: None,
            help_text: None,
        };

        for attr in attrs {
            if attr.path().is_ident("field") {
                // Parse the attribute tokens manually for now
                let tokens = attr.meta.to_token_stream().to_string();

                // Check for required
                if tokens.contains("required = true") {
                    field_attrs.required = true;
                }

                // Check for email
                if tokens.contains("email = true") {
                    field_attrs.validators.push(ValidatorType::Email);
                }

                // Check for url
                if tokens.contains("url = true") {
                    field_attrs.validators.push(ValidatorType::Url);
                }

                // Check for min_length
                if let Some(min_len) = extract_number_from_tokens(&tokens, "min_length") {
                    field_attrs
                        .validators
                        .push(ValidatorType::MinLength(min_len));
                }

                // Check for max_length
                if let Some(max_len) = extract_number_from_tokens(&tokens, "max_length") {
                    field_attrs
                        .validators
                        .push(ValidatorType::MaxLength(max_len));
                }

                // Check for pattern
                if let Some(pattern) = extract_string_from_tokens(&tokens, "pattern") {
                    field_attrs.validators.push(ValidatorType::Pattern(pattern));
                }

                // Check for default values
                if let Some(default_str) = extract_string_from_tokens(&tokens, "default") {
                    field_attrs.default_value = Some(DefaultValue::String(default_str));
                } else if let Some(default_num) = extract_number_from_tokens(&tokens, "default") {
                    field_attrs.default_value = Some(DefaultValue::Number(default_num as f64));
                } else if tokens.contains("default = true") {
                    field_attrs.default_value = Some(DefaultValue::Boolean(true));
                } else if tokens.contains("default = false") {
                    field_attrs.default_value = Some(DefaultValue::Boolean(false));
                }

                // Check for label
                if let Some(label) = extract_string_from_tokens(&tokens, "label") {
                    field_attrs.label = Some(label);
                }

                // Check for placeholder
                if let Some(placeholder) = extract_string_from_tokens(&tokens, "placeholder") {
                    field_attrs.placeholder = Some(placeholder);
                }

                // Check for help_text
                if let Some(help_text) = extract_string_from_tokens(&tokens, "help_text") {
                    field_attrs.help_text = Some(help_text);
                }
            }
        }

        field_attrs
    }
}

/// Validator types
#[derive(Debug, Clone, PartialEq)]
enum ValidatorType {
    Required,
    Email,
    Url,
    MinLength(usize),
    MaxLength(usize),
    Pattern(String),
}

/// Default value types
#[derive(Debug, Clone)]
enum DefaultValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

/// Helper function to extract numbers from token strings
fn extract_number_from_tokens(tokens: &str, key: &str) -> Option<usize> {
    let pattern = format!("{} = ", key);
    if let Some(start) = tokens.find(&pattern) {
        let start = start + pattern.len();
        if let Some(end) = tokens[start..].find(|c: char| !c.is_ascii_digit()) {
            tokens[start..start + end].parse::<usize>().ok()
        } else {
            tokens[start..].parse::<usize>().ok()
        }
    } else {
        None
    }
}

/// Helper function to extract strings from token strings
fn extract_string_from_tokens(tokens: &str, key: &str) -> Option<String> {
    let pattern = format!("{} = \"", key);
    if let Some(start) = tokens.find(&pattern) {
        let start = start + pattern.len();
        if let Some(end) = tokens[start..].find('"') {
            Some(tokens[start..start + end].to_string())
        } else {
            None
        }
    } else {
        None
    }
}
