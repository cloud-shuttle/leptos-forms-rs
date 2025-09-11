use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Meta, Type};

/// Derive macro for implementing the Form trait
#[proc_macro_derive(Form, attributes(form))]
pub fn derive_form(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;

    // Parse the struct fields and their attributes
    let fields = match input.data {
        Data::Struct(data) => data.fields,
        _ => panic!("Form derive macro only supports structs"),
    };

    let field_metadata = generate_field_metadata(&fields);
    let field_accessors = generate_field_accessors(&fields);
    let field_setters = generate_field_setters(&fields);
    let default_values = generate_default_values(&fields);
    let validation_impl = generate_validation_impl(&fields);

    let expanded = quote! {
        impl #generics leptos_forms_rs::core::traits::Form for #name #generics {
            fn field_metadata() -> Vec<leptos_forms_rs::core::traits::FieldMetadata> {
                vec![#field_metadata]
            }

            fn get_field(&self, name: &str) -> Option<leptos_forms_rs::core::types::FieldValue> {
                match name {
                    #field_accessors
                    _ => None,
                }
            }

            fn set_field(&mut self, name: &str, value: leptos_forms_rs::core::types::FieldValue) -> Result<(), leptos_forms_rs::core::types::FieldError> {
                match name {
                    #field_setters
                    _ => Err(leptos_forms_rs::core::types::FieldError::new(
                        name.to_string(),
                        "Unknown field".to_string()
                    )),
                }
            }

            fn default_values() -> Self {
                Self {
                    #default_values
                }
            }

            fn validate(&self) -> Result<(), leptos_forms_rs::validation::ValidationErrors> {
    let mut errors = leptos_forms_rs::validation::ValidationErrors::new();

                #validation_impl

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }

            fn schema() -> leptos_forms_rs::core::traits::FormSchema {
    let mut schema = leptos_forms_rs::core::traits::FormSchema::new();

                for field in Self::field_metadata() {
                    schema.add_field(field);
                }

                schema
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
            let _field_type = determine_field_type(&field.ty);
            let validators = extract_validators(&field.attrs);
            let is_required = has_validator(&field.attrs, "required");
            let default_value = extract_default_value(&field.attrs, &field.ty);

            quote! {
                leptos_forms_rs::core::traits::FieldMetadata {
                    name: stringify!(#field_name).to_string(),
                    field_type: #_field_type,
                    validators: vec![#validators],
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

/// Generate field accessors for get_field method
fn generate_field_accessors(fields: &Fields) -> proc_macro2::TokenStream {
    let accessors: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;

            quote! {
                stringify!(#field_name) => {
                    let value = &self.#field_name;
                    Some(leptos_forms_rs::utils::field_utils::convert_to_field_value(value))
                }
            }
        })
        .collect();

    quote! {
        #(#accessors),*
    }
}

/// Generate field setters for set_field method
fn generate_field_setters(fields: &Fields) -> proc_macro2::TokenStream {
    let setters: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;

            quote! {
                stringify!(#field_name) => {
                    match value {
                        leptos_forms_rs::core::types::FieldValue::String(s) => {
                            self.#field_name = s;
                            Ok(())
                        }
                        leptos_forms_rs::core::types::FieldValue::Boolean(b) => {
                            self.#field_name = b;
                            Ok(())
                        }
                        _ => Err(leptos_forms_rs::core::types::FieldError::new(
                            stringify!(#field_name).to_string(),
                            "Invalid value type".to_string()
                        )),
                    }
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
            let _field_type = &field.ty;
            let default_value = extract_default_value(&field.attrs, &field.ty);

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
            let validators = extract_validators(&field.attrs);

            quote! {
                // Validate field: #field_name
                let field_value = leptos_forms_rs::utils::field_utils::convert_to_field_value(&self.#field_name);
                for validator in &[#validators] {
                    if let Err(error) = leptos_forms_rs::validation::validate_field_value(&field_value, validator) {
                        errors.add_field_error(stringify!(#field_name).to_string(), error);
                    }
                }
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
            let type_name = &type_path.path.segments.last().unwrap().ident;
            match type_name.to_string().as_str() {
                "String" => quote! { leptos_forms_rs::core::types::FieldType::Text },
                "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => {
                    quote! { leptos_forms_rs::core::types::FieldType::Number(leptos_forms_rs::core::types::NumberType { min: None, max: None, step: None }) }
                }
                "bool" => quote! { leptos_forms_rs::core::types::FieldType::Boolean },
                _ => quote! { leptos_forms_rs::core::types::FieldType::Text },
            }
        }
        _ => quote! { leptos_forms_rs::core::types::FieldType::Text },
    }
}

/// Extract validators from field attributes
fn extract_validators(attrs: &[Attribute]) -> proc_macro2::TokenStream {
    let mut validators = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("form") {
            if let Meta::List(meta_list) = &attr.meta {
                // Parse the tokens manually for now
                // This is a simplified version - in a real implementation you'd want more robust parsing
                let tokens = meta_list.tokens.to_string();
                if tokens.contains("required") {
                    validators
                        .push(quote! { leptos_forms_rs::core::types::ValidatorConfig::Required });
                }
                if tokens.contains("email") {
                    validators
                        .push(quote! { leptos_forms_rs::core::types::ValidatorConfig::Email });
                }
                if tokens.contains("url") {
                    validators.push(quote! { leptos_forms_rs::core::types::ValidatorConfig::Url });
                }
                // For now, we'll use simple string matching for numeric values
                if let Some(min_length) = extract_number_from_tokens(&tokens, "min_length") {
                    validators.push(quote! { leptos_forms_rs::core::types::ValidatorConfig::MinLength(#min_length) });
                }
                if let Some(max_length) = extract_number_from_tokens(&tokens, "max_length") {
                    validators.push(quote! { leptos_forms_rs::core::types::ValidatorConfig::MaxLength(#max_length) });
                }
            }
        }
    }

    if validators.is_empty() {
        quote! {}
    } else {
        quote! { #(#validators),* }
    }
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

/// Check if a field has a specific validator
fn has_validator(attrs: &[Attribute], validator_name: &str) -> proc_macro2::TokenStream {
    for attr in attrs {
        if attr.path().is_ident("form") {
            if let Meta::List(meta_list) = &attr.meta {
                let tokens = meta_list.tokens.to_string();
                if tokens.contains(validator_name) {
                    return quote! { true };
                }
            }
        }
    }
    quote! { false }
}

/// Extract default value from field attributes
fn extract_default_value(attrs: &[Attribute], field_type: &Type) -> proc_macro2::TokenStream {
    for attr in attrs {
        if attr.path().is_ident("form") {
            if let Meta::List(meta_list) = &attr.meta {
                let tokens = meta_list.tokens.to_string();
                if tokens.contains("default = true") {
                    return quote! { true };
                }
                if tokens.contains("default = false") {
                    return quote! { false };
                }
                if tokens.contains("default = \"en\"") {
                    return quote! { "en".to_string() };
                }
            }
        }
    }

    // Use the appropriate default based on the field type
    match field_type {
        Type::Path(type_path) => {
            let type_name = &type_path.path.segments.last().unwrap().ident;
            match type_name.to_string().as_str() {
                "String" => quote! { String::new() },
                "bool" => quote! { false },
                "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => quote! { 0 },
                _ => quote! { Default::default() },
            }
        }
        _ => quote! { Default::default() },
    }
}

/// Helper function to convert Rust values to FieldValue
#[allow(dead_code)]
fn convert_to_field_value(_value: &impl std::fmt::Debug) -> proc_macro2::TokenStream {
    quote! {
        match value {
            s if std::any::Any::is::<String>(s) => {
                leptos_forms_rs::core::types::FieldValue::String(s.to_string())
            }
            b if std::any::Any::is::<bool>(b) => {
                leptos_forms_rs::core::types::FieldValue::Boolean(*b)
            }
            _ => leptos_forms_rs::core::types::FieldValue::Null,
        }
    }
}
