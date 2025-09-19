# Types Module Refactoring Design

## 📋 Overview

**Current State**: `core/types.rs` (417 lines) - Critical file exceeding target by 1.4x
**Target State**: 6 focused modules, each under 300 lines
**Priority**: P0 - Critical (core types used throughout library)

## 🎯 Refactoring Goals

1. **Split monolithic types module** into focused, single-responsibility modules
2. **Maintain type safety** across the entire library
3. **Improve type organization** and discoverability
4. **Enhance type documentation** and usage examples

## 🏗️ New Module Structure

```
core/types/
├── mod.rs                 # Re-exports and module organization (~50 lines)
├── field_value.rs         # FieldValue enum and value operations (~120 lines)
├── field_type.rs          # FieldType enum and type definitions (~100 lines)
├── field_config.rs        # FieldConfig and field configuration (~80 lines)
├── submission.rs          # FormSubmissionResult and submission types (~70 lines)
└── options.rs             # ValidationMode, SubmissionMode, and options (~90 lines)
```

## 📝 Module Specifications

### 1. `mod.rs` (Re-exports)

**Target Size**: ~50 lines
**Purpose**: Module organization and public API

```rust
//! Core types module - Fundamental types for Leptos Forms
//!
//! This module provides:
//! - Field value representations
//! - Field type definitions
//! - Field configuration options
//! - Form submission results
//! - Validation and submission modes

pub mod field_value;
pub mod field_type;
pub mod field_config;
pub mod submission;
pub mod options;

// Re-export public API
pub use field_value::*;
pub use field_type::*;
pub use field_config::*;
pub use submission::*;
pub use options::*;

// Type aliases for convenience
pub type TrackingFunction = Box<dyn Fn(&str, &str, &str) + Send + Sync>;
```

### 2. `field_value.rs` (FieldValue Enum)

**Target Size**: ~120 lines
**Purpose**: Dynamic field value representation and operations

**Key Components**:

- `FieldValue` enum with all variants
- Value conversion methods
- Value validation helpers
- Display and Debug implementations
- Serialization support

**Public API**:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldValue {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Date(NaiveDate),
    DateTime(DateTime<Utc>),
    Array(Vec<FieldValue>),
    Object(HashMap<String, FieldValue>),
    File(FileData),
    Null,
}

impl FieldValue {
    pub fn as_string(&self) -> Option<&String>
    pub fn as_number(&self) -> Option<f64>
    pub fn as_boolean(&self) -> Option<bool>
    pub fn as_array(&self) -> Option<&Vec<FieldValue>>
    pub fn as_object(&self) -> Option<&HashMap<String, FieldValue>>
    pub fn is_null(&self) -> bool
    pub fn is_empty(&self) -> bool
}

impl std::fmt::Display for FieldValue { ... }
```

### 3. `field_type.rs` (FieldType Enum)

**Target Size**: ~100 lines
**Purpose**: Field type definitions and constraints

**Key Components**:

- `FieldType` enum with all variants
- `NumberType` struct for numeric constraints
- `SelectOption` struct for select fields
- `FileConstraints` struct for file uploads
- Type validation helpers

**Public API**:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    Text,
    Email,
    Password,
    Number(NumberType),
    Boolean,
    Select(Vec<SelectOption>),
    MultiSelect(Vec<SelectOption>),
    Date,
    DateTime,
    File(FileConstraints),
    RichText,
    Markdown,
    Code,
    Array(Box<FieldType>),
    Nested(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberType {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConstraints {
    pub max_size: Option<usize>,
    pub accept: Vec<String>,
    pub multiple: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileData {
    pub name: String,
    pub size: usize,
    pub mime_type: String,
    pub data: Vec<u8>,
}
```

### 4. `field_config.rs` (FieldConfig)

**Target Size**: ~80 lines
**Purpose**: Field configuration and attributes

**Key Components**:

- `FieldConfig` struct
- Configuration building methods
- Attribute management
- Field state options

**Public API**:

```rust
#[derive(Debug, Clone)]
pub struct FieldConfig {
    pub name: String,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
    pub disabled: bool,
    pub readonly: bool,
    pub hidden: bool,
    pub attributes: HashMap<String, String>,
}

impl FieldConfig {
    pub fn new(name: String) -> Self
    pub fn with_label(mut self, label: String) -> Self
    pub fn with_placeholder(mut self, placeholder: String) -> Self
    pub fn with_help_text(mut self, help_text: String) -> Self
    pub fn disabled(mut self) -> Self
    pub fn readonly(mut self) -> Self
    pub fn hidden(mut self) -> Self
    pub fn with_attribute(mut self, key: String, value: String) -> Self
}
```

### 5. `submission.rs` (FormSubmissionResult)

**Target Size**: ~70 lines
**Purpose**: Form submission results and handling

**Key Components**:

- `FormSubmissionResult` struct
- Success and failure constructors
- Warning management
- Result utilities

**Public API**:

```rust
#[derive(Debug, Clone)]
pub struct FormSubmissionResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub errors: Vec<FormError>,
    pub warnings: Vec<String>,
}

impl<T> FormSubmissionResult<T> {
    pub fn success(data: T) -> Self
    pub fn failure(errors: Vec<FormError>) -> Self
    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self
}
```

### 6. `options.rs` (Validation and Submission Options)

**Target Size**: ~90 lines
**Purpose**: Configuration options for validation and submission

**Key Components**:

- `ValidationMode` enum
- `SubmissionMode` enum
- `PersistenceOptions` struct
- `AnalyticsOptions` struct
- Option builders and defaults

**Public API**:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationMode {
    OnBlur,
    OnChange,
    OnSubmit,
    OnBlurAndChange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubmissionMode {
    Immediate,
    Manual,
    Debounced(std::time::Duration),
}

#[derive(Debug, Clone)]
pub struct PersistenceOptions {
    pub enabled: bool,
    pub storage_key: Option<String>,
    pub auto_save: bool,
    pub auto_save_interval: Option<std::time::Duration>,
    pub clear_on_submit: bool,
}

impl Default for PersistenceOptions { ... }

pub struct AnalyticsOptions {
    pub enabled: bool,
    pub track_field_interactions: bool,
    pub track_validation_errors: bool,
    pub track_submission_attempts: bool,
    pub custom_tracking: Option<TrackingFunction>,
}

impl Default for AnalyticsOptions { ... }
impl std::fmt::Debug for AnalyticsOptions { ... }
impl Clone for AnalyticsOptions { ... }
```

## 🔄 Migration Strategy

### Step 1: Create New Module Structure

1. Create new module files with empty implementations
2. Move type definitions to appropriate modules
3. Update imports in `mod.rs`

### Step 2: Move Implementation Code

1. Move `FieldValue` enum to `field_value.rs`
2. Move `FieldType` and related types to `field_type.rs`
3. Move `FieldConfig` to `field_config.rs`
4. Move `FormSubmissionResult` to `submission.rs`
5. Move option enums and structs to `options.rs`

### Step 3: Update Dependencies

1. Update imports throughout codebase
2. Ensure all tests still pass
3. Update documentation

### Step 4: Validation

1. Run full test suite
2. Verify type safety maintained
3. Check file sizes meet targets

## 🧪 Testing Strategy

### Unit Tests

- Each module gets its own test file
- Test type creation and manipulation
- Test type conversion methods
- Test type validation helpers

### Integration Tests

- Test types work together
- Test serialization/deserialization
- Test type compatibility across modules

### Type Safety Tests

- Test type constraints
- Test type validation
- Test type conversion edge cases

## 📊 Success Criteria

### File Size Compliance

- ✅ All modules under 300 lines
- ✅ `mod.rs` under 100 lines
- ✅ Largest module under 150 lines

### Functionality

- ✅ All existing tests pass
- ✅ No breaking changes to type system
- ✅ Type safety maintained
- ✅ Serialization compatibility preserved

### Code Quality

- ✅ Clear type organization
- ✅ Single responsibility per module
- ✅ Consistent type patterns
- ✅ Comprehensive type documentation

## 🔍 Implementation Notes

### Dependencies

- `field_value.rs` - Depends on `chrono`, `serde`
- `field_type.rs` - Depends on `serde`
- `field_config.rs` - No internal dependencies
- `submission.rs` - Depends on `form_error.rs`
- `options.rs` - No internal dependencies
- `mod.rs` - Re-exports all modules

### Type Safety

- Maintain existing type constraints
- Preserve type validation behavior
- Keep type conversion utilities
- Maintain serialization compatibility

### Performance Considerations

- Minimize allocations in type operations
- Cache type validation results
- Optimize type conversion paths

## 📚 References

- [Original core/types.rs](../../../leptos-forms-rs/src/core/types.rs)
- [Type Tests](../../../tests/unit/src/form_types.rs)
- [Field Value Tests](../../../tests/unit/src/form_handle_comprehensive.rs)
