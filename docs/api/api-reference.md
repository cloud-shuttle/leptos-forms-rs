# API Reference

Complete API documentation for Leptos Forms RS.

## 📚 **Table of Contents**

- [Core Types](#core-types)
- [Form Data Derive Macro](#form-data-derive-macro)
- [Form Handle](#form-handle)
- [Validation](#validation)
- [Components](#components)
- [Hooks](#hooks)
- [Utilities](#utilities)
- [Error Types](#error-types)

## 🏗️ **Core Types**

### **FormData**

The core trait that enables form functionality for your data structures.

```rust
pub trait FormData: Clone + Debug + 'static {
    type ValidationError;

    fn validate(&self) -> Result<(), ValidationErrors<Self::ValidationError>>;
    fn default_values() -> Self;
    fn field_names() -> Vec<String>;
}
```

**Implementations:**

- Automatically implemented by the `#[derive(FormData)]` macro
- Provides default implementations for common types
- Can be manually implemented for custom types

### **FormHandle**

The main form controller that manages form state and provides methods for interaction.

```rust
pub struct FormHandle<T: FormData> {
    // Internal state management
}
```

**Key Methods:**

- `new()` - Create a new form instance
- `with_persistence(key)` - Enable form persistence
- `handle_input(event)` - Handle input events
- `handle_submit(event)` - Handle form submission
- `reset()` - Reset form to initial state

## 🎯 **Form Data Derive Macro**

### **Basic Usage**

```rust
#[derive(Clone, Debug, FormData)]
pub struct UserForm {
    username: String,
    email: String,
    password: String,
}
```

### **Validation Attributes**

#### **Required Fields**

```rust
#[derive(Clone, Debug, FormData)]
pub struct Form {
    #[form(required)]
    name: String,

    #[form(required = "This field is required")]
    email: String,
}
```

#### **String Validation**

```rust
#[derive(Clone, Debug, FormData)]
pub struct Form {
    #[form(min_length = 3)]
    username: String,

    #[form(max_length = 100)]
    description: String,

    #[form(length = 10)]
    code: String,

    #[form(pattern = r"^[A-Z]{2}\d{4}$")]
    product_id: String,
}
```

#### **Numeric Validation**

```rust
#[derive(Clone, Debug, FormData)]
pub struct Form {
    #[form(min = 0)]
    age: u32,

    #[form(max = 100)]
    percentage: f64,

    #[form(range = 1..=10)]
    rating: u8,
}
```

#### **Email Validation**

```rust
#[derive(Clone, Debug, FormData)]
pub struct Form {
    #[form(email)]
    primary_email: String,

    #[form(email, custom = "validate_domain")]
    work_email: String,
}
```

#### **Custom Validation**

```rust
#[derive(Clone, Debug, FormData)]
pub struct Form {
    #[form(custom = "validate_username")]
    username: String,

    #[form(custom = "validate_password_strength")]
    password: String,
}

fn validate_username(username: &str) -> Result<(), String> {
    if username.contains(' ') {
        Err("Username cannot contain spaces".to_string())
    } else {
        Ok(())
    }
}

fn validate_password_strength(password: &str) -> Result<(), String> {
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());

    if has_upper && has_lower && has_digit {
        Ok(())
    } else {
        Err("Password must contain uppercase, lowercase, and digit".to_string())
    }
}
```

#### **Conditional Validation**

```rust
#[derive(Clone, Debug, FormData)]
pub struct Form {
    #[form(required)]
    account_type: String,

    #[form(required, when = "account_type == 'business'")]
    company_name: String,

    #[form(required, when = "account_type == 'personal'")]
    birth_date: String,
}
```

#### **Field Arrays**

```rust
#[derive(Clone, Debug, FormData)]
pub struct Form {
    #[form(array)]
    tags: Vec<String>,

    #[form(array, min_items = 1)]
    categories: Vec<String>,

    #[form(array, max_items = 5)]
    images: Vec<String>,
}
```

#### **Default Values**

```rust
#[derive(Clone, Debug, FormData)]
pub struct Form {
    #[form(default = "Anonymous")]
    username: String,

    #[form(default = false)]
    newsletter: bool,

    #[form(default = 18)]
    age: u32,

    #[form(default = vec!["general"])]
    tags: Vec<String>,
}
```

#### **Optional Fields**

```rust
#[derive(Clone, Debug, FormData)]
pub struct Form {
    #[form(optional)]
    phone: Option<String>,

    #[form(optional, email)]
    backup_email: Option<String>,
}
```

## 🎮 **Form Handle**

### **Creation and Configuration**

```rust
// Basic form
let form = use_form::<UserForm>();

// With persistence
let form = use_form::<UserForm>()
    .with_persistence("user-form-data");

// With custom configuration
let form = use_form::<UserForm>()
    .with_persistence("user-form-data")
    .with_validation_mode(ValidationMode::OnBlur);
```

### **State Access**

```rust
let form = use_form::<UserForm>();

// Form validity
let is_valid = form.is_valid();
let is_dirty = form.is_dirty();
let is_submitting = form.is_submitting();
let is_touched = form.is_touched();

// Field values
let username = form.field_value("username");
let email = form.field_value("email");

// Field errors
let username_error = form.field_error("username");
let all_errors = form.validation_errors();

// Form data
let form_data = form.form_data();
```

### **Event Handling**

```rust
let form = use_form::<UserForm>();

// Input events
<input on:input=form.handle_input />

// Change events
<select on:change=form.handle_input />

// Submit events
<form on:submit=form.handle_submit />

// Blur events
<input on:blur=form.handle_blur />

// Focus events
<input on:focus=form.handle_focus />
```

### **Form Manipulation**

```rust
let form = use_form::<UserForm>();

// Set field values
form.set_field_value("username", "john_doe");
form.set_field_value("email", "john@example.com");

// Clear fields
form.clear_field("username");

// Reset form
form.reset();

// Clear all errors
form.clear_errors();

// Validate specific field
form.validate_field("username");

// Validate entire form
form.validate();
```

### **Field Arrays**

```rust
let form = use_form::<ProductForm>();

// Add new item
form.add_field_array_item("tags");

// Remove item at index
form.remove_field_array_item("tags", 0);

// Get array items
let tags = form.field_array("tags");

// Set array item value
form.set_field_array_item_value("tags", 0, "electronics");
```

### **Persistence**

```rust
let form = use_form::<UserForm>()
    .with_persistence("user-form-data");

// Save form data
form.save_to_storage();

// Load form data
form.load_from_storage();

// Clear persisted data
form.clear_persistence();

// Check if data exists
let has_data = form.has_persisted_data();
```

## ✅ **Validation**

### **Validation Modes**

```rust
pub enum ValidationMode {
    OnInput,    // Validate as user types
    OnBlur,     // Validate when field loses focus
    OnSubmit,   // Validate only on form submission
    Manual,     // Manual validation only
}
```

### **Validation Errors**

```rust
pub struct ValidationErrors<T> {
    pub field_errors: HashMap<String, Vec<T>>,
    pub form_errors: Vec<T>,
}

pub struct FieldError {
    pub message: String,
    pub code: String,
    pub field: String,
}
```

### **Built-in Validators**

#### **String Validators**

- `required` - Field must have a value
- `email` - Must be a valid email format
- `url` - Must be a valid URL format
- `min_length(n)` - Minimum string length
- `max_length(n)` - Maximum string length
- `length(n)` - Exact string length
- `pattern(regex)` - Must match regex pattern

#### **Numeric Validators**

- `min(n)` - Minimum value
- `max(n)` - Maximum value
- `range(start..end)` - Value must be in range
- `positive` - Must be positive
- `negative` - Must be negative

#### **Collection Validators**

- `min_items(n)` - Minimum number of items
- `max_items(n)` - Maximum number of items
- `unique` - All items must be unique

### **Custom Validation**

```rust
#[derive(Clone, Debug, FormData)]
pub struct Form {
    #[form(custom = "validate_complex_field")]
    complex_field: String,
}

fn validate_complex_field(value: &str) -> Result<(), String> {
    // Complex validation logic
    if value.len() < 5 {
        return Err("Field must be at least 5 characters".to_string());
    }

    if !value.chars().any(|c| c.is_uppercase()) {
        return Err("Field must contain uppercase letter".to_string());
    }

    if !value.chars().any(|c| c.is_numeric()) {
        return Err("Field must contain a number".to_string());
    }

    Ok(())
}
```

## 🧩 **Components**

### **Text Input**

```rust
<TextInput
    name="username"
    label="Username"
    form=form.clone()
    placeholder="Enter username"
    required
    disabled=move || form.is_submitting()
/>
```

**Props:**

- `name: String` - Field name
- `label: String` - Field label
- `form: FormHandle<T>` - Form instance
- `placeholder: Option<String>` - Placeholder text
- `required: bool` - Whether field is required
- `disabled: bool` - Whether field is disabled
- `class: Option<String>` - Additional CSS classes

### **Email Input**

```rust
<EmailInput
    name="email"
    label="Email Address"
    form=form.clone()
    required
/>
```

### **Password Input**

```rust
<PasswordInput
    name="password"
    label="Password"
    form=form.clone()
    required
    show_toggle=true
/>
```

### **Select Input**

```rust
<SelectInput
    name="country"
    label="Country"
    options=vec![
        ("us", "United States"),
        ("ca", "Canada"),
        ("uk", "United Kingdom"),
    ]
    form=form.clone()
    required
/>
```

### **Checkbox Input**

```rust
<CheckboxInput
    name="terms"
    label="I agree to the terms and conditions"
    form=form.clone()
    required
/>
```

### **Radio Group**

```rust
<RadioGroup
    name="gender"
    label="Gender"
    options=vec![
        ("male", "Male"),
        ("female", "Female"),
        ("other", "Other"),
    ]
    form=form.clone()
    required
/>
```

### **Textarea Input**

```rust
<TextareaInput
    name="message"
    label="Message"
    form=form.clone()
    placeholder="Enter your message"
    rows=5
    required
/>
```

### **File Input**

```rust
<FileInput
    name="avatar"
    label="Profile Picture"
    form=form.clone()
    accept="image/*"
    multiple=false
/>
```

## 🪝 **Hooks**

### **use_form**

The main hook for creating form instances.

```rust
pub fn use_form<T: FormData>() -> FormHandle<T>
```

**Usage:**

```rust
let form = use_form::<UserForm>();
```

### **use_form_with_persistence**

Create a form with automatic persistence.

```rust
pub fn use_form_with_persistence<T: FormData>(
    storage_key: &str
) -> FormHandle<T>
```

**Usage:**

```rust
let form = use_form_with_persistence::<UserForm>("user-form");
```

### **use_form_validation**

Access form validation state.

```rust
pub fn use_form_validation<T: FormData>(
    form: &FormHandle<T>
) -> FormValidationState
```

**Usage:**

```rust
let form = use_form::<UserForm>();
let validation = use_form_validation(&form);
```

## 🛠️ **Utilities**

### **Form Utils**

```rust
// Check if value is empty
pub fn is_empty(value: &str) -> bool

// Validate email format
pub fn is_valid_email(email: &str) -> bool

// Validate URL format
pub fn is_valid_url(url: &str) -> bool

// Sanitize input
pub fn sanitize_input(input: &str) -> String

// Format validation error
pub fn format_error(field: &str, message: &str) -> String
```

### **Storage Utils**

```rust
// Save to localStorage
pub fn save_to_storage<T: Serialize>(key: &str, data: &T) -> Result<(), String>

// Load from localStorage
pub fn load_from_storage<T: DeserializeOwned>(key: &str) -> Result<T, String>

// Remove from localStorage
pub fn remove_from_storage(key: &str) -> Result<(), String>

// Check if storage is available
pub fn is_storage_available() -> bool
```

### **Validation Utils**

```rust
// Create validation error
pub fn validation_error(message: &str) -> FieldError

// Create field-specific error
pub fn field_error(field: &str, message: &str) -> FieldError

// Merge validation errors
pub fn merge_errors(errors: Vec<ValidationErrors>) -> ValidationErrors

// Check if form is valid
pub fn is_form_valid<T: FormData>(form: &T) -> bool
```

## ❌ **Error Types**

### **FormError**

```rust
pub enum FormError {
    ValidationError(ValidationErrors<FieldError>),
    StorageError(String),
    SerializationError(String),
    DeserializationError(String),
    FieldNotFound(String),
    InvalidFieldType(String),
}
```

### **ValidationError**

```rust
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub code: String,
    pub value: Option<String>,
}
```

### **FieldError**

```rust
pub struct FieldError {
    pub message: String,
    pub code: String,
    pub field: String,
    pub details: Option<HashMap<String, String>>,
}
```

## 🔧 **Configuration**

### **Form Configuration**

```rust
pub struct FormConfig {
    pub validation_mode: ValidationMode,
    pub persistence_enabled: bool,
    pub storage_key: Option<String>,
    pub auto_validate: bool,
    pub debounce_ms: u32,
    pub error_display_mode: ErrorDisplayMode,
}
```

### **Validation Configuration**

```rust
pub struct ValidationConfig {
    pub stop_on_first_error: bool,
    pub custom_error_messages: HashMap<String, String>,
    pub locale: String,
    pub strict_mode: bool,
}
```

## 📱 **Mobile Support**

### **Touch Events**

```rust
// Handle touch events
<input on:touchstart=form.handle_touch_start />
<input on:touchend=form.handle_touch_end />
<input on:touchmove=form.handle_touch_move />
```

### **Mobile Validation**

```rust
// Mobile-specific validation
#[form(mobile_optimized = true)]
mobile_field: String,

// Touch-friendly components
<TouchFriendlyInput
    name="mobile_input"
    form=form.clone()
    touch_target_size=44
/>
```

## ♿ **Accessibility**

### **ARIA Attributes**

```rust
// Automatic ARIA attributes
<TextInput
    name="username"
    label="Username"
    form=form.clone()
    aria-describedby="username-error"
    aria-invalid=move || form.field_error("username").is_some()
/>
```

### **Screen Reader Support**

```rust
// Error announcements
<div role="alert" aria-live="polite">
    {move || form.field_error("username").map(|error|
        view! { <span>{error}</span> }
    )}
</div>
```

## 🚀 **Performance**

### **Optimization Tips**

```rust
// Use memoization for expensive computations
let expensive_value = Memo::new(move |_| {
    // Expensive computation
    compute_expensive_value(form.form_data())
});

// Debounce input handlers
let debounced_handler = Memo::new(move |_| {
    debounce(100, form.handle_input)
});

// Lazy load large forms
let form_sections = Memo::new(move |_| {
    if form.current_step() > 0 {
        load_form_section(form.current_step())
    } else {
        vec![]
    }
});
```

---

This API reference covers all the public interfaces of Leptos Forms RS. For more detailed examples and advanced usage patterns, see the [Examples](../examples/) directory and [Getting Started Guide](getting-started.md).
