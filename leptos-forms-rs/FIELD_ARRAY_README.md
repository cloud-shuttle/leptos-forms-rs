# Field Array Component

The `FieldArray` component is a powerful, flexible component for managing dynamic arrays of form fields in Leptos applications. It provides a complete solution for handling collections of form data with validation, custom rendering, and intuitive user interactions.

## Features

### ✨ Core Functionality
- **Dynamic Item Management**: Add, remove, and manage form items dynamically
- **Custom Rendering**: Flexible item rendering through callback functions
- **Validation Support**: Built-in min/max constraints and validation
- **Responsive Design**: Mobile-friendly layout with responsive breakpoints
- **Accessibility**: Proper ARIA labels, keyboard navigation, and focus management

### 🎨 UI/UX Features
- **Modern Design**: Clean, professional appearance with smooth animations
- **Interactive Elements**: Hover effects, focus states, and visual feedback
- **Drag & Drop Ready**: Infrastructure for future drag-and-drop reordering
- **Empty States**: Helpful messaging when no items exist
- **Progress Indicators**: Visual feedback for validation constraints

### 🔧 Technical Features
- **Leptos 0.8 Compatible**: Built with the latest Leptos framework
- **Type Safe**: Full Rust type safety with generic support
- **Performance Optimized**: Efficient signal handling and minimal re-renders
- **Extensible**: Easy to extend with custom functionality

## Basic Usage

```rust
use leptos::prelude::*;
use leptos_forms_rs::components::FieldArray;
use leptos_forms_rs::core::{Form, FormHandle, FieldValue};

#[component]
pub fn MyForm() -> impl IntoView {
    let form_handle = use_form::<MyFormType>();
    
    // Render function for each item
    let render_item = Callback::new(|(index, value): (usize, FieldValue)| {
        view! {
            <input
                type="text"
                value=move || value.as_string().unwrap_or_default()
                placeholder="Enter item text"
            />
        }
    });

    view! {
        <FieldArray
            field_name="items".to_string()
            form_handle=form_handle
            render_item=render_item
            min=Some(1)
            max=Some(10)
        />
    }
}
```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `field_name` | `String` | **Required** | The name of the field in the form |
| `form_handle` | `FormHandle<T>` | **Required** | The form handle managing the form state |
| `render_item` | `Callback<(usize, FieldValue), View<()>>` | **Required** | Function to render each array item |
| `min` | `Option<usize>` | `None` | Minimum number of items required |
| `max` | `Option<usize>` | `None` | Maximum number of items allowed |
| `can_add` | `Option<Signal<bool>>` | `None` | Control whether items can be added |
| `can_remove` | `Option<Signal<bool>>` | `None` | Control whether items can be removed |
| `show_validation` | `Option<Signal<bool>>` | `None` | Show validation messages |
| `allow_reordering` | `Option<Signal<bool>>` | `None` | Enable drag-and-drop reordering |

## Advanced Usage

### Complex Item Rendering

```rust
let render_complex_item = Callback::new(|(index, value): (usize, FieldValue)| {
    if let FieldValue::Object(obj) = value {
        let title = obj.get("title").and_then(|v| v.as_string()).unwrap_or_default();
        let description = obj.get("description").and_then(|v| v.as_string()).unwrap_or_default();
        let active = obj.get("active").and_then(|v| v.as_boolean()).unwrap_or(false);
        
        view! {
            <div class="complex-item">
                <div class="item-header">
                    <input
                        type="text"
                        value=title
                        placeholder="Item title"
                        class="item-title-input"
                    />
                    <label class="checkbox-label">
                        <input type="checkbox" checked=active />
                        "Active"
                    </label>
                </div>
                <textarea
                    placeholder="Item description"
                    class="item-description-input"
                >
                    {description}
                </textarea>
            </div>
        }
    } else {
        view! { <div class="error">"Invalid item data"</div> }
    }
});
```

### Conditional Rendering

```rust
let render_item = Callback::new(|(index, value): (usize, FieldValue)| {
    let is_even = index % 2 == 0;
    
    if is_even {
        view! {
            <div class="even-item">
                <span class="item-label">"Even Item {index}"</span>
                <input type="text" value=move || value.as_string().unwrap_or_default() />
            </div>
        }
    } else {
        view! {
            <div class="odd-item">
                <span class="item-label">"Odd Item {index}"</span>
                <textarea>{move || value.as_string().unwrap_or_default()}</textarea>
            </div>
        }
    }
});
```

### Validation Integration

```rust
let render_item = Callback::new(|(index, value): (usize, FieldValue)| {
    let has_error = Signal::derive(move || {
        // Custom validation logic
        let text = value.as_string().unwrap_or_default();
        text.len() < 3
    });
    
    view! {
        <div class="validated-item">
            <input
                type="text"
                value=move || value.as_string().unwrap_or_default()
                class:error=has_error
                placeholder="Enter at least 3 characters"
            />
            {move || {
                if has_error.get() {
                    view! {
                        <span class="error-message">"Must be at least 3 characters"</span>
                    }
                } else {
                    view! { <div></div> }
                }
            }}
        </div>
    }
});
```

## Styling

The component comes with comprehensive CSS that provides:

- **Modern Design**: Clean, professional appearance
- **Responsive Layout**: Mobile-first responsive design
- **Interactive States**: Hover, focus, and active states
- **Animations**: Smooth transitions and animations
- **Dark Mode Support**: Automatic dark mode detection
- **Accessibility**: High contrast and focus indicators

### Custom Styling

You can customize the appearance by overriding CSS variables or classes:

```css
.field-array {
    --primary-color: #8b5cf6;
    --border-radius: 12px;
    --shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
}

.field-array-item {
    background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%);
    border: 2px solid var(--primary-color);
}
```

## Integration with Form Validation

The Field Array component integrates seamlessly with the form validation system:

```rust
// In your form validation
fn validate(&self) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();
    
    // Validate array constraints
    if self.items.len() < 2 {
        errors.add_field_error("items".to_string(), "At least 2 items are required".to_string());
    }
    
    if self.items.len() > 10 {
        errors.add_field_error("items".to_string(), "Maximum 10 items allowed".to_string());
    }
    
    // Validate individual items
    for (index, item) in self.items.iter().enumerate() {
        if item.title.is_empty() {
            errors.add_field_error(
                format!("items[{}].title", index),
                "Title is required".to_string()
            );
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

## Performance Considerations

- **Signal Optimization**: Uses efficient signal handling to minimize re-renders
- **Lazy Rendering**: Items are only rendered when visible
- **Memory Management**: Proper cleanup of event listeners and signals
- **Batch Updates**: Multiple changes are batched for optimal performance

## Accessibility Features

- **ARIA Labels**: Proper labeling for screen readers
- **Keyboard Navigation**: Full keyboard support for all interactions
- **Focus Management**: Logical tab order and focus indicators
- **Screen Reader Support**: Semantic HTML and ARIA attributes
- **High Contrast**: Support for high contrast mode

## Browser Support

- **Modern Browsers**: Chrome 90+, Firefox 88+, Safari 14+, Edge 90+
- **Mobile Support**: iOS Safari 14+, Chrome Mobile 90+
- **Progressive Enhancement**: Graceful degradation for older browsers

## Future Enhancements

Planned features for upcoming versions:

- **Drag & Drop Reordering**: Full drag-and-drop support
- **Bulk Operations**: Select multiple items for batch actions
- **Advanced Validation**: Real-time validation with custom rules
- **Virtual Scrolling**: Performance optimization for large arrays
- **Export/Import**: Data import/export functionality
- **Undo/Redo**: Action history and undo support

## Examples

See the `field_array_example.rs` file for complete working examples demonstrating:

- Simple string arrays
- Complex object arrays
- Custom validation
- Conditional rendering
- Form integration

## Contributing

Contributions are welcome! Please see the main project README for contribution guidelines.

## License

This component is part of the leptos-forms-rs project and is licensed under the MIT License.
