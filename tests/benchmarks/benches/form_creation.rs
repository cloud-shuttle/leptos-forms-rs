use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leptos::*;
use leptos_forms_rs::*;

#[derive(Clone, Debug, PartialEq)]
struct BenchmarkForm {
    field1: String,
    field2: String,
    field3: String,
    field4: String,
    field5: String,
}

impl Form for BenchmarkForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "field1".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "field2".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "field3".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "field4".to_string(),
                field_type: FieldType::Text,
                validators: vec![ValidatorConfig::Required],
                is_required: true,
                default_value: None,
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            FieldMetadata {
                name: "field5".to_string(),
                field_type: FieldType::Text,
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

        if self.field1.is_empty() {
            errors.add_field_error("field1", FieldError::new("Field 1 is required"));
        }
        if self.field2.is_empty() {
            errors.add_field_error("field2", FieldError::new("Field 2 is required"));
        }
        if self.field3.is_empty() {
            errors.add_field_error("field3", FieldError::new("Field 3 is required"));
        }
        if self.field4.is_empty() {
            errors.add_field_error("field4", FieldError::new("Field 4 is required"));
        }
        if self.field5.is_empty() {
            errors.add_field_error("field5", FieldError::new("Field 5 is required"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn get_field(&self, name: &str) -> Option<FieldValue> {
        match name {
            "field1" => Some(FieldValue::String(self.field1.clone())),
            "field2" => Some(FieldValue::String(self.field2.clone())),
            "field3" => Some(FieldValue::String(self.field3.clone())),
            "field4" => Some(FieldValue::String(self.field4.clone())),
            "field5" => Some(FieldValue::String(self.field5.clone())),
            _ => None,
        }
    }

    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), String> {
        match name {
            "field1" => {
                if let FieldValue::String(s) = value {
                    self.field1 = s;
                    Ok(())
                } else {
                    Err("Expected string value for field1".to_string())
                }
            }
            "field2" => {
                if let FieldValue::String(s) = value {
                    self.field2 = s;
                    Ok(())
                } else {
                    Err("Expected string value for field2".to_string())
                }
            }
            "field3" => {
                if let FieldValue::String(s) = value {
                    self.field3 = s;
                    Ok(())
                } else {
                    Err("Expected string value for field3".to_string())
                }
            }
            "field4" => {
                if let FieldValue::String(s) = value {
                    self.field4 = s;
                    Ok(())
                } else {
                    Err("Expected string value for field4".to_string())
                }
            }
            "field5" => {
                if let FieldValue::String(s) = value {
                    self.field5 = s;
                    Ok(())
                } else {
                    Err("Expected string value for field5".to_string())
                }
            }
            _ => Err(format!("Unknown field: {}", name)),
        }
    }

    fn default_values() -> Self {
        BenchmarkForm {
            field1: String::new(),
            field2: String::new(),
            field3: String::new(),
            field4: String::new(),
            field5: String::new(),
        }
    }

    fn schema() -> FormSchema {
        FormSchema::new(Self::field_metadata())
    }
}

fn benchmark_form_creation(c: &mut Criterion) {
    c.bench_function("form_creation", |b| {
        b.iter(|| {
            black_box(use_form::<BenchmarkForm>());
        });
    });
}

criterion_group!(benches, benchmark_form_creation);
criterion_main!(benches);
