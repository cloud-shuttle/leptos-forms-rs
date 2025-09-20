# AGENTS.md - Leptos Forms RS Development Guide

## Build/Test/Lint Commands

- `cargo test` - Run unit tests
- `cargo test --package <package>` - Run tests for specific package (unit, contracts)
- `./tests/run_tests.sh all` - Run all test suites (unit, integration, e2e, benchmarks)
- `./tests/run_tests.sh unit` - Run unit tests only
- `./tests/run_tests.sh integration` - Run integration tests only
- `pnpm run test:e2e` - Run E2E tests in all browsers (Chrome, Firefox, WebKit, Mobile)
- `wasm-pack test --headless --firefox` - Run WASM tests in browser
- `cargo clippy` - Run linting
- `cargo fmt` - Format code
- `cargo check` - Check compilation without building
- `make ci` - Run full CI checks (fmt, clippy, test, wasm-test)

## Architecture

- **Workspace**: Multi-crate project with `leptos-forms-rs` (core), `leptos-forms-rs-macro` (proc macros)
- **Core modules**: `core/`, `validation/`, `components/`, `hooks/`, `events/`, `utils/`
- **Examples**: `examples/basic-form/`, `examples/complex-form/`
- **Testing**: Comprehensive test suites in `tests/` (unit, integration, e2e, benchmarks, contracts)
- **WASM target**: Compiles to WebAssembly with optimized profiles

## Code Style Guidelines

- **Imports**: Use workspace dependencies, explicit feature flags for web-sys
- **Naming**: Snake_case for modules/functions, PascalCase for types/traits, SCREAMING_SNAKE_CASE for constants
- **Error handling**: Custom error types in `error/` module, use `Result<T, ValidationErrors>`
- **Documentation**: Comprehensive rustdoc comments with examples
- **Formatting**: Standard rustfmt with workspace-level Cargo.toml settings
- **Types**: Heavy use of serde traits, leptos signals for reactivity
- **Testing**: Test all public APIs, use descriptive test names, cross-browser compatibility required
