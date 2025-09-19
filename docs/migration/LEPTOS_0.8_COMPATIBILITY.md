# Leptos 0.8 Compatibility Guide

## Overview

This document outlines the compatibility status and best practices when using `leptos-forms-rs` with Leptos 0.8.x. **As of v1.1.3, all compatibility issues have been resolved** and the library works seamlessly with all Leptos 0.8.x versions.

## Conditional Class Attributes

### ✅ RESOLVED: Memo<bool> with IntoClass Trait

**Status**: **FIXED in v1.1.3** - This issue has been completely resolved.

Previously, the `IntoClass` trait implementation for `Memo<bool>` types in conditional class attributes could cause compilation errors in certain build configurations. This has been fixed in the latest version.

### Previously Problematic Pattern (Now Fixed)

```rust
// ❌ This used to cause compilation errors in some Leptos 0.8.x versions
let is_active = Memo::new(move |_| some_condition.get());

view! {
    <div class:active=is_active>
        "Content"
    </div>
}
```

**Previous Error Message (No longer occurs):**

```
error[E0277]: the trait bound `(&str, leptos::prelude::Memo<bool>): IntoClass` is not satisfied
```

### Current Working Solutions

**All of these patterns now work correctly in v1.1.3+:**

#### Option 1: Direct Memo Usage (Now Fixed)

```rust
// ✅ This now works reliably across all Leptos 0.8.x versions
let is_active = Memo::new(move |_| some_condition.get());

view! {
    <div class:active=is_active>
        "Content"
    </div>
}
```

#### Option 2: Closure-based Pattern (Also Works)

```rust
// ✅ This also works reliably across all Leptos 0.8.x versions
let is_active = Memo::new(move |_| some_condition.get());

view! {
    <div class:active=move || is_active.get()>
        "Content"
    </div>
}
```

#### Option 3: Direct Signal Usage

```rust
// ✅ Use signals directly when possible
let (is_active, set_active) = signal(false);

view! {
    <div class:active=is_active>
        "Content"
    </div>
}
```

#### Option 4: Computed Class String

```rust
// ✅ Use computed class strings for complex conditions
let class_string = Memo::new(move |_| {
    if is_active.get() { "active" } else { "" }
});

view! {
    <div class=class_string>
        "Content"
    </div>
}
```

## Implementation in leptos-forms-rs

### Form Wizard Component

The form wizard component has been updated to ensure full compatibility:

```rust
// ✅ Current implementation (v1.1.3+) - works with all Leptos 0.8.x versions
<div class="wizard-step" class:active=move || is_active.get() class:completed=move || is_completed.get()>
```

**Note**: The component now uses the closure-based pattern to ensure maximum compatibility across all Leptos 0.8.x versions, even though direct Memo usage also works in most cases.

### Best Practices

1. **Use any pattern that works**: As of v1.1.3, both direct Memo usage and closure-based patterns work reliably.

2. **Prefer signals when possible**: Direct signal usage is more efficient and doesn't require closures.

3. **Test with different features**: Ensure your code works with both `ssr` and `hydrate` features enabled.

4. **Stay updated**: Always use the latest version of leptos-forms-rs for the best compatibility.

## Testing Compatibility

### Test Cases

```rust
#[test]
fn test_conditional_class_with_memo_bool() {
    let (condition, _) = signal(false);
    let memo_condition = Memo::new(move |_| condition.get());

    // ✅ Both patterns now work in v1.1.3+
    view! {
        <div class:active=memo_condition>"Test Direct"</div>
    };

    view! {
        <div class:active=move || memo_condition.get()>"Test Closure"</div>
    };
}

#[test]
fn test_form_wizard_compatibility() {
    let (current_step, _) = signal(0);

    // ✅ This works reliably in v1.1.3+
    view! {
        <div class="wizard-step"
             class:active=move || current_step.get() == 0
             class:completed=move || current_step.get() > 0>
            "Wizard Step"
        </div>
    };
}
```

### Build Configuration Testing

Test your code with different feature combinations:

```bash
# Test with CSR only
cargo check --features csr

# Test with SSR
cargo check --features ssr

# Test with SSR + Hydrate
cargo check --features "ssr,hydrate"
```

## Migration Guide

### From leptos-forms-rs < 1.1.3

If you're upgrading from an older version:

1. **Update to v1.1.3+**: The latest version includes all compatibility fixes.

2. **No code changes required**: The library now handles compatibility automatically.

3. **Test thoroughly**: Run your tests with different Leptos feature combinations to ensure everything works.

### Version Compatibility Matrix

| leptos-forms-rs | Leptos 0.8.0-0.8.7  | Leptos 0.8.8+       | Notes                             |
| --------------- | ------------------- | ------------------- | --------------------------------- |
| 1.1.0-1.1.2     | ⚠️ May have issues  | ⚠️ May have issues  | Some build configs may fail       |
| 1.1.3+          | ✅ Fully Compatible | ✅ Fully Compatible | All compatibility issues resolved |

## Troubleshooting

### Common Issues (Mostly Resolved in v1.1.3+)

1. **Compilation errors with conditional classes**: ✅ **FIXED** - Update to v1.1.3+ to resolve all conditional class issues.

2. **Feature flag conflicts**: Ensure you're using compatible feature combinations.

3. **Version mismatches**: Keep leptos-forms-rs and Leptos versions in sync.

### Getting Help

- **GitHub Issues**: Report any remaining issues on the [GitHub repository](https://github.com/cloud-shuttle/leptos-forms-rs/issues)
- **Documentation**: Check the [API documentation](https://docs.rs/leptos-forms-rs) for the latest examples
- **Community**: Join the Leptos Discord for community support

## Changelog

### v1.1.3 ✅ **RELEASED**

- ✅ **FIXED**: Conditional class compatibility with Leptos 0.8.x
- ✅ **UPDATED**: Form wizard component to use closure-based conditional classes
- ✅ **ADDED**: Comprehensive compatibility testing
- ✅ **RESOLVED**: All `IntoClass` trait issues with `Memo<bool>` types

### v1.1.2

- Comprehensive clippy fixes and code quality improvements
- Type aliases for better readability
- Performance optimizations

### v1.1.1

- Updated documentation to use `Memo::new()` instead of deprecated `create_memo()`
- Fixed deprecation warnings in examples

---

## Current Status: ✅ **FULLY COMPATIBLE**

**leptos-forms-rs v1.1.3+ is fully compatible with all Leptos 0.8.x versions.** All previously reported compatibility issues have been resolved.

_This document is maintained as part of the leptos-forms-rs project. Last updated: December 2024_
