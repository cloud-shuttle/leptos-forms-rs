# Utils Module Refactoring Design

## 📋 Overview

**Current State**: `utils/mod.rs` (328 lines) - Large file exceeding target by 1.1x
**Target State**: 5 focused modules, each under 300 lines
**Priority**: P1 - High (utility functions used throughout library)

## 🎯 Refactoring Goals

1. **Split monolithic utils module** into focused, single-responsibility modules
2. **Maintain utility function compatibility** - all existing public interfaces preserved
3. **Improve utility function organization** and discoverability
4. **Enhance utility function documentation** and usage examples

## 🏗️ New Module Structure

```
utils/
├── mod.rs                 # Re-exports and module organization (~50 lines)
├── serialization.rs       # Serialization utilities (~80 lines)
├── validation.rs          # Validation utilities (~70 lines)
├── formatting.rs          # Formatting utilities (~60 lines)
├── conversion.rs          # Type conversion utilities (~50 lines)
└── helpers.rs             # General helper functions (~40 lines)
```

## 📝 Module Specifications

### 1. `mod.rs` (Re-exports)

**Target Size**: ~50 lines
**Purpose**: Module organization and public API

```rust
//! Utils module - Utility functions for Leptos Forms
//!
//! This module provides:
//! - Serialization utilities
//! - Validation utilities
//! - Formatting utilities
//! - Type conversion utilities
//! - General helper functions

pub mod serialization;
pub mod validation;
pub mod formatting;
pub mod conversion;
pub mod helpers;

// Re-export public API
pub use serialization::*;
pub use validation::*;
pub use formatting::*;
pub use conversion::*;
pub use helpers::*;
```

### 2. `serialization.rs` (Serialization Utilities)

**Target Size**: ~80 lines
**Purpose**: Form data serialization and deserialization

**Key Components**:

- Form data serialization
- Form data deserialization
- Serialization error handling
- Serialization format support

**Public API**:

```rust
/// Serialize form data to JSON
pub fn serialize_to_json<T: Serialize>(data: &T) -> Result<String, serde_json::Error>

/// Deserialize form data from JSON
pub fn deserialize_from_json<T: DeserializeOwned>(json: &str) -> Result<T, serde_json::Error>

/// Serialize form data to URL-encoded format
pub fn serialize_to_url_encoded<T: Serialize>(data: &T) -> Result<String, serde_urlencoded::ser::Error>

/// Deserialize form data from URL-encoded format
pub fn deserialize_from_url_encoded<T: DeserializeOwned>(data: &str) -> Result<T, serde_urlencoded::de::Error>

/// Serialize form data to FormData
pub fn serialize_to_form_data<T: Serialize>(data: &T) -> Result<FormData, FormDataError>
```

### 3. `validation.rs` (Validation Utilities)

**Target Size**: ~70 lines
**Purpose**: Validation helper functions

**Key Components**:

- Validation rule helpers
- Validation error helpers
- Validation state helpers
- Validation timing helpers

**Public API**:

```rust
/// Check if a value is required
pub fn is_required(value: &FieldValue) -> bool

/// Check if a value is empty
pub fn is_empty(value: &FieldValue) -> bool

/// Check if a value matches a pattern
pub fn matches_pattern(value: &str, pattern: &str) -> bool

/// Check if a value is a valid email
pub fn is_valid_email(value: &str) -> bool

/// Check if a value is a valid URL
pub fn is_valid_url(value: &str) -> bool

/// Check if a value is within a range
pub fn is_in_range(value: f64, min: f64, max: f64) -> bool

/// Check if a value has minimum length
pub fn has_min_length(value: &str, min: usize) -> bool

/// Check if a value has maximum length
pub fn has_max_length(value: &str, max: usize) -> bool
```

### 4. `formatting.rs` (Formatting Utilities)

**Target Size**: ~60 lines
**Purpose**: Data formatting and display utilities

**Key Components**:

- Date formatting
- Number formatting
- String formatting
- Display formatting

**Public API**:

```rust
/// Format a date for display
pub fn format_date(date: &NaiveDate, format: &str) -> String

/// Format a datetime for display
pub fn format_datetime(datetime: &DateTime<Utc>, format: &str) -> String

/// Format a number for display
pub fn format_number(number: f64, precision: usize) -> String

/// Format a currency value
pub fn format_currency(amount: f64, currency: &str) -> String

/// Format a percentage value
pub fn format_percentage(value: f64, precision: usize) -> String

/// Format a file size
pub fn format_file_size(bytes: usize) -> String

/// Format a duration
pub fn format_duration(duration: Duration) -> String
```

### 5. `conversion.rs` (Type Conversion Utilities)

**Target Size**: ~50 lines
**Purpose**: Type conversion and transformation utilities

**Key Components**:

- Type conversion functions
- Value transformation
- Type validation
- Conversion error handling

**Public API**:

```rust
/// Convert a value to a string
pub fn to_string(value: &FieldValue) -> String

/// Convert a value to a number
pub fn to_number(value: &FieldValue) -> Option<f64>

/// Convert a value to a boolean
pub fn to_boolean(value: &FieldValue) -> Option<bool>

/// Convert a value to a date
pub fn to_date(value: &FieldValue) -> Option<NaiveDate>

/// Convert a value to a datetime
pub fn to_datetime(value: &FieldValue) -> Option<DateTime<Utc>>

/// Convert a value to an array
pub fn to_array(value: &FieldValue) -> Option<Vec<FieldValue>>

/// Convert a value to an object
pub fn to_object(value: &FieldValue) -> Option<HashMap<String, FieldValue>>
```

### 6. `helpers.rs` (General Helper Functions)

**Target Size**: ~40 lines
**Purpose**: General utility functions

**Key Components**:

- String helpers
- Array helpers
- Object helpers
- General utilities

**Public API**:

```rust
/// Generate a unique ID
pub fn generate_id() -> String

/// Generate a unique ID with prefix
pub fn generate_id_with_prefix(prefix: &str) -> String

/// Deep clone a value
pub fn deep_clone<T: Clone>(value: &T) -> T

/// Check if two values are equal
pub fn are_equal<T: PartialEq>(a: &T, b: &T) -> bool

/// Get the current timestamp
pub fn current_timestamp() -> i64

/// Get the current date
pub fn current_date() -> NaiveDate

/// Get the current datetime
pub fn current_datetime() -> DateTime<Utc>

/// Sleep for a duration
pub async fn sleep(duration: Duration) -> ()
```

## 🔄 Migration Strategy

### Step 1: Create New Module Structure

1. Create new module files with empty implementations
2. Move utility functions to appropriate modules
3. Update imports in `mod.rs`

### Step 2: Move Implementation Code

1. Move serialization functions to `serialization.rs`
2. Move validation functions to `validation.rs`
3. Move formatting functions to `formatting.rs`
4. Move conversion functions to `conversion.rs`
5. Move helper functions to `helpers.rs`

### Step 3: Update Dependencies

1. Update imports throughout codebase
2. Ensure all tests still pass
3. Update documentation

### Step 4: Validation

1. Run full test suite
2. Verify utility functions still work
3. Check file sizes meet targets

## 🧪 Testing Strategy

### Unit Tests

- Each module gets its own test file
- Test all utility functions
- Test error conditions
- Test edge cases

### Integration Tests

- Test utility functions work together
- Test utility function integration
- Test utility function performance

### Compatibility Tests

- Test utility function compatibility
- Test utility function migration
- Test utility function behavior

## 📊 Success Criteria

### File Size Compliance

- ✅ All modules under 300 lines
- ✅ `mod.rs` under 100 lines
- ✅ Largest module under 100 lines

### Functionality

- ✅ All existing tests pass
- ✅ No breaking changes to utility functions
- ✅ Utility function behavior maintained
- ✅ Performance maintained or improved

### Code Quality

- ✅ Clear utility function organization
- ✅ Single responsibility per module
- ✅ Consistent utility function patterns
- ✅ Comprehensive utility function documentation

## 🔍 Implementation Notes

### Dependencies

- `mod.rs` - Re-exports and module organization
- `serialization.rs` - Depends on serde crates
- `validation.rs` - No internal dependencies
- `formatting.rs` - Depends on chrono crate
- `conversion.rs` - No internal dependencies
- `helpers.rs` - No internal dependencies

### Utility Function Patterns

- Maintain existing utility function API
- Preserve utility function behavior
- Keep utility function performance
- Maintain utility function error handling

### Performance Considerations

- Minimize allocations in utility functions
- Optimize utility function performance
- Cache utility function results where appropriate
- Optimize utility function error handling

## 📚 References

- [Original utils/mod.rs](../../../leptos-forms-rs/src/utils/mod.rs)
- [Utility Tests](../../../tests/unit/src/utils_tests.rs)
- [Serialization Tests](../../../tests/unit/src/serialization_tests.rs)
- [Validation Tests](../../../tests/unit/src/validation_engine_tests.rs)
