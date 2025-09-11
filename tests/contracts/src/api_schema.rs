//! API Schema generation and validation for contract testing

use crate::*;
use serde_json;
use std::collections::HashMap;

/// Generate API schema for contract testing
pub fn generate_api_schema() -> serde_json::Value {
    serde_json::json!({
        "version": "1.1.0",
        "traits": {
            "Form": {
                "required_methods": [
                    "field_metadata",
                    "validate",
                    "default_values",
                    "schema",
                    "get_field_value",
                    "set_field_value"
                ],
                "signatures": {
                    "field_metadata": "fn() -> Vec<FieldMetadata>",
                    "validate": "fn(&self) -> Result<(), ValidationErrors>",
                    "default_values": "fn() -> Self",
                    "schema": "fn() -> FormSchema",
                    "get_field_value": "fn(&self, field_name: &str) -> FieldValue",
                    "set_field_value": "fn(&mut self, field_name: &str, value: FieldValue)"
                },
                "stability": "stable"
            }
        },
        "types": {
            "FieldMetadata": {
                "fields": [
                    "name",
                    "field_type",
                    "is_required",
                    "default_value",
                    "dependencies",
                    "attributes",
                    "validators"
                ],
                "stability": "stable"
            },
            "FormSchema": {
                "fields": [
                    "name",
                    "field_metadata"
                ],
                "methods": [
                    "new",
                    "get_field"
                ],
                "stability": "stable"
            },
            "ValidationErrors": {
                "methods": [
                    "new",
                    "is_empty",
                    "has_field_error",
                    "add_field_error"
                ],
                "stability": "stable"
            },
            "FieldValue": {
                "variants": [
                    "String",
                    "Number",
                    "Integer",
                    "Boolean",
                    "Date",
                    "DateTime",
                    "Array",
                    "Object",
                    "File",
                    "Null"
                ],
                "stability": "stable"
            },
            "FieldType": {
                "variants": [
                    "Text",
                    "Email",
                    "Password",
                    "Number",
                    "Boolean",
                    "Select",
                    "MultiSelect",
                    "Date",
                    "DateTime",
                    "File",
                    "Array",
                    "Nested"
                ],
                "stability": "stable"
            },
            "Validator": {
                "variants": [
                    "Required",
                    "Email",
                    "Url",
                    "MinLength",
                    "MaxLength",
                    "Min",
                    "Max",
                    "Pattern",
                    "Custom"
                ],
                "stability": "stable"
            }
        },
        "hooks": {
            "use_form": {
                "signature": "fn<T: Form>(initial_values: Option<T>) -> (FormHandle<T>, Callback<T>, Callback<()>)",
                "stability": "stable"
            },
            "use_field_value": {
                "signature": "fn<T: Form>(form_handle: &FormHandle<T>, field_name: &str) -> Memo<FieldValue>",
                "stability": "stable"
            },
            "use_field_error": {
                "signature": "fn<T: Form>(form_handle: &FormHandle<T>, field_name: &str) -> Memo<Option<String>>",
                "stability": "stable"
            }
        },
        "components": {
            "Form": {
                "props": [
                    "form_handle",
                    "class",
                    "id",
                    "novalidate",
                    "children"
                ],
                "stability": "stable"
            },
            "FormField": {
                "props": [
                    "name",
                    "label",
                    "description",
                    "required",
                    "disabled",
                    "field_type",
                    "class",
                    "children"
                ],
                "stability": "stable"
            },
            "FormSubmit": {
                "props": [
                    "form_handle",
                    "on_submit",
                    "class",
                    "disabled_class",
                    "loading_text",
                    "children"
                ],
                "stability": "stable"
            },
            "FormReset": {
                "props": [
                    "form_handle",
                    "class",
                    "confirm_message",
                    "children"
                ],
                "stability": "stable"
            }
        }
    })
}

/// Validate API schema against baseline
pub fn validate_schema_against_baseline(
    current_schema: &serde_json::Value,
    baseline_schema: &serde_json::Value,
) -> SchemaValidationResult {
    let mut result = SchemaValidationResult::new();

    // Check version compatibility
    let current_version = current_schema["version"].as_str().unwrap_or("unknown");
    let baseline_version = baseline_schema["version"].as_str().unwrap_or("unknown");

    if current_version != baseline_version {
        result.add_breaking_change(format!(
            "Version changed from {} to {}",
            baseline_version, current_version
        ));
    }

    // Check traits
    if let (Some(current_traits), Some(baseline_traits)) = (
        current_schema["traits"].as_object(),
        baseline_schema["traits"].as_object(),
    ) {
        for (trait_name, baseline_trait) in baseline_traits {
            if let Some(current_trait) = current_traits.get(trait_name) {
                validate_trait_contract(trait_name, current_trait, baseline_trait, &mut result);
            } else {
                result.add_breaking_change(format!("Trait {} was removed", trait_name));
            }
        }

        // Check for new traits
        for trait_name in current_traits.keys() {
            if !baseline_traits.contains_key(trait_name) {
                result.add_new_feature(format!("New trait {} was added", trait_name));
            }
        }
    }

    // Check types
    if let (Some(current_types), Some(baseline_types)) = (
        current_schema["types"].as_object(),
        baseline_schema["types"].as_object(),
    ) {
        for (type_name, baseline_type) in baseline_types {
            if let Some(current_type) = current_types.get(type_name) {
                validate_type_contract(type_name, current_type, baseline_type, &mut result);
            } else {
                result.add_breaking_change(format!("Type {} was removed", type_name));
            }
        }

        // Check for new types
        for type_name in current_types.keys() {
            if !baseline_types.contains_key(type_name) {
                result.add_new_feature(format!("New type {} was added", type_name));
            }
        }
    }

    result
}

fn validate_trait_contract(
    trait_name: &str,
    current_trait: &serde_json::Value,
    baseline_trait: &serde_json::Value,
    result: &mut SchemaValidationResult,
) {
    // Check required methods
    if let (Some(current_methods), Some(baseline_methods)) = (
        current_trait["required_methods"].as_array(),
        baseline_trait["required_methods"].as_array(),
    ) {
        for method in baseline_methods {
            if let Some(method_name) = method.as_str() {
                if !current_methods
                    .iter()
                    .any(|m| m.as_str() == Some(method_name))
                {
                    result.add_breaking_change(format!(
                        "Required method {}.{} was removed",
                        trait_name, method_name
                    ));
                }
            }
        }
    }

    // Check method signatures
    if let (Some(current_sigs), Some(baseline_sigs)) = (
        current_trait["signatures"].as_object(),
        baseline_trait["signatures"].as_object(),
    ) {
        for (method_name, baseline_sig) in baseline_sigs {
            if let Some(current_sig) = current_sigs.get(method_name) {
                if current_sig != baseline_sig {
                    result.add_breaking_change(format!(
                        "Method signature changed for {}.{}: {} -> {}",
                        trait_name, method_name, baseline_sig, current_sig
                    ));
                }
            }
        }
    }
}

fn validate_type_contract(
    type_name: &str,
    current_type: &serde_json::Value,
    baseline_type: &serde_json::Value,
    result: &mut SchemaValidationResult,
) {
    // Check fields
    if let (Some(current_fields), Some(baseline_fields)) = (
        current_type["fields"].as_array(),
        baseline_type["fields"].as_array(),
    ) {
        for field in baseline_fields {
            if let Some(field_name) = field.as_str() {
                if !current_fields
                    .iter()
                    .any(|f| f.as_str() == Some(field_name))
                {
                    result.add_breaking_change(format!(
                        "Field {} was removed from type {}",
                        field_name, type_name
                    ));
                }
            }
        }
    }

    // Check methods
    if let (Some(current_methods), Some(baseline_methods)) = (
        current_type["methods"].as_array(),
        baseline_type["methods"].as_array(),
    ) {
        for method in baseline_methods {
            if let Some(method_name) = method.as_str() {
                if !current_methods
                    .iter()
                    .any(|m| m.as_str() == Some(method_name))
                {
                    result.add_breaking_change(format!(
                        "Method {} was removed from type {}",
                        method_name, type_name
                    ));
                }
            }
        }
    }

    // Check variants
    if let (Some(current_variants), Some(baseline_variants)) = (
        current_type["variants"].as_array(),
        baseline_type["variants"].as_array(),
    ) {
        for variant in baseline_variants {
            if let Some(variant_name) = variant.as_str() {
                if !current_variants
                    .iter()
                    .any(|v| v.as_str() == Some(variant_name))
                {
                    result.add_breaking_change(format!(
                        "Variant {} was removed from type {}",
                        variant_name, type_name
                    ));
                }
            }
        }
    }
}

/// Schema validation result
#[derive(Debug, Clone)]
pub struct SchemaValidationResult {
    pub breaking_changes: Vec<String>,
    pub new_features: Vec<String>,
    pub warnings: Vec<String>,
}

impl SchemaValidationResult {
    pub fn new() -> Self {
        Self {
            breaking_changes: Vec::new(),
            new_features: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_breaking_change(&mut self, change: String) {
        self.breaking_changes.push(change);
    }

    pub fn add_new_feature(&mut self, feature: String) {
        self.new_features.push(feature);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn is_breaking(&self) -> bool {
        !self.breaking_changes.is_empty()
    }

    pub fn has_changes(&self) -> bool {
        !self.breaking_changes.is_empty() || !self.new_features.is_empty()
    }

    pub fn summary(&self) -> String {
        let mut summary = String::new();

        if !self.breaking_changes.is_empty() {
            summary.push_str(&format!(
                "Breaking changes: {}\n",
                self.breaking_changes.len()
            ));
            for change in &self.breaking_changes {
                summary.push_str(&format!("  - {}\n", change));
            }
        }

        if !self.new_features.is_empty() {
            summary.push_str(&format!("New features: {}\n", self.new_features.len()));
            for feature in &self.new_features {
                summary.push_str(&format!("  - {}\n", feature));
            }
        }

        if !self.warnings.is_empty() {
            summary.push_str(&format!("Warnings: {}\n", self.warnings.len()));
            for warning in &self.warnings {
                summary.push_str(&format!("  - {}\n", warning));
            }
        }

        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_api_schema() {
        let schema = generate_api_schema();

        // Verify schema structure
        assert!(schema["version"].is_string());
        assert!(schema["traits"].is_object());
        assert!(schema["types"].is_object());
        assert!(schema["hooks"].is_object());
        assert!(schema["components"].is_object());

        // Verify Form trait exists
        assert!(schema["traits"]["Form"].is_object());
        assert!(schema["traits"]["Form"]["required_methods"].is_array());
        assert!(schema["traits"]["Form"]["signatures"].is_object());

        // Verify required methods
        let required_methods = schema["traits"]["Form"]["required_methods"]
            .as_array()
            .unwrap();
        assert!(required_methods
            .iter()
            .any(|m| m.as_str() == Some("field_metadata")));
        assert!(required_methods
            .iter()
            .any(|m| m.as_str() == Some("validate")));
        assert!(required_methods
            .iter()
            .any(|m| m.as_str() == Some("default_values")));
    }

    #[test]
    fn test_schema_validation() {
        let baseline = generate_api_schema();
        let current = generate_api_schema();

        let result = validate_schema_against_baseline(&current, &baseline);

        // Should be no breaking changes for identical schemas
        assert!(!result.is_breaking());
        assert!(!result.has_changes());
    }

    #[test]
    fn test_schema_validation_result() {
        let mut result = SchemaValidationResult::new();

        result.add_breaking_change("Test breaking change".to_string());
        result.add_new_feature("Test new feature".to_string());
        result.add_warning("Test warning".to_string());

        assert!(result.is_breaking());
        assert!(result.has_changes());
        assert_eq!(result.breaking_changes.len(), 1);
        assert_eq!(result.new_features.len(), 1);
        assert_eq!(result.warnings.len(), 1);

        let summary = result.summary();
        assert!(summary.contains("Breaking changes: 1"));
        assert!(summary.contains("New features: 1"));
        assert!(summary.contains("Warnings: 1"));
    }
}
