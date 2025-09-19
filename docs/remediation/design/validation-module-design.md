# Validation Module Refactoring Design

## 📋 Overview

**Current State**: `validation/mod.rs` (934 lines) - Critical file exceeding target by 3x
**Target State**: 6 focused modules, each under 300 lines
**Priority**: P0 - Critical (affects entire validation system)

## 🎯 Refactoring Goals

1. **Split monolithic validation module** into focused, single-responsibility modules
2. **Maintain API compatibility** - all existing public interfaces preserved
3. **Improve testability** - each module can be tested independently
4. **Enhance maintainability** - easier to understand and modify specific validation logic

## 🏗️ New Module Structure

```
validation/
├── mod.rs                 # Re-exports and module organization (~50 lines)
├── errors.rs              # ValidationErrors and error handling (~150 lines)
├── validators.rs          # Built-in validator implementations (~200 lines)
├── engine.rs              # ValidationRuleEngine core logic (~180 lines)
├── conditional.rs         # ConditionalValidator and FieldCondition (~160 lines)
└── rules.rs               # Validator enum and validation rules (~120 lines)
```

## 📝 Module Specifications

### 1. `mod.rs` (Re-exports)

**Target Size**: ~50 lines
**Purpose**: Module organization and public API

```rust
//! Validation module - Type-safe form validation for Leptos Forms
//!
//! This module provides comprehensive validation capabilities including:
//! - Built-in validators (email, URL, patterns, etc.)
//! - Custom validation rules
//! - Conditional validation
//! - Validation error handling

pub mod errors;
pub mod validators;
pub mod engine;
pub mod conditional;
pub mod rules;

// Re-export public API
pub use errors::*;
pub use validators::*;
pub use engine::*;
pub use conditional::*;
pub use rules::*;
```

### 2. `errors.rs` (ValidationErrors)

**Target Size**: ~150 lines
**Purpose**: Error handling and validation error types

**Key Components**:

- `ValidationErrors` struct
- Error aggregation and merging
- Field error management
- Error display formatting
- Error conversion utilities

**Public API**:

```rust
pub struct ValidationErrors { ... }
impl ValidationErrors {
    pub fn new() -> Self
    pub fn add_field_error(&mut self, field: &str, error: String)
    pub fn add_form_error(&mut self, error: String)
    pub fn is_empty(&self) -> bool
    pub fn has_errors(&self) -> bool
    pub fn merge(&mut self, other: ValidationErrors)
    // ... other methods
}
```

### 3. `validators.rs` (Built-in Validators)

**Target Size**: ~200 lines
**Purpose**: Implementation of built-in validation functions

**Key Components**:

- Email validation
- URL validation
- Length validation (min/max)
- Pattern validation
- Number validation (min/max/range)
- Custom validator implementations
- Luhn algorithm for credit cards

**Public API**:

```rust
pub struct Validators;

impl Validators {
    pub fn required(value: &FieldValue) -> Result<(), String>
    pub fn email(value: &FieldValue) -> Result<(), String>
    pub fn url(value: &FieldValue) -> Result<(), String>
    pub fn min_length(value: &FieldValue, min: usize) -> Result<(), String>
    pub fn max_length(value: &FieldValue, max: usize) -> Result<(), String>
    pub fn pattern(value: &FieldValue, pattern: &str) -> Result<(), String>
    pub fn phone(value: &FieldValue) -> Result<(), String>
    pub fn credit_card(value: &FieldValue) -> Result<(), String>
    // ... other validators
}
```

### 4. `engine.rs` (ValidationRuleEngine)

**Target Size**: ~180 lines
**Purpose**: Core validation engine logic

**Key Components**:

- `ValidationRuleEngine` struct
- Validator registration
- Field validation execution
- Error collection and aggregation
- Built-in validator registration

**Public API**:

```rust
pub struct ValidationRuleEngine { ... }

impl ValidationRuleEngine {
    pub fn new() -> Self
    pub fn register_validator(&mut self, name: &str, validator: FieldValidator)
    pub fn validate_value(&self, value: FieldValue) -> Result<(), ValidationErrors>
    pub fn validate_field(&self, field_name: &str, value: &FieldValue, validators: &[Validator]) -> Vec<String>
    // ... other methods
}
```

### 5. `conditional.rs` (Conditional Validation)

**Target Size**: ~160 lines
**Purpose**: Conditional validation logic

**Key Components**:

- `ConditionalValidator` struct
- `FieldCondition` enum
- `ConditionalRule` struct
- Condition evaluation logic
- Conditional validation execution

**Public API**:

```rust
pub struct ConditionalValidator { ... }
pub enum FieldCondition { ... }
pub struct ConditionalRule { ... }

impl ConditionalValidator {
    pub fn new() -> Self
    pub fn add_rule(&mut self, rule: ConditionalRule)
    pub fn validate_conditional_fields<T: Form>(&self, form: &T, field_name: &str, field_value: &FieldValue) -> Result<(), String>
    // ... other methods
}
```

### 6. `rules.rs` (Validator Rules)

**Target Size**: ~120 lines
**Purpose**: Validator enum and rule definitions

**Key Components**:

- `Validator` enum
- `FieldValidator` type alias
- Rule validation logic
- Validator configuration

**Public API**:

```rust
pub type FieldValidator = Box<dyn Fn(&FieldValue) -> Result<(), String> + Send + Sync>;

pub enum Validator {
    Required,
    Email,
    Url,
    MinLength(usize),
    MaxLength(usize),
    Min(f64),
    Max(f64),
    Pattern(String),
    Range(f64, f64),
    Custom(String),
}
```

## 🔄 Migration Strategy

### Step 1: Create New Module Structure

1. Create new module files with empty implementations
2. Move type definitions to appropriate modules
3. Update imports in `mod.rs`

### Step 2: Move Implementation Code

1. Move `ValidationErrors` to `errors.rs`
2. Move built-in validators to `validators.rs`
3. Move `ValidationRuleEngine` to `engine.rs`
4. Move conditional validation to `conditional.rs`
5. Move `Validator` enum to `rules.rs`

### Step 3: Update Dependencies

1. Update imports throughout codebase
2. Ensure all tests still pass
3. Update documentation

### Step 4: Validation

1. Run full test suite
2. Verify API compatibility
3. Check file sizes meet targets

## 🧪 Testing Strategy

### Unit Tests

- Each module gets its own test file
- Test all public APIs
- Test error conditions
- Test edge cases

### Integration Tests

- Test modules work together
- Test validation flows end-to-end
- Test error aggregation across modules

### Compatibility Tests

- Ensure existing code still works
- Verify public API hasn't changed
- Test migration scenarios

## 📊 Success Criteria

### File Size Compliance

- ✅ All modules under 300 lines
- ✅ `mod.rs` under 100 lines
- ✅ Largest module under 250 lines

### Functionality

- ✅ All existing tests pass
- ✅ No breaking changes to public API
- ✅ Performance maintained or improved
- ✅ New integration tests added

### Code Quality

- ✅ Clear separation of concerns
- ✅ Single responsibility per module
- ✅ Consistent naming conventions
- ✅ Comprehensive documentation

## 🔍 Implementation Notes

### Dependencies

- `errors.rs` - No internal dependencies
- `validators.rs` - Depends on `errors.rs`
- `engine.rs` - Depends on `errors.rs`, `validators.rs`, `rules.rs`
- `conditional.rs` - Depends on `errors.rs`, `rules.rs`
- `rules.rs` - No internal dependencies
- `mod.rs` - Re-exports all modules

### Error Handling

- Maintain existing error types
- Preserve error message formats
- Keep error aggregation behavior

### Performance Considerations

- Minimize allocations in hot paths
- Cache compiled regex patterns
- Optimize validation loops

## 📚 References

- [Original validation/mod.rs](../../../leptos-forms-rs/src/validation/mod.rs)
- [Validation Tests](../../../tests/unit/src/validation_engine_tests.rs)
- [Validation Rules Tests](../../../tests/unit/src/validation_rules_engine_tests.rs)
