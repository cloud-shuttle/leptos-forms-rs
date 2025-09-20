# FormField Component Design

**File**: `leptos-forms-rs/src/components/form_field.rs`
**Max Size**: 250 lines
**Priority**: CRITICAL
**Dependencies**: FormHandle, FieldType, ValidationEngine

## Purpose

The FormField component is the primary UI component users interact with. It provides two-way data binding between form state and HTML input elements with built-in validation display.

## API Design

### Component Signature

```rust
#[component]
pub fn FormField<T>(
    /// Form handle containing the state
    form: FormHandle<T>,
    /// Field name (must match struct field)
    name: String,
    /// Override detected input type
    #[prop(optional)] field_type: Option<FieldType>,
    /// Additional CSS classes
    #[prop(optional)] class: Option<String>,
    /// Placeholder text
    #[prop(optional)] placeholder: Option<String>,
    /// Disable the field
    #[prop(optional)] disabled: Option<bool>,
) -> impl IntoView
where
    T: Form + Clone + 'static,
```

## Input Type Detection

### Automatic Detection Logic

```rust
fn detect_input_type(field_name: &str, metadata: &FieldMetadata) -> InputType {
    match metadata.field_type {
        FieldType::Email => InputType::Email,
        FieldType::Password => InputType::Password,
        FieldType::Number => InputType::Number,
        FieldType::Date => InputType::Date,
        FieldType::Boolean => InputType::Checkbox,
        FieldType::Text | FieldType::String => {
            // Smart detection based on field name
            if field_name.contains("email") { InputType::Email }
            else if field_name.contains("password") { InputType::Password }
            else if field_name.contains("phone") { InputType::Tel }
            else { InputType::Text }
        }
    }
}
```

## Signal Integration

### Reactive Updates

```rust
pub fn FormField<T>(/* ... */) -> impl IntoView {
    let field_value = create_memo(move |_| {
        form.get_field_value(&name)
    });

    let field_error = create_memo(move |_| {
        form.get_field_error(&name)
    });

    let on_input = move |event: web_sys::Event| {
        let input: web_sys::HtmlInputElement = event_target(&event);
        let value = input.value();
        form.set_field_value(&name, value.into());

        // Trigger validation after input
        if form.should_validate_on_input() {
            form.validate_field(&name);
        }
    };

    // ... rest of component
}
```

## Validation Integration

### Error Display Strategy

```rust
fn render_validation_errors(errors: &Option<ValidationError>) -> impl IntoView {
    match errors {
        Some(ValidationError::Single(msg)) => view! {
            <div class="field-error" role="alert">
                {msg}
            </div>
        },
        Some(ValidationError::Multiple(msgs)) => view! {
            <div class="field-errors" role="alert">
                <ul>
                    {msgs.iter().map(|msg| view! {
                        <li>{msg}</li>
                    }).collect::<Vec<_>>()}
                </ul>
            </div>
        },
        None => view! {}.into_view()
    }
}
```

## Accessibility Features

### ARIA Implementation

```rust
fn generate_aria_attributes(
    field_name: &str,
    has_error: bool,
    required: bool
) -> Vec<(&'static str, String)> {
    let mut attrs = vec![
        ("aria-label", field_name.to_string()),
        ("id", format!("field-{}", field_name)),
    ];

    if has_error {
        attrs.push(("aria-invalid", "true".to_string()));
        attrs.push(("aria-describedby", format!("field-{}-error", field_name)));
    }

    if required {
        attrs.push(("aria-required", "true".to_string()));
    }

    attrs
}
```

## File Structure

### Single File Organization (< 250 lines)

```rust
// form_field.rs structure:

// 1. Imports and types (20 lines)
use leptos::*;
use crate::core::{FormHandle, FieldType, ValidationError};

// 2. Helper functions (60 lines)
fn detect_input_type() -> InputType { /* ... */ }
fn generate_aria_attributes() -> Vec<(&str, String)> { /* ... */ }
fn render_validation_errors() -> impl IntoView { /* ... */ }

// 3. Main component (150 lines)
#[component]
pub fn FormField<T>() -> impl IntoView { /* ... */ }

// 4. Private sub-components (20 lines)
fn render_input_element() -> impl IntoView { /* ... */ }
```

## Styling Strategy

### CSS Classes

```rust
fn generate_css_classes(
    base_class: Option<&str>,
    field_type: FieldType,
    has_error: bool,
    is_focused: bool
) -> String {
    let mut classes = vec!["form-field"];

    if let Some(base) = base_class {
        classes.push(base);
    }

    classes.push(match field_type {
        FieldType::Email => "form-field--email",
        FieldType::Password => "form-field--password", // pragma: allowlist secret
        FieldType::Number => "form-field--number",
        _ => "form-field--text"
    });

    if has_error { classes.push("form-field--error"); }
    if is_focused { classes.push("form-field--focused"); }

    classes.join(" ")
}
```

## Performance Considerations

### Optimization Strategies

- Use `create_memo` for expensive computations
- Debounce validation calls (300ms default)
- Only re-render when field value or error state changes
- Cache input type detection results

### Memory Management

- Clean up event listeners on component drop
- Avoid cloning large form state unnecessarily
- Use references where possible in signal updates

## Testing Requirements

### Unit Tests (50 lines each)

```rust
#[cfg(test)]
mod tests {
    // Test input type detection
    // Test signal updates
    // Test validation error display
    // Test accessibility attributes
    // Test CSS class generation
}
```

### Integration Tests

- Render with different field types
- Two-way data binding functionality
- Validation error lifecycle
- Event handling (focus, blur, input)

## Error Handling

### Graceful Degradation

- Fallback to text input if type detection fails
- Default error messages if validation fails
- Accessible error states even without JavaScript

### Error Boundaries

```rust
fn handle_render_error(error: &str) -> impl IntoView {
    log::error!("FormField render error: {}", error);
    view! {
        <input
            type="text"
            class="form-field form-field--error"
            placeholder="Error loading field"
            disabled
        />
    }
}
```

## Usage Examples

### Basic Usage

```rust
view! {
    <FormField form=form name="email" />
    <FormField form=form name="password" field_type=FieldType::Password />
    <FormField form=form name="age" field_type=FieldType::Number class="custom-field" />
}
```

### Advanced Usage

```rust
view! {
    <FormField
        form=form
        name="bio"
        field_type=FieldType::TextArea
        placeholder="Tell us about yourself"
        class="large-field"
        disabled=is_loading
    />
}
```

## Success Criteria

- [ ] Component renders correctly for all field types
- [ ] Two-way data binding works without lag
- [ ] Validation errors display immediately
- [ ] All accessibility attributes present
- [ ] Component stays under 250 lines
- [ ] Performance: <16ms render time
- [ ] Memory: No leaks after component unmount
