# API Contract Testing Strategy

## Overview

This document outlines the API contract testing strategy for leptos-forms-rs, ensuring backward compatibility, API stability, and consumer confidence.

## 1. API Contract Definition

### 1.1 Core API Contracts

#### Form Trait Contract

```rust
/// Core form trait - MUST maintain backward compatibility
pub trait Form: for<'de> Deserialize<'de> + Serialize + Clone + 'static {
    /// REQUIRED: Get field metadata
    fn field_metadata() -> Vec<FieldMetadata>;

    /// REQUIRED: Validate form data
    fn validate(&self) -> Result<(), ValidationErrors>;

    /// REQUIRED: Get default values
    fn default_values() -> Self;

    /// REQUIRED: Get form schema
    fn schema() -> FormSchema;

    /// REQUIRED: Get field value by name
    fn get_field_value(&self, field_name: &str) -> FieldValue;

    /// REQUIRED: Set field value by name
    fn set_field_value(&mut self, field_name: &str, value: FieldValue);
}
```

#### FormHandle Contract

```rust
/// Form handle interface - MUST maintain backward compatibility
pub struct FormHandle<T: Form> {
    // Public API surface that consumers depend on
    pub fn new(form: T) -> Self;
    pub fn validate(&self) -> Result<(), ValidationErrors>;
    pub fn set_field_value(&self, field_name: &str, value: FieldValue);
    pub fn get_field_value(&self, field_name: &str) -> FieldValue;
    pub fn is_valid(&self) -> Memo<bool>;
    pub fn is_dirty(&self) -> Memo<bool>;
    pub fn is_submitting(&self) -> Memo<bool>;
}
```

### 1.2 Breaking Change Policy

#### Major Version (X.0.0)

- Breaking changes to trait signatures
- Removal of public APIs
- Changes to core data structures

#### Minor Version (0.X.0)

- New optional parameters
- New methods
- New types (non-breaking)

#### Patch Version (0.0.X)

- Bug fixes only
- No API changes

## 2. Contract Testing Implementation

### 2.1 API Schema Generation

```rust
// tests/contracts/api_schema.rs
use leptos_forms_rs::core::*;
use serde_json;

/// Generate API schema for contract testing
pub fn generate_api_schema() -> serde_json::Value {
    let schema = serde_json::json!({
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
                }
            }
        },
        "types": {
            "FieldMetadata": {
                "fields": ["name", "field_type", "is_required", "default_value", "dependencies", "attributes", "validators"]
            },
            "FormSchema": {
                "fields": ["name", "field_metadata"]
            },
            "ValidationErrors": {
                "methods": ["is_empty", "has_field_error", "add_field_error"]
            }
        }
    });
    schema
}
```

### 2.2 Contract Validation Tests

```rust
// tests/contracts/contract_validation.rs
use leptos_forms_rs::core::*;
use serde_json;

#[test]
fn test_form_trait_contract() {
    // Test that Form trait maintains required methods
    let test_form = TestContractForm::default();

    // Verify required methods exist and work
    let metadata = TestContractForm::field_metadata();
    assert!(!metadata.is_empty());

    let validation = test_form.validate();
    // Should not panic, regardless of validation result

    let defaults = TestContractForm::default_values();
    assert_eq!(defaults, TestContractForm::default());

    let schema = TestContractForm::schema();
    assert!(!schema.name.is_empty());

    let field_value = test_form.get_field_value("test_field");
    // Should return a valid FieldValue

    let mut mutable_form = test_form.clone();
    mutable_form.set_field_value("test_field", FieldValue::String("test".to_string()));
    // Should not panic
}

#[test]
fn test_form_handle_contract() {
    let form = TestContractForm::default();
    let handle = FormHandle::new(form);

    // Test required FormHandle methods
    let _validation = handle.validate();
    let _is_valid = handle.is_valid();
    let _is_dirty = handle.is_dirty();
    let _is_submitting = handle.is_submitting();

    handle.set_field_value("test_field", FieldValue::String("test".to_string()));
    let _field_value = handle.get_field_value("test_field");
}

#[test]
fn test_backward_compatibility() {
    // Test that old API usage still works
    let form = LegacyForm::default();
    let handle = FormHandle::new(form);

    // These should work exactly as they did in previous versions
    let result = handle.validate();
    assert!(result.is_err()); // Empty form should be invalid

    handle.set_field_value("name", FieldValue::String("Test".to_string()));
    let result = handle.validate();
    // Should work without breaking changes
}
```

### 2.3 Consumer Contract Tests

```rust
// tests/contracts/consumer_contracts.rs
use leptos_forms_rs::*;

/// Test that consumers can implement the Form trait
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ConsumerForm {
    name: String,
    email: String,
}

impl Form for ConsumerForm {
    fn field_metadata() -> Vec<FieldMetadata> {
        vec![
            FieldMetadata {
                name: "name".to_string(),
                field_type: FieldType::Text,
                is_required: true,
                default_value: Some(FieldValue::String(String::new())),
                dependencies: Vec::new(),
                attributes: std::collections::HashMap::new(),
                validators: vec![Validator::Required],
            },
            FieldMetadata {
                name: "email".to_string(),
                field_type: FieldType::Email,
                is_required: true,
                default_value: Some(FieldValue::String(String::new())),
                dependencies: Vec::new(),
                attributes: std::collections::HashMap::new(),
                validators: vec![Validator::Required, Validator::Email],
            },
        ]
    }

    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.name.is_empty() {
            errors.add_field_error("name", "Name is required".to_string());
        }

        if self.email.is_empty() {
            errors.add_field_error("email", "Email is required".to_string());
        } else if !self.email.contains('@') {
            errors.add_field_error("email", "Invalid email format".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn default_values() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
        }
    }

    fn get_field_value(&self, field_name: &str) -> FieldValue {
        match field_name {
            "name" => FieldValue::String(self.name.clone()),
            "email" => FieldValue::String(self.email.clone()),
            _ => FieldValue::String(String::new()),
        }
    }

    fn set_field_value(&mut self, field_name: &str, value: FieldValue) {
        match field_name {
            "name" => {
                if let FieldValue::String(s) = value {
                    self.name = s;
                }
            }
            "email" => {
                if let FieldValue::String(s) = value {
                    self.email = s;
                }
            }
            _ => {}
        }
    }
}

#[test]
fn test_consumer_can_implement_form() {
    let form = ConsumerForm::default();
    let handle = FormHandle::new(form);

    // Consumer should be able to use all FormHandle functionality
    let result = handle.validate();
    assert!(result.is_err()); // Empty form should be invalid

    handle.set_field_value("name", FieldValue::String("John Doe".to_string()));
    handle.set_field_value("email", FieldValue::String("john@example.com".to_string()));

    let result = handle.validate();
    assert!(result.is_ok()); // Valid form should pass
}
```

## 3. Automated Contract Testing

### 3.1 CI/CD Integration

```yaml
# .github/workflows/contract-tests.yml
name: API Contract Tests

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  contract-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run Contract Tests
        run: |
          cargo test --package leptos-forms-rs contract

      - name: Generate API Schema
        run: |
          cargo test --package leptos-forms-rs generate_api_schema

      - name: Validate Schema Changes
        run: |
          # Compare with previous schema
          # Fail if breaking changes detected
```

### 3.2 Schema Versioning

```rust
// src/contracts/schema_version.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSchema {
    pub version: String,
    pub traits: std::collections::HashMap<String, TraitSchema>,
    pub types: std::collections::HashMap<String, TypeSchema>,
    pub breaking_changes: Vec<BreakingChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakingChange {
    pub description: String,
    pub affected_api: String,
    pub migration_guide: String,
}

impl ApiSchema {
    pub fn compare_with(&self, other: &ApiSchema) -> SchemaComparison {
        // Compare schemas and detect breaking changes
        SchemaComparison {
            breaking_changes: Vec::new(),
            new_features: Vec::new(),
            deprecated_apis: Vec::new(),
        }
    }
}
```

## 4. Contract Testing Tools

### 4.1 Custom Contract Testing Framework

```rust
// tests/contracts/framework.rs
use std::collections::HashMap;

pub struct ContractTester {
    schema: ApiSchema,
    test_cases: Vec<ContractTestCase>,
}

pub struct ContractTestCase {
    pub name: String,
    pub description: String,
    pub test_fn: Box<dyn Fn() -> Result<(), String>>,
}

impl ContractTester {
    pub fn new() -> Self {
        Self {
            schema: ApiSchema::current(),
            test_cases: Vec::new(),
        }
    }

    pub fn add_test_case(&mut self, test_case: ContractTestCase) {
        self.test_cases.push(test_case);
    }

    pub fn run_all_tests(&self) -> ContractTestResults {
        let mut results = ContractTestResults::new();

        for test_case in &self.test_cases {
            match (test_case.test_fn)() {
                Ok(()) => results.add_success(test_case.name.clone()),
                Err(error) => results.add_failure(test_case.name.clone(), error),
            }
        }

        results
    }
}
```

## 5. Monitoring and Alerts

### 5.1 Contract Violation Detection

```rust
// src/contracts/monitoring.rs
pub struct ContractMonitor {
    baseline_schema: ApiSchema,
    current_schema: ApiSchema,
}

impl ContractMonitor {
    pub fn detect_violations(&self) -> Vec<ContractViolation> {
        let comparison = self.baseline_schema.compare_with(&self.current_schema);

        comparison.breaking_changes.into_iter()
            .map(|change| ContractViolation::BreakingChange(change))
            .collect()
    }

    pub fn should_fail_build(&self) -> bool {
        !self.detect_violations().is_empty()
    }
}
```

## 6. Implementation Plan

### Phase 1: Basic Contract Testing (Week 1-2)

- [ ] Implement basic contract validation tests
- [ ] Create API schema generation
- [ ] Add consumer contract tests

### Phase 2: Automated Testing (Week 3-4)

- [ ] Integrate with CI/CD pipeline
- [ ] Add schema versioning
- [ ] Implement breaking change detection

### Phase 3: Advanced Features (Week 5-6)

- [ ] Add contract monitoring
- [ ] Implement migration guides
- [ ] Add performance contract testing

## 7. Benefits

1. **Backward Compatibility**: Ensures API changes don't break existing consumers
2. **Consumer Confidence**: Provides guarantees about API stability
3. **Early Detection**: Catches breaking changes before release
4. **Documentation**: Automatically generates API contracts
5. **Migration Support**: Helps consumers upgrade to new versions

## 8. Tools and Libraries

- **Custom Framework**: Built-in contract testing
- **Serde**: Schema serialization
- **Cargo**: Test integration
- **GitHub Actions**: CI/CD integration
- **JSON Schema**: Contract definition format
