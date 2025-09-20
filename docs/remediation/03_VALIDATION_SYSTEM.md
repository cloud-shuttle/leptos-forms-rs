# Remediation Plan 03: Validation System Overhaul

**Priority**: HIGH
**Estimated Effort**: 3-4 days
**Blockers**: 02_CORE_COMPONENTS must be complete
**Dependencies**: Working components, helper functions

## Overview

Fix performance issues and implement missing validation features in the validation engine.

## Current Problems

### Performance Issues

**File**: `leptos-forms-rs/src/validation/engine.rs` (419 lines → needs splitting)

- `ValidationRuleEngine` recreated on every validation call
- Excessive cloning and allocation
- No caching of compiled validators

### Missing Features

- Async validation support
- Cross-field validation
- Conditional validation rules
- Custom validation messages
- Validation rule composition

## Solution Architecture

### Split validation engine into focused modules

```
leptos-forms-rs/src/validation/
├── engine/
│   ├── mod.rs (< 100 lines)
│   ├── sync_validator.rs (< 200 lines)
│   ├── async_validator.rs (< 200 lines)
│   └── cache.rs (< 150 lines)
├── rules/
│   ├── mod.rs (< 50 lines)
│   ├── built_in.rs (< 250 lines)
│   ├── custom.rs (< 150 lines)
│   └── cross_field.rs (< 200 lines)
├── errors.rs (< 200 lines)
└── mod.rs (< 100 lines)
```

## Implementation Plan

### 1. Performance Optimizations

**Files**: `validation/engine/cache.rs`, `validation/engine/sync_validator.rs`

```rust
// Cached validation engine
pub struct CachedValidationEngine {
    rules: HashMap<String, CompiledRule>,
    regex_cache: HashMap<String, Regex>,
}

impl CachedValidationEngine {
    pub fn new() -> Self { /* ... */ }

    pub fn validate_field(&self, field: &str, value: &FieldValue) -> ValidationResult {
        // Use cached compiled rules
    }

    pub fn validate_form<T>(&self, form: &T) -> ValidationResult
    where T: Form { /* ... */ }
}
```

### 2. Async Validation Support

**File**: `validation/engine/async_validator.rs`

```rust
#[async_trait(?Send)]
pub trait AsyncValidator {
    async fn validate(&self, value: &FieldValue) -> ValidationResult;
}

pub struct AsyncValidationEngine {
    validators: HashMap<String, Box<dyn AsyncValidator>>,
}
```

### 3. Cross-Field Validation

**File**: `validation/rules/cross_field.rs`

```rust
pub trait CrossFieldValidator {
    fn validate_fields(&self, form_data: &HashMap<String, FieldValue>) -> ValidationResult;
}

// Example: Password confirmation
pub struct PasswordConfirmationValidator {
    password_field: String,
    confirm_field: String,
}
```

### 4. Validation Rule Builder

**File**: `validation/rules/custom.rs`

```rust
pub struct ValidationRuleBuilder {
    rules: Vec<Box<dyn Validator>>,
}

impl ValidationRuleBuilder {
    pub fn required(mut self) -> Self { /* ... */ }
    pub fn min_length(mut self, len: usize) -> Self { /* ... */ }
    pub fn custom<F>(mut self, validator: F) -> Self
    where F: Fn(&FieldValue) -> ValidationResult + 'static { /* ... */ }
    pub fn build(self) -> ValidationRule { /* ... */ }
}
```

## Testing Strategy

### Performance Tests

- Benchmark validation speed with/without caching
- Memory usage analysis
- Concurrent validation testing

### Functional Tests

- All built-in validators with edge cases
- Cross-field validation scenarios
- Async validation with simulated network delays
- Custom validator integration

### Integration Tests

- Form-level validation with mixed sync/async rules
- Error message customization
- Validation rule composition

## Acceptance Criteria

### Performance

- [ ] Validation engine reuses compiled rules
- [ ] <1ms validation time for simple forms (<10 fields)
- [ ] <10ms validation time for complex forms (<50 fields)
- [ ] Memory usage stable under repeated validation

### Features

- [ ] Async validation works with proper loading states
- [ ] Cross-field validation (e.g., password confirmation)
- [ ] Custom validation rules can be easily added
- [ ] Validation error messages are customizable
- [ ] Conditional validation based on other field values

### Code Quality

- [ ] All files under 300 lines
- [ ] Clear separation of sync vs async validation
- [ ] Comprehensive error handling
- [ ] Thread-safe validation engine

## Example Usage

```rust
// Simple validation
let validator = ValidationRuleBuilder::new()
    .required()
    .min_length(8)
    .custom(|value| {
        if value.as_string().contains("password") {
            Err("Password cannot contain 'password'".into())
        } else {
            Ok(())
        }
    })
    .build();

// Cross-field validation
let password_match = CrossFieldValidator::new()
    .fields(&["password", "password_confirm"])
    .rule(|values| {
        if values["password"] != values["password_confirm"] {
            Err("Passwords must match".into())
        } else {
            Ok(())
        }
    });
```

## Next Steps

After completion, proceed to `04_TESTING_INFRASTRUCTURE.md`
