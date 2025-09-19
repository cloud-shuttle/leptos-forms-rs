# Hooks Module Refactoring Design

## 📋 Overview

**Current State**: `hooks/mod.rs` (369 lines) - Critical file exceeding target by 1.2x
**Target State**: 8 focused modules, each under 300 lines
**Priority**: P0 - Critical (hooks are central to form functionality)

## 🎯 Refactoring Goals

1. **Split monolithic hooks module** into focused, single-responsibility modules
2. **Maintain hook API compatibility** - all existing public interfaces preserved
3. **Improve hook testability** - each hook can be tested independently
4. **Enhance hook reusability** and maintainability

## 🏗️ New Module Structure

```
hooks/
├── mod.rs                 # Re-exports and module organization (~50 lines)
├── use_form.rs            # use_form hook (~80 lines)
├── use_field.rs           # use_field hook (~100 lines)
├── use_field_array.rs     # use_field_array hook (~90 lines)
├── use_validation.rs      # use_validation hook (~70 lines)
├── use_submission.rs      # use_submission hook (~60 lines)
├── use_persistence.rs     # use_persistence hook (~50 lines)
└── use_analytics.rs       # use_analytics hook (~40 lines)
```

## 📝 Module Specifications

### 1. `mod.rs` (Re-exports)

**Target Size**: ~50 lines
**Purpose**: Module organization and public API

```rust
//! Hooks module - React-like hooks for Leptos Forms
//!
//! This module provides:
//! - Form management hooks
//! - Field management hooks
//! - Validation hooks
//! - Submission hooks
//! - Persistence hooks
//! - Analytics hooks

pub mod use_form;
pub mod use_field;
pub mod use_field_array;
pub mod use_validation;
pub mod use_submission;
pub mod use_persistence;
pub mod use_analytics;

// Re-export public API
pub use use_form::*;
pub use use_field::*;
pub use use_field_array::*;
pub use use_validation::*;
pub use use_submission::*;
pub use use_persistence::*;
pub use use_analytics::*;
```

### 2. `use_form.rs` (use_form Hook)

**Target Size**: ~80 lines
**Purpose**: Main form management hook

**Key Components**:

- Form creation and initialization
- Form state management
- Form configuration
- Form lifecycle management

**Public API**:

```rust
/// Main hook for creating and managing forms
pub fn use_form<T: Form + PartialEq + Clone + Send + Sync>(
    initial_values: T,
    #[prop(optional)] validation_mode: Option<ValidationMode>,
    #[prop(optional)] submission_mode: Option<SubmissionMode>,
    #[prop(optional)] persistence_options: Option<PersistenceOptions>,
    #[prop(optional)] analytics_options: Option<AnalyticsOptions>,
) -> FormHandle<T>
```

### 3. `use_field.rs` (use_field Hook)

**Target Size**: ~100 lines
**Purpose**: Field management hook

**Key Components**:

- Field value management
- Field validation
- Field state tracking
- Field event handling

**Public API**:

```rust
/// Hook for managing individual form fields
pub fn use_field<T: Form + PartialEq + Clone + Send + Sync>(
    form: FormHandle<T>,
    field_name: String,
    #[prop(optional)] validators: Option<Vec<Validator>>,
    #[prop(optional)] validation_mode: Option<ValidationMode>,
) -> FieldHandle
```

### 4. `use_field_array.rs` (use_field_array Hook)

**Target Size**: ~90 lines
**Purpose**: Field array management hook

**Key Components**:

- Array field management
- Array item operations
- Array validation
- Array state tracking

**Public API**:

```rust
/// Hook for managing field arrays
pub fn use_field_array<T: Form + PartialEq + Clone + Send + Sync>(
    form: FormHandle<T>,
    field_name: String,
    #[prop(optional)] validators: Option<Vec<Validator>>,
    #[prop(optional)] min_items: Option<usize>,
    #[prop(optional)] max_items: Option<usize>,
) -> FieldArrayHandle
```

### 5. `use_validation.rs` (use_validation Hook)

**Target Size**: ~70 lines
**Purpose**: Validation management hook

**Key Components**:

- Validation state management
- Validation error handling
- Validation mode control
- Validation timing

**Public API**:

```rust
/// Hook for managing form validation
pub fn use_validation<T: Form + PartialEq + Clone + Send + Sync>(
    form: FormHandle<T>,
    #[prop(optional)] validation_mode: Option<ValidationMode>,
) -> ValidationHandle
```

### 6. `use_submission.rs` (use_submission Hook)

**Target Size**: ~60 lines
**Purpose**: Form submission management hook

**Key Components**:

- Submission state management
- Submission error handling
- Submission success handling
- Submission timing

**Public API**:

```rust
/// Hook for managing form submission
pub fn use_submission<T: Form + PartialEq + Clone + Send + Sync>(
    form: FormHandle<T>,
    #[prop(optional)] submission_mode: Option<SubmissionMode>,
) -> SubmissionHandle
```

### 7. `use_persistence.rs` (use_persistence Hook)

**Target Size**: ~50 lines
**Purpose**: Form persistence management hook

**Key Components**:

- Form state persistence
- Auto-save functionality
- Persistence error handling
- Persistence configuration

**Public API**:

```rust
/// Hook for managing form persistence
pub fn use_persistence<T: Form + PartialEq + Clone + Send + Sync>(
    form: FormHandle<T>,
    #[prop(optional)] persistence_options: Option<PersistenceOptions>,
) -> PersistenceHandle
```

### 8. `use_analytics.rs` (use_analytics Hook)

**Target Size**: ~40 lines
**Purpose**: Form analytics management hook

**Key Components**:

- Analytics tracking
- Event logging
- Analytics configuration
- Analytics reporting

**Public API**:

```rust
/// Hook for managing form analytics
pub fn use_analytics<T: Form + PartialEq + Clone + Send + Sync>(
    form: FormHandle<T>,
    #[prop(optional)] analytics_options: Option<AnalyticsOptions>,
) -> AnalyticsHandle
```

## 🔄 Migration Strategy

### Step 1: Create New Module Structure

1. Create new module files with empty implementations
2. Move hook implementations to appropriate modules
3. Update imports in `mod.rs`

### Step 2: Move Implementation Code

1. Move use_form hook to `use_form.rs`
2. Move use_field hook to `use_field.rs`
3. Move use_field_array hook to `use_field_array.rs`
4. Move use_validation hook to `use_validation.rs`
5. Move use_submission hook to `use_submission.rs`
6. Move use_persistence hook to `use_persistence.rs`
7. Move use_analytics hook to `use_analytics.rs`

### Step 3: Update Dependencies

1. Update imports throughout codebase
2. Ensure all tests still pass
3. Update documentation

### Step 4: Validation

1. Run full test suite
2. Verify hooks still work
3. Check file sizes meet targets

## 🧪 Testing Strategy

### Unit Tests

- Each hook gets its own test file
- Test hook behavior
- Test hook state management
- Test hook error handling

### Integration Tests

- Test hooks work together
- Test hook interactions
- Test hook lifecycle

### Performance Tests

- Test hook performance
- Test hook memory usage
- Test hook state updates

## 📊 Success Criteria

### File Size Compliance

- ✅ All modules under 300 lines
- ✅ `mod.rs` under 100 lines
- ✅ Largest module under 120 lines

### Functionality

- ✅ All existing tests pass
- ✅ No breaking changes to hook API
- ✅ Hook behavior maintained
- ✅ Hook state management preserved

### Code Quality

- ✅ Clear hook separation
- ✅ Single responsibility per hook
- ✅ Consistent hook patterns
- ✅ Comprehensive hook documentation

## 🔍 Implementation Notes

### Dependencies

- `mod.rs` - Re-exports and module organization
- `use_form.rs` - Depends on FormHandle and form state
- `use_field.rs` - Depends on FormHandle and field operations
- `use_field_array.rs` - Depends on FormHandle and array operations
- `use_validation.rs` - Depends on FormHandle and validation
- `use_submission.rs` - Depends on FormHandle and submission
- `use_persistence.rs` - Depends on FormHandle and persistence
- `use_analytics.rs` - Depends on FormHandle and analytics

### Hook Patterns

- Maintain existing hook API
- Preserve hook behavior
- Keep hook state management
- Maintain hook lifecycle

### Performance Considerations

- Minimize hook re-renders
- Optimize hook state updates
- Cache hook calculations
- Optimize hook event handling

## 📚 References

- [Original hooks/mod.rs](../../../leptos-forms-rs/src/hooks/mod.rs)
- [Hook Tests](../../../tests/unit/src/hooks_tests.rs)
- [Form Hook Tests](../../../tests/unit/src/form_hooks_tests.rs)
- [Field Hook Tests](../../../tests/unit/src/field_hooks_tests.rs)
