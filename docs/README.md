# Leptos Forms RS Documentation

Welcome to the comprehensive documentation for Leptos Forms RS - the type-safe, reactive form handling library for Leptos applications.

## 🎯 **Project Status: Production Ready** ✅

- **✅ 100% Test Success Rate** - 210 tests passing across all browsers
- **✅ Cross-Browser Compatible** - Chrome, Firefox, WebKit, Mobile Chrome, Mobile Safari
- **✅ Leptos 0.6 Compatible** - Stable and production-ready
- **✅ Comprehensive E2E Testing** - Playwright-powered browser automation

## 📚 **Documentation Structure**

### **🚀 Getting Started**
- [**Quick Start Guide**](./getting-started.md) - Get up and running in minutes
- [**Installation Guide**](./installation.md) - Setup and configuration
- [**First Form**](./first-form.md) - Build your first form

### **📖 Core Documentation**
- [**API Reference**](./api-reference.md) - Complete API documentation
- [**Form Architecture**](./architecture/form-architecture.md) - Understanding the design
- [**Validation System**](./validation/validation-guide.md) - How validation works
- [**State Management**](./state-management.md) - Form state and reactivity

### **🎨 Components & Patterns**
- [**Form Components**](./components/form-components.md) - Pre-built components
- [**Custom Components**](./components/custom-components.md) - Building your own
- [**Form Patterns**](./patterns/form-patterns.md) - Common form patterns
- [**Advanced Patterns**](./patterns/advanced-patterns.md) - Complex form scenarios

### **✅ Validation & Error Handling**
- [**Validation Guide**](./validation/validation-guide.md) - Comprehensive validation
- [**Custom Validators**](./validation/custom-validators.md) - Building custom rules
- [**Error Handling**](./validation/error-handling.md) - Managing validation errors
- [**Real-time Validation**](./validation/real-time-validation.md) - Live feedback

### **🔄 Advanced Features**
- [**Field Arrays**](./advanced/field-arrays.md) - Dynamic lists of fields
- [**Conditional Fields**](./advanced/conditional-fields.md) - Show/hide based on state
- [**Multi-step Forms**](./advanced/multi-step-forms.md) - Wizard-style forms
- [**Form Persistence**](./advanced/form-persistence.md) - Save/restore form data

### **📱 Mobile & Accessibility**
- [**Mobile Forms**](./mobile/mobile-forms.md) - Touch-friendly design
- [**Accessibility Guide**](./accessibility/accessibility-guide.md) - ARIA and screen readers
- [**Keyboard Navigation**](./accessibility/keyboard-navigation.md) - Keyboard-only usage
- [**Touch Events**](./mobile/touch-events.md) - Mobile interaction handling

### **🔧 Integration & Performance**
- [**API Integration**](./integration/api-integration.md) - Forms with backend APIs
- [**State Management**](./integration/state-management.md) - Forms with global state
- [**Routing Integration**](./integration/routing-integration.md) - Forms with navigation
- [**Performance Guide**](./performance/performance-guide.md) - Optimization techniques

### **🧪 Testing & Quality**
- [**Testing Strategy**](./testing/testing-strategy.md) - Comprehensive testing approach
- [**Unit Testing**](./testing/unit-testing.md) - Testing form logic
- [**Integration Testing**](./testing/integration-testing.md) - Testing components
- [**E2E Testing**](./testing/e2e-testing.md) - Browser testing
- [**Test Examples**](./testing/test-examples.md) - Testing patterns

### **📖 Examples**
- [**Examples Directory**](./examples/) - Complete working examples
- [**Basic Forms**](./examples/basic-forms.md) - Simple form examples
- [**Complex Forms**](./examples/complex-forms.md) - Advanced form examples
- [**Component Examples**](./examples/components.md) - Component usage examples

### **🏗️ Architecture & Design**
- [**System Design**](./architecture/system-design.md) - High-level architecture
- [**Component Architecture**](./architecture/component-architecture.md) - Component design
- [**Data Flow**](./architecture/data-flow.md) - How data flows through forms
- [**Performance Architecture**](./architecture/performance-architecture.md) - Performance considerations

### **🛠️ Development & Contributing**
- [**Contributing Guide**](./contributing.md) - How to contribute
- [**Development Workflow**](./development/development-workflow.md) - Development practices
- [**CI/CD Pipeline**](./development/cicd-pipeline.md) - Automated testing and deployment
- [**Release Process**](./development/release-process.md) - How releases work

### **📊 Reference Materials**
- [**Changelog**](./reference/changelog.md) - Version history and changes
- [**Migration Guide**](./reference/migration-guide.md) - Upgrading between versions
- [**Compatibility Matrix**](./reference/compatibility.md) - Browser and platform support
- [**Performance Benchmarks**](./reference/benchmarks.md) - Performance metrics

## 🎯 **Quick Navigation**

### **By Use Case**

- **New to Forms**: Start with [Getting Started](./getting-started.md)
- **Building Components**: See [Form Components](./components/form-components.md)
- **Adding Validation**: Check [Validation Guide](./validation/validation-guide.md)
- **Mobile Development**: Visit [Mobile Forms](./mobile/mobile-forms.md)
- **Testing**: Explore [Testing Strategy](./testing/testing-strategy.md)

### **By Skill Level**

- **Beginner**: [Getting Started](./getting-started.md) → [First Form](./first-form.md) → [Basic Examples](./examples/)
- **Intermediate**: [Components](./components/) → [Patterns](./patterns/) → [Advanced Features](./advanced/)
- **Advanced**: [Architecture](./architecture/) → [Performance](./performance/) → [Contributing](./contributing.md)

### **By Feature**

- **Validation**: [Validation Guide](./validation/validation-guide.md)
- **Components**: [Form Components](./components/form-components.md)
- **State Management**: [State Management](./state-management.md)
- **Testing**: [Testing Strategy](./testing/testing-strategy.md)
- **Performance**: [Performance Guide](./performance/performance-guide.md)

## 🚀 **Getting Started Quickly**

### **1. Installation**

```toml
[dependencies]
leptos-forms-rs = "0.1.0"
leptos = "0.6"
```

### **2. Basic Form**

```rust
use leptos::*;
use leptos_forms_rs::*;

#[derive(Clone, Debug, FormData)]
pub struct UserForm {
    #[form(required)]
    username: String,
    
    #[form(required, email)]
    email: String,
}

#[component]
pub fn UserForm() -> impl IntoView {
    let form = use_form::<UserForm>();
    
    view! {
        <form on:submit=form.handle_submit>
            <input
                type="text"
                name="username"
                on:input=form.handle_input
                placeholder="Username"
                required
            />
            <input
                type="email"
                name="email"
                on:input=form.handle_input
                placeholder="Email"
                required
            />
            <button type="submit">"Submit"</button>
        </form>
    }
}
```

### **3. Run Tests**

```bash
# Install dependencies
pnpm install

# Run all tests
pnpm run test:e2e

# Start development
pnpm run dev
```

## 🔍 **Search Documentation**

Looking for something specific? Try these common searches:

- **"How to validate email"** → [Validation Guide](./validation/validation-guide.md)
- **"Custom components"** → [Custom Components](./components/custom-components.md)
- **"Field arrays"** → [Field Arrays](./advanced/field-arrays.md)
- **"Mobile forms"** → [Mobile Forms](./mobile/mobile-forms.md)
- **"Testing forms"** → [Testing Strategy](./testing/testing-strategy.md)
- **"Performance"** → [Performance Guide](./performance/performance-guide.md)

## 📚 **Additional Resources**

### **External Links**
- [**Leptos Framework**](https://leptos.dev/) - The web framework
- [**Rust Book**](https://doc.rust-lang.org/book/) - Learn Rust
- [**WASM Guide**](https://rustwasm.github.io/docs/book/) - WebAssembly with Rust
- [**Playwright**](https://playwright.dev/) - Browser testing framework

### **Community**
- [**GitHub Repository**](https://github.com/your-org/leptos-forms-rs)
- [**GitHub Discussions**](https://github.com/your-org/leptos-forms-rs/discussions)
- [**GitHub Issues**](https://github.com/your-org/leptos-forms-rs/issues)
- [**Discord Community**](https://discord.gg/leptos)

## 🤝 **Getting Help**

### **Documentation Issues**
- Found a typo? [Submit a PR](https://github.com/your-org/leptos-forms-rs/pulls)
- Missing information? [Open an issue](https://github.com/your-org/leptos-forms-rs/issues)
- Want to contribute? See [Contributing Guide](./contributing.md)

### **Technical Support**
- Check the [FAQ](./faq.md) for common questions
- Search existing [issues](https://github.com/your-org/leptos-forms-rs/issues)
- Ask in [GitHub Discussions](https://github.com/your-org/leptos-forms-rs/discussions)
- Join our [Discord](https://discord.gg/leptos) for real-time help

## 📈 **Documentation Status**

- **✅ Complete**: Core documentation, API reference, examples
- **🔄 In Progress**: Advanced patterns, performance optimization
- **📋 Planned**: Video tutorials, interactive examples, cookbook

## 🙏 **Contributing to Documentation**

We welcome contributions to improve our documentation! See our [Contributing Guide](./contributing.md) for details on:

- Writing documentation
- Improving examples
- Fixing typos and errors
- Adding new sections
- Translating documentation

---

**Happy form building!** 🎉

This documentation is designed to help you succeed with Leptos Forms RS at every level. Start with the basics and gradually explore more advanced features as you become comfortable with the library.
