# Remediation Plan 04: Testing Infrastructure Overhaul

**Priority**: CRITICAL
**Estimated Effort**: 3-4 days
**Blockers**: 01_BUILD_FIXES must be complete
**Dependencies**: Working components and validation

## Overview

Replace the current meaningless test suite with comprehensive, meaningful tests that actually validate functionality.

## Current Test Problems

### False Metrics

- 144 "tests" that mostly contain `assert!(true)`
- No actual functionality testing
- Inflated test count with trivial assertions
- Claims of cross-browser testing with zero browser tests

### Missing Test Categories

- No component rendering tests
- No WASM/browser integration tests
- No E2E tests despite Playwright claims
- No performance/stress testing
- No accessibility testing

## New Test Architecture

### 1. Unit Tests

**Location**: `tests/unit/src/`
**Target**: Test individual functions and modules in isolation

```
tests/unit/src/
├── validation/
│   ├── engine_tests.rs (< 200 lines)
│   ├── rules_tests.rs (< 250 lines)
│   └── async_tests.rs (< 200 lines)
├── core/
│   ├── form_handle_tests.rs (< 250 lines)
│   ├── field_value_tests.rs (< 150 lines)
│   └── helpers_tests.rs (< 150 lines)
├── components/ (after components are implemented)
└── lib.rs (< 100 lines)
```

### 2. WASM Tests

**Location**: `leptos-forms-rs/src/tests/`
**Target**: Browser-specific functionality

```rust
#[cfg(test)]
#[wasm_bindgen_test]
mod wasm_tests {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn form_field_renders_in_browser() {
        // Test component rendering in actual browser
    }

    #[wasm_bindgen_test]
    fn form_submission_handles_events() {
        // Test DOM event handling
    }
}
```

### 3. Integration Tests

**Location**: `tests/integration/`
**Target**: Component interaction and form workflows

### 4. Contract Tests

**Location**: `tests/contracts/src/`
**Target**: API compatibility and external interface contracts

Fix existing contract tests that are mostly empty stubs:

- `consumer_contracts.rs` (662 lines of stub code)
- `framework.rs` (511 lines of stub code)
- `monitoring.rs` (502 lines of stub code)

## Test Requirements by Category

### Validation Tests

- [ ] Every built-in validator with valid/invalid inputs
- [ ] Edge cases (empty strings, null values, extreme lengths)
- [ ] Custom validator integration
- [ ] Async validation with timeout scenarios
- [ ] Cross-field validation
- [ ] Performance under load (1000+ field validations)

### Component Tests (after implementation)

- [ ] FormField rendering with different input types
- [ ] Signal updates propagate correctly
- [ ] Validation errors display properly
- [ ] Accessibility attributes are present
- [ ] Event handling (input, focus, blur)

### Form Integration Tests

- [ ] Complete form lifecycle (create → populate → validate → submit)
- [ ] Field arrays (add/remove/reorder items)
- [ ] Multi-step forms
- [ ] Form persistence (localStorage)
- [ ] Error recovery scenarios

### WASM/Browser Tests

- [ ] Components render in actual browser DOM
- [ ] Event listeners work correctly
- [ ] WASM memory management
- [ ] Performance in browser context

### E2E Tests (Future)

- [ ] Real browser automation with Playwright
- [ ] Cross-browser compatibility (Chrome, Firefox, Safari)
- [ ] Mobile responsiveness
- [ ] Accessibility testing with screen readers

## Test Quality Standards

### Code Coverage

- Minimum 80% line coverage for all modules
- 95% coverage for critical validation logic
- 70% coverage for UI components (harder to test)

### Test Naming

```rust
// Good: Descriptive test names
#[test]
fn required_validator_rejects_empty_string() { }

#[test]
fn email_validator_accepts_valid_email_formats() { }

// Bad: Generic test names
#[test]
fn test_validation() { }

#[test]
fn test_success() { }
```

### Test Organization

- One test file per module (under 300 lines each)
- Logical grouping with mod blocks
- Setup/teardown helpers for complex scenarios
- Property-based testing for validation logic

## Realistic Test Metrics

### Target Numbers

- **Unit tests**: ~60-80 meaningful tests
- **Integration tests**: ~20-30 workflow tests
- **WASM tests**: ~15-25 browser tests
- **Contract tests**: ~10-15 API tests
- **Total**: ~105-150 **meaningful** tests

### Quality over Quantity

- Every test must assert meaningful behavior
- Failed tests must indicate specific problems
- Tests must be maintainable and readable
- Fast execution (entire suite <30 seconds)

## Implementation Plan

### Phase 1: Foundation (Day 1)

- Remove existing trivial tests
- Set up proper test structure
- Implement core validation tests

### Phase 2: Component Tests (Day 2-3)

- Test form handle functionality
- Add helper function tests
- Implement field value tests

### Phase 3: WASM Integration (Day 4)

- Set up wasm-bindgen-test environment
- Add browser-specific tests
- Test DOM integration

### Phase 4: Contracts & CI (Final)

- Fix contract test implementations
- Set up GitHub Actions for all test types
- Add performance benchmarking

## Acceptance Criteria

- [ ] `cargo test` runs meaningful tests that can fail
- [ ] `wasm-pack test` works in multiple browsers
- [ ] Test failures provide actionable error information
- [ ] All test files under 300 lines
- [ ] Realistic test metrics (no more false advertising)
- [ ] CI runs all test categories
- [ ] Performance benchmarks in place

## Example Meaningful Test

```rust
#[test]
fn email_validator_comprehensive_test() {
    let validator = EmailValidator::new();

    // Valid cases
    assert!(validator.validate(&FieldValue::String("user@example.com".to_string())).is_ok());
    assert!(validator.validate(&FieldValue::String("test+tag@domain.org".to_string())).is_ok());

    // Invalid cases
    assert!(validator.validate(&FieldValue::String("invalid-email".to_string())).is_err());
    assert!(validator.validate(&FieldValue::String("@domain.com".to_string())).is_err());
    assert!(validator.validate(&FieldValue::Empty).is_err());

    // Edge cases
    let long_email = format!("{}@example.com", "a".repeat(250));
    assert!(validator.validate(&FieldValue::String(long_email)).is_err());
}
```

## Next Steps

After completion, proceed to `05_DEPENDENCY_UPDATES.md`
