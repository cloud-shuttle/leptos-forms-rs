use crate::core::traits::Form;
use crate::core::types::FieldValue;
use crate::core::FormHandle;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Hook for managing form state
pub fn use_form<T: Form + PartialEq + Clone + Send + Sync>(
    initial_values: T,
) -> (FormHandle<T>, Callback<()>, Callback<()>) {
    let form_handle = FormHandle::new(initial_values);

    let form_clone1 = form_handle.clone();
    let submit = Callback::new(move |_| {
        let form_clone = form_clone1.clone();
        spawn_local(async move {
            if let Err(error) = form_clone.submit() {
                log::error!("Form submission failed: {:?}", error);
            }
        });
    });

    let form_clone2 = form_handle.clone();
    let reset = Callback::new(move |_| {
        let form_clone = form_clone2.clone();
        form_clone.reset();
    });

    (form_handle, submit, reset)
}

/// Hook for getting a field value
pub fn use_field_value<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
    field_name: &str,
) -> Memo<FieldValue> {
    let form_handle = form_handle.clone();
    let field_name = field_name.to_string();
    Memo::new(move |_| {
        form_handle
            .get_field_value(&field_name)
            .unwrap_or(FieldValue::String(String::new()))
    })
}

/// Hook for getting field errors
pub fn use_field_error<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
    field_name: &str,
) -> Memo<Vec<String>> {
    let form_handle = form_handle.clone();
    let field_name = field_name.to_string();
    Memo::new(move |_| {
        form_handle
            .errors()
            .get()
            .get_field_error(&field_name)
            .cloned()
            .unwrap_or_default()
    })
}

/// Hook for checking if a field is dirty
pub fn use_field_dirty<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
) -> Memo<bool> {
    form_handle.is_dirty()
}

/// Hook for checking if a field has been touched
pub fn use_field_touched<T: Form + PartialEq + Clone + Send + Sync>(
    _form_handle: &FormHandle<T>,
) -> Memo<bool> {
    // For now, return false - this would need to be tracked in FormState
    Memo::new(move |_| false)
}

/// Hook for form validation
pub fn use_form_validation<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
) -> (Memo<bool>, Callback<()>) {
    let is_valid = form_handle.is_valid();

    let form_clone = form_handle.clone();
    let validate = Callback::new(move |_| {
        let form_clone = form_clone.clone();
        spawn_local(async move {
            if let Err(error) = form_clone.validate() {
                log::error!("Form validation failed: {:?}", error);
            }
        });
    });

    (is_valid, validate)
}

/// Hook for form submission
pub fn use_form_submission<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
) -> (Memo<bool>, Callback<()>) {
    let is_submitting = form_handle.is_submitting();

    let form_clone = form_handle.clone();
    let submit = Callback::new(move |_| {
        let form_clone = form_clone.clone();
        spawn_local(async move {
            if let Err(error) = form_clone.submit() {
                log::error!("Form submission failed: {:?}", error);
            }
        });
    });

    (is_submitting, submit)
}

/// Hook for form persistence
pub fn use_form_persistence<T: Form + PartialEq + Clone + Send + Sync>(
    _form_handle: &FormHandle<T>,
    storage_key: &str,
) -> (Callback<()>, Callback<()>, Callback<()>) {
    let storage_key = storage_key.to_string();
    let storage_key1 = storage_key.clone();
    let storage_key2 = storage_key.clone();
    let storage_key3 = storage_key.clone();

    let save = Callback::new(move |_| {
        let storage_key = storage_key1.clone();
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(_storage)) = window.local_storage() {
                    log::info!("Saving form data to storage with key: {}", storage_key);
                }
            }
        });
    });

    let load = Callback::new(move |_| {
        let storage_key = storage_key2.clone();
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(_storage)) = window.local_storage() {
                    log::info!("Loading form data from storage with key: {}", storage_key);
                }
            }
        });
    });

    let clear = Callback::new(move |_| {
        let storage_key = storage_key3.clone();
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(_storage)) = window.local_storage() {
                    log::info!("Clearing form data from storage with key: {}", storage_key);
                }
            }
        });
    });

    (save, load, clear)
}

/// Hook for form analytics
pub fn use_form_analytics<T: Form + PartialEq + Clone + Send + Sync>(
    _form_handle: &FormHandle<T>,
) -> Callback<&'static str> {
    Callback::new(move |event: &'static str| {
        log::info!("Form analytics event: {}", event);
    })
}

/// Hook for managing field arrays
pub fn use_field_array<
    T: Form + PartialEq + Clone + Send + Sync,
    U: std::fmt::Debug + Clone + Send + Sync + 'static,
>(
    form_handle: &FormHandle<T>,
    field_name: &str,
) -> FieldArrayHandle<U> {
    let form_clone = form_handle.clone();
    let field_name = field_name.to_string();
    let field_name_clone1 = field_name.clone();
    let add_item = Callback::new(move |value: U| {
        let form_clone = form_clone.clone();
        let field_name = field_name_clone1.clone();
        spawn_local(async move {
            let field_value = FieldValue::String(format!("{:?}", value));
            form_clone.add_array_item(&field_name, field_value);
        });
    });

    let form_clone2 = form_handle.clone();
    let field_name_clone2 = field_name.clone();
    let remove_item = Callback::new(move |index: usize| {
        let form_clone = form_clone2.clone();
        let field_name = field_name_clone2.clone();
        spawn_local(async move {
            form_clone.remove_array_item(&field_name, index);
        });
    });

    let form_clone3 = form_handle.clone();
    let field_name3 = field_name.clone();
    let move_item = Callback::new(move |(from_index, to_index): (usize, usize)| {
        let form_clone = form_clone3.clone();
        let field_name = field_name3.clone();
        spawn_local(async move {
            form_clone.move_array_item(&field_name, from_index, to_index);
        });
    });

    let form_clone4 = form_handle.clone();
    let field_name4 = field_name.clone();
    let clear_array = Callback::new(move |_| {
        let form_clone = form_clone4.clone();
        let field_name = field_name4.clone();
        spawn_local(async move {
            form_clone.clear_array(&field_name);
        });
    });

    FieldArrayHandle {
        add_item,
        remove_item,
        move_item,
        clear_array,
    }
}

/// Handle for field array operations
#[derive(Clone)]
pub struct FieldArrayHandle<U: 'static> {
    pub add_item: Callback<U>,
    pub remove_item: Callback<usize>,
    pub move_item: Callback<(usize, usize)>,
    pub clear_array: Callback<()>,
}

/// Hook for form wizard functionality
pub fn use_form_wizard<T: Form + PartialEq + Clone + Send + Sync>(
    steps: Vec<String>,
) -> (
    ReadSignal<usize>,
    Callback<()>,
    Callback<()>,
    Callback<usize>,
    Callback<()>,
) {
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

/// Hook for real-time validation
pub fn use_real_time_validation<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
    field_name: &str,
    delay_ms: u32,
) -> (ReadSignal<Option<String>>, Callback<FieldValue>) {
    let validation_error = RwSignal::new(None::<String>);

    let form_clone = form_handle.clone();
    let field_name = field_name.to_string();
    let set_error = validation_error.clone();
    let validate_field = Callback::new(move |_value: FieldValue| {
        let form_clone = form_clone.clone();
        let field_name = field_name.clone();
        let set_error = set_error.clone();

        spawn_local(async move {
            // Simulate validation delay
            gloo_timers::callback::Timeout::new(delay_ms, move || {
                if let Err(errors) = form_clone.validate_field(&field_name) {
                    if let Some(error) = errors.get_field_error(&field_name).and_then(|v| v.first())
                    {
                        set_error.set(Some(error.clone()));
                    }
                } else {
                    set_error.set(None);
                }
            })
            .forget();
        });
    });

    (validation_error.read_only(), validate_field)
}

/// Hook for conditional validation
pub fn use_conditional_validation<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
    field_name: &str,
    condition: impl Fn(&T) -> bool + Send + Sync + 'static,
) -> Memo<bool> {
    let form_handle = form_handle.clone();
    let _field_name = field_name.to_string();
    Memo::new(move |_| {
        let values = form_handle.values().get();
        condition(&values)
    })
}

/// Hook for form performance monitoring
pub fn use_form_performance<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
) -> (
    ReadSignal<crate::core::performance::FormPerformanceMetrics>,
    Callback<()>,
) {
    let metrics = RwSignal::new(crate::core::performance::FormPerformanceMetrics::new());

    let form_clone = form_handle.clone();
    let metrics_clone = metrics.clone();

    // Benchmark callback for measuring performance
    let benchmark = Callback::new(move |_| {
        let form_clone = form_clone.clone();
        let metrics_clone = metrics_clone.clone();

        spawn_local(async move {
            let start = std::time::Instant::now();

            // Measure form creation time
            let _form = FormHandle::new(form_clone.values().get());
            let creation_time = start.elapsed();

            // Update metrics
            metrics_clone.update(|m| {
                m.record_form_creation(creation_time);
            });
        });
    });

    (metrics.read_only(), benchmark)
}
