//! Tests for Leptos 0.8.x compatibility, specifically for conditional class attributes

use leptos::prelude::*;
use leptos_forms_rs::components::*;

/// Test that conditional classes work correctly with Memo<bool> using closure pattern
#[test]
fn test_conditional_class_with_memo_bool_closure() {
    let (condition, _) = signal(false);
    let memo_condition = Memo::new(move |_| condition.get());

    // This should compile and work with the closure pattern
    let _view = view! {
        <div class:active=move || memo_condition.get()>
            "Test Content"
        </div>
    };

    // Test that the memo updates correctly
    assert!(!memo_condition.get());
}

/// Test form wizard compatibility with conditional classes
#[test]
fn test_form_wizard_conditional_classes() {
    let (current_step, _) = signal(0);

    // Use signals directly - this is the recommended pattern
    let is_active = Memo::new(move |_| current_step.get() == 0);
    let is_completed = Memo::new(move |_| current_step.get() > 0);

    // This should compile and work with the closure pattern
    let _view = view! {
        <div class="wizard-step"
             class:active=move || is_active.get()
             class:completed=move || is_completed.get()>
            "Wizard Step"
        </div>
    };

    assert_eq!(current_step.get(), 0);
}

/// Test that direct signal usage works (preferred pattern)
#[test]
fn test_conditional_class_with_direct_signal() {
    let (is_active, _) = signal(true);

    // This should work without issues
    let _view = view! {
        <div class:active=is_active>
            "Direct Signal Test"
        </div>
    };

    assert!(is_active.get());
}

/// Test computed class string pattern
#[test]
fn test_computed_class_string() {
    let (is_active, _) = signal(true);
    let (is_disabled, _) = signal(false);

    let class_string = Memo::new(move |_| {
        let mut classes = vec!["base-class"];
        if is_active.get() {
            classes.push("active");
        }
        if is_disabled.get() {
            classes.push("disabled");
        }
        classes.join(" ")
    });

    let _view = view! {
        <div class=class_string>
            "Computed Class Test"
        </div>
    };

    assert_eq!(class_string.get(), "base-class active");
}

/// Test multiple conditional classes with different patterns
#[test]
fn test_multiple_conditional_classes() {
    let (step1_active, _) = signal(true);
    let (step2_completed, _) = signal(false);
    let memo_step3_active = Memo::new(move |_| step1_active.get() && !step2_completed.get());

    let _view = view! {
        <div class="multi-step"
             class:step1=step1_active
             class:step2=step2_completed
             class:step3=move || memo_step3_active.get()>
            "Multi Step Test"
        </div>
    };

    assert!(step1_active.get());
    assert!(!step2_completed.get());
    assert!(memo_step3_active.get());
}

/// Test that the problematic pattern (Memo<bool> without closure) is avoided
#[test]
fn test_avoid_problematic_pattern() {
    let (condition, _) = signal(true);
    let memo_condition = Memo::new(move |_| condition.get());

    // This demonstrates the correct pattern to avoid compilation issues
    // The closure pattern ensures compatibility across all Leptos 0.8.x versions

    let _view = view! {
        <div class:active=move || memo_condition.get()>
            "Correct Pattern"
        </div>
    };

    assert!(memo_condition.get());
}

/// Test form wizard component specifically
#[test]
fn test_form_wizard_component_compatibility() {
    // This test ensures the form wizard component works correctly
    // with the updated conditional class syntax

    let (current_step, _) = signal(1);
    let steps = vec![
        "Step 1".to_string(),
        "Step 2".to_string(),
        "Step 3".to_string(),
    ];

    // Simulate the form wizard step rendering
    let _wizard_steps = steps.iter().enumerate().map(|(step_index, step_name)| {
        let is_active = Memo::new(move |_| current_step.get() == step_index);
        let is_completed = Memo::new(move |_| current_step.get() > step_index);

        view! {
            <div class="wizard-step"
                 class:active=move || is_active.get()
                 class:completed=move || is_completed.get()>
                <div class="step-indicator">
                    {if is_completed.get() { "✓".to_string() } else { (step_index + 1).to_string() }}
                </div>
                <div class="step-label">
                    {step_name.clone()}
                </div>
            </div>
        }
    }).collect::<Vec<_>>();

    assert_eq!(current_step.get(), 1);
}
