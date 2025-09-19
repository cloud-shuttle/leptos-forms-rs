# Event Handling System Design

## 🎯 Overview

This document outlines the design for a comprehensive event handling system for the Leptos Forms library, addressing the critical issue of broken event handlers that prevent form submission and reset functionality from working.

## 🚨 Current Problem

The current implementation has broken event handlers with TODO comments:

```rust
// TODO: Fix event handler for Leptos 0.8
// on:click=submit_handler
```

This prevents:

- Form submission buttons from working
- Form reset buttons from working
- Any interactive form functionality

## 🏗️ Architecture Design

### Core Event Handling System

```
┌─────────────────────────────────────────────────────────┐
│                  Event Handling Layer                   │
├─────────────────────────────────────────────────────────┤
│  Event Types  │ Event Handlers │ Event Propagation     │
│  - Click      │ - Submit       │ - Bubbling            │
│  - Change     │ - Reset        │ - Capturing           │
│  - Focus      │ - Validation   │ - Custom Events       │
│  - Blur       │ - Field Update │ - Event Delegation    │
├─────────────────────────────────────────────────────────┤
│                  Form Integration Layer                 │
├─────────────────────────────────────────────────────────┤
│  Form Events  │ Field Events   │ Validation Events     │
│  - Submit     │ - Change       │ - Real-time           │
│  - Reset      │ - Focus        │ - On-blur             │
│  - Validation │ - Blur         │ - Cross-field         │
│  - Error      │ - Input        │ - Async               │
├─────────────────────────────────────────────────────────┤
│                  Leptos Integration Layer               │
├─────────────────────────────────────────────────────────┤
│  Leptos 0.8   │ Signal System  │ Component System      │
│  - on:click   │ - RwSignal     │ - Event Callbacks     │
│  - on:change  │ - ReadSignal   │ - Event Propagation   │
│  - on:focus   │ - Memo         │ - Event Handling      │
│  - on:blur    │ - Callback     │ - Event Cleanup       │
└─────────────────────────────────────────────────────────┘
```

## 📋 Module Structure

### 1. Event Types Module (`event_types.rs` - ~150 lines)

```rust
//! Event types and event data structures

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Form event types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FormEventType {
    Submit,
    Reset,
    Validation,
    Error,
    Success,
}

/// Field event types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldEventType {
    Change,
    Focus,
    Blur,
    Input,
    KeyDown,
    KeyUp,
}

/// Event data structure
#[derive(Debug, Clone)]
pub struct FormEvent<T> {
    pub event_type: FormEventType,
    pub form_data: T,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source: String,
}

/// Field event data structure
#[derive(Debug, Clone)]
pub struct FieldEvent {
    pub event_type: FieldEventType,
    pub field_name: String,
    pub field_value: crate::core::types::FieldValue,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source: String,
}

/// Event handler result
#[derive(Debug, Clone)]
pub enum EventResult {
    Success,
    Error(String),
    PreventDefault,
    StopPropagation,
}
```

### 2. Event Handlers Module (`event_handlers.rs` - ~200 lines)

```rust
//! Event handler implementations

use crate::core::*;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Form submission handler
pub struct FormSubmitHandler<T: Form> {
    form_handle: FormHandle<T>,
    on_success: Option<Callback<T>>,
    on_error: Option<Callback<crate::error::FormError>>,
}

impl<T: Form + PartialEq + Clone + Send + Sync> FormSubmitHandler<T> {
    pub fn new(form_handle: FormHandle<T>) -> Self {
        Self {
            form_handle,
            on_success: None,
            on_error: None,
        }
    }

    pub fn on_success(mut self, callback: Callback<T>) -> Self {
        self.on_success = Some(callback);
        self
    }

    pub fn on_error(mut self, callback: Callback<crate::error::FormError>) -> Self {
        self.on_error = Some(callback);
        self
    }

    pub fn handle_submit(&self) -> impl Fn(leptos::ev::MouseEvent) + Clone {
        let form_handle = self.form_handle.clone();
        let on_success = self.on_success.clone();
        let on_error = self.on_error.clone();

        move |event: leptos::ev::MouseEvent| {
            event.prevent_default();

            let form_handle = form_handle.clone();
            let on_success = on_success.clone();
            let on_error = on_error.clone();

            spawn_local(async move {
                match form_handle.submit() {
                    Ok(form_data) => {
                        if let Some(callback) = on_success {
                            callback.call(form_data);
                        }
                    }
                    Err(error) => {
                        if let Some(callback) = on_error {
                            callback.call(error);
                        }
                    }
                }
            });
        }
    }
}

/// Form reset handler
pub struct FormResetHandler<T: Form> {
    form_handle: FormHandle<T>,
    confirm_message: Option<String>,
    on_reset: Option<Callback<()>>,
}

impl<T: Form + PartialEq + Clone + Send + Sync> FormResetHandler<T> {
    pub fn new(form_handle: FormHandle<T>) -> Self {
        Self {
            form_handle,
            confirm_message: None,
            on_reset: None,
        }
    }

    pub fn with_confirmation(mut self, message: String) -> Self {
        self.confirm_message = Some(message);
        self
    }

    pub fn on_reset(mut self, callback: Callback<()>) -> Self {
        self.on_reset = Some(callback);
        self
    }

    pub fn handle_reset(&self) -> impl Fn(leptos::ev::MouseEvent) + Clone {
        let form_handle = self.form_handle.clone();
        let confirm_message = self.confirm_message.clone();
        let on_reset = self.on_reset.clone();

        move |event: leptos::ev::MouseEvent| {
            event.prevent_default();

            let form_handle = form_handle.clone();
            let on_reset = on_reset.clone();

            if let Some(message) = &confirm_message {
                if let Some(window) = web_sys::window() {
                    if !window.confirm_with_message(message).unwrap_or(false) {
                        return;
                    }
                }
            }

            form_handle.reset();

            if let Some(callback) = on_reset {
                callback.call(());
            }
        }
    }
}
```

### 3. Field Event Handlers Module (`field_event_handlers.rs` - ~180 lines)

```rust
//! Field-specific event handlers

use crate::core::*;
use leptos::prelude::*;

/// Field change handler
pub struct FieldChangeHandler<T: Form> {
    form_handle: FormHandle<T>,
    field_name: String,
    on_change: Option<Callback<crate::core::types::FieldValue>>,
}

impl<T: Form + PartialEq + Clone + Send + Sync> FieldChangeHandler<T> {
    pub fn new(form_handle: FormHandle<T>, field_name: String) -> Self {
        Self {
            form_handle,
            field_name,
            on_change: None,
        }
    }

    pub fn on_change(mut self, callback: Callback<crate::core::types::FieldValue>) -> Self {
        self.on_change = Some(callback);
        self
    }

    pub fn handle_change(&self) -> impl Fn(leptos::ev::Event) + Clone {
        let form_handle = self.form_handle.clone();
        let field_name = self.field_name.clone();
        let on_change = self.on_change.clone();

        move |event: leptos::ev::Event| {
            if let Some(target) = event.target() {
                if let Some(input) = target.dyn_into::<web_sys::HtmlInputElement>().ok() {
                    let value = crate::core::types::FieldValue::String(input.value());
                    form_handle.set_field_value(&field_name, value);

                    if let Some(callback) = &on_change {
                        callback.call(value);
                    }
                }
            }
        }
    }
}

/// Field focus handler
pub struct FieldFocusHandler<T: Form> {
    form_handle: FormHandle<T>,
    field_name: String,
    on_focus: Option<Callback<()>>,
}

impl<T: Form + PartialEq + Clone + Send + Sync> FieldFocusHandler<T> {
    pub fn new(form_handle: FormHandle<T>, field_name: String) -> Self {
        Self {
            form_handle,
            field_name,
            on_focus: None,
        }
    }

    pub fn on_focus(mut self, callback: Callback<()>) -> Self {
        self.on_focus = Some(callback);
        self
    }

    pub fn handle_focus(&self) -> impl Fn(leptos::ev::FocusEvent) + Clone {
        let on_focus = self.on_focus.clone();

        move |_event: leptos::ev::FocusEvent| {
            if let Some(callback) = &on_focus {
                callback.call(());
            }
        }
    }
}
```

### 4. Event System Module (`event_system.rs` - ~120 lines)

```rust
//! Central event system coordination

use crate::core::*;
use leptos::prelude::*;
use std::collections::HashMap;

/// Central event system for form management
pub struct EventSystem<T: Form> {
    form_handle: FormHandle<T>,
    event_handlers: HashMap<String, Box<dyn EventHandler>>,
}

/// Trait for event handlers
pub trait EventHandler: Send + Sync {
    fn handle(&self, event: &dyn std::any::Any);
}

impl<T: Form + PartialEq + Clone + Send + Sync> EventSystem<T> {
    pub fn new(form_handle: FormHandle<T>) -> Self {
        Self {
            form_handle,
            event_handlers: HashMap::new(),
        }
    }

    /// Register an event handler
    pub fn register_handler(&mut self, name: String, handler: Box<dyn EventHandler>) {
        self.event_handlers.insert(name, handler);
    }

    /// Get form submit handler
    pub fn get_submit_handler(&self) -> FormSubmitHandler<T> {
        FormSubmitHandler::new(self.form_handle.clone())
    }

    /// Get form reset handler
    pub fn get_reset_handler(&self) -> FormResetHandler<T> {
        FormResetHandler::new(self.form_handle.clone())
    }

    /// Get field change handler
    pub fn get_field_change_handler(&self, field_name: String) -> FieldChangeHandler<T> {
        FieldChangeHandler::new(self.form_handle.clone(), field_name)
    }

    /// Get field focus handler
    pub fn get_field_focus_handler(&self, field_name: String) -> FieldFocusHandler<T> {
        FieldFocusHandler::new(self.form_handle.clone(), field_name)
    }
}
```

## 🔧 Implementation Strategy

### Phase 1: Core Event Handlers (Week 1)

1. **Implement FormSubmitHandler**
   - Fix the broken submit button functionality
   - Add proper error handling
   - Add success callbacks

2. **Implement FormResetHandler**
   - Fix the broken reset button functionality
   - Add confirmation dialogs
   - Add reset callbacks

### Phase 2: Field Event Handlers (Week 2)

1. **Implement FieldChangeHandler**
   - Handle field value changes
   - Trigger validation
   - Update form state

2. **Implement FieldFocusHandler**
   - Handle field focus events
   - Track touched state
   - Trigger field-specific validation

### Phase 3: Event System Integration (Week 3)

1. **Implement EventSystem**
   - Central event coordination
   - Event handler registration
   - Event propagation management

2. **Update Components**
   - Fix FormSubmit component
   - Fix FormReset component
   - Update FormField component

## 🧪 Testing Strategy

### Unit Tests

1. **Event Handler Tests**
   - Test form submission handling
   - Test form reset handling
   - Test field change handling
   - Test error scenarios

2. **Event System Tests**
   - Test event registration
   - Test event propagation
   - Test event cleanup

### Integration Tests

1. **Component Integration Tests**
   - Test FormSubmit component
   - Test FormReset component
   - Test FormField component

2. **End-to-End Tests**
   - Test complete form submission flow
   - Test form reset flow
   - Test field interaction flow

## 📊 Success Metrics

### Functional Requirements

- ✅ Form submission buttons work
- ✅ Form reset buttons work
- ✅ Field change events trigger validation
- ✅ Field focus events work
- ✅ Event handlers are properly integrated

### Performance Requirements

- ✅ Event handlers respond within 100ms
- ✅ No memory leaks in event system
- ✅ Event cleanup works properly

### Quality Requirements

- ✅ All event handlers have comprehensive tests
- ✅ Event system is well-documented
- ✅ Event handlers follow Leptos 0.8 patterns

## 🚀 Migration Plan

### Step 1: Update FormSubmit Component

```rust
// Before (broken)
// TODO: Fix event handler for Leptos 0.8
// on:click=submit_handler

// After (working)
on:click=move |event| {
    let handler = event_system.get_submit_handler();
    handler.handle_submit()(event);
}
```

### Step 2: Update FormReset Component

```rust
// Before (broken)
// TODO: Fix event handler for Leptos 0.8
// on:click=reset_handler

// After (working)
on:click=move |event| {
    let handler = event_system.get_reset_handler();
    handler.handle_reset()(event);
}
```

### Step 3: Update Examples

Replace manual form handling with proper event system integration.

## 📋 File Structure

```
leptos-forms-rs/src/events/
├── mod.rs                    # Event system module exports
├── event_types.rs           # Event types and data structures (~150 lines)
├── event_handlers.rs        # Form event handlers (~200 lines)
├── field_event_handlers.rs  # Field event handlers (~180 lines)
└── event_system.rs          # Central event system (~120 lines)
```

**Total: ~650 lines across 4 focused modules (all under 300 lines each)**

## 🎯 Expected Outcomes

After implementation:

1. **Form submission buttons will work** - Users can submit forms
2. **Form reset buttons will work** - Users can reset forms
3. **Field interactions will work** - Real-time validation and updates
4. **Examples will be functional** - Demonstrating real library capabilities
5. **Event system will be extensible** - Easy to add new event types

This design provides a solid foundation for a comprehensive event handling system that integrates seamlessly with Leptos 0.8 and provides the functionality needed for production-ready forms.
