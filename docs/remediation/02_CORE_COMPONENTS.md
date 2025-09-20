# Remediation Plan 02: Core Component Implementation

**Priority**: HIGH
**Estimated Effort**: 4-5 days
**Blockers**: 01_BUILD_FIXES must be complete
**Dependencies**: Working derive macro, helper functions

## Overview

Implement the essential Leptos components that users expect from a form library.

## Components to Implement

### 1. FormField Component

**File**: `leptos-forms-rs/src/components/form_field.rs`
**Purpose**: Render form inputs with proper signal binding
**Max Lines**: 250

```rust
#[component]
pub fn FormField(
    form: FormHandle<T>,
    name: String,
    #[prop(optional)] field_type: Option<FieldType>,
    #[prop(optional)] class: Option<String>
) -> impl IntoView
```

**Features**:

- Automatic input type detection
- Two-way data binding with form signals
- Validation error display
- Accessibility attributes

### 2. FormSubmit Component

**File**: `leptos-forms-rs/src/components/form_submit.rs`
**Purpose**: Handle form submission with validation
**Max Lines**: 150

```rust
#[component]
pub fn FormSubmit<T, F>(
    form: FormHandle<T>,
    on_submit: F
) -> impl IntoView
where F: Fn(T) -> Result<(), ValidationErrors> + 'static
```

### 3. FormReset Component

**File**: `leptos-forms-rs/src/components/form_reset.rs`
**Purpose**: Reset form to default state
**Max Lines**: 100

### 4. FormErrors Component

**File**: `leptos-forms-rs/src/components/form_errors.rs`
**Purpose**: Display validation errors
**Max Lines**: 200

## Implementation Requirements

### Signal Integration

- Use `create_effect` for reactive updates
- Proper cleanup on component unmount
- Efficient re-rendering (avoid unnecessary updates)

### Accessibility

- ARIA labels and descriptions
- Keyboard navigation support
- Screen reader compatibility
- Focus management

### Validation Integration

- Real-time validation on input
- Display validation errors inline
- Support for async validation
- Debounced validation for performance

## File Structure Changes

**Split large files**:

```
leptos-forms-rs/src/components/
├── form_field/
│   ├── mod.rs (< 100 lines)
│   ├── input_types.rs (< 200 lines)
│   ├── validation.rs (< 150 lines)
│   └── accessibility.rs (< 100 lines)
├── form_submit.rs (< 150 lines)
├── form_reset.rs (< 100 lines)
├── form_errors.rs (< 200 lines)
└── mod.rs (< 50 lines)
```

## Testing Requirements

### Unit Tests

- Component rendering with different props
- Signal updates and reactivity
- Validation error handling
- Accessibility attribute generation

### Integration Tests

- Full form lifecycle (render → input → validate → submit)
- Error state management
- Component interaction

### WASM Tests

- Browser event handling
- DOM manipulation
- Signal updates in browser context

## Acceptance Criteria

- [ ] Components render correctly in Leptos applications
- [ ] Two-way data binding works with form state
- [ ] Validation errors display properly
- [ ] Accessibility attributes are present
- [ ] Components are responsive to form state changes
- [ ] WASM compilation succeeds
- [ ] All components under 300 lines
- [ ] Test coverage > 80% for each component

## Example Usage

```rust
view! {
    <div class="form">
        <FormField form=form name="email" />
        <FormField form=form name="password" field_type=FieldType::Password />
        <FormErrors form=form />
        <FormSubmit form=form on_submit=handle_submit />
        <FormReset form=form />
    </div>
}
```

## Next Steps

After completion, proceed to `03_VALIDATION_SYSTEM.md`
