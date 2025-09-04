use leptos::prelude::{
    Callback, Memo, ReadSignal, 
    signal, Get, Set, Update, Callable
};
use leptos::task::spawn_local;
use serde::{Serialize, Deserialize};
use crate::core::*;
use crate::validation::ValidationErrors;

/// Main hook for form management
pub fn use_form<T: Form + PartialEq + Clone + Send + Sync>() -> FormHandle<T> {
    FormHandle::new()
}

/// Hook for form with initial values
pub fn use_form_with_values<T: Form + PartialEq + Clone + Send + Sync>(values: T) -> FormHandle<T> {
    FormHandle::with_values(values)
}

/// Hook for form field value
pub fn use_field_value<T: Form + PartialEq + Clone + Send + Sync>(form: &mut FormHandle<T>, field_name: &str) -> ReadSignal<Option<FieldValue>> {
    let field_signal = form.get_field_signal(field_name);
    if let Some(signal) = field_signal {
        signal.value
    } else {
        signal(None).0
    }
}

/// Hook for form field error
pub fn use_field_error<T: Form + PartialEq + Clone + Send + Sync>(form: &mut FormHandle<T>, field_name: &str) -> ReadSignal<Option<String>> {
    let field_signal = form.get_field_signal(field_name);
    if let Some(signal) = field_signal {
        signal.error
    } else {
        signal(None).0
    }
}

/// Hook for form field dirty state
pub fn use_field_dirty<T: Form + PartialEq + Clone + Send + Sync>(form: &mut FormHandle<T>, field_name: &str) -> ReadSignal<bool> {
    let field_signal = form.get_field_signal(field_name);
    if let Some(signal) = field_signal {
        signal.is_dirty
    } else {
        signal(false).0
    }
}

/// Hook for form field touched state
pub fn use_field_touched<T: Form + PartialEq + Clone + Send + Sync>(form: &mut FormHandle<T>, field_name: &str) -> ReadSignal<bool> {
    let field_signal = form.get_field_signal(field_name);
    if let Some(signal) = field_signal {
        signal.is_touched
    } else {
        signal(false).0
    }
}

/// Hook for form validation
pub fn use_form_validation<T: Form + PartialEq + Send + Sync>(form: &FormHandle<T>) -> (
    Memo<ValidationErrors>,
    Memo<bool>,
    Callback<(), ()>
) {
    let errors = form.get_errors();
    let is_valid = form.is_valid();
    let form_clone = form.clone();
    let validate = Callback::new(move |_| {
        let _ = form_clone.validate_form();
    });
    
    (errors, is_valid, validate)
}

/// Hook for form submission
pub fn use_form_submission<T: Form + PartialEq + Send + Sync, F>(form: &FormHandle<T>, handler: F) -> (
    Memo<bool>,
    Callback<(), ()>
) 
where
    F: Fn(&T) -> Result<(), FormError> + 'static + Clone + Send + Sync,
{
    let is_submitting = form.is_submitting();
    let form_clone = form.clone();
    
    let submit = Callback::new(move |_| {
        let form_handle = form_clone.clone();
        let handler_clone = handler.clone();
        
        spawn_local(async move {
            // Get the current form values
            let form_data = form_handle.get_values();
            
            // Call the handler with the form data
            let result = handler_clone(&form_data.get());
            
            // Handle the result
            match result {
                Ok(_) => {
                    // Success - could emit a success signal here
                }
                Err(error) => {
                    // Error - could emit an error signal here
                    log::error!("Form submission error: {:?}", error);
                }
            }
        });
    });
    
    (is_submitting, submit)
}

/// Hook for form persistence
pub fn use_form_persistence<T: Form + PartialEq + Send + Sync>(form: &FormHandle<T>, storage_key: Option<String>) -> (
    Callback<(), ()>,
    Callback<(), ()>,
    Callback<(), ()>
) {
    let form_clone1 = form.clone();
    let form_clone2 = form.clone();
    let storage_key = storage_key.unwrap_or_else(|| format!("form-{}", std::any::type_name::<T>()));
    let key1 = storage_key.clone();
    let key2 = storage_key.clone();
    let key3 = storage_key;
    
    let save = Callback::new(move |_| {
        let form_handle = form_clone1.clone();
        let key = key1.clone();
        
        spawn_local(async move {
            // Get current form values
            let form_data = form_handle.get_values();
            
            // Serialize to JSON
            if let Ok(_json) = serde_json::to_string(&form_data) {
                // Save to localStorage (in a real implementation, this would use web_sys)
                log::info!("Saving form data for key: {}", key);
                // For now, just log - in a real implementation, you'd use web_sys::Storage
            }
        });
    });
    
    let load = Callback::new(move |_| {
        let _form_handle = form_clone2.clone();
        let key = key2.clone();
        
        spawn_local(async move {
            log::info!("Loading form data for key: {}", key);
            // In a real implementation, you'd load from web_sys::Storage
            // and update the form with the loaded values
        });
    });
    
    let clear = Callback::new(move |_| {
        let key = key3.clone();
        
        spawn_local(async move {
            log::info!("Clearing form data for key: {}", key);
            // In a real implementation, you'd clear from web_sys::Storage
        });
    });
    
    (save, load, clear)
}

/// Hook for form analytics
pub fn use_form_analytics<T: Form + PartialEq + Send + Sync>(_form: &FormHandle<T>) -> FormAnalyticsHandle {
    FormAnalyticsHandle::new()
}

/// Handle for form analytics
pub struct FormAnalyticsHandle {
    track_view: Callback<String, ()>,
    track_field_interaction: Callback<(String, String, String), ()>,
    track_submission: Callback<(String, bool), ()>,
    track_validation_errors: Callback<(String, ValidationErrors), ()>,
}

impl FormAnalyticsHandle {
    pub fn new() -> Self {
        Self {
            track_view: Callback::new(|_| {}),
            track_field_interaction: Callback::new(|_| {}),
            track_submission: Callback::new(|_| {}),
            track_validation_errors: Callback::new(|_| {}),
        }
    }
    
    pub fn track_view(&self, form_name: String) {
        self.track_view.run(form_name);
    }
    
    pub fn track_field_interaction(&self, form_name: String, field_name: String, action: String) {
        self.track_field_interaction.run((form_name, field_name, action));
    }
    
    pub fn track_submission(&self, form_name: String, success: bool) {
        self.track_submission.run((form_name, success));
    }
    
    pub fn track_validation_errors(&self, form_name: String, errors: ValidationErrors) {
        self.track_validation_errors.run((form_name, errors));
    }
}

/// Hook for form field array management
pub fn use_field_array<T: Form + PartialEq + Send + Sync, U>(form: &FormHandle<T>, field_name: &str) -> FieldArrayHandle<U> 
where
    U: Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    FieldArrayHandle::new(form, field_name)
}

/// Handle for field array operations
#[derive(Clone)]
pub struct FieldArrayHandle<T: 'static> {
    items: ReadSignal<Vec<T>>,
    add: Callback<T, ()>,
    remove: Callback<usize, ()>,
    move_item: Callback<(usize, usize), ()>,
    clear: Callback<(), ()>,
}

impl<T: Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static> FieldArrayHandle<T> {
    pub fn new<U: Form>(_form: &FormHandle<U>, _field_name: &str) -> Self {
        let (items, set_items) = signal(Vec::<T>::new());
        
        let add = Callback::new(move |item: T| {
            set_items.update(|items| items.push(item));
        });
        
        let remove = Callback::new(move |index: usize| {
            set_items.update(|items| {
                if index < items.len() {
                    items.remove(index);
                }
            });
        });
        
        let move_item = Callback::new(move |(from, to): (usize, usize)| {
            set_items.update(|items| {
                if from < items.len() && to < items.len() && from != to {
                    let item = items.remove(from);
                    items.insert(to, item);
                }
            });
        });
        
        let clear = Callback::new(move |_| {
            set_items.set(Vec::new());
        });
        
        Self {
            items,
            add,
            remove,
            move_item,
            clear,
        }
    }
    
    pub fn items(&self) -> ReadSignal<Vec<T>> {
        self.items
    }
    
    pub fn add(&self, item: T) {
        self.add.run(item);
    }
    
    pub fn remove(&self, index: usize) {
        self.remove.run(index);
    }
    
    pub fn move_item(&self, from: usize, to: usize) {
        self.move_item.run((from, to));
    }
    
    pub fn clear(&self) {
        self.clear.run(());
    }
    
    pub fn len(&self) -> Memo<usize> {
        let items_signal = self.items;
        Memo::new(move |_| items_signal.get().len())
    }
    
    pub fn is_empty(&self) -> Memo<bool> {
        let items_signal = self.items;
        Memo::new(move |_| items_signal.get().is_empty())
    }
}

/// Hook for form wizard/multi-step forms
pub fn use_form_wizard<T: Form + PartialEq + Send + Sync>(_form: &FormHandle<T>, steps: Vec<String>) -> FormWizardHandle {
    FormWizardHandle::new(steps)
}

/// Handle for form wizard operations
pub struct FormWizardHandle {
    current_step: ReadSignal<usize>,
    steps: Vec<String>,
    next: Callback<(), ()>,
    previous: Callback<(), ()>,
    go_to_step: Callback<usize, ()>,
    is_first_step: Memo<bool>,
    is_last_step: Memo<bool>,
}

impl FormWizardHandle {
    pub fn new(steps: Vec<String>) -> Self {
        let (current_step, set_current_step) = signal(0);
        let steps_clone1 = steps.clone();
        let steps_clone2 = steps.clone();
        let steps_clone3 = steps.clone();
        
        let next = Callback::new(move |_| {
            set_current_step.update(|step| {
                if *step < steps_clone1.len() - 1 {
                    *step += 1;
                }
            });
        });
        
        let previous = Callback::new(move |_| {
            set_current_step.update(|step| {
                if *step > 0 {
                    *step -= 1;
                }
            });
        });
        
        let go_to_step = Callback::new(move |target_step: usize| {
            set_current_step.set(target_step.min(steps_clone2.len() - 1));
        });
        
        let current_step_signal = current_step;
        let is_first_step = Memo::new(move |_| current_step_signal.get() == 0);
        let is_last_step = Memo::new(move |_| current_step_signal.get() == steps_clone3.len() - 1);
        
        Self {
            current_step,
            steps,
            next,
            previous,
            go_to_step,
            is_first_step,
            is_last_step,
        }
    }
    
    pub fn current_step(&self) -> ReadSignal<usize> {
        self.current_step
    }
    
    pub fn current_step_name(&self) -> Memo<String> {
        let current_step_signal = self.current_step;
        let steps_clone = self.steps.clone();
        Memo::new(move |_| {
            let step = current_step_signal.get();
            if step < steps_clone.len() {
                steps_clone[step].clone()
            } else {
                String::new()
            }
        })
    }
    
    pub fn steps(&self) -> &[String] {
        &self.steps
    }
    
    pub fn next(&self) {
        self.next.run(());
    }
    
    pub fn previous(&self) {
        self.previous.run(());
    }
    
    pub fn go_to_step(&self, step: usize) {
        self.go_to_step.run(step);
    }
    
    pub fn is_first_step(&self) -> Memo<bool> {
        self.is_first_step
    }
    
    pub fn is_last_step(&self) -> Memo<bool> {
        self.is_last_step
    }
    
    pub fn progress(&self) -> Memo<f64> {
        let current_step_signal = self.current_step;
        let total_steps = self.steps.len();
        Memo::new(move |_| {
            let step = current_step_signal.get();
            if total_steps > 0 {
                (step as f64) / (total_steps as f64)
            } else {
                0.0
            }
        })
    }
}
