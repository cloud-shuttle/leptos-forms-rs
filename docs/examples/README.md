# Examples

This directory contains comprehensive examples demonstrating how to use Leptos Forms RS in various scenarios.

## 📚 **Example Categories**

### **🚀 Getting Started**

- [**Basic Form**](./basic-form/) - Simple form with validation
- [**User Registration**](./user-registration/) - Complete registration form
- [**Contact Form**](./contact-form/) - Multi-field contact form

### **🎨 Form Components**

- [**Input Types**](./input-types/) - All available input components
- [**Custom Components**](./custom-components/) - Building custom form components
- [**Component Library**](./component-library/) - Pre-built component showcase

### **✅ Validation Patterns**

- [**Basic Validation**](./basic-validation/) - Required fields and simple rules
- [**Advanced Validation**](./advanced-validation/) - Custom validation functions
- [**Conditional Validation**](./conditional-validation/) - Field dependencies
- [**Real-time Validation**](./real-time-validation/) - Live validation feedback

### **🔄 Advanced Patterns**

- [**Field Arrays**](./field-arrays/) - Dynamic lists of fields
- [**Conditional Fields**](./conditional-fields/) - Show/hide based on state
- [**Multi-step Forms**](./multi-step-forms/) - Wizard-style forms
- [**Form Persistence**](./form-persistence/) - Save/restore form data

### **📱 Mobile & Accessibility**

- [**Mobile Forms**](./mobile-forms/) - Touch-friendly form design
- [**Accessible Forms**](./accessible-forms/) - ARIA and screen reader support
- [**Keyboard Navigation**](./keyboard-navigation/) - Keyboard-only form usage

### **🔧 Integration Examples**

- [**API Integration**](./api-integration/) - Forms with backend APIs
- [**State Management**](./state-management/) - Forms with global state
- [**Routing Integration**](./routing-integration/) - Forms with navigation

### **🧪 Testing Examples**

- [**Unit Testing**](./unit-testing/) - Testing form logic
- [**Integration Testing**](./integration-testing/) - Testing form components
- [**E2E Testing**](./e2e-testing/) - Browser testing examples

## 🚀 **Running Examples**

### **Prerequisites**

```bash
# Install dependencies
pnpm install

# Build the project
cargo build

# Start the development server
pnpm run dev
```

### **Available Examples**

```bash
# Run basic form example
pnpm run dev:basic

# Run complex form example
pnpm run dev:complex

# Run all examples
pnpm run dev:all
```

## 📖 **Example Structure**

Each example follows this structure:

```
example-name/
├── README.md              # Example description and usage
├── src/
│   ├── main.rs           # Main application entry point
│   ├── form.rs           # Form data structures
│   ├── components.rs     # Form components
│   └── validation.rs     # Custom validation logic
├── tests/                # Example-specific tests
├── styles/               # CSS and styling
└── assets/               # Images and other assets
```

## 🎯 **Learning Path**

### **Beginner**

1. Start with [Basic Form](./basic-form/)
2. Learn [Input Types](./input-types/)
3. Understand [Basic Validation](./basic-validation/)

### **Intermediate**

1. Explore [Field Arrays](./field-arrays/)
2. Build [Custom Components](./custom-components/)
3. Implement [Conditional Fields](./conditional-fields/)

### **Advanced**

1. Master [Multi-step Forms](./multi-step-forms/)
2. Optimize [Performance](./performance-optimization/)
3. Integrate with [APIs](./api-integration/)

## 🔍 **Finding Examples**

### **By Feature**

Looking for a specific feature? Use these tags:

- **Validation**: `#validation`, `#custom-validation`, `#real-time`
- **Components**: `#components`, `#custom-components`, `#input-types`
- **Patterns**: `#field-arrays`, `#conditional`, `#multi-step`
- **Integration**: `#api`, `#state`, `#routing`
- **Testing**: `#unit-testing`, `#integration`, `#e2e`

### **By Complexity**

- **Simple**: Basic forms, single validation rules
- **Medium**: Multiple fields, custom validation, field arrays
- **Complex**: Multi-step forms, conditional logic, API integration

## 🧪 **Testing Examples**

Each example includes comprehensive tests:

```bash
# Run tests for specific example
cd examples/basic-form
cargo test

# Run all example tests
pnpm run test:examples

# Run E2E tests for examples
pnpm run test:e2e --grep "Basic Form"
```

## 📝 **Contributing Examples**

Want to add an example? See our [Contributing Guide](../contributing.md) and:

1. **Create the example** in the appropriate category
2. **Include comprehensive tests** (unit + E2E)
3. **Write clear documentation** with usage examples
4. **Follow the example structure** outlined above
5. **Ensure accessibility** and mobile compatibility

## 🤝 **Getting Help**

- **Example Issues**: Check if your question is answered in an example
- **Documentation**: Review the [API Reference](../api-reference.md)
- **Community**: Ask in [GitHub Discussions](https://github.com/your-org/leptos-forms-rs/discussions)
- **Examples**: Look at similar examples for patterns

## 📚 **Additional Resources**

- [**Getting Started Guide**](../getting-started.md) - Quick start tutorial
- [**API Reference**](../api-reference.md) - Complete API documentation
- [**Testing Strategy**](../testing-strategy.md) - Testing best practices
- [**Performance Guide**](../performance-guide.md) - Optimization techniques

---

**Happy form building!** 🎉

Start with the examples that match your skill level and gradually work your way up to more complex patterns. Each example is designed to be self-contained and educational.
