//! Form wizard hook
//!
//! This module provides the `use_form_wizard` hook for managing
//! multi-step form wizards.

use crate::core::traits::Form;
use leptos::prelude::*;

/// Type alias for wizard hook return type
pub type WizardHookReturn = (
    ReadSignal<usize>,
    Callback<()>,
    Callback<()>,
    Callback<usize>,
    Callback<()>,
);

/// Hook for form wizard functionality
pub fn use_form_wizard<T: Form + PartialEq + Clone + Send + Sync>(
    steps: Vec<String>,
) -> WizardHookReturn {
    let current_step = RwSignal::new(0);
    let steps1 = steps.clone();

    let next_step = Callback::new(move |_| {
        let step = current_step.get();
        if step < steps1.len() - 1 {
            current_step.set(step + 1);
        }
    });

    let prev_step = Callback::new(move |_| {
        let step = current_step.get();
        if step > 0 {
            current_step.set(step - 1);
        }
    });

    let steps3 = steps.clone();
    let go_to_step = Callback::new(move |step: usize| {
        if step < steps3.len() {
            current_step.set(step);
        }
    });

    let reset_wizard = Callback::new(move |_| {
        current_step.set(0);
    });

    (
        current_step.read_only(),
        next_step,
        prev_step,
        go_to_step,
        reset_wizard,
    )
}
