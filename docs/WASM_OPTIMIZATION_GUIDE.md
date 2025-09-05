# WASM Optimization Guide

This guide covers the comprehensive WASM optimizations implemented for the leptos-forms library.

## Overview

The leptos-forms library includes several layers of WASM optimizations to minimize bundle size, improve runtime performance, and enhance developer experience.

## Build Configurations

### 1. Cargo Configuration (`.cargo/config.toml`)

```toml
[target.wasm32-unknown-unknown]
rustflags = [
    "-C", "opt-level=s",      # Optimize for size
    "-C", "lto=fat",          # Link-time optimization
    "-C", "codegen-units=1",  # Single codegen unit
    "-C", "panic=abort",      # Use abort instead of unwind
    "-C", "strip=debuginfo",  # Strip debug info
]
```

### 2. WASM-Pack Configuration (`leptos-forms-rs/wasm-pack.toml`)

```toml
[build.release]
opt-level = "s"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### 3. Cargo Profiles

The library includes optimized profiles for different use cases:

- `release`: Standard release optimizations
- `wasm-release`: WASM-specific optimizations with fat LTO

## Build Scripts

### Optimized Build

```bash
# Build with all optimizations
pnpm run build:wasm:optimized

# Or use the script directly
./scripts/build-wasm.sh
```

### Minimal Build

```bash
# Build with minimal features
pnpm run build:wasm:minimal
```

### Analysis

```bash
# Analyze WASM bundle size and optimization opportunities
pnpm run analyze:wasm
```

## Optimization Features

### 1. Size Optimizations

- **LTO (Link Time Optimization)**: Enables cross-crate optimizations
- **Single Codegen Unit**: Improves optimization opportunities
- **Panic Abort**: Reduces binary size by removing unwinding code
- **Debug Info Stripping**: Removes debug symbols from release builds
- **wasm-opt Integration**: Post-build optimization with binaryen

### 2. Runtime Optimizations

- **Memory Management**: Efficient memory allocation patterns
- **Event Delegation**: Reduced event listener overhead
- **String Interning**: Common field names are interned
- **Regex Pre-compilation**: Validation patterns are pre-compiled

### 3. Conditional Compilation

The library uses feature flags for WASM-specific optimizations:

```rust
#[cfg(feature = "wasm-opt")]
// WASM-specific optimized code

#[cfg(target_arch = "wasm32")]
// WASM-only code
```

## Bundle Analysis

### File Sizes

After optimization, typical bundle sizes are:

- **WASM**: ~50-100KB (gzipped: ~20-40KB)
- **JS Glue**: ~10-20KB (gzipped: ~5-10KB)
- **TypeScript Definitions**: ~5-10KB

### Compression

The build process includes compression analysis:

```bash
# Check compression ratios
./scripts/analyze-wasm.sh
```

## Integration with Bundlers

### Webpack

```javascript
// webpack.config.js
module.exports = {
  experiments: {
    asyncWebAssembly: true,
  },
  optimization: {
    splitChunks: {
      cacheGroups: {
        wasm: {
          test: /\.wasm$/,
          name: "wasm",
          chunks: "all",
        },
      },
    },
  },
};
```

### Vite

```javascript
// vite.config.js
export default defineConfig({
  build: {
    target: "esnext",
    minify: "terser",
    rollupOptions: {
      output: {
        manualChunks: {
          "leptos-forms": ["./leptos-forms-rs/pkg/leptos_forms_rs.js"],
        },
      },
    },
  },
});
```

## Performance Monitoring

### Memory Usage

```rust
// Get WASM memory usage statistics
let memory_usage = get_memory_usage();
```

### Performance Metrics

The library includes built-in performance monitoring for WASM environments.

## Best Practices

### 1. Feature Flags

Use feature flags to enable optimizations:

```toml
[dependencies]
leptos-forms-rs = { version = "1.0", features = ["wasm-opt"] }
```

### 2. Conditional Imports

```rust
#[cfg(target_arch = "wasm32")]
use leptos_forms_rs::wasm_optimizations::init_wasm_optimizations;
```

### 3. Lazy Loading

Load WASM modules only when needed:

```javascript
// Dynamic import for better performance
const { init_wasm_optimizations } = await import("leptos-forms-rs");
init_wasm_optimizations();
```

## Troubleshooting

### Common Issues

1. **Large Bundle Size**: Ensure all optimizations are enabled
2. **Slow Initialization**: Use lazy loading for WASM modules
3. **Memory Issues**: Monitor memory usage with built-in tools

### Debug Mode

For debugging, use the development profile:

```bash
wasm-pack build --dev
```

## Future Optimizations

Planned improvements include:

- **SIMD Support**: Vector operations for validation
- **Threading**: Web Workers for heavy computations
- **Streaming**: Progressive WASM loading
- **Tree Shaking**: Better dead code elimination

## References

- [WebAssembly Optimization Guide](https://rustwasm.github.io/book/reference/code-size.html)
- [wasm-pack Documentation](https://rustwasm.github.io/wasm-pack/)
- [Binaryen wasm-opt](https://github.com/WebAssembly/binaryen)
