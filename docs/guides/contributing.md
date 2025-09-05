# Contributing to Leptos Forms RS

Thank you for your interest in contributing to Leptos Forms RS! This guide will help you get started with development and contributing to the project.

## 🚀 **Getting Started**

### **Prerequisites**

- **Rust** 1.89+ with WASM target support
- **Node.js** 18+ and pnpm
- **Git** for version control
- **Nix** (optional, for reproducible development environments)

### **Development Setup**

1. **Fork and clone the repository**

   ```bash
   git clone https://github.com/YOUR_USERNAME/leptos-forms-rs.git
   cd leptos-forms-rs
   ```

2. **Set up the development environment**

   ```bash
   # Install Rust WASM target
   rustup target add wasm32-unknown-unknown

   # Install wasm-pack
   cargo install wasm-pack

   # Install Node.js dependencies
   pnpm install
   ```

3. **Verify the setup**

   ```bash
   # Run tests to ensure everything works
   pnpm run test:e2e

   # Build the project
   cargo build
   ```

## 🏗️ **Project Structure**

```
leptos-forms-rs/
├── leptos-forms-rs/          # Main library crate
│   ├── src/
│   │   ├── lib.rs            # Library entry point
│   │   ├── core/             # Core form logic
│   │   ├── components/       # Pre-built form components
│   │   ├── validation/       # Validation system
│   │   ├── hooks/            # React-like hooks
│   │   ├── utils/            # Utility functions
│   │   └── error/            # Error types
│   └── Cargo.toml
├── leptos-forms-rs-macro/    # Procedural macro crate
├── examples/                  # Example applications
├── tests/                    # Test suite
│   ├── e2e/                 # End-to-end tests
│   └── unit/                # Unit tests
├── docs/                     # Documentation
└── Cargo.toml               # Workspace configuration
```

## 🧪 **Testing**

### **Test Philosophy**

We maintain a **100% test success rate** across all browsers. All contributions must include comprehensive tests.

### **Running Tests**

```bash
# Run all tests across all browsers
pnpm run test:e2e

# Run specific test suite
pnpm run test:e2e --grep "Form Components"

# Run tests in specific browser
pnpm run test:e2e --project=chromium

# Run tests with detailed output
pnpm run test:e2e --reporter=line

# Run Rust unit tests
cargo test

# Run WASM tests
wasm-pack test --node
```

### **Test Coverage Requirements**

- **Unit Tests**: 100% coverage for all public APIs
- **Integration Tests**: All components must have integration tests
- **E2E Tests**: All user workflows must be tested in real browsers
- **Cross-Browser**: Tests must pass in Chrome, Firefox, WebKit, Mobile Chrome, Mobile Safari

### **Writing Tests**

#### **Unit Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_validation() {
        let form = use_form::<UserForm>();

        // Test initial state
        assert!(!form.is_valid());

        // Test validation logic
        form.set_field_value("username", "john_doe");
        form.set_field_value("email", "john@example.com");

        assert!(form.is_valid());
    }

    #[test]
    fn test_custom_validation() {
        let form = use_form::<ValidatedForm>();

        // Test custom validation function
        form.set_field_value("email", "invalid@wrongdomain.com");
        assert!(!form.is_valid());
        assert!(form.field_error("email").is_some());
    }
}
```

#### **E2E Tests**

```rust
#[test]
fn test_form_submission() {
    let page = create_page();

    // Navigate to form
    page.goto("/form").await?;

    // Fill form fields
    page.fill("[name=username]", "testuser").await?;
    page.fill("[name=email]", "test@example.com").await?;

    // Submit form
    page.click("[type=submit]").await?;

    // Verify success
    expect(page.locator(".success-message")).toBeVisible();
}
```

## 📝 **Coding Standards**

### **Rust Code Style**

- Follow [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for common issues
- Maximum line length: 100 characters
- Use meaningful variable and function names

### **Documentation Standards**

- All public APIs must have comprehensive documentation
- Include examples in doc comments
- Use proper markdown formatting
- Document error conditions and edge cases

````rust
/// Creates a new form instance with the specified configuration.
///
/// # Examples
///
/// ```rust
/// let form = use_form::<UserForm>()
///     .with_persistence("user-data")
///     .with_validation_mode(ValidationMode::OnBlur);
/// ```
///
/// # Arguments
///
/// * `config` - Form configuration options
///
/// # Returns
///
/// A configured form handle
///
/// # Errors
///
/// Returns an error if the configuration is invalid
pub fn use_form_with_config<T: FormData>(
    config: FormConfig
) -> Result<FormHandle<T>, FormError> {
    // Implementation
}
````

### **Component Standards**

- Components should be accessible by default
- Include proper ARIA attributes
- Support keyboard navigation
- Be mobile-friendly
- Follow consistent prop naming conventions

```rust
#[component]
pub fn TextInput(
    name: String,
    label: String,
    form: FormHandle<T>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] required: bool,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    // Implementation with accessibility features
}
```

## 🔄 **Development Workflow**

### **1. Create a Feature Branch**

```bash
git checkout -b feature/your-feature-name
```

**Branch Naming Convention:**

- `feature/feature-name` - New features
- `fix/bug-description` - Bug fixes
- `docs/documentation-update` - Documentation changes
- `test/test-improvement` - Test improvements

### **2. Make Your Changes**

- Write code following the coding standards
- Include comprehensive tests
- Update documentation as needed
- Ensure all tests pass

### **3. Commit Your Changes**

```bash
git add .
git commit -m "feat: add custom validation support

- Add #[form(custom = "function")] attribute
- Support custom validation functions
- Include comprehensive tests
- Update documentation"
```

**Commit Message Format:**

```
type(scope): description

- Bullet point for changes
- Another bullet point

Closes #123
```

**Types:**

- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation
- `style` - Code style changes
- `refactor` - Code refactoring
- `test` - Test changes
- `chore` - Maintenance tasks

### **4. Push and Create Pull Request**

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub with:

- Clear description of changes
- Link to related issues
- Screenshots for UI changes
- Test results showing all tests pass

## 🧪 **Quality Assurance**

### **Pre-commit Checklist**

Before committing, ensure:

- [ ] All tests pass (`pnpm run test:e2e`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Clippy checks pass (`cargo clippy`)
- [ ] Documentation is updated
- [ ] No breaking changes (unless intentional)
- [ ] Cross-browser compatibility maintained

### **CI/CD Pipeline**

Our CI pipeline automatically:

- Runs all tests across all browsers
- Checks code formatting and linting
- Builds the project
- Generates documentation
- Publishes to crates.io (on release)

### **Performance Requirements**

- Forms should render in < 16ms (60 FPS)
- Validation should complete in < 100ms
- Bundle size should not increase significantly
- Memory usage should be reasonable

## 🐛 **Bug Reports**

### **Reporting Bugs**

When reporting bugs, include:

1. **Clear description** of the issue
2. **Steps to reproduce** the problem
3. **Expected behavior** vs actual behavior
4. **Environment details** (OS, browser, Rust version)
5. **Minimal reproduction** code
6. **Screenshots** or error messages

### **Bug Report Template**

````markdown
## Bug Description

Brief description of the issue

## Steps to Reproduce

1. Step 1
2. Step 2
3. Step 3

## Expected Behavior

What should happen

## Actual Behavior

What actually happens

## Environment

- OS: [e.g., macOS 14.0]
- Browser: [e.g., Chrome 120]
- Rust: [e.g., 1.89.0]
- Leptos: [e.g., 0.6]

## Reproduction Code

```rust
// Minimal code to reproduce
```
````

## Additional Context

Any other relevant information

````

## 💡 **Feature Requests**

### **Requesting Features**

When requesting features:

1. **Describe the use case** clearly
2. **Explain the benefits** to users
3. **Provide examples** of how it would work
4. **Consider alternatives** and trade-offs
5. **Show community interest** if applicable

### **Feature Request Template**

```markdown
## Feature Description
Clear description of the requested feature

## Use Case
Explain when and why this feature would be useful

## Proposed API
```rust
// Example of how the API would work
````

## Benefits

What problems does this solve?

## Alternatives Considered

What other approaches were considered?

## Implementation Notes

Any technical considerations or challenges

```

## 📚 **Documentation**

### **Documentation Standards**

- Write clear, concise explanations
- Include practical examples
- Use consistent terminology
- Keep documentation up-to-date
- Include troubleshooting sections

### **Documentation Types**

1. **API Documentation** - Comprehensive API reference
2. **User Guides** - Step-by-step tutorials
3. **Examples** - Working code examples
4. **Architecture** - System design and decisions
5. **Contributing** - Development guidelines

## 🤝 **Community Guidelines**

### **Code of Conduct**

- Be respectful and inclusive
- Welcome newcomers
- Provide constructive feedback
- Focus on technical merit
- Help others learn and grow

### **Communication Channels**

- **GitHub Issues** - Bug reports and feature requests
- **GitHub Discussions** - Questions and general discussion
- **Pull Requests** - Code reviews and collaboration
- **Discord** - Real-time chat and support

## 🏆 **Recognition**

### **Contributor Recognition**

Contributors are recognized through:

- **GitHub Contributors** page
- **Release notes** for significant contributions
- **Contributor spotlight** in documentation
- **Special thanks** for major features

### **Types of Contributions**

We welcome all types of contributions:

- **Code** - Features, bug fixes, improvements
- **Tests** - Test coverage and quality
- **Documentation** - Guides, examples, API docs
- **Design** - UI/UX improvements
- **Review** - Code review and feedback
- **Community** - Helping other users

## 🚀 **Getting Help**

### **Development Questions**

- Check existing documentation
- Search GitHub issues and discussions
- Ask in GitHub discussions
- Join our Discord community

### **Technical Support**

- Review the troubleshooting guide
- Check the FAQ section
- Look at similar issues
- Create a minimal reproduction

## 📋 **Checklist for Contributors**

Before submitting your contribution:

- [ ] Code follows project standards
- [ ] All tests pass locally
- [ ] Documentation is updated
- [ ] No breaking changes (unless intentional)
- [ ] Cross-browser compatibility maintained
- [ ] Performance impact considered
- [ ] Accessibility requirements met
- [ ] Security implications reviewed
- [ ] Commit messages follow convention
- [ ] Pull request description is clear

## 🙏 **Thank You**

Thank you for contributing to Leptos Forms RS! Your contributions help make the library better for everyone in the Leptos ecosystem.

Whether you're fixing a bug, adding a feature, improving documentation, or helping other users, every contribution is valuable and appreciated.

Happy coding! 🎉
```
