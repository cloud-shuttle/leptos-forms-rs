# API Design Document

**Project**: Leptos Forms Library
**Version**: 1.0
**Date**: 2025-01-02
**Status**: Draft

## 1. API Design Principles

### 1.1 Core Principles

- **Type Safety First**: All APIs leverage Rust's type system for compile-time guarantees
- **Ergonomic by Default**: Common use cases require minimal code
- **Composable**: APIs work together seamlessly
- **Performance Conscious**: Zero-cost abstractions wherever possible
- **Future Compatible**: Designed for extensibility without breaking changes

### 1.2 API Categories

1. **Core APIs**: Essential form handling functionality
2. **Component APIs**: UI component integration
3. **Validation APIs**: Form and field validation
4. **Utility APIs**: Helper functions and convenience methods
5. **Extension APIs**: Plugin and customization interfaces

## 2. Public API Surface

### 2.1 Core Form API

#### Primary Entry Point

```rust
/// Main hook for form state management
pub fn use_form<T: Form>(
    initial_values: Option<T>,
    options: FormOptions<T>,
) -> FormHandle<T>
where
    T: Form + Clone + 'static;

/// Form configuration options
#[derive(Clone)]
pub struct FormOptions<T: Form> {
    /// Validation timing strategy
    pub validation_mode: ValidationMode,

    /// Form submission handler
    pub on_submit: Option<Callback<T>>,

    /// Async form submission handler
    pub on_submit_async: Option<AsyncCallback<T, Result<(), ValidationErrors>>>,

    /// Enable auto-save functionality
    pub auto_save: AutoSaveConfig,

    /// Cache configuration
    pub cache_config: CacheConfig,
}

/// Validation timing options
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationMode {
    /// Validate on form submission only
    OnSubmit,

    /// Validate when field loses focus
    OnBlur,

    /// Validate on every field change
    OnChange,

    /// Custom validation timing
    Custom(ValidationTiming),
}
```

#### Form Handle Interface

```rust
/// Primary interface for form state and actions
#[derive(Clone)]
pub struct FormHandle<T: Form> {
    // === State Signals ===
    /// Current form values
    pub values: ReadSignal<T>,

    /// Form validation errors
    pub errors: ReadSignal<ValidationErrors>,

    /// Fields that have been touched by user
    pub touched: ReadSignal<HashSet<String>>,

    /// Fields that have been modified from initial values
    pub dirty_fields: ReadSignal<HashSet<String>>,

    /// Whether form is currently being submitted
    pub is_submitting: ReadSignal<bool>,

    /// Number of submission attempts
    pub submit_count: ReadSignal<u32>,

    // === Derived State ===
    /// True if form has no validation errors
    pub is_valid: Signal<bool>,

    /// True if any field has been modified
    pub is_dirty: Signal<bool>,

    /// True if any field has been touched
    pub is_touched: Signal<bool>,

    /// True if form can be submitted (valid and not submitting)
    pub can_submit: Signal<bool>,

    // === Actions ===
    /// Set entire form values
    pub set_values: WriteSignal<T>,

    /// Update specific field value
    pub set_field_value: Callback<(String, FieldValue)>,

    /// Set error for specific field
    pub set_field_error: Callback<(String, String)>,

    /// Clear error for specific field
    pub clear_field_error: Callback<String>,

    /// Mark field as touched
    pub touch_field: Callback<String>,

    /// Reset form to initial state
    pub reset: Callback<()>,

    /// Reset specific field to initial value
    pub reset_field: Callback<String>,

    /// Trigger form validation
    pub validate: Callback<()>,

    /// Trigger validation for specific field
    pub validate_field: Callback<String>,

    /// Submit form programmatically
    pub submit: Callback<()>,

    /// Register a field for form management
    pub register: Callback<String, FieldRegistration>,

    /// Unregister a field
    pub unregister: Callback<String>,
}
```

### 2.2 Field Registration API

#### Field Registration Interface

```rust
/// Registration object returned for each form field
#[derive(Clone)]
pub struct FieldRegistration {
    // === Identity ===
    /// Field name
    pub name: String,

    /// Generated field ID for HTML elements
    pub id: String,

    /// Field type information
    pub field_type: FieldType,

    // === State Signals ===
    /// Current field value
    pub value: Signal<FieldValue>,

    /// Current field error (if any)
    pub error: Signal<Option<String>>,

    /// Whether field has been touched
    pub is_touched: Signal<bool>,

    /// Whether field has been modified
    pub is_dirty: Signal<bool>,

    /// Whether field is currently focused
    pub is_focused: Signal<bool>,

    /// Whether field is required
    pub is_required: Signal<bool>,

    /// Whether field is disabled
    pub is_disabled: Signal<bool>,

    // === Event Handlers ===
    /// Input event handler (real-time changes)
    pub on_input: Callback<Event>,

    /// Change event handler (committed changes)
    pub on_change: Callback<Event>,

    /// Focus event handler
    pub on_focus: Callback<FocusEvent>,

    /// Blur event handler
    pub on_blur: Callback<FocusEvent>,

    // === DOM Properties ===
    /// Properties ready for DOM element binding
    pub props: FieldProps,
}

/// DOM properties for direct element binding
#[derive(Clone)]
pub struct FieldProps {
    pub id: String,
    pub name: String,
    pub value: Signal<String>,
    pub disabled: Signal<bool>,
    pub required: Signal<bool>,
    pub aria_invalid: Signal<bool>,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_required: Signal<bool>,
}
```

### 2.3 Component API

#### Form Container Components

```rust
/// Root form component with submission handling
#[component]
pub fn Form<T: Form>(
    /// Form handle from use_form
    form_handle: FormHandle<T>,

    /// Additional CSS classes
    #[prop(optional, into)] class: Option<AttributeValue>,

    /// Prevent default form submission
    #[prop(optional)] prevent_default: bool,

    /// Form encoding type
    #[prop(optional)] enctype: Option<String>,

    /// Form submission method
    #[prop(optional)] method: Option<String>,

    /// Form action URL
    #[prop(optional)] action: Option<String>,

    /// Child components
    children: Children,
) -> impl IntoView;

/// Form field wrapper with label and error display
#[component]
pub fn FormField(
    /// Field name
    #[prop(into)] name: String,

    /// Field label
    #[prop(into)] label: String,

    /// Optional field description
    #[prop(optional)] description: Option<String>,

    /// Whether field is required
    #[prop(optional)] required: bool,

    /// Additional CSS classes
    #[prop(optional, into)] class: Option<AttributeValue>,

    /// Custom error display component
    #[prop(optional)] error_component: Option<ErrorComponent>,

    /// Child input components
    children: Children,
) -> impl IntoView;
```

#### Input Components

```rust
/// Text input component
#[component]
pub fn TextInput(
    /// Field name
    #[prop(into)] name: String,

    /// Input type (text, email, password, etc.)
    #[prop(optional)] input_type: Option<String>,

    /// Placeholder text
    #[prop(optional)] placeholder: Option<String>,

    /// Autocomplete attribute
    #[prop(optional)] autocomplete: Option<String>,

    /// Whether input is disabled
    #[prop(optional)] disabled: Option<bool>,

    /// Additional CSS classes
    #[prop(optional, into)] class: Option<AttributeValue>,

    /// Additional HTML attributes
    #[prop(spread)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView;

/// Textarea component
#[component]
pub fn TextArea(
    /// Field name
    #[prop(into)] name: String,

    /// Number of visible rows
    #[prop(optional)] rows: Option<u32>,

    /// Number of visible columns
    #[prop(optional)] cols: Option<u32>,

    /// Placeholder text
    #[prop(optional)] placeholder: Option<String>,

    /// Whether textarea is disabled
    #[prop(optional)] disabled: Option<bool>,

    /// Additional CSS classes
    #[prop(optional, into)] class: Option<AttributeValue>,

    /// Additional HTML attributes
    #[prop(spread)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView;

/// Select dropdown component
#[component]
pub fn Select<T: SelectOption>(
    /// Field name
    #[prop(into)] name: String,

    /// Available options
    #[prop(into)] options: Signal<Vec<T>>,

    /// Placeholder text
    #[prop(optional)] placeholder: Option<String>,

    /// Allow multiple selections
    #[prop(optional)] multiple: bool,

    /// Whether select is disabled
    #[prop(optional)] disabled: Option<bool>,

    /// Additional CSS classes
    #[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView;

/// Checkbox component
#[component]
pub fn Checkbox(
    /// Field name
    #[prop(into)] name: String,

    /// Whether checkbox is disabled
    #[prop(optional)] disabled: Option<bool>,

    /// Additional CSS classes
    #[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView;

/// Radio button group component
#[component]
pub fn RadioGroup<T: RadioOption>(
    /// Field name
    #[prop(into)] name: String,

    /// Available radio options
    #[prop(into)] options: Signal<Vec<T>>,

    /// Layout direction
    #[prop(optional)] direction: Option<Direction>,

    /// Whether radio group is disabled
    #[prop(optional)] disabled: Option<bool>,

    /// Additional CSS classes
    #[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView;
```

### 2.4 Advanced Components API

#### Dynamic Field Components

```rust
/// Dynamic array of form fields
#[component]
pub fn FieldArray<T: Clone + Default + 'static>(
    /// Field name
    #[prop(into)] name: String,

    /// Render function for each array item
    render_item: Callback<(usize, Signal<T>, ArrayHelpers<T>), View>,

    /// Minimum number of items
    #[prop(optional)] min_items: Option<usize>,

    /// Maximum number of items
    #[prop(optional)] max_items: Option<usize>,

    /// Default value for new items
    #[prop(optional)] default_value: Option<T>,

    /// Additional CSS classes
    #[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView;

/// Helper methods for array field manipulation
#[derive(Clone)]
pub struct ArrayHelpers<T> {
    /// Add item to end of array
    pub push: Callback<T>,

    /// Add item to beginning of array
    pub unshift: Callback<T>,

    /// Insert item at specific index
    pub insert: Callback<(usize, T)>,

    /// Remove item at index
    pub remove: Callback<usize>,

    /// Move item from one index to another
    pub move_item: Callback<(usize, usize)>,

    /// Swap two items
    pub swap: Callback<(usize, usize)>,

    /// Clear all items
    pub clear: Callback<()>,
}
```

#### Conditional Field Components

```rust
/// Conditionally render fields based on form state
#[component]
pub fn ConditionalField<T: Form>(
    /// Condition function that determines visibility
    when: Signal<bool>,

    /// Whether to unmount component when hidden
    #[prop(optional)] unmount_on_hide: bool,

    /// Animation configuration for show/hide
    #[prop(optional)] animate: Option<AnimationConfig>,

    /// Child components to conditionally render
    children: Children,
) -> impl IntoView;

/// Multi-step form wizard component
#[component]
pub fn FormWizard<T: Form>(
    /// Form handle
    form_handle: FormHandle<T>,

    /// Wizard steps configuration
    steps: Vec<WizardStep<T>>,

    /// Show progress indicator
    #[prop(optional)] show_progress: bool,

    /// Allow clicking on steps to navigate
    #[prop(optional)] allow_step_clicking: bool,

    /// Callback when wizard is completed
    #[prop(optional)] on_complete: Option<Callback<T>>,

    /// Additional CSS classes
    #[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView;

/// Configuration for a single wizard step
#[derive(Clone)]
pub struct WizardStep<T: Form> {
    /// Step identifier
    pub id: String,

    /// Step title
    pub title: String,

    /// Optional step description
    pub description: Option<String>,

    /// Fields included in this step
    pub fields: Vec<String>,

    /// Custom validation for this step
    pub validate: Option<Callback<T, Result<(), ValidationErrors>>>,

    /// Whether step can be skipped
    pub skippable: bool,
}
```

### 2.5 Validation API

#### Core Validation Interface

```rust
/// Main trait for form validation
pub trait Form: Clone + Serialize + DeserializeOwned + 'static {
    /// Get field metadata for runtime introspection
    fn field_metadata() -> Vec<FieldMetadata>;

    /// Validate the entire form
    fn validate(&self) -> Result<(), ValidationErrors>;

    /// Validate a specific field
    fn validate_field(&self, field_name: &str) -> Result<(), String>;

    /// Get field value by name
    fn get_field(&self, name: &str) -> Option<FieldValue>;

    /// Set field value by name
    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), FieldError>;

    /// Get default form values
    fn default_values() -> Self;

    /// Get field dependencies
    fn field_dependencies(field_name: &str) -> Vec<String> {
        Vec::new() // Default implementation
    }
}

/// Validation errors container
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationErrors {
    /// Field-specific errors
    pub field_errors: HashMap<String, String>,

    /// Form-level errors
    pub form_errors: Vec<String>,
}

impl ValidationErrors {
    /// Create new empty error container
    pub fn new() -> Self;

    /// Check if any errors exist
    pub fn is_empty(&self) -> bool;

    /// Add field error
    pub fn add_field_error(&mut self, field: String, message: String);

    /// Add form-level error
    pub fn add_form_error(&mut self, message: String);

    /// Clear field error
    pub fn clear_field(&mut self, field: &str);

    /// Clear all errors
    pub fn clear(&mut self);

    /// Get error for specific field
    pub fn get_field_error(&self, field: &str) -> Option<&String>;
}
```

#### Built-in Validators

```rust
/// Built-in validation functions
pub mod validators {
    use super::*;

    /// Require field to have a value
    pub fn required(value: &FieldValue) -> Result<(), String>;

    /// Validate email format
    pub fn email(value: &FieldValue) -> Result<(), String>;

    /// Validate URL format
    pub fn url(value: &FieldValue) -> Result<(), String>;

    /// Validate minimum string length
    pub fn min_length(min: usize) -> impl Fn(&FieldValue) -> Result<(), String>;

    /// Validate maximum string length
    pub fn max_length(max: usize) -> impl Fn(&FieldValue) -> Result<(), String>;

    /// Validate numeric minimum
    pub fn min_value(min: f64) -> impl Fn(&FieldValue) -> Result<(), String>;

    /// Validate numeric maximum
    pub fn max_value(max: f64) -> impl Fn(&FieldValue) -> Result<(), String>;

    /// Validate against regex pattern
    pub fn pattern(regex: &str) -> Result<impl Fn(&FieldValue) -> Result<(), String>, RegexError>;

    /// Custom validation function
    pub fn custom<F>(validator: F) -> impl Fn(&FieldValue) -> Result<(), String>
    where
        F: Fn(&FieldValue) -> Result<(), String>;

    /// Async validation function
    pub fn async_custom<F, Fut>(validator: F) -> impl Fn(FieldValue) -> Fut
    where
        F: Fn(FieldValue) -> Fut,
        Fut: Future<Output = Result<(), String>>;
}

/// Schema-based validation builder
pub struct ValidationSchema<T: Form> {
    rules: HashMap<String, Vec<ValidationRule>>,
    form_rules: Vec<FormValidationRule<T>>,
}

impl<T: Form> ValidationSchema<T> {
    /// Create new validation schema
    pub fn new() -> Self;

    /// Add field validation rule
    pub fn field<R>(mut self, field_name: &str, rule: R) -> Self
    where
        R: Into<ValidationRule>;

    /// Add form-level validation rule
    pub fn form_rule<F>(mut self, rule: F) -> Self
    where
        F: Fn(&T) -> Result<(), String> + 'static;

    /// Validate form against schema
    pub fn validate(&self, form: &T) -> Result<(), ValidationErrors>;
}
```

### 2.6 Utility APIs

#### Form State Utilities

```rust
/// Utilities for form state management
pub mod utils {
    use super::*;

    /// Create form with initial values and options
    pub fn create_form<T: Form>(
        initial: Option<T>,
        options: FormOptions<T>,
    ) -> FormHandle<T>;

    /// Clone form state for comparison
    pub fn clone_form_state<T: Form>(form: &T) -> T;

    /// Compare form states for equality
    pub fn forms_equal<T: Form + PartialEq>(a: &T, b: &T) -> bool;

    /// Get form changes between two states
    pub fn form_diff<T: Form>(original: &T, current: &T) -> FormDiff;

    /// Reset form fields to default values
    pub fn reset_form<T: Form>(form: &mut T, fields: Option<Vec<String>>);

    /// Validate form with custom schema
    pub fn validate_with_schema<T: Form>(
        form: &T,
        schema: &ValidationSchema<T>,
    ) -> Result<(), ValidationErrors>;
}

/// Form state difference information
#[derive(Debug, Clone)]
pub struct FormDiff {
    /// Fields that have changed
    pub changed_fields: HashMap<String, FieldChange>,

    /// Fields that were added
    pub added_fields: Vec<String>,

    /// Fields that were removed
    pub removed_fields: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FieldChange {
    /// Previous value
    pub from: FieldValue,

    /// Current value
    pub to: FieldValue,
}
```

#### Integration Utilities

```rust
/// Utilities for UI library integration
pub mod integration {
    use super::*;

    /// Create field props for native HTML elements
    pub fn native_field_props(registration: &FieldRegistration) -> NativeFieldProps;

    /// Create field props for custom components
    pub fn custom_field_props<T>(registration: &FieldRegistration) -> CustomFieldProps<T>;

    /// Convert FieldValue to specific type
    pub fn field_value_to<T>(value: &FieldValue) -> Result<T, ConversionError>
    where
        T: TryFrom<FieldValue>;

    /// Convert specific type to FieldValue
    pub fn value_to_field_value<T>(value: T) -> FieldValue
    where
        T: Into<FieldValue>;

    /// Create event handler for custom input components
    pub fn create_input_handler(
        field_name: String,
        set_value: Callback<(String, FieldValue)>,
    ) -> Callback<InputEvent>;
}
```

## 3. Breaking Change Policy

### 3.1 Semantic Versioning

- **Major Version (1.0.0 → 2.0.0)**: Breaking API changes
- **Minor Version (1.0.0 → 1.1.0)**: New features, backward compatible
- **Patch Version (1.0.0 → 1.0.1)**: Bug fixes, no API changes

### 3.2 Breaking Change Guidelines

**What Constitutes a Breaking Change:**

- Removing public APIs or changing their signatures
- Changing behavior of existing APIs
- Changing default values that affect behavior
- Removing or renaming public types
- Changing trait bounds or generic constraints

**What Does NOT Constitute a Breaking Change:**

- Adding new optional parameters with defaults
- Adding new methods to traits (with default implementations)
- Adding new public APIs
- Internal implementation changes
- Performance improvements
- Bug fixes that restore documented behavior

### 3.3 Deprecation Process

1. **Mark as deprecated** with `#[deprecated]` attribute
2. **Provide migration path** in deprecation message
3. **Keep deprecated API** for at least one major version
4. **Remove in subsequent major version**

```rust
#[deprecated(
    since = "1.2.0",
    note = "Use `FormHandle::set_field_value` instead"
)]
pub fn update_field_value<T: Form>(
    form: &FormHandle<T>,
    field: &str,
    value: FieldValue,
) {
    form.set_field_value.call((field.to_string(), value));
}
```

## 4. Versioning Strategy

### 4.1 Release Channels

- **Stable**: Production-ready releases (1.0.0, 1.1.0, etc.)
- **Beta**: Pre-release testing (1.1.0-beta.1)
- **Alpha**: Early development (1.1.0-alpha.1)
- **Nightly**: Daily builds from main branch

### 4.2 Long Term Support (LTS)

- **LTS Versions**: Every 6 months (1.0 LTS, 1.6 LTS, etc.)
- **Support Duration**: 18 months of security updates
- **Migration Windows**: 12 months overlap between LTS versions

### 4.3 Version Compatibility Matrix

| Version | Rust  | Leptos | MSRV | Support Status |
| ------- | ----- | ------ | ---- | -------------- |
| 1.0.x   | 1.70+ | 0.6+   | 1.70 | LTS            |
| 0.9.x   | 1.65+ | 0.5+   | 1.65 | Maintenance    |
| 0.8.x   | 1.60+ | 0.4+   | 1.60 | End of Life    |

## 5. Migration Paths

### 5.1 From React Hook Form

```rust
// React Hook Form
const { register, handleSubmit, formState: { errors } } = useForm();

// Leptos Forms
let form = use_form::<MyForm>(None, FormOptions::default());
let field = (form.register)("field_name");
```

### 5.2 From Formik

```javascript
// Formik
<Formik initialValues={initialValues} onSubmit={handleSubmit}>
  <Field name="email" type="email" />
</Formik>

// Leptos Forms
view! {
    <Form form_handle=form>
        <TextInput name="email" input_type="email" />
    </Form>
}
```

### 5.3 Version Migration Guides

Each major version will include:

- **Breaking Changes**: Detailed list with migration instructions
- **New Features**: Guide to adopting new functionality
- **Performance Improvements**: Optimization opportunities
- **Code Examples**: Before/after code samples
- **Automated Migration Tools**: Where applicable

## 6. API Stability Guarantees

### 6.1 Stable APIs

- Core form handling (use_form, FormHandle)
- Basic components (Form, FormField, TextInput, etc.)
- Validation interface (Form trait, ValidationErrors)
- Built-in validators

### 6.2 Experimental APIs

- Advanced animations
- Plugin system
- DevTools integration
- Performance optimizations

Experimental APIs are marked with:

```rust
#[cfg(feature = "experimental")]
pub mod experimental {
    // Unstable APIs
}
```

## 7. Documentation Requirements

### 7.1 API Documentation Standards

- **100% Coverage**: All public APIs must have rustdoc comments
- **Examples**: Every public function includes usage example
- **Error Conditions**: Document all possible error states
- **Panics**: Document all panic conditions
- **Safety**: Document unsafe code and requirements

### 7.2 Example Requirements

````rust
/// Creates a new form handle with the specified initial values and options.
///
/// # Examples
///
/// ```rust
/// use leptos_forms::*;
///
/// #[derive(Form, Clone, Serialize, Deserialize)]
/// struct LoginForm {
///     #[form(validators(required, email))]
///     email: String,
///
///     #[form(validators(required, min_length = 8))]
///     password: String,
/// }
///
/// let form = use_form::<LoginForm>(
///     None,
///     FormOptions {
///         validation_mode: ValidationMode::OnBlur,
///         ..Default::default()
///     }
/// );
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

---

**Document Control**

- **Created**: 2025-01-02
- **Last Modified**: 2025-01-02
- **Next Review**: 2025-02-15
- **Version**: 1.0
- **Classification**: Public API Specification
