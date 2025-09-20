//! Simple derive macro test

use crate::core::traits::Form;
use crate::core::types::FieldValue;
use leptos_forms_rs_macro::Form;

#[derive(Form, Clone, Debug, serde::Serialize, serde::Deserialize)]
struct SimpleForm {
    name: String,
    age: i32,
    active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_derive() {
        let form = SimpleForm::default_values();
        println!("Default form: {:?}", form);

        let metadata = SimpleForm::field_metadata();
        println!("Field metadata count: {}", metadata.len());

        // Test field access
        let name_value = form.get_field_value("name");
        println!("Name value: {:?}", name_value);

        let age_value = form.get_field_value("age");
        println!("Age value: {:?}", age_value);

        let active_value = form.get_field_value("active");
        println!("Active value: {:?}", active_value);
    }
}
