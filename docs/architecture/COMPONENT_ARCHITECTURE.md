# Leptos Forms - Component Architecture Design

## Architecture Overview

The component architecture follows a layered approach with clear separation of concerns, enabling maximum reusability and type safety while maintaining performance.

```
┌─────────────────────────────────────────────────────────┐
│                  Application Layer                      │
│  User Forms, Business Logic, Custom Components         │
├─────────────────────────────────────────────────────────┤
│                  Composition Layer                      │
│  FormWizard, ConditionalField, FieldArray              │
├─────────────────────────────────────────────────────────┤
│                  Component Layer                        │
│  FormField, TextInput, Select, FileInput               │
├─────────────────────────────────────────────────────────┤
│                  Integration Layer                      │
│  UI Library Adapters (shadcn-ui, radix-leptos)        │
├─────────────────────────────────────────────────────────┤
│                   Primitive Layer                       │
│  Form Hook, Field Registration, Event Handling         │
├─────────────────────────────────────────────────────────┤
│                   Foundation Layer                      │
│  Form Trait, Validation, State Management              │
└─────────────────────────────────────────────────────────┘
```

## Core Principles

### 1. Headless Architecture

- **Separation**: Logic separated from presentation
- **Flexibility**: Works with any UI library
- **Customization**: Full control over styling
- **Accessibility**: Built-in ARIA support

### 2. Reactive Design

- **Signals**: Leptos-native reactivity
- **Granular Updates**: Minimal re-renders
- **Derived State**: Computed properties
- **Effect Management**: Automatic cleanup

### 3. Type Safety

- **Compile-time Validation**: Catch errors early
- **Generic Design**: Type-safe field handling
- **Trait Bounds**: Interface contracts
- **Zero-cost Abstractions**: No runtime overhead

## Component Hierarchy

### Foundation Components

#### 1. FormProvider (`components/provider.rs`)

**Purpose**: Context provider for form state and configuration.

```rust
#[component]
pub fn FormProvider<T: Form>(
    form_handle: FormHandle<T>,
    children: Children,
) -> impl IntoView {
    view! {
        <Provider value={form_handle}>
            {children()}
        </Provider>
    }
}
```

**Responsibilities**:

- Form context distribution
- State management coordination
- Error boundary handling
- Event propagation

#### 2. FormRoot (`components/root.rs`)

**Purpose**: Root form element with submission handling.

```rust
#[component]
pub fn Form<T: Form>(
    #[prop(into)] class: Option<String>,
    #[prop(optional)] on_submit: Option<Callback<T>>,
    #[prop(optional)] prevent_default: bool,
    children: Children,
) -> impl IntoView
```

**Features**:

- Form submission coordination
- Validation trigger
- Error handling
- Accessibility attributes

### Field Components

#### 1. FormField (`components/field.rs`)

**Purpose**: Wrapper component providing field context and error display.

```rust
#[component]
pub fn FormField(
    #[prop(into)] name: String,
    #[prop(into)] label: String,
    #[prop(optional)] description: Option<String>,
    #[prop(optional)] required: bool,
    #[prop(optional)] error_display: ErrorDisplay,
    children: Children,
) -> impl IntoView
```

**Architecture**:

```
FormField
├── Label (with required indicator)
├── Description (optional)
├── Field Content (children)
└── Error Message (conditional)
```

**Responsibilities**:

- Field registration coordination
- Label association
- Error state management
- Accessibility compliance

#### 2. Field Input Components

##### TextInput (`components/inputs/text.rs`)

```rust
#[component]
pub fn TextInput(
    #[prop(into)] name: String,
    #[prop(optional)] input_type: InputType,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] autocomplete: Option<String>,
    #[prop(optional)] disabled: Signal<bool>,
    #[prop(spread)] additional_props: Vec<(&'static str, Attribute)>,
) -> impl IntoView
```

##### SelectInput (`components/inputs/select.rs`)

```rust
#[component]
pub fn Select<T: SelectOption>(
    #[prop(into)] name: String,
    #[prop(into)] options: Signal<Vec<T>>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] multiple: bool,
    #[prop(optional)] searchable: bool,
) -> impl IntoView
```

##### FileInput (`components/inputs/file.rs`)

```rust
#[component]
pub fn FileInput(
    #[prop(into)] name: String,
    #[prop(optional)] accept: Option<String>,
    #[prop(optional)] multiple: bool,
    #[prop(optional)] max_size: Option<usize>,
    #[prop(optional)] on_progress: Option<Callback<f32>>,
) -> impl IntoView
```

### Composition Components

#### 1. FieldArray (`components/array.rs`)

**Purpose**: Dynamic field arrays with add/remove/reorder functionality.

```rust
#[component]
pub fn FieldArray<T: Clone + Default + 'static>(
    #[prop(into)] name: String,
    #[prop(into)] render_item: Callback<(usize, Signal<T>, ArrayHelpers<T>), View>,
    #[prop(optional)] min_items: Option<usize>,
    #[prop(optional)] max_items: Option<usize>,
) -> impl IntoView
```

**Component Structure**:

```
FieldArray
├── Array Items (dynamic)
│   ├── Item Content (render_item callback)
│   └── Control Buttons (move, remove)
└── Add Button
```

**Array Helpers API**:

```rust
pub struct ArrayHelpers<T> {
    pub append: Callback<T>,
    pub prepend: Callback<T>,
    pub insert: Callback<(usize, T)>,
    pub remove: Callback<usize>,
    pub move_item: Callback<(usize, usize)>,
    pub swap: Callback<(usize, usize)>,
}
```

#### 2. ConditionalField (`components/conditional.rs`)

**Purpose**: Show/hide fields based on form state.

```rust
#[component]
pub fn ConditionalField<T: Form>(
    #[prop(into)] when: Signal<bool>,
    #[prop(optional)] unmount_on_hide: bool,
    #[prop(optional)] animate: bool,
    children: Children,
) -> impl IntoView
```

**Behavior Options**:

- **Visible/Hidden**: CSS display control
- **Mount/Unmount**: DOM presence control
- **Animated**: Transition support

#### 3. FormWizard (`components/wizard/`)

**Purpose**: Multi-step form with navigation and progress tracking.

```rust
#[component]
pub fn FormWizard<T: Form>(
    steps: Vec<WizardStep<T>>,
    #[prop(optional)] show_progress: bool,
    #[prop(optional)] allow_step_clicking: bool,
    #[prop(optional)] on_complete: Option<Callback<T>>,
) -> impl IntoView
```

**Wizard Architecture**:

```
FormWizard
├── StepIndicator (progress display)
├── StepContent (current step fields)
└── Navigation
    ├── Previous Button
    ├── Next Button
    └── Submit Button (final step)
```

## Field Registration System

### Registration Flow

```
1. Field Component Mounts
   ↓
2. Calls form.register(name)
   ↓
3. Creates FieldRegistration
   ↓
4. Returns reactive signals & handlers
   ↓
5. Component binds to DOM element
   ↓
6. User interaction triggers handlers
   ↓
7. State updates propagate
```

### FieldRegistration Interface

```rust
pub struct FieldRegistration {
    // Identity
    pub name: String,
    pub field_type: FieldType,

    // State Signals
    pub value: Signal<FieldValue>,
    pub error: Signal<Option<String>>,
    pub is_touched: Signal<bool>,
    pub is_dirty: Signal<bool>,
    pub is_focused: Signal<bool>,

    // Event Handlers
    pub on_input: Callback<Event>,
    pub on_change: Callback<Event>,
    pub on_blur: Callback<FocusEvent>,
    pub on_focus: Callback<FocusEvent>,

    // DOM Properties
    pub props: FieldProps,

    // Methods
    pub set_value: Callback<FieldValue>,
    pub clear_error: Callback<()>,
    pub validate: Callback<()>,
}
```

### FieldProps for DOM Binding

```rust
pub struct FieldProps {
    pub id: String,
    pub name: String,
    pub value: Signal<String>,
    pub disabled: Signal<bool>,
    pub required: Signal<bool>,

    // Accessibility
    pub aria_invalid: Signal<bool>,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_required: Signal<bool>,

    // Event Handlers (spread-ready)
    pub on_input: Callback<Event>,
    pub on_change: Callback<Event>,
    pub on_blur: Callback<FocusEvent>,
    pub on_focus: Callback<FocusEvent>,
}
```

## UI Library Integration

### Adapter Pattern

Each UI library gets its own adapter module that translates between the library's API and leptos-forms.

#### Shadcn-ui Integration (`integrations/shadcn.rs`)

```rust
#[component]
pub fn ShadcnInput(
    #[prop(into)] name: String,
    #[prop(optional)] variant: InputVariant,
    #[prop(spread)] props: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let form = use_form_context::<T>();
    let field = form.register(&name);

    view! {
        <cn::Input
            class={cn::input_variants(variant, field.error.get().is_some())}
            value={field.value}
            on:input={field.on_input}
            on:blur={field.on_blur}
            aria-invalid={field.error.get().is_some()}
            ..props
        />
    }
}
```

#### Radix-leptos Integration (`integrations/radix.rs`)

```rust
#[component]
pub fn RadixTextField(
    #[prop(into)] name: String,
    #[prop(into)] label: String,
    #[prop(spread)] props: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let field = use_field_registration(&name);

    view! {
        <TextField
            value={field.value}
            error={field.error}
            label={label}
            on_value_change={field.on_change}
            ..props
        />
    }
}
```

## Event System Architecture

### Event Flow

```
DOM Event → Field Handler → Validation → State Update → UI Re-render
    ↑                                            ↓
    └─── Focus Management ←─── Error Display ←──┘
```

### Event Handlers

#### Input Event Handler

```rust
let on_input = {
    let field_name = field_name.clone();
    let set_field_value = set_field_value.clone();

    Callback::new(move |ev: Event| {
        let value = extract_input_value(&ev);
        set_field_value(&field_name, value);

        // Trigger validation if configured
        if validation_mode == ValidationMode::OnChange {
            trigger_field_validation(&field_name);
        }
    })
};
```

#### Blur Event Handler

```rust
let on_blur = {
    let field_name = field_name.clone();
    let set_touched = set_touched.clone();

    Callback::new(move |_: FocusEvent| {
        set_touched.update(|touched| {
            touched.insert(field_name.clone());
        });

        // Trigger validation on blur
        if validation_mode == ValidationMode::OnBlur {
            trigger_field_validation(&field_name);
        }
    })
};
```

## State Management Architecture

### State Structure

```rust
pub struct FormState<T: Form> {
    // Primary State
    pub values: T,
    pub errors: ValidationErrors,
    pub touched: HashSet<String>,
    pub dirty_fields: HashSet<String>,

    // UI State
    pub is_submitting: bool,
    pub submit_count: u32,
    pub focused_field: Option<String>,

    // Configuration
    pub validation_mode: ValidationMode,
    pub submit_on_enter: bool,
}
```

### Derived State

```rust
// Computed automatically from base state
pub struct DerivedState {
    pub is_valid: bool,           // errors.is_empty()
    pub is_dirty: bool,           // !dirty_fields.is_empty()
    pub is_pristine: bool,        // !is_dirty
    pub is_touched: bool,         // !touched.is_empty()
    pub can_submit: bool,         // is_valid && !is_submitting
}
```

### State Updates

#### Value Changes

```rust
fn update_field_value(field: &str, value: FieldValue) {
    // 1. Update form values
    form_state.values.set_field(field, value);

    // 2. Mark field as dirty
    form_state.dirty_fields.insert(field);

    // 3. Trigger validation if needed
    if validation_mode.should_validate_on_change() {
        validate_field(field);
    }

    // 4. Update derived state
    update_derived_state();
}
```

## Accessibility Architecture

### ARIA Implementation

Every form component includes proper ARIA attributes:

```rust
pub fn aria_attributes(field: &FieldRegistration) -> Vec<(&'static str, Attribute)> {
    vec![
        ("id", field.id.into()),
        ("aria-invalid", field.error.get().is_some().into()),
        ("aria-required", field.is_required.into()),
        ("aria-describedby", {
            let mut ids = Vec::new();
            if let Some(error_id) = field.error_id.get() {
                ids.push(error_id);
            }
            if let Some(desc_id) = field.description_id.get() {
                ids.push(desc_id);
            }
            (!ids.is_empty()).then(|| ids.join(" "))
        }.into()),
    ]
}
```

### Keyboard Navigation

- Tab order management
- Enter key submission
- Arrow key navigation in arrays
- Escape key for cancellation

### Screen Reader Support

- Proper labeling
- Error announcements
- Progress indicators
- State changes

## Performance Optimizations

### Memoization Strategy

```rust
// Memoize expensive computations
let validation_result = create_memo(move |_| {
    values.get().validate()
});

// Memoize field-specific state
let field_value = create_memo(move |_| {
    values.get().get_field(&field_name)
});
```

### Lazy Rendering

```rust
// Only render visible fields
<Show when=move || is_field_visible(&field_name)>
    <FieldComponent name={&field_name} />
</Show>
```

### Debounced Validation

```rust
let debounced_validate = use_debounced_callback(
    move |field_name: String| {
        validate_field(&field_name);
    },
    300, // ms
);
```

## Testing Architecture

### Component Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos_testing::*;

    #[test]
    fn test_text_input_registration() {
        let form = create_test_form(TestForm::default());

        mount_component(|| {
            view! {
                <FormProvider form_handle={form}>
                    <TextInput name="test_field" />
                </FormProvider>
            }
        });

        // Test field registration
        assert!(form.is_field_registered("test_field"));

        // Test input handling
        simulate_input("test_field", "test value");
        assert_eq!(form.values.get().test_field, "test value");
    }
}
```

### Integration Testing

```rust
#[test]
fn test_form_wizard_navigation() {
    let form = create_test_form(WizardForm::default());

    mount_wizard_component(form.clone());

    // Test step navigation
    assert_eq!(get_current_step(), 0);

    fill_step_fields();
    click_next_button();

    assert_eq!(get_current_step(), 1);
}
```

This component architecture provides a solid foundation for building flexible, accessible, and performant form components that work seamlessly with any UI library while maintaining type safety and excellent developer experience.
