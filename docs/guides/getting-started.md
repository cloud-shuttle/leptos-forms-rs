# Getting Started with Leptos Forms RS

Welcome to Leptos Forms RS! This guide will get you up and running with type-safe, reactive forms in your Leptos applications.

## 🚀 **Quick Start**

### **1. Installation**

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
leptos-forms-rs = "0.1.0"
leptos = "0.6"
```

### **2. Basic Form Example**

Here's a simple form that demonstrates the core concepts:

```rust
use leptos::*;
use leptos_forms_rs::*;

#[derive(Clone, Debug, FormData)]
pub struct UserForm {
    #[form(required)]
    username: String,

    #[form(required, email)]
    email: String,

    #[form(min_length = 8)]
    password: String,
}

#[component]
pub fn UserRegistrationForm() -> impl IntoView {
    let form = use_form::<UserForm>();

    view! {
        <div class="user-form">
            <h2>"User Registration"</h2>

            <form on:submit=form.handle_submit>
                <div class="form-group">
                    <label for="username">"Username"</label>
                    <input
                        type="text"
                        id="username"
                        name="username"
                        on:input=form.handle_input
                        required
                    />
                    {move || form.field_error("username").map(|error|
                        view! { <span class="error">{error}</span> }
                    )}
                </div>

                <div class="form-group">
                    <label for="email">"Email"</label>
                    <input
                        type="email"
                        id="email"
                        name="email"
                        on:input=form.handle_input
                        required
                    />
                    {move || form.field_error("email").map(|error|
                        view! { <span class="error">{error}</span> }
                    )}
                </div>

                <div class="form-group">
                    <label for="password">"Password"</label>
                    <input
                        type="password"
                        id="password"
                        name="password"
                        on:input=form.handle_input
                        required
                    />
                    {move || form.field_error("password").map(|error|
                        view! { <span class="error">{error}</span> }
                    )}
                </div>

                <button type="submit" disabled=move || !form.is_valid()>
                    "Register"
                </button>
            </form>

            {move || form.form_data().map(|data|
                view! {
                    <div class="form-data">
                        <h3>"Form Data:"</h3>
                        <pre>{format!("{:#?}", data)}</pre>
                    </div>
                }
            )}
        </div>
    }
}
```

## 📚 **Core Concepts**

### **1. Form Data Structs**

Define your form structure using the `FormData` derive macro:

```rust
#[derive(Clone, Debug, FormData)]
pub struct ContactForm {
    #[form(required)]
    name: String,

    #[form(required, email)]
    email: String,

    #[form(required)]
    message: String,

    #[form(optional)]
    phone: Option<String>,

    #[form(default = false)]
    newsletter: bool,
}
```

### **2. Form Validation**

Use validation attributes to enforce rules:

```rust
#[derive(Clone, Debug, FormData)]
pub struct ValidatedForm {
    #[form(required, min_length = 3, max_length = 50)]
    username: String,

    #[form(required, email, custom = "validate_domain")]
    email: String,

    #[form(required, min = 18, max = 120)]
    age: u32,

    #[form(required, pattern = r"^\d{3}-\d{3}-\d{4}$")]
    phone: String,
}

// Custom validation function
fn validate_domain(email: &str) -> Result<(), String> {
    if email.ends_with("@example.com") {
        Ok(())
    } else {
        Err("Email must be from example.com domain".to_string())
    }
}
```

### **3. Form State Management**

The `use_form` hook provides reactive form state:

```rust
let form = use_form::<MyForm>();

// Access form state
let is_valid = form.is_valid();
let is_dirty = form.is_dirty();
let is_submitting = form.is_submitting();

// Access field values
let username = form.field_value("username");
let email = form.field_value("email");

// Access field errors
let username_error = form.field_error("username");
let all_errors = form.validation_errors();
```

## 🎨 **Form Components**

### **1. Pre-built Components**

Use the provided form components for common inputs:

```rust
use leptos_forms_rs::components::*;

#[component]
pub fn FormWithComponents() -> impl IntoView {
    let form = use_form::<UserForm>();

    view! {
        <form on:submit=form.handle_submit>
            <TextInput
                name="username"
                label="Username"
                form=form.clone()
                required
            />

            <EmailInput
                name="email"
                label="Email"
                form=form.clone()
                required
            />

            <PasswordInput
                name="password"
                label="Password"
                form=form.clone()
                required
            />

            <SelectInput
                name="country"
                label="Country"
                options=vec![
                    ("us", "United States"),
                    ("ca", "Canada"),
                    ("uk", "United Kingdom"),
                ]
                form=form.clone()
            />

            <CheckboxInput
                name="terms"
                label="I agree to the terms"
                form=form.clone()
                required
            />

            <button type="submit">"Submit"</button>
        </form>
    }
}
```

### **2. Custom Components**

Create your own form components:

```rust
#[component]
pub fn CustomInput(
    name: String,
    label: String,
    form: FormHandle<UserForm>,
) -> impl IntoView {
    let value = form.field_value(name.clone());
    let error = form.field_error(name.clone());

    view! {
        <div class="custom-input">
            <label for=name.clone()>{label}</label>
            <input
                type="text"
                id=name.clone()
                name=name.clone()
                value=value
                on:input=form.handle_input
            />
            {move || error.map(|e| view! { <span class="error">{e}</span> })}
        </div>
    }
}
```

## 🔄 **Advanced Patterns**

### **1. Field Arrays**

Handle dynamic lists of fields:

```rust
#[derive(Clone, Debug, FormData)]
pub struct ProductForm {
    #[form(required)]
    name: String,

    #[form(required)]
    price: f64,

    #[form(array)]
    tags: Vec<String>,
}

#[component]
pub fn ProductFormWithTags() -> impl IntoView {
    let form = use_form::<ProductForm>();

    view! {
        <form on:submit=form.handle_submit>
            <input
                type="text"
                name="name"
                on:input=form.handle_input
                placeholder="Product name"
            />

            <input
                type="number"
                name="price"
                on:input=form.handle_input
                placeholder="Price"
                step="0.01"
            />

            <div class="tags-section">
                <h3>"Tags"</h3>
                {move || {
                    let tags = form.field_array("tags");
                    tags.into_iter().enumerate().map(|(index, _)| {
                        view! {
                            <div key=index class="tag-input">
                                <input
                                    type="text"
                                    name=format!("tags[{}]", index)
                                    on:input=form.handle_input
                                    placeholder="Tag"
                                />
                                <button
                                    type="button"
                                    on:click=move |_| form.remove_field_array_item("tags", index)
                                >
                                    "Remove"
                                </button>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}

                <button
                    type="button"
                    on:click=move |_| form.add_field_array_item("tags")
                >
                    "Add Tag"
                </button>
            </div>

            <button type="submit">"Create Product"</button>
        </form>
    }
}
```

### **2. Conditional Fields**

Show/hide fields based on form state:

```rust
#[derive(Clone, Debug, FormData)]
pub struct ConditionalForm {
    #[form(required)]
    account_type: String,

    #[form(required, when = "account_type == 'business'")]
    company_name: String,

    #[form(required, when = "account_type == 'business'")]
    tax_id: String,

    #[form(required, when = "account_type == 'personal'")]
    birth_date: String,
}

#[component]
pub fn ConditionalFormExample() -> impl IntoView {
    let form = use_form::<ConditionalForm>();
    let account_type = form.field_value("account_type");

    view! {
        <form on:submit=form.handle_submit>
            <select name="account_type" on:change=form.handle_input>
                <option value="">"Select account type"</option>
                <option value="personal">"Personal"</option>
                <option value="business">"Business"</option>
            </select>

            {move || {
                if account_type.get() == Some("business".to_string()) {
                    view! {
                        <>
                            <input
                                type="text"
                                name="company_name"
                                on:input=form.handle_input
                                placeholder="Company name"
                            />
                            <input
                                type="text"
                                name="tax_id"
                                on:input=form.handle_input
                                placeholder="Tax ID"
                            />
                        </>
                    }
                } else if account_type.get() == Some("personal".to_string()) {
                    view! {
                        <input
                            type="date"
                            name="birth_date"
                            on:input=form.handle_input
                        />
                    }
                } else {
                    view! { <div>"Please select an account type"</div> }
                }
            }}

            <button type="submit">"Submit"</button>
        </form>
    }
}
```

### **3. Form Persistence**

Save and restore form data:

```rust
#[component]
pub fn PersistentForm() -> impl IntoView {
    let form = use_form::<UserForm>()
        .with_persistence("user-form-data"); // Automatically saves to localStorage

    // Form data is automatically restored when component mounts

    view! {
        <form on:submit=form.handle_submit>
            <input
                type="text"
                name="username"
                on:input=form.handle_input
                placeholder="Username"
            />

            <input
                type="email"
                name="email"
                on:input=form.handle_input
                placeholder="Email"
            />

            <button type="submit">"Save"</button>
            <button
                type="button"
                on:click=move |_| form.clear_persistence()
            >
                "Clear Saved Data"
            </button>
        </form>
    }
}
```

## 🧪 **Testing Your Forms**

### **1. Unit Testing**

Test form logic in isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_validation() {
        let form = use_form::<UserForm>();

        // Test required field validation
        assert!(!form.is_valid());

        // Fill required fields
        form.set_field_value("username", "john_doe");
        form.set_field_value("email", "john@example.com");
        form.set_field_value("password", "secure123");

        assert!(form.is_valid());
    }

    #[test]
    fn test_email_validation() {
        let form = use_form::<UserForm>();

        form.set_field_value("email", "invalid-email");
        assert!(!form.is_valid());
        assert!(form.field_error("email").is_some());

        form.set_field_value("email", "valid@example.com");
        assert!(form.field_error("email").is_none());
    }
}
```

### **2. Integration Testing**

Test forms in the browser:

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_form_submission() {
        let app = view! {
            <UserRegistrationForm />
        };

        // Mount the component
        mount_to_body(app);

        // Test form interaction
        // (This would use a testing framework like Playwright)
    }
}
```

## 🎯 **Best Practices**

### **1. Form Structure**

- **Use semantic HTML** - `<form>`, `<label>`, `<fieldset>`
- **Group related fields** with `<fieldset>` and `<legend>`
- **Provide clear labels** for all form controls
- **Use appropriate input types** (`email`, `tel`, `url`, etc.)

### **2. Validation**

- **Validate on input** for immediate feedback
- **Validate on blur** for field-level validation
- **Validate on submit** for final form validation
- **Show clear error messages** near the relevant fields

### **3. Accessibility**

- **Use proper ARIA attributes** (`aria-describedby`, `aria-invalid`)
- **Ensure keyboard navigation** works properly
- **Provide error announcements** for screen readers
- **Use semantic HTML** for better accessibility

### **4. Performance**

- **Debounce input handlers** for better performance
- **Use `Memo::new`** for expensive computations
- **Avoid unnecessary re-renders** by optimizing dependencies
- **Lazy load** large forms when possible

## 🚀 **Next Steps**

Now that you have the basics, explore:

1. **[API Reference](api-reference.md)** - Complete API documentation
2. **[Examples](../examples/)** - More complex examples
3. **[Validation Guide](validation/validation-guide.md)** - Advanced validation patterns
4. **[Testing Guide](testing-strategy.md)** - Comprehensive testing strategies
5. **[Performance Guide](performance-guide.md)** - Optimization techniques

## 🤝 **Getting Help**

- **GitHub Issues** - Report bugs or request features
- **Discussions** - Ask questions and share solutions
- **Examples** - Check the examples directory for working code
- **Documentation** - Browse the full documentation

Happy form building! 🎉
