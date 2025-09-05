# Leptos Forms - System Design & Architecture

## Executive Summary

Leptos Forms is a type-safe, reactive form handling library for Leptos that provides compile-time validation, zero-cost abstractions, and seamless UI component integration. The system leverages Rust's type system to eliminate runtime errors and provide developer-friendly APIs.

## System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    User Interface Layer                 │
├─────────────────────────────────────────────────────────┤
│  Components  │  Hooks  │  Wizards  │  Conditional     │
│  - FormField │ - Form  │ - Multi   │  - Dynamic       │
│  - TextInput │ - Array │   Step    │  - Conditional   │
│  - FileInput │ - Field │ - Progress│  - Dependencies  │
├─────────────────────────────────────────────────────────┤
│                    Core Engine Layer                    │
├─────────────────────────────────────────────────────────┤
│  Form Trait  │ Validation│ State Mgmt│  Event System   │
│  - Metadata  │ - Rules   │ - Signals │  - Change       │
│  - Getters   │ - Errors  │ - Derived │  - Submit       │
│  - Setters   │ - Custom  │ - Touched │  - Blur/Focus   │
├─────────────────────────────────────────────────────────┤
│                   Macro System Layer                    │
├─────────────────────────────────────────────────────────┤
│  Derive Macro│ Validators│ Field Attr│  Code Gen       │
│  - Form      │ - Required│ - Types   │  - Impl         │
│  - Traits    │ - Email   │ - Rules   │  - Metadata     │
│  - Metadata  │ - Custom  │ - Props   │  - Validation   │
├─────────────────────────────────────────────────────────┤
│                    Foundation Layer                     │
├─────────────────────────────────────────────────────────┤
│    Leptos    │   Serde   │   Web     │    Testing      │
│  - Signals   │ - Traits  │ - Events  │  - Utils        │
│  - Effects   │ - JSON    │ - DOM API │  - Mocks        │
│  - Context   │ - Derive  │ - Files   │  - Assertions   │
└─────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Form Trait System (`core/traits.rs`)

**Purpose**: Central trait that all forms must implement for runtime introspection and type safety.

**Key Features**:

- Field metadata extraction
- Dynamic field access
- Validation orchestration
- Serialization support

**Implementation Strategy**:

```rust
pub trait Form: Clone + Serialize + DeserializeOwned + 'static {
    fn field_metadata() -> Vec<FieldMetadata>;
    fn validate(&self) -> Result<(), ValidationErrors>;
    fn get_field(&self, name: &str) -> Option<FieldValue>;
    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), FieldError>;
    fn default_values() -> Self;
}
```

### 2. Derive Macro System (`leptos-forms-macro/`)

**Purpose**: Automatic trait implementation with declarative field configuration.

**Code Generation**:

- Field metadata extraction from struct definitions
- Validation rule compilation
- Getter/setter method generation
- Type-safe field accessors

**Attribute Processing**:

- `#[form(validators(...))]` → validation rules
- `#[form(depends_on = [...])]` → field dependencies
- `#[form(skip)]` → field exclusion
- `#[form(default = ...)]` → default values

### 3. Reactive State Management (`hooks/use_form.rs`)

**Purpose**: Central state management with Leptos reactive primitives.

**State Architecture**:

```rust
pub struct FormHandle<T: Form> {
    // Core State
    values: ReadSignal<T>,
    errors: ReadSignal<ValidationErrors>,
    touched: ReadSignal<HashSet<String>>,
    dirty_fields: ReadSignal<HashSet<String>>,

    // Derived State
    is_valid: Signal<bool>,
    is_dirty: Signal<bool>,
    is_submitting: ReadSignal<bool>,

    // Actions
    set_field_value: Rc<dyn Fn(&str, FieldValue)>,
    register: Rc<dyn Fn(&str) -> FieldRegistration>,
    handle_submit: Rc<dyn Fn(web_sys::Event)>,
}
```

**Reactive Updates**:

- Value changes trigger validation
- Error state updates UI immediately
- Touch/dirty tracking for UX optimization
- Submission state for loading indicators

### 4. Validation Engine (`validation/`)

**Purpose**: Comprehensive validation with built-in and custom rules.

**Validation Types**:

- **Synchronous**: Immediate field validation
- **Asynchronous**: Server-side validation
- **Cross-field**: Dependencies between fields
- **Form-level**: Global form constraints

**Built-in Validators**:

- `Required`, `Email`, `Url`, `Pattern`
- `MinLength`, `MaxLength`, `Min`, `Max`
- `Custom`, `AsyncCustom`

**Validation Modes**:

- `OnChange`: Real-time validation
- `OnBlur`: Validate on field exit
- `OnSubmit`: Only validate on submission

## Component Architecture

### Field Registration System

**Purpose**: Connect form fields to reactive state with automatic prop generation.

**Flow**:

1. Component calls `form.register("field_name")`
2. System creates field-specific signals
3. Returns `FieldRegistration` with props and handlers
4. Component spreads props to input elements

**Benefits**:

- Type-safe field binding
- Automatic error handling
- Consistent event handling
- Accessibility compliance

### UI Component Integration

**Design Philosophy**:

- **Headless Core**: No UI assumptions in form logic
- **Component Agnostic**: Works with any UI library
- **Prop Spreading**: Easy integration with existing components
- **Accessibility First**: ARIA attributes included by default

**Integration Patterns**:

```rust
// Native HTML
<input {...form.register("email").props} />

// With shadcn-ui
<Input {...form.register("email")} />

// With radix-leptos
<TextField
    value={field.value}
    on_change={field.on_change}
    error={field.error}
/>
```

## Advanced Features

### 1. Array Fields (`fields/array.rs`)

**Purpose**: Dynamic lists with add/remove/reorder functionality.

**Capabilities**:

- Dynamic item management
- Validation per item
- Drag-and-drop reordering
- Nested form support

### 2. File Upload (`fields/file.rs`)

**Purpose**: File handling with progress tracking and validation.

**Features**:

- Multiple file support
- Size/type constraints
- Upload progress tracking
- Server-side processing

### 3. Multi-Step Forms (`wizard/`)

**Purpose**: Complex forms broken into manageable steps.

**Features**:

- Step navigation
- Progress indication
- Step-specific validation
- Data persistence across steps

### 4. Conditional Fields (`fields/conditional.rs`)

**Purpose**: Show/hide fields based on form state.

**Implementation**:

- Reactive field visibility
- Cleanup on hide (optional)
- Dependency tracking
- Validation updates

## Data Flow Architecture

### Form Lifecycle

```
1. Form Initialization
   ↓
2. Field Registration
   ↓
3. User Input
   ↓
4. Validation (if enabled)
   ↓
5. State Updates
   ↓
6. UI Re-render
   ↓
7. Form Submission
   ↓
8. Final Validation
   ↓
9. Success/Error Handling
```

### Event Flow

```
User Input → Event Handler → Field Update → Validation → State Change → UI Update
     ↑                                                           ↓
     └─────────────────── Error Display ←──────────────────────┘
```

## Testing Strategy

### Unit Testing

- Form trait implementations
- Validation rule testing
- State management logic
- Macro code generation

### Integration Testing

- Component interaction
- Form submission flows
- Error handling
- File upload processing

### E2E Testing

- Multi-step forms
- Complex validation scenarios
- Cross-browser compatibility
- Accessibility compliance

## Performance Considerations

### Optimization Strategies

**Compile-time Optimizations**:

- Zero-cost abstractions
- Macro-generated code optimization
- Dead code elimination
- Type-level guarantees

**Runtime Optimizations**:

- Reactive granularity
- Memoized computations
- Lazy validation
- Efficient DOM updates

**Bundle Size**:

- Tree-shakeable design
- Optional features
- Minimal dependencies
- Code splitting support

## Security Considerations

### Input Validation

- Client-side validation for UX
- Server-side validation required
- XSS prevention
- CSRF protection

### File Upload Security

- MIME type validation
- File size limits
- Virus scanning integration
- Secure storage

## Error Handling Strategy

### Error Types

- **Validation Errors**: User input issues
- **Network Errors**: API communication failures
- **System Errors**: Unexpected failures
- **User Errors**: Incorrect usage

### Error Recovery

- Graceful degradation
- Retry mechanisms
- User-friendly messages
- Developer debugging info

## Deployment & Distribution

### Package Structure

```
leptos-forms/
├── leptos-forms/          # Core library
├── leptos-forms-macro/    # Derive macros
├── leptos-forms-ui/       # Optional UI components
└── examples/              # Usage examples
```

### Version Strategy

- Semantic versioning
- Feature flags for breaking changes
- Migration guides
- Backward compatibility

## Future Roadmap

### Phase 1: Core Implementation

- Form trait system
- Basic validation
- Simple components
- Documentation

### Phase 2: Advanced Features

- Array fields
- File uploads
- Multi-step forms
- Testing utilities

### Phase 3: Ecosystem Integration

- UI library integrations
- Additional validators
- Developer tools
- Performance optimizations

### Phase 4: Enterprise Features

- Accessibility enhancements
- Internationalization
- Advanced testing
- Monitoring integration

## Success Metrics

### Developer Experience

- API simplicity score
- Documentation completeness
- Example coverage
- Community adoption

### Performance Metrics

- Bundle size impact
- Runtime performance
- Memory usage
- Compilation time

### Quality Metrics

- Test coverage (>95%)
- Type safety guarantees
- Error handling completeness
- Security audit results

This system design provides a solid foundation for building a production-ready form handling library that leverages Rust's type system for safety while providing an excellent developer experience with Leptos applications.
