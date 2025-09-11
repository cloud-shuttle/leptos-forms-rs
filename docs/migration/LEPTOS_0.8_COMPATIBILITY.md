# Leptos 0.8 Compatibility Guide

## Overview

This document outlines compatibility considerations and best practices when using `leptos-forms-rs` with Leptos 0.8.x, particularly regarding conditional class attributes and the `IntoClass` trait.

## Conditional Class Attributes

### Issue: Memo<bool> with IntoClass Trait

In Leptos 0.8.x, the `IntoClass` trait implementation for `Memo<bool>` types in conditional class attributes can cause compilation errors in certain build configurations.

### Problematic Pattern

```rust
// ❌ This can cause compilation errors in some Leptos 0.8.x versions
let is_active = Memo::new(move |_| some_condition.get());

view! {
    <div class:active=is_active>
        "Content"
    </div>
}
```

**Error Message:**

```
error[E0277]: the trait bound `(&str, leptos::prelude::Memo<bool>): IntoClass` is not satisfied
```

### Recommended Solution

Use closure-based conditional classes with Memo types:

```rust
// ✅ This works reliably across all Leptos 0.8.x versions
let is_active = Memo::new(move |_| some_condition.get());

view! {
    <div class:active=move || is_active.get()>
        "Content"
    </div>
}
```

**Important**: Direct closures (like `move || condition.get()`) don't work with conditional classes. You must use Memo types with closures.

### Alternative Solutions

#### Option 1: Direct Signal Usage

```rust
// ✅ Use signals directly when possible
let (is_active, set_active) = signal(false);

view! {
    <div class:active=is_active>
        "Content"
    </div>
}
```

#### Option 2: Computed Class String

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

The form wizard component has been updated to use the recommended pattern:

```rust
// Before (problematic)
<div class="wizard-step" class:active=is_active class:completed=is_completed>

// After (compatible)
<div class="wizard-step" class:active=move || is_active.get() class:completed=move || is_completed.get()>
```

### Best Practices

1. **Always use closures for Memo types**: When using `Memo<bool>` in conditional classes, wrap them in closures.

2. **Prefer signals when possible**: Direct signal usage is more efficient and doesn't require closures.

3. **Test with different features**: Ensure your code works with both `ssr` and `hydrate` features enabled.

## Testing Compatibility

### Test Cases

```rust
#[test]
fn test_conditional_class_with_memo_bool() {
    let (condition, _) = signal(false);
    let memo_condition = Memo::new(move |_| condition.get());

    // This should compile and work
    view! {
        <div class:active=move || memo_condition.get()>"Test"</div>
    };
}

#[test]
fn test_form_wizard_compatibility() {
    let (current_step, _) = signal(0);

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

If you're upgrading from an older version and encounter compilation errors:

1. **Update to v1.1.3+**: The latest version includes compatibility fixes.

2. **Check your custom components**: If you have custom components using `Memo<bool>` in conditional classes, update them to use closures.

3. **Test thoroughly**: Run your tests with different Leptos feature combinations.

### Version Compatibility Matrix

| leptos-forms-rs | Leptos 0.8.0-0.8.7 | Leptos 0.8.8+ | Notes                       |
| --------------- | ------------------ | ------------- | --------------------------- |
| 1.1.0-1.1.2     | ⚠️ May have issues | ✅ Compatible | Some build configs may fail |
| 1.1.3+          | ✅ Compatible      | ✅ Compatible | Full compatibility          |

## Troubleshooting

### Common Issues

1. **Compilation errors with conditional classes**: Use the closure pattern described above.

2. **Feature flag conflicts**: Ensure you're using compatible feature combinations.

3. **Version mismatches**: Keep leptos-forms-rs and Leptos versions in sync.

### Getting Help

- **GitHub Issues**: Report compatibility issues on the [GitHub repository](https://github.com/cloud-shuttle/leptos-forms-rs/issues)
- **Documentation**: Check the [API documentation](https://docs.rs/leptos-forms-rs) for the latest examples
- **Community**: Join the Leptos Discord for community support

## Changelog

### v1.1.3 (Planned)

- Fixed conditional class compatibility with Leptos 0.8.x
- Updated form wizard component to use closure-based conditional classes
- Added comprehensive compatibility testing

### v1.1.2

- Comprehensive clippy fixes and code quality improvements
- Type aliases for better readability
- Performance optimizations

---

_This document is maintained as part of the leptos-forms-rs project. Last updated: December 2024_
