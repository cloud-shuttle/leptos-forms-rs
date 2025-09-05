# Development Workflow Guide - Leptos Forms

**Project**: Leptos Forms Library
**Version**: 1.0
**Date**: 2025-01-02
**Status**: Development Guide

## 1. Development Environment Setup

### 1.1 Prerequisites

```bash
# Rust toolchain (required)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
rustup default stable

# Required targets
rustup target add wasm32-unknown-unknown

# Required tools
cargo install wasm-pack
cargo install cargo-watch
cargo install cargo-tarpaulin  # For coverage
cargo install cargo-audit      # For security audits
cargo install cargo-outdated   # For dependency management

# Node.js for examples and tooling
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
nvm use 18

# Global npm packages
npm install -g serve http-server
```

### 1.2 Repository Setup

```bash
# Clone repository
git clone https://github.com/leptos-rs/leptos-forms.git
cd leptos-forms

# Install pre-commit hooks
cp .githooks/pre-commit .git/hooks/
chmod +x .git/hooks/pre-commit

# Install development dependencies
cd examples/contact-form
npm install
cd ../..

# Verify setup
./scripts/verify-setup.sh
```

### 1.3 IDE Configuration

#### VS Code Setup (`/.vscode/settings.json`)

```json
{
  "rust-analyzer.cargo.features": ["all"],
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.check.allTargets": true,
  "rust-analyzer.procMacro.enable": true,
  "rust-analyzer.imports.granularity.group": "module",
  "rust-analyzer.completion.addCallArgumentSnippets": true,
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll.rust-analyzer": true
  },
  "files.watcherExclude": {
    "**/target/**": true,
    "**/pkg/**": true,
    "**/node_modules/**": true
  },
  "rust-analyzer.lens.enable": true,
  "rust-analyzer.lens.implementations.enable": true,
  "rust-analyzer.lens.references.enable": true
}
```

#### VS Code Extensions (`.vscode/extensions.json`)

```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "serayuzgur.crates",
    "vadimcn.vscode-lldb",
    "ms-vscode.vscode-typescript-next",
    "bradlc.vscode-tailwindcss",
    "formulahendry.auto-rename-tag",
    "christian-kohler.path-intellisense"
  ]
}
```

## 2. Git Workflow Strategy

### 2.1 Branch Naming Conventions

```
feat/feature-name      # New features
fix/issue-description  # Bug fixes
docs/update-readme     # Documentation updates
chore/update-deps      # Maintenance tasks
perf/optimize-cache    # Performance improvements
test/add-integration   # Test improvements
refactor/cleanup-api   # Code refactoring
```

### 2.2 Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

#### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code formatting (no logic change)
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Performance improvement
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `build`: Build system or external dependency changes

#### Examples

```
feat(validation): add async validator support

Add support for async validators that can perform server-side
validation such as username availability checks.

- Add AsyncValidator trait
- Implement async validation in FormHandle
- Add debouncing to prevent excessive API calls
- Update validation timing modes

Closes #123
```

### 2.3 Pull Request Workflow

#### 1. Create Feature Branch

```bash
git checkout main
git pull origin main
git checkout -b feat/new-feature
```

#### 2. Development Cycle

```bash
# Make changes and commit frequently
git add .
git commit -m "feat(scope): implement core functionality"

# Keep up to date with main
git fetch origin main
git rebase origin/main

# Push to remote
git push origin feat/new-feature
```

#### 3. Pre-PR Checklist

```bash
# Run all checks locally
./scripts/pre-pr-check.sh

# This script runs:
# - cargo fmt --check
# - cargo clippy --all-targets --all-features -- -D warnings
# - cargo test --all-features
# - cargo doc --all-features --no-deps
# - wasm-pack build --target web
# - examples/contact-form build and test
```

#### 4. Create Pull Request

**PR Template** (`.github/pull_request_template.md`):

```markdown
## Description

Brief description of changes

## Type of Change

- [ ] Bug fix (non-breaking change that fixes an issue)
- [ ] New feature (non-breaking change that adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing

- [ ] Tests pass locally
- [ ] Added new tests for new functionality
- [ ] Updated existing tests as needed
- [ ] Manual testing completed

## Checklist

- [ ] Code follows project style guidelines
- [ ] Self-review of code completed
- [ ] Code is commented where necessary
- [ ] Documentation updated if needed
- [ ] No new warnings introduced
- [ ] Performance impact considered

## Related Issues

Fixes #(issue number)

## Screenshots (if applicable)

## Additional Notes
```

### 2.4 Code Review Process

#### Review Guidelines

1. **Functionality**: Does the code do what it's supposed to do?
2. **Architecture**: Is the code well-structured and maintainable?
3. **Performance**: Are there any performance implications?
4. **Security**: Are there any security concerns?
5. **Testing**: Is the code adequately tested?
6. **Documentation**: Is the code properly documented?

#### Review Checklist

```markdown
## Code Review Checklist

### Functionality

- [ ] Code implements requirements correctly
- [ ] Edge cases are handled appropriately
- [ ] Error conditions are handled gracefully

### Code Quality

- [ ] Code is readable and well-organized
- [ ] Functions are focused and not too long
- [ ] Variable names are descriptive
- [ ] No code duplication

### Testing

- [ ] Adequate test coverage (>90%)
- [ ] Tests cover edge cases
- [ ] Tests are readable and maintainable

### Performance

- [ ] No obvious performance issues
- [ ] Memory usage is reasonable
- [ ] Algorithms are efficient

### Security

- [ ] No security vulnerabilities
- [ ] Input validation where appropriate
- [ ] No sensitive data exposure
```

## 3. Development Environment

### 3.1 Local Development Commands

#### Basic Development

```bash
# Watch and rebuild on changes (library)
cargo watch -x "check --all-features"

# Watch with tests
cargo watch -x "test --all-features"

# Watch examples
cd examples/contact-form
npm run dev  # Starts dev server with hot reload

# Format code
cargo fmt

# Lint code
cargo clippy --all-targets --all-features -- -D warnings

# Generate documentation
cargo doc --all-features --open
```

#### Testing Commands

```bash
# Run all tests
cargo test --all-features

# Run specific test
cargo test test_form_validation

# Run tests with coverage
cargo tarpaulin --all-features --out Html

# Run integration tests only
cargo test --test integration_tests

# Run doc tests
cargo test --doc

# Run with output
cargo test -- --nocapture

# Run single-threaded (useful for debugging)
cargo test -- --test-threads=1
```

#### WASM Development

```bash
# Build WASM package
wasm-pack build --target web

# Build in development mode
wasm-pack build --dev --target web

# Test WASM in browser
wasm-pack test --headless --chrome

# Test WASM in Node.js
wasm-pack test --node

# Serve examples locally
cd examples/contact-form
wasm-pack build --target web
npm run serve
# Open http://localhost:8000
```

### 3.2 Development Scripts

#### Setup Verification (`scripts/verify-setup.sh`)

```bash
#!/bin/bash
set -e

echo "🔍 Verifying development setup..."

# Check Rust toolchain
echo "📦 Checking Rust toolchain..."
rustc --version
cargo --version

# Check required targets
echo "🎯 Checking WASM target..."
rustup target list --installed | grep wasm32-unknown-unknown || {
    echo "❌ WASM target not installed"
    exit 1
}

# Check required tools
echo "🔧 Checking required tools..."
wasm-pack --version || {
    echo "❌ wasm-pack not installed"
    exit 1
}

# Check Node.js
echo "📦 Checking Node.js..."
node --version
npm --version

# Build project
echo "🏗️ Testing project build..."
cargo check --all-features

# Build WASM
echo "🌐 Testing WASM build..."
wasm-pack build --target web

# Run tests
echo "🧪 Running test suite..."
cargo test --all-features

# Check examples
echo "📝 Checking examples..."
cd examples/contact-form
npm install
npm run build
cd ../..

echo "✅ Development setup verified!"
```

#### Pre-PR Check (`scripts/pre-pr-check.sh`)

```bash
#!/bin/bash
set -e

echo "🚦 Running pre-PR checks..."

# Format check
echo "📐 Checking code formatting..."
cargo fmt --all -- --check

# Clippy
echo "📎 Running Clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Tests
echo "🧪 Running tests..."
cargo test --all-features

# Documentation
echo "📚 Checking documentation..."
cargo doc --all-features --no-deps

# WASM build
echo "🌐 Building WASM..."
wasm-pack build --target web

# Examples
echo "📝 Testing examples..."
cd examples/contact-form
npm run build
cd ../..

# Audit
echo "🛡️ Security audit..."
cargo audit

echo "✅ All pre-PR checks passed!"
```

## 4. Code Style Guidelines

### 4.1 Rust Style Guide

#### Naming Conventions

```rust
// Types: PascalCase
struct FormHandle<T> {}
enum ValidationMode {}
trait FormValidation {}

// Functions and variables: snake_case
fn validate_field() {}
let field_name = "email";

// Constants: SCREAMING_SNAKE_CASE
const MAX_FIELD_LENGTH: usize = 1000;

// Modules: snake_case
mod form_validation;
```

#### Code Organization

```rust
// File structure
src/
├── lib.rs              // Public API exports
├── core/               // Core functionality
│   ├── mod.rs
│   ├── traits.rs       // Form trait definition
│   └── types.rs        // Core types
├── validation/         // Validation system
│   ├── mod.rs
│   ├── validators.rs   // Built-in validators
│   └── schema.rs       // Schema validation
├── components/         // UI components
│   ├── mod.rs
│   ├── form.rs         // Form component
│   └── field.rs        // Field components
└── utils/              // Utilities
    ├── mod.rs
    └── testing.rs      // Testing utilities

// Import organization
use std::collections::HashMap;    // Standard library
use serde::{Serialize, Deserialize}; // External crates
use leptos::*;                    // Framework
use crate::core::Form;            // Internal crate imports
```

#### Documentation Standards

````rust
/// Creates a new form handle with the specified configuration.
///
/// # Examples
///
/// ```rust
/// use leptos_forms::*;
///
/// #[derive(Form, Clone, Serialize, Deserialize)]
/// struct ContactForm {
///     #[form(validators(required, email))]
///     email: String,
/// }
///
/// let form = use_form::<ContactForm>(None, FormOptions::default());
/// ```
///
/// # Panics
///
/// This function will panic if called outside of a Leptos reactive context.
///
/// # Errors
///
/// Returns `FormError` if initial validation fails.
pub fn use_form<T: Form>(
    initial_values: Option<T>,
    options: FormOptions<T>,
) -> FormHandle<T> {
    // Implementation
}
````

### 4.2 Component Style Guide

#### Component Structure

```rust
/// Contact form component with validation and submission handling.
#[component]
pub fn ContactForm(
    /// Initial form values
    #[prop(optional)] initial_values: Option<ContactFormData>,

    /// Form submission handler
    #[prop(into)] on_submit: Callback<ContactFormData>,

    /// Additional CSS classes
    #[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView {
    let form = use_form::<ContactFormData>(
        initial_values,
        FormOptions {
            validation_mode: ValidationMode::OnBlur,
            on_submit: Some(on_submit),
            ..Default::default()
        },
    );

    view! {
        <Form form_handle=form class=class>
            // Component content
        </Form>
    }
}
```

#### Props Documentation

```rust
/// Props for the FormField component.
#[derive(Debug, Clone, PartialEq)]
pub struct FormFieldProps {
    /// Field name (required)
    pub name: String,

    /// Field label text (required)
    pub label: String,

    /// Optional description text
    pub description: Option<String>,

    /// Whether the field is required
    #[prop(optional)]
    pub required: bool,

    /// Additional CSS classes
    #[prop(optional, into)]
    pub class: Option<AttributeValue>,
}
```

### 4.3 Testing Style Guide

#### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    mod form_handle_tests {
        use super::*;

        #[test]
        fn test_field_registration_creates_signals() {
            // Arrange
            let form = create_test_form(TestForm::default());

            // Act
            let field = form.register.call("test_field".to_string());

            // Assert
            assert_eq!(field.name, "test_field");
            assert!(field.error.get().is_none());
        }
    }

    mod validation_tests {
        use super::*;

        #[test]
        fn test_email_validator_rejects_invalid_format() {
            // Arrange
            let invalid_email = FieldValue::String("invalid".to_string());

            // Act
            let result = validators::email(&invalid_email);

            // Assert
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Invalid email format");
        }
    }
}
```

#### Test Utilities

```rust
// tests/utils/mod.rs
pub fn create_test_form<T: Form>(initial: T) -> FormHandle<T> {
    use_form(Some(initial), FormOptions::default())
}

pub fn simulate_user_input(field: &FieldRegistration, value: &str) {
    let event = create_mock_input_event(value);
    field.on_input.call(event);
}

pub fn assert_field_has_error(field: &FieldRegistration, expected_error: &str) {
    let error = field.error.get();
    assert!(error.is_some(), "Expected field to have error");
    assert_eq!(error.unwrap(), expected_error);
}
```

## 5. Performance Development Guidelines

### 5.1 Performance Monitoring

```rust
// Performance testing in development
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_field_update_performance() {
        let form = create_large_test_form();

        let start = Instant::now();
        form.set_field_value.call(("field_name".to_string(), FieldValue::String("value".to_string())));
        let duration = start.elapsed();

        assert!(duration.as_millis() < 1, "Field update took {}ms, expected <1ms", duration.as_millis());
    }
}
```

### 5.2 Bundle Size Monitoring

```bash
# Check bundle size during development
wasm-pack build --release --target web
wc -c pkg/leptos_forms_bg.wasm

# With compression
gzip -k pkg/leptos_forms_bg.wasm
wc -c pkg/leptos_forms_bg.wasm.gz

# Bundle analysis
wasm-twiggy top pkg/leptos_forms_bg.wasm
```

## 6. Debugging Guidelines

### 6.1 Development Debugging

```rust
// Enable debug logging
#[cfg(debug_assertions)]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        web_sys::console::log_1(&format!($($arg)*).into());
    };
}

#[cfg(not(debug_assertions))]
macro_rules! debug_log {
    ($($arg:tt)*) => {};
}

// Usage in code
debug_log!("Field {} updated with value: {:?}", field_name, value);
```

### 5.2 Browser DevTools Integration

```rust
// Add debug information to form handle
impl<T: Form> FormHandle<T> {
    #[cfg(debug_assertions)]
    pub fn debug_info(&self) -> FormDebugInfo {
        FormDebugInfo {
            values: format!("{:?}", self.values.get()),
            errors: self.errors.get(),
            touched_fields: self.touched.get().clone(),
            dirty_fields: self.dirty_fields.get().clone(),
        }
    }
}
```

## 7. Release Preparation

### 7.1 Pre-release Checklist

```markdown
## Pre-release Checklist

### Code Quality

- [ ] All tests passing (100%)
- [ ] Test coverage >95%
- [ ] No Clippy warnings
- [ ] Documentation complete
- [ ] Examples working

### Performance

- [ ] Bundle size <15KB gzipped
- [ ] Field update time <1ms
- [ ] Form validation time <2ms
- [ ] Memory usage reasonable

### Security

- [ ] Security audit passing
- [ ] No known vulnerabilities
- [ ] Dependencies up to date

### Documentation

- [ ] Changelog updated
- [ ] Version bumped
- [ ] Migration guide (if breaking changes)
- [ ] Examples updated

### Release

- [ ] Git tag created
- [ ] Release notes prepared
- [ ] crates.io ready
- [ ] Documentation site updated
```

### 7.2 Version Bumping

```bash
# Bump version in all relevant files
cargo install cargo-edit
cargo set-version 1.2.3

# Update CHANGELOG.md
# Update README.md if needed
# Update examples/*/Cargo.toml

# Commit version bump
git add .
git commit -m "chore: bump version to 1.2.3"
git tag v1.2.3
git push origin main --tags
```

This development workflow ensures consistent, high-quality code development with clear processes for collaboration, testing, and release management.

---

**Document Control**

- **Created**: 2025-01-02
- **Last Modified**: 2025-01-02
- **Next Review**: Quarterly
- **Version**: 1.0
- **Classification**: Development Guide
