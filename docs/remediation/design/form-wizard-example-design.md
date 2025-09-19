# Form Wizard Example Refactoring Design

## 📋 Overview

**Current State**: `components/form_wizard_example.rs` (432 lines) - Large example file exceeding target by 1.4x
**Target State**: 4 focused modules, each under 300 lines
**Priority**: P1 - High (example code should be well-organized and maintainable)

## 🎯 Refactoring Goals

1. **Split monolithic example file** into focused, single-responsibility modules
2. **Maintain example functionality** - all existing features preserved
3. **Improve example organization** and readability
4. **Enhance example documentation** and usage examples

## 🏗️ New Module Structure

```
components/examples/
├── mod.rs                 # Re-exports and module organization (~50 lines)
├── wizard_steps.rs        # Wizard step definitions (~120 lines)
├── wizard_navigation.rs   # Wizard navigation logic (~100 lines)
├── wizard_validation.rs   # Wizard validation logic (~90 lines)
└── wizard_ui.rs           # Wizard UI components (~100 lines)
```

## 📝 Module Specifications

### 1. `mod.rs` (Re-exports)

**Target Size**: ~50 lines
**Purpose**: Module organization and public API

```rust
//! Form Wizard Example - Multi-step form wizard implementation
//!
//! This module provides:
//! - Wizard step definitions
//! - Wizard navigation logic
//! - Wizard validation logic
//! - Wizard UI components

pub mod wizard_steps;
pub mod wizard_navigation;
pub mod wizard_validation;
pub mod wizard_ui;

// Re-export public API
pub use wizard_steps::*;
pub use wizard_navigation::*;
pub use wizard_validation::*;
pub use wizard_ui::*;

// Main wizard component
pub use wizard_ui::FormWizardExample;
```

### 2. `wizard_steps.rs` (Wizard Step Definitions)

**Target Size**: ~120 lines
**Purpose**: Define wizard steps and step data structures

**Key Components**:

- Step data structures
- Step configuration
- Step validation rules
- Step navigation rules

**Public API**:

```rust
/// Wizard step data structure
#[derive(Debug, Clone)]
pub struct WizardStep {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub fields: Vec<WizardField>,
    pub validation_rules: Vec<Validator>,
    pub navigation_rules: NavigationRules,
}

/// Wizard field definition
#[derive(Debug, Clone)]
pub struct WizardField {
    pub name: String,
    pub label: String,
    pub field_type: FieldType,
    pub required: bool,
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
}

/// Navigation rules for wizard steps
#[derive(Debug, Clone)]
pub struct NavigationRules {
    pub can_go_back: bool,
    pub can_skip: bool,
    pub next_step_condition: Option<String>,
    pub previous_step_condition: Option<String>,
}

/// Wizard step definitions
pub struct WizardSteps;

impl WizardSteps {
    pub fn get_steps() -> Vec<WizardStep>
    pub fn get_step_by_id(id: &str) -> Option<WizardStep>
    pub fn get_step_index(id: &str) -> Option<usize>
    pub fn get_total_steps() -> usize
}
```

### 3. `wizard_navigation.rs` (Wizard Navigation Logic)

**Target Size**: ~100 lines
**Purpose**: Handle wizard navigation and step transitions

**Key Components**:

- Navigation state management
- Step transition logic
- Navigation validation
- Navigation history

**Public API**:

```rust
/// Wizard navigation state
#[derive(Debug, Clone)]
pub struct WizardNavigation {
    pub current_step: usize,
    pub total_steps: usize,
    pub step_history: Vec<usize>,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub can_skip: bool,
}

/// Wizard navigation logic
pub struct WizardNavigationLogic;

impl WizardNavigationLogic {
    pub fn new(total_steps: usize) -> WizardNavigation
    pub fn go_to_step(navigation: &mut WizardNavigation, step: usize) -> Result<(), String>
    pub fn go_to_next_step(navigation: &mut WizardNavigation) -> Result<(), String>
    pub fn go_to_previous_step(navigation: &mut WizardNavigation) -> Result<(), String>
    pub fn skip_current_step(navigation: &mut WizardNavigation) -> Result<(), String>
    pub fn can_navigate_to_step(navigation: &WizardNavigation, step: usize) -> bool
    pub fn get_available_steps(navigation: &WizardNavigation) -> Vec<usize>
    pub fn reset_navigation(navigation: &mut WizardNavigation)
}
```

### 4. `wizard_validation.rs` (Wizard Validation Logic)

**Target Size**: ~90 lines
**Purpose**: Handle wizard step validation and form validation

**Key Components**:

- Step validation logic
- Form validation integration
- Validation error handling
- Validation state management

**Public API**:

```rust
/// Wizard validation state
#[derive(Debug, Clone)]
pub struct WizardValidation {
    pub step_errors: std::collections::HashMap<usize, ValidationErrors>,
    pub form_errors: ValidationErrors,
    pub is_valid: bool,
    pub validation_mode: ValidationMode,
}

/// Wizard validation logic
pub struct WizardValidationLogic;

impl WizardValidationLogic {
    pub fn new() -> WizardValidation
    pub fn validate_step<T: Form>(
        validation: &mut WizardValidation,
        form: &FormHandle<T>,
        step: &WizardStep,
    ) -> Result<(), ValidationErrors>
    pub fn validate_all_steps<T: Form>(
        validation: &mut WizardValidation,
        form: &FormHandle<T>,
        steps: &[WizardStep],
    ) -> Result<(), ValidationErrors>
    pub fn clear_step_errors(validation: &mut WizardValidation, step: usize)
    pub fn clear_all_errors(validation: &mut WizardValidation)
    pub fn has_step_errors(validation: &WizardValidation, step: usize) -> bool
    pub fn has_any_errors(validation: &WizardValidation) -> bool
    pub fn get_step_errors(validation: &WizardValidation, step: usize) -> Option<&ValidationErrors>
}
```

### 5. `wizard_ui.rs` (Wizard UI Components)

**Target Size**: ~100 lines
**Purpose**: Wizard UI components and rendering

**Key Components**:

- Main wizard component
- Step navigation UI
- Step content rendering
- Wizard progress indicator

**Public API**:

```rust
/// Main wizard component
#[component]
pub fn FormWizardExample() -> impl IntoView

/// Wizard step navigation component
#[component]
pub fn WizardStepNavigation(
    navigation: ReadSignal<WizardNavigation>,
    on_navigate: impl Fn(usize) + 'static + Clone,
) -> impl IntoView

/// Wizard step content component
#[component]
pub fn WizardStepContent(
    step: ReadSignal<WizardStep>,
    form: FormHandle<WizardFormData>,
    validation: ReadSignal<WizardValidation>,
) -> impl IntoView

/// Wizard progress indicator component
#[component]
pub fn WizardProgressIndicator(
    navigation: ReadSignal<WizardNavigation>,
) -> impl IntoView

/// Wizard form data structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WizardFormData {
    pub personal_info: PersonalInfo,
    pub contact_info: ContactInfo,
    pub preferences: Preferences,
    pub confirmation: Confirmation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalInfo {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<NaiveDate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContactInfo {
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    pub newsletter: bool,
    pub notifications: bool,
    pub theme: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Confirmation {
    pub terms_accepted: bool,
    pub privacy_accepted: bool,
}
```

## 🔄 Migration Strategy

### Step 1: Create New Module Structure

1. Create new module files with empty implementations
2. Move wizard logic to appropriate modules
3. Update imports in `mod.rs`

### Step 2: Move Implementation Code

1. Move step definitions to `wizard_steps.rs`
2. Move navigation logic to `wizard_navigation.rs`
3. Move validation logic to `wizard_validation.rs`
4. Move UI components to `wizard_ui.rs`

### Step 3: Update Dependencies

1. Update imports throughout codebase
2. Ensure all tests still pass
3. Update documentation

### Step 4: Validation

1. Run full test suite
2. Verify wizard functionality still works
3. Check file sizes meet targets

## 🧪 Testing Strategy

### Unit Tests

- Each module gets its own test file
- Test wizard step definitions
- Test navigation logic
- Test validation logic
- Test UI components

### Integration Tests

- Test wizard flows end-to-end
- Test wizard navigation
- Test wizard validation
- Test wizard UI interactions

### Example Tests

- Test wizard example functionality
- Test wizard example UI
- Test wizard example validation
- Test wizard example navigation

## 📊 Success Criteria

### File Size Compliance

- ✅ All modules under 300 lines
- ✅ `mod.rs` under 100 lines
- ✅ Largest module under 150 lines

### Functionality

- ✅ All existing tests pass
- ✅ No breaking changes to wizard functionality
- ✅ Wizard behavior maintained
- ✅ Wizard UI preserved

### Code Quality

- ✅ Clear wizard organization
- ✅ Single responsibility per module
- ✅ Consistent wizard patterns
- ✅ Comprehensive wizard documentation

## 🔍 Implementation Notes

### Dependencies

- `mod.rs` - Re-exports and module organization
- `wizard_steps.rs` - Depends on field types and validation
- `wizard_navigation.rs` - No internal dependencies
- `wizard_validation.rs` - Depends on validation and form handle
- `wizard_ui.rs` - Depends on all other wizard modules

### Wizard Patterns

- Maintain existing wizard functionality
- Preserve wizard behavior
- Keep wizard UI styling
- Maintain wizard accessibility

### Performance Considerations

- Minimize wizard re-renders
- Optimize wizard state updates
- Cache wizard calculations
- Optimize wizard navigation

## 📚 References

- [Original form_wizard_example.rs](../../../leptos-forms-rs/src/components/form_wizard_example.rs)
- [Wizard Tests](../../../tests/unit/src/multi_step_forms_wizard_tests.rs)
- [Multi-step Form Tests](../../../tests/unit/src/multi_step_forms_tests.rs)
- [Form Wizard Tests](../../../tests/unit/src/form_wizard_tests.rs)
