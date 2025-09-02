# Leptos 0.6 to 0.8 Migration Guide

## Quick Migration

### 1. Update Dependencies
```toml
[dependencies]
leptos-forms-rs = { version = "0.1", features = ["leptos-0-8"] }
```

### 2. Add Recursion Limit
```rust
#![recursion_limit = "256"]
```

### 3. Use Compatibility Layer
```rust
use leptos_forms_rs::compat::*;

// Replace create_signal(cx, value) with signal(value)
let (count, set_count) = signal(0);

// Replace #[component] with compat_component!
compat_component! {
    fn MyComponent() -> impl IntoView {
        view! { <div>"Hello"</div> }
    }
}
```

## Key Changes

- **Scope parameter removed** from components and signals
- **New tachys rendering system** for better performance
- **Enhanced signal types** with `Signal<T>` wrapper
- **Resource API redesign** with mandatory serialization

## Migration Examples

### Before (0.6)
```rust
#[component]
fn Counter(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    view! { cx, <button>{count}</button> }
}
```

### After (0.8 with Compatibility)
```rust
compat_component! {
    fn Counter() -> impl IntoView {
        let (count, set_count) = signal(0);
        view! { <button>{count}</button> }
    }
}
```

## Testing Both Versions

```bash
# Test with Leptos 0.6
cargo test --features leptos-0-6

# Test with Leptos 0.8  
cargo test --features leptos-0-8
```

The compatibility layer handles all version differences automatically!
