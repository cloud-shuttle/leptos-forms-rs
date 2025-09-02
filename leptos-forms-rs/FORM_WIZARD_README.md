# Form Wizard Component

The `FormWizard` component is a powerful, feature-rich component for creating multi-step forms in Leptos applications. It provides a complete solution for building complex forms with step-by-step navigation, validation, and progress tracking.

## Features

### ✨ Core Functionality
- **Multi-step Navigation**: Intuitive step-by-step form progression
- **Progress Tracking**: Visual progress bar and step indicators
- **Step Validation**: Built-in validation with error handling
- **Flexible Navigation**: Forward, backward, and direct step access
- **Custom Step Content**: Flexible rendering of step-specific content
- **Responsive Design**: Mobile-first responsive layout

### 🎨 UI/UX Features
- **Modern Design**: Clean, professional appearance with smooth animations
- **Visual Feedback**: Step completion indicators and validation states
- **Interactive Elements**: Hover effects, focus states, and visual feedback
- **Progress Visualization**: Animated progress bar with shimmer effects
- **Step Indicators**: Clear visual representation of current and completed steps
- **Error Handling**: Comprehensive error display and validation feedback

### 🔧 Technical Features
- **Leptos 0.8 Compatible**: Built with the latest Leptos framework
- **Type Safe**: Full Rust type safety with generic support
- **Performance Optimized**: Efficient signal handling and minimal re-renders
- **Extensible**: Easy to extend with custom functionality
- **Accessibility**: Full keyboard navigation and screen reader support

## Basic Usage

```rust
use leptos::prelude::*;
use leptos_forms_rs::components::{FormWizard, WizardStep};

#[component]
pub fn MyWizard() -> impl IntoView {
    let (current_step, set_current_step) = signal(0);
    
    let steps = vec![
        WizardStep {
            title: "Step 1".to_string(),
            description: Some("First step description".to_string()),
            content: view! { <div>"Step 1 content"</div> },
        },
        WizardStep {
            title: "Step 2".to_string(),
            description: Some("Second step description".to_string()),
            content: view! { <div>"Step 2 content"</div> },
        },
    ];

    let on_step_change = Callback::new(move |step_index: usize| {
        set_current_step.set(step_index);
    });

    view! {
        <FormWizard
            steps=steps
            current_step=current_step.into()
            on_step_change=on_step_change
            form_handle=form_handle
        />
    }
}
```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `steps` | `Vec<WizardStep>` | **Required** | Array of wizard steps |
| `current_step` | `Signal<usize>` | **Required** | Current step index |
| `on_step_change` | `Callback<usize, ()>` | **Required** | Callback when step changes |
| `form_handle` | `FormHandle<T>` | **Required** | Form handle for validation |
| `show_progress_bar` | `Option<Signal<bool>>` | `true` | Show progress bar |
| `show_step_numbers` | `Option<Signal<bool>>` | `true` | Show step numbers |
| `allow_back_navigation` | `Option<Signal<bool>>` | `true` | Allow going back |
| `allow_skip_steps` | `Option<Signal<bool>>` | `false` | Allow skipping steps |
| `validate_on_step_change` | `Option<Signal<bool>>` | `true` | Validate on step change |
| `on_step_validation` | `Option<Callback<(usize, bool), ()>>` | `None` | Step validation callback |
| `on_wizard_complete` | `Option<Callback<T, ()>>` | `None` | Wizard completion callback |

## WizardStep Structure

```rust
#[derive(Clone)]
pub struct WizardStep {
    pub title: String,                    // Step title
    pub description: Option<String>,      // Optional step description
    pub content: View<HtmlElement<Div, (), ()>>, // Step content
}
```

## Advanced Usage

### Step Validation

```rust
let on_step_validation = Callback::new(move |(step_index, is_valid): (usize, bool)| {
    if is_valid {
        log::info!("Step {} is valid", step_index);
    } else {
        log::info!("Step {} has validation errors", step_index);
    }
});

<FormWizard
    // ... other props
    validate_on_step_change=signal(true).0.into()
    on_step_validation=Some(on_step_validation)
/>
```

### Custom Navigation Logic

```rust
let on_step_change = Callback::new(move |step_index: usize| {
    // Custom logic before changing step
    if can_proceed_to_step(step_index) {
        set_current_step.set(step_index);
        log::info!("Changed to step: {}", step_index);
    } else {
        log::warn!("Cannot proceed to step: {}", step_index);
    }
});
```

### Form Completion Handling

```rust
let on_wizard_complete = Callback::new(move |form_data: MyFormType| {
    log::info!("Wizard completed: {:?}", form_data);
    
    // Submit form data
    spawn_local(async move {
        match submit_form(form_data).await {
            Ok(_) => log::info!("Form submitted successfully"),
            Err(e) => log::error!("Form submission failed: {}", e),
        }
    });
});

<FormWizard
    // ... other props
    on_wizard_complete=Some(on_wizard_complete)
/>
```

### Conditional Step Rendering

```rust
let steps = vec![
    WizardStep {
        title: "Basic Info".to_string(),
        description: Some("Enter basic information".to_string()),
        content: view! {
            <div class="basic-info-step">
                <BasicInfoForm />
            </div>
        },
    },
    WizardStep {
        title: "Advanced Settings".to_string(),
        description: Some("Configure advanced options".to_string()),
        content: view! {
            <div class="advanced-settings-step">
                <AdvancedSettingsForm />
            </div>
        },
    },
];
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
.form-wizard {
    --primary-color: #8b5cf6;
    --border-radius: 12px;
    --shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
}

.wizard-step.current .step-number {
    background: var(--primary-color);
    box-shadow: 0 0 0 4px rgba(139, 92, 246, 0.1);
}
```

## Integration with Form Validation

The Form Wizard integrates seamlessly with the form validation system:

```rust
// In your form validation
fn validate(&self) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();
    
    // Validate current step based on step index
    match self.current_step {
        0 => self.validate_personal_info(&mut errors)?,
        1 => self.validate_preferences(&mut errors)?,
        2 => self.validate_confirmation(&mut errors)?,
        _ => return Err(ValidationErrors::new()),
    }
    
    Ok(())
}

// Step-specific validation
fn validate_personal_info(&self, errors: &mut ValidationErrors) -> Result<(), ValidationErrors> {
    if self.personal_info.first_name.is_empty() {
        errors.add_field_error("first_name".to_string(), "First name is required".to_string());
    }
    
    if self.personal_info.email.is_empty() {
        errors.add_field_error("email".to_string(), "Email is required".to_string());
    }
    
    Ok(())
}
```

## Performance Considerations

- **Signal Optimization**: Uses efficient signal handling to minimize re-renders
- **Lazy Content**: Step content is only rendered when visible
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

- **Step Dependencies**: Conditional step availability based on previous steps
- **Dynamic Step Generation**: Generate steps based on form data
- **Advanced Validation**: Real-time validation with custom rules
- **Step Templates**: Pre-built step templates for common use cases
- **Export/Import**: Step configuration import/export functionality
- **Undo/Redo**: Step history and undo support

## Examples

See the `form_wizard_example.rs` file for complete working examples demonstrating:

- Basic wizard setup
- Step validation
- Custom navigation
- Form completion handling
- Responsive design
- Accessibility features

## Common Use Cases

### User Registration
```rust
let steps = vec![
    WizardStep {
        title: "Account Details".to_string(),
        description: Some("Create your account".to_string()),
        content: view! { <AccountDetailsForm /> },
    },
    WizardStep {
        title: "Profile Information".to_string(),
        description: Some("Tell us about yourself".to_string()),
        content: view! { <ProfileForm /> },
    },
    WizardStep {
        title: "Verification".to_string(),
        description: Some("Verify your email".to_string()),
        content: view! { <VerificationForm /> },
    },
];
```

### Product Configuration
```rust
let steps = vec![
    WizardStep {
        title: "Product Selection".to_string(),
        description: Some("Choose your product".to_string()),
        content: view! { <ProductSelection /> },
    },
    WizardStep {
        title: "Customization".to_string(),
        description: Some("Customize your product".to_string()),
        content: view! { <ProductCustomization /> },
    },
    WizardStep {
        title: "Review & Order".to_string(),
        description: Some("Review and place order".to_string()),
        content: view! { <OrderReview /> },
    },
];
```

### Settings Configuration
```rust
let steps = vec![
    WizardStep {
        title: "General Settings".to_string(),
        description: Some("Basic configuration".to_string()),
        content: view! { <GeneralSettings /> },
    },
    WizardStep {
        title: "Security".to_string(),
        description: Some("Security preferences".to_string()),
        content: view! { <SecuritySettings /> },
    },
    WizardStep {
        title: "Notifications".to_string(),
        description: Some("Notification preferences".to_string()),
        content: view! { <NotificationSettings /> },
    },
];
```

## Contributing

Contributions are welcome! Please see the main project README for contribution guidelines.

## License

This component is part of the leptos-forms-rs project and is licensed under the MIT License.
