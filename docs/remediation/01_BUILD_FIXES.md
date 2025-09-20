# Remediation Plan 01: Critical Build Fixes

**Priority**: CRITICAL
**Estimated Effort**: 2-3 days
**Blockers**: None
**Dependencies**: Must complete before any other work

## Overview

Fix the compilation failures that prevent the library from being used in any real application.

## Issues to Address

### 1. Missing Macro Helper Functions

**File**: `leptos-forms-rs-macro/src/lib.rs`
**Problem**: Generates calls to undefined functions
**Functions needed**:

- `convert_to_field_value()`
- `convert_from_field_value()`
- `get_type_default()`

**Solution**:

```rust
// In leptos-forms-rs/src/core/helpers.rs
pub fn convert_to_field_value<T: Serialize>(value: T) -> FieldValue {
    // Implementation here
}

pub fn convert_from_field_value<T: DeserializeOwned>(value: FieldValue) -> Result<T, ValidationError> {
    // Implementation here
}

pub fn get_type_default<T: Default>() -> T {
    T::default()
}
```

### 2. Broken Re-exports

**File**: `leptos-forms-rs/src/lib.rs`
**Problem**: Re-exporting non-existent items
**Solution**: Remove or comment out re-exports until features are implemented

### 3. Missing Component Implementations

**Files**: All component modules
**Problem**: Empty implementations being re-exported
**Solution**: Provide minimal working implementations or remove from public API

## Acceptance Criteria

- [ ] `cargo check` passes for the entire workspace
- [ ] `cargo test` passes for the entire workspace
- [ ] Derive macro compiles and generates working code
- [ ] Basic example from README compiles and runs
- [ ] WASM compilation works (`cargo check --target wasm32-unknown-unknown`)

## Testing Requirements

- [ ] Add integration test using the derive macro
- [ ] Test that derived forms can be instantiated
- [ ] Test that basic validation works with derived forms
- [ ] Test WASM compilation in CI

## Definition of Done

A user can:

1. Add the crate as a dependency
2. Use `#[derive(Form)]` on a struct
3. Create a form instance with `use_form()`
4. Perform basic validation
5. Compile to WASM target

## Files to Modify

```
leptos-forms-rs-macro/src/lib.rs (544 lines → split into multiple files)
leptos-forms-rs/src/core/helpers.rs (new file)
leptos-forms-rs/src/lib.rs (trim re-exports)
leptos-forms-rs/src/components/ (minimal implementations)
```

## Next Steps

After completion, proceed to `02_CORE_COMPONENTS.md`
