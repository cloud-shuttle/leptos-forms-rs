//! API Contract Testing for leptos-forms-rs
//!
//! This module provides comprehensive API contract testing to ensure
//! backward compatibility and API stability across versions.

pub mod basic_contract_tests;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// API Schema definition for contract testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSchema {
    pub version: String,
    pub traits: HashMap<String, TraitSchema>,
    pub types: HashMap<String, TypeSchema>,
    pub breaking_changes: Vec<BreakingChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitSchema {
    pub required_methods: Vec<String>,
    pub signatures: HashMap<String, String>,
    pub stability: ApiStability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeSchema {
    pub fields: Vec<String>,
    pub methods: Vec<String>,
    pub stability: ApiStability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiStability {
    Stable,
    Experimental,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakingChange {
    pub description: String,
    pub affected_api: String,
    pub migration_guide: String,
}

/// Contract test results
#[derive(Debug, Clone)]
pub struct ContractTestResults {
    pub passed: Vec<String>,
    pub failed: Vec<(String, String)>,
    pub total: usize,
}

impl Default for ContractTestResults {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractTestResults {
    pub fn new() -> Self {
        Self {
            passed: Vec::new(),
            failed: Vec::new(),
            total: 0,
        }
    }

    pub fn add_success(&mut self, test_name: String) {
        self.passed.push(test_name);
        self.total += 1;
    }

    pub fn add_failure(&mut self, test_name: String, error: String) {
        self.failed.push((test_name, error));
        self.total += 1;
    }

    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.passed.len() as f64 / self.total as f64
        }
    }

    pub fn is_success(&self) -> bool {
        self.failed.is_empty()
    }
}

/// Contract violation types
#[derive(Debug, Clone)]
pub enum ContractViolation {
    BreakingChange(BreakingChange),
    MissingMethod(String),
    SignatureMismatch(String, String, String), // method, expected, actual
    TypeMismatch(String, String, String),      // type, expected, actual
}

/// Contract monitoring for detecting violations
pub struct ContractMonitor {
    baseline_schema: ApiSchema,
    current_schema: ApiSchema,
}

impl ContractMonitor {
    pub fn new(baseline: ApiSchema, current: ApiSchema) -> Self {
        Self {
            baseline_schema: baseline,
            current_schema: current,
        }
    }

    pub fn detect_violations(&self) -> Vec<ContractViolation> {
        let mut violations = Vec::new();

        // Check for breaking changes in traits
        for (trait_name, baseline_trait) in &self.baseline_schema.traits {
            if let Some(current_trait) = self.current_schema.traits.get(trait_name) {
                // Check for missing methods
                for method in &baseline_trait.required_methods {
                    if !current_trait.required_methods.contains(method) {
                        violations.push(ContractViolation::MissingMethod(format!(
                            "{}.{}",
                            trait_name, method
                        )));
                    }
                }

                // Check for signature changes
                for (method, expected_sig) in &baseline_trait.signatures {
                    if let Some(actual_sig) = current_trait.signatures.get(method) {
                        if expected_sig != actual_sig {
                            violations.push(ContractViolation::SignatureMismatch(
                                format!("{}.{}", trait_name, method),
                                expected_sig.clone(),
                                actual_sig.clone(),
                            ));
                        }
                    }
                }
            }
        }

        violations
    }

    pub fn should_fail_build(&self) -> bool {
        !self.detect_violations().is_empty()
    }
}

/// Generate current API schema
pub fn generate_current_schema() -> ApiSchema {
    ApiSchema {
        version: "1.1.0".to_string(),
        traits: {
            let mut traits = HashMap::new();

            // Form trait schema
            traits.insert(
                "Form".to_string(),
                TraitSchema {
                    required_methods: vec![
                        "field_metadata".to_string(),
                        "validate".to_string(),
                        "default_values".to_string(),
                        "schema".to_string(),
                        "get_field_value".to_string(),
                        "set_field_value".to_string(),
                    ],
                    signatures: {
                        let mut sigs = HashMap::new();
                        sigs.insert(
                            "field_metadata".to_string(),
                            "fn() -> Vec<FieldMetadata>".to_string(),
                        );
                        sigs.insert(
                            "validate".to_string(),
                            "fn(&self) -> Result<(), ValidationErrors>".to_string(),
                        );
                        sigs.insert("default_values".to_string(), "fn() -> Self".to_string());
                        sigs.insert("schema".to_string(), "fn() -> FormSchema".to_string());
                        sigs.insert(
                            "get_field_value".to_string(),
                            "fn(&self, field_name: &str) -> FieldValue".to_string(),
                        );
                        sigs.insert(
                            "set_field_value".to_string(),
                            "fn(&mut self, field_name: &str, value: FieldValue)".to_string(),
                        );
                        sigs
                    },
                    stability: ApiStability::Stable,
                },
            );

            traits
        },
        types: {
            let mut types = HashMap::new();

            // FieldMetadata type schema
            types.insert(
                "FieldMetadata".to_string(),
                TypeSchema {
                    fields: vec![
                        "name".to_string(),
                        "field_type".to_string(),
                        "is_required".to_string(),
                        "default_value".to_string(),
                        "dependencies".to_string(),
                        "attributes".to_string(),
                        "validators".to_string(),
                    ],
                    methods: Vec::new(),
                    stability: ApiStability::Stable,
                },
            );

            // FormSchema type schema
            types.insert(
                "FormSchema".to_string(),
                TypeSchema {
                    fields: vec!["name".to_string(), "field_metadata".to_string()],
                    methods: vec!["new".to_string(), "get_field".to_string()],
                    stability: ApiStability::Stable,
                },
            );

            // ValidationErrors type schema
            types.insert(
                "ValidationErrors".to_string(),
                TypeSchema {
                    fields: Vec::new(),
                    methods: vec![
                        "new".to_string(),
                        "is_empty".to_string(),
                        "has_field_error".to_string(),
                        "add_field_error".to_string(),
                    ],
                    stability: ApiStability::Stable,
                },
            );

            types
        },
        breaking_changes: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_generation() {
        let schema = generate_current_schema();
        assert_eq!(schema.version, "1.1.0");
        assert!(schema.traits.contains_key("Form"));
        assert!(schema.types.contains_key("FieldMetadata"));
        assert!(schema.types.contains_key("FormSchema"));
        assert!(schema.types.contains_key("ValidationErrors"));
    }

    #[test]
    fn test_contract_monitor() {
        let baseline = generate_current_schema();
        let current = generate_current_schema();

        let monitor = ContractMonitor::new(baseline, current);
        let violations = monitor.detect_violations();

        assert!(violations.is_empty());
        assert!(!monitor.should_fail_build());
    }

    #[test]
    fn test_contract_test_results() {
        let mut results = ContractTestResults::new();

        results.add_success("test1".to_string());
        results.add_success("test2".to_string());
        results.add_failure("test3".to_string(), "error".to_string());

        assert_eq!(results.total, 3);
        assert_eq!(results.passed.len(), 2);
        assert_eq!(results.failed.len(), 1);
        assert!((results.success_rate() - 0.6666666666666666).abs() < 0.001);
        assert!(!results.is_success());
    }
}
