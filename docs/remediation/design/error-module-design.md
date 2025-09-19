# Error Module Refactoring Design

## 📋 Overview

**Current State**: `error/mod.rs` (471 lines) - Critical file exceeding target by 1.5x
**Target State**: 6 focused modules, each under 300 lines
**Priority**: P0 - Critical (error handling is fundamental)

## 🎯 Refactoring Goals

1. **Split monolithic error module** into focused, single-responsibility modules
2. **Maintain error handling consistency** across the entire library
3. **Improve error context and debugging** capabilities
4. **Enhance error reporting and logging** functionality

## 🏗️ New Module Structure

```
error/
├── mod.rs                 # Re-exports and module organization (~50 lines)
├── form_error.rs          # FormError enum and core error types (~120 lines)
├── field_error.rs         # FieldError struct and field-specific errors (~80 lines)
├── context.rs             # ErrorContext and debugging information (~100 lines)
├── handler.rs             # ErrorHandler trait and default implementations (~90 lines)
└── reporter.rs            # ErrorReporter trait and implementations (~80 lines)
```

## 📝 Module Specifications

### 1. `mod.rs` (Re-exports)

**Target Size**: ~50 lines
**Purpose**: Module organization and public API

```rust
//! Error handling module - Comprehensive error management for Leptos Forms
//!
//! This module provides:
//! - Form and field error types
//! - Error context and debugging information
//! - Error handling and reporting
//! - Error recovery strategies

pub mod form_error;
pub mod field_error;
pub mod context;
pub mod handler;
pub mod reporter;

// Re-export public API
pub use form_error::*;
pub use field_error::*;
pub use context::*;
pub use handler::*;
pub use reporter::*;

// Type aliases for convenience
pub type FormResult<T> = Result<T, FormError>;
```

### 2. `form_error.rs` (FormError Enum)

**Target Size**: ~120 lines
**Purpose**: Core form error types and handling

**Key Components**:

- `FormError` enum with all variants
- Error creation methods
- Error classification methods
- Error conversion utilities
- Display and Debug implementations

**Public API**:

```rust
#[derive(Debug, Clone)]
pub enum FormError {
    FieldError { field: String, message: String, code: Option<String> },
    ValidationError { message: String, field_errors: Vec<FieldError> },
    SerializationError { message: String, field: Option<String> },
    SubmissionError { message: String, status_code: Option<u16>, response: Option<String> },
    StateError { message: String, operation: String },
    PersistenceError { message: String, storage_type: String },
    ConfigurationError { message: String, component: String },
    Unknown { message: String, source: Option<String> },
}

impl FormError {
    pub fn field_error(field: impl Into<String>, message: impl Into<String>) -> Self
    pub fn field_error_with_code(field: impl Into<String>, message: impl Into<String>, code: impl Into<String>) -> Self
    pub fn validation_error(message: impl Into<String>, field_errors: Vec<FieldError>) -> Self
    pub fn serialization_error(message: impl Into<String>, field: Option<String>) -> Self
    pub fn submission_error(message: impl Into<String>, status_code: Option<u16>, response: Option<String>) -> Self
    pub fn state_error(message: impl Into<String>, operation: impl Into<String>) -> Self
    pub fn persistence_error(message: impl Into<String>, storage_type: impl Into<String>) -> Self
    pub fn configuration_error(message: impl Into<String>, component: impl Into<String>) -> Self
    pub fn unknown(message: impl Into<String>, source: Option<String>) -> Self

    pub fn message(&self) -> &str
    pub fn is_field_error(&self) -> bool
    pub fn is_validation_error(&self) -> bool
    pub fn is_submission_error(&self) -> bool
    pub fn field_name(&self) -> Option<&str>
    pub fn error_code(&self) -> Option<&str>
    pub fn as_field_error(&self) -> Option<FieldError>
}
```

### 3. `field_error.rs` (FieldError Struct)

**Target Size**: ~80 lines
**Purpose**: Field-specific error handling

**Key Components**:

- `FieldError` struct
- Error creation and manipulation
- Error code management
- Display and Debug implementations

**Public API**:

```rust
#[derive(Debug, Clone)]
pub struct FieldError {
    pub field: String,
    pub message: String,
    pub code: Option<String>,
}

impl FieldError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self
    pub fn with_code(mut self, code: impl Into<String>) -> Self
    pub fn field_name(&self) -> &str
    pub fn message(&self) -> &str
    pub fn code(&self) -> Option<&str>
}

impl std::fmt::Display for FieldError { ... }
impl std::error::Error for FieldError { ... }
```

### 4. `context.rs` (ErrorContext)

**Target Size**: ~100 lines
**Purpose**: Error context and debugging information

**Key Components**:

- `ErrorContext` struct
- Context building methods
- Debugging information collection
- Timestamp and metadata handling

**Public API**:

```rust
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub form_name: Option<String>,
    pub field_name: Option<String>,
    pub operation: Option<String>,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub user_agent: Option<String>,
    pub additional_data: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    pub fn new() -> Self
    pub fn with_form_name(mut self, form_name: impl Into<String>) -> Self
    pub fn with_field_name(mut self, field_name: impl Into<String>) -> Self
    pub fn with_operation(mut self, operation: impl Into<String>) -> Self
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self
    pub fn with_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self
}

impl Default for ErrorContext { ... }
```

### 5. `handler.rs` (ErrorHandler Trait)

**Target Size**: ~90 lines
**Purpose**: Error handling strategies and implementations

**Key Components**:

- `ErrorHandler` trait
- `DefaultErrorHandler` implementation
- Error handling strategies
- Logging and reporting decisions

**Public API**:

```rust
pub trait ErrorHandler {
    fn handle_error(&self, error: &FormError, context: &ErrorContext);
    fn should_log_error(&self, error: &FormError) -> bool;
    fn should_report_error(&self, error: &FormError) -> bool;
}

pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
    fn handle_error(&self, error: &FormError, context: &ErrorContext) { ... }
    fn should_log_error(&self, _error: &FormError) -> bool { ... }
    fn should_report_error(&self, error: &FormError) -> bool { ... }
}
```

### 6. `reporter.rs` (ErrorReporter Trait)

**Target Size**: ~80 lines
**Purpose**: Error reporting to external services

**Key Components**:

- `ErrorReporter` trait
- `ConsoleErrorReporter` implementation
- Error reporting strategies
- External service integration points

**Public API**:

```rust
pub trait ErrorReporter {
    fn report_error(&self, error: &FormError, context: &ErrorContext);
}

pub struct ConsoleErrorReporter;

impl ErrorReporter for ConsoleErrorReporter {
    fn report_error(&self, error: &FormError, context: &ErrorContext) { ... }
}

// Utility functions for error handling
pub mod utils {
    pub fn field_error_from_validation(field: &str, message: &str) -> FormError
    pub fn validation_error_from_fields(message: &str, field_errors: Vec<FieldError>) -> FormError
    pub fn extract_field_errors(error: &FormError) -> Vec<FieldError>
    pub fn is_recoverable_error(error: &FormError) -> bool
    pub fn is_critical_error(error: &FormError) -> bool
}
```

## 🔄 Migration Strategy

### Step 1: Create New Module Structure

1. Create new module files with empty implementations
2. Move type definitions to appropriate modules
3. Update imports in `mod.rs`

### Step 2: Move Implementation Code

1. Move `FormError` enum to `form_error.rs`
2. Move `FieldError` struct to `field_error.rs`
3. Move `ErrorContext` to `context.rs`
4. Move `ErrorHandler` trait to `handler.rs`
5. Move `ErrorReporter` trait to `reporter.rs`
6. Move utility functions to `reporter.rs::utils`

### Step 3: Update Dependencies

1. Update imports throughout codebase
2. Ensure all tests still pass
3. Update documentation

### Step 4: Validation

1. Run full test suite
2. Verify error handling still works
3. Check file sizes meet targets

## 🧪 Testing Strategy

### Unit Tests

- Each module gets its own test file
- Test error creation and manipulation
- Test error classification methods
- Test error context building

### Integration Tests

- Test error handling across modules
- Test error reporting flows
- Test error recovery scenarios

### Error Scenario Tests

- Test various error conditions
- Test error aggregation
- Test error display formatting

## 📊 Success Criteria

### File Size Compliance

- ✅ All modules under 300 lines
- ✅ `mod.rs` under 100 lines
- ✅ Largest module under 150 lines

### Functionality

- ✅ All existing tests pass
- ✅ No breaking changes to error handling
- ✅ Error context and debugging improved
- ✅ Error reporting functionality maintained

### Code Quality

- ✅ Clear separation of error types
- ✅ Single responsibility per module
- ✅ Consistent error handling patterns
- ✅ Comprehensive error documentation

## 🔍 Implementation Notes

### Dependencies

- `form_error.rs` - Depends on `field_error.rs`
- `field_error.rs` - No internal dependencies
- `context.rs` - Depends on `chrono` crate
- `handler.rs` - Depends on `form_error.rs`, `context.rs`
- `reporter.rs` - Depends on `form_error.rs`, `context.rs`
- `mod.rs` - Re-exports all modules

### Error Handling Patterns

- Maintain existing error types and variants
- Preserve error message formats
- Keep error classification behavior
- Maintain error conversion utilities

### Performance Considerations

- Minimize allocations in error creation
- Cache error context information
- Optimize error display formatting

## 📚 References

- [Original error/mod.rs](../../../leptos-forms-rs/src/error/mod.rs)
- [Error Handling Tests](../../../tests/unit/src/form_handle_comprehensive.rs)
- [Validation Error Tests](../../../tests/unit/src/validation_engine_tests.rs)
