Leptos Forms - Comprehensive Form Handling Library Design
Overview
A type-safe, reactive form handling library for Leptos that leverages Rust's type system to provide compile-time validation, zero-cost abstractions, and seamless integration with your existing component libraries.
Core Architecture
1. Form Trait System
rust// leptos-forms/src/core/traits.rs

use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core trait that all forms must implement
pub trait Form: Clone + Serialize + DeserializeOwned + 'static {
    /// Get field metadata for runtime introspection
    fn field_metadata() -> Vec<FieldMetadata>;
    
    /// Validate the entire form
    fn validate(&self) -> Result<(), ValidationErrors>;
    
    /// Get a field value by name (for dynamic access)
    fn get_field(&self, name: &str) -> Option<FieldValue>;
    
    /// Set a field value by name (for dynamic updates)
    fn set_field(&mut self, name: &str, value: FieldValue) -> Result<(), FieldError>;
    
    /// Get default values
    fn default_values() -> Self;
}

/// Metadata about a form field
#[derive(Debug, Clone)]
pub struct FieldMetadata {
    pub name: String,
    pub field_type: FieldType,
    pub validators: Vec<Validator>,
    pub is_required: bool,
    pub default_value: Option<FieldValue>,
    pub dependencies: Vec<String>, // Other fields this depends on
}

/// Supported field types
#[derive(Debug, Clone)]
pub enum FieldType {
    Text,
    Email,
    Password,
    Number(NumberType),
    Boolean,
    Select(Vec<SelectOption>),
    MultiSelect(Vec<SelectOption>),
    Date,
    DateTime,
    File(FileConstraints),
    Array(Box<FieldType>),
    Nested(String), // Type name for nested forms
}

/// Dynamic field value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldValue {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Date(chrono::NaiveDate),
    DateTime(chrono::DateTime<chrono::Utc>),
    Array(Vec<FieldValue>),
    Object(HashMap<String, FieldValue>),
    File(FileData),
    Null,
}
2. Derive Macro Design
rust// leptos-forms-macro/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Example usage:
/// ```rust
/// #[derive(Form, Debug, Clone, Serialize, Deserialize)]
/// #[form(on_submit = "handle_submit")]
/// struct LoginForm {
///     #[form(
///         validators(required, email),
///         placeholder = "Enter your email",
///         autocomplete = "email"
///     )]
///     email: String,
///     
///     #[form(
///         validators(required, min_length = 8, max_length = 128),
///         input_type = "password"
///     )]
///     password: String,
///     
///     #[form(
///         validators(custom = "validate_password_strength"),
///         depends_on = ["password"]
///     )]
///     password_strength: PasswordStrength,
///     
///     #[form(default = true)]
///     remember_me: bool,
///     
///     #[form(skip)] // Skip this field in form handling
///     metadata: Option<Metadata>,
/// }
/// ```

#[proc_macro_derive(Form, attributes(form))]
pub fn derive_form(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Generate implementation
    let name = &input.ident;
    let fields = extract_fields(&input);
    
    let field_metadata = generate_field_metadata(&fields);
    let validate_impl = generate_validate_impl(&fields);
    let getters_setters = generate_field_accessors(&fields);
    
    quote! {
        impl Form for #name {
            fn field_metadata() -> Vec<FieldMetadata> {
                vec![#field_metadata]
            }
            
            fn validate(&self) -> Result<(), ValidationErrors> {
                #validate_impl
            }
            
            #getters_setters
        }
    }
    .into()
}
3. Form Hook Implementation
rust// leptos-forms/src/hooks/use_form.rs

use leptos::*;
use std::rc::Rc;

/// Main form hook that manages all form state
pub struct FormHandle<T: Form> {
    // Core state
    pub values: ReadSignal<T>,
    pub errors: ReadSignal<ValidationErrors>,
    pub touched: ReadSignal<HashSet<String>>,
    pub dirty_fields: ReadSignal<HashSet<String>>,
    
    // Derived state
    pub is_valid: Signal<bool>,
    pub is_dirty: Signal<bool>,
    pub is_submitting: ReadSignal<bool>,
    pub submit_count: ReadSignal<u32>,
    
    // Actions
    pub set_value: WriteSignal<T>,
    pub set_field_value: Rc<dyn Fn(&str, FieldValue)>,
    pub set_field_error: Rc<dyn Fn(&str, String)>,
    pub clear_field_error: Rc<dyn Fn(&str)>,
    pub touch_field: Rc<dyn Fn(&str)>,
    pub reset: Rc<dyn Fn()>,
    pub reset_field: Rc<dyn Fn(&str)>,
    
    // Field registration
    pub register: Rc<dyn Fn(&str) -> FieldRegistration>,
    
    // Form submission
    pub handle_submit: Rc<dyn Fn(web_sys::Event)>,
    pub submit_form: Rc<dyn Fn()>,
}

/// Registration object returned for each field
#[derive(Clone)]
pub struct FieldRegistration {
    pub name: String,
    pub value: Signal<FieldValue>,
    pub error: Signal<Option<String>>,
    pub is_touched: Signal<bool>,
    pub is_dirty: Signal<bool>,
    
    // Event handlers
    pub on_change: Callback<Event>,
    pub on_blur: Callback<FocusEvent>,
    pub on_focus: Callback<FocusEvent>,
    
    // Props to spread
    pub props: FieldProps,
}

/// Props that can be spread onto input elements
#[derive(Clone)]
pub struct FieldProps {
    pub id: String,
    pub name: String,
    pub value: Signal<String>,
    pub on_input: Callback<Event>,
    pub on_change: Callback<Event>,
    pub on_blur: Callback<FocusEvent>,
    pub aria_invalid: Signal<bool>,
    pub aria_describedby: Signal<Option<String>>,
}

/// Create a form with all the reactive state management
#[component]
pub fn use_form<T: Form>(
    initial_values: Option<T>,
    options: FormOptions<T>,
) -> FormHandle<T> {
    let initial = initial_values.unwrap_or_else(T::default_values);
    
    // Core state
    let (values, set_values) = create_signal(initial.clone());
    let (errors, set_errors) = create_signal(ValidationErrors::new());
    let (touched, set_touched) = create_signal(HashSet::new());
    let (dirty_fields, set_dirty_fields) = create_signal(HashSet::new());
    let (is_submitting, set_submitting) = create_signal(false);
    let (submit_count, set_submit_count) = create_signal(0);
    
    // Validation trigger
    let validate_form = {
        let values = values.clone();
        move || -> Result<(), ValidationErrors> {
            let current_values = values.get();
            current_values.validate()
        }
    };
    
    // Validate on change if configured
    if options.mode == ValidationMode::OnChange {
        create_effect(move |_| {
            let values = values.get();
            if let Err(errors) = values.validate() {
                set_errors.set(errors);
            } else {
                set_errors.set(ValidationErrors::new());
            }
        });
    }
    
    // Field value setter with validation
    let set_field_value = {
        let set_values = set_values.clone();
        let set_dirty = set_dirty_fields.clone();
        
        Rc::new(move |field_name: &str, value: FieldValue| {
            set_values.update(|form| {
                if form.set_field(field_name, value).is_ok() {
                    set_dirty.update(|fields| {
                        fields.insert(field_name.to_string());
                    });
                    
                    // Trigger validation for this field if needed
                    if options.mode == ValidationMode::OnBlur 
                        || options.mode == ValidationMode::OnChange {
                        // Validate just this field
                        if let Some(field_validators) = get_field_validators(field_name) {
                            // Run validators
                        }
                    }
                }
            });
        })
    };
    
    // Field registration
    let register = {
        let values = values.clone();
        let errors = errors.clone();
        let touched = touched.clone();
        let dirty_fields = dirty_fields.clone();
        let set_field_value = set_field_value.clone();
        
        Rc::new(move |field_name: &str| -> FieldRegistration {
            let field_name_owned = field_name.to_string();
            
            // Create derived signals for this field
            let field_value = create_memo({
                let field_name = field_name_owned.clone();
                move |_| {
                    values.get().get_field(&field_name).unwrap_or(FieldValue::Null)
                }
            });
            
            let field_error = create_memo({
                let field_name = field_name_owned.clone();
                move |_| {
                    errors.get().field_errors.get(&field_name).cloned()
                }
            });
            
            let is_touched = create_memo({
                let field_name = field_name_owned.clone();
                move |_| touched.get().contains(&field_name)
            });
            
            let is_dirty = create_memo({
                let field_name = field_name_owned.clone();
                move |_| dirty_fields.get().contains(&field_name)
            });
            
            // Create event handlers
            let on_change = {
                let field_name = field_name_owned.clone();
                let set_field_value = set_field_value.clone();
                
                Callback::new(move |ev: Event| {
                    let target = ev.target().unwrap();
                    let value = extract_value_from_event(&target);
                    set_field_value(&field_name, value);
                })
            };
            
            let on_blur = {
                let field_name = field_name_owned.clone();
                let set_touched = set_touched.clone();
                
                Callback::new(move |_: FocusEvent| {
                    set_touched.update(|fields| {
                        fields.insert(field_name.clone());
                    });
                    
                    // Validate on blur if configured
                    if options.mode == ValidationMode::OnBlur {
                        // Trigger validation for this field
                    }
                })
            };
            
            FieldRegistration {
                name: field_name_owned.clone(),
                value: field_value.into(),
                error: field_error.into(),
                is_touched: is_touched.into(),
                is_dirty: is_dirty.into(),
                on_change,
                on_blur,
                on_focus: Callback::new(|_| {}),
                props: FieldProps {
                    id: format!("field-{}", field_name_owned),
                    name: field_name_owned,
                    value: field_value.into(),
                    // ... other props
                },
            }
        })
    };
    
    // Form submission
    let handle_submit = {
        let values = values.clone();
        let set_errors = set_errors.clone();
        let set_submitting = set_submitting.clone();
        let set_submit_count = set_submit_count.clone();
        let on_submit = options.on_submit.clone();
        
        Rc::new(move |ev: web_sys::Event| {
            ev.prevent_default();
            
            set_submit_count.update(|c| *c += 1);
            
            // Validate entire form
            let current_values = values.get();
            match current_values.validate() {
                Ok(()) => {
                    set_submitting.set(true);
                    
                    // Call async submit handler
                    spawn_local({
                        let values = current_values.clone();
                        let set_submitting = set_submitting.clone();
                        let set_errors = set_errors.clone();
                        
                        async move {
                            match on_submit(values).await {
                                Ok(()) => {
                                    // Success handling
                                }
                                Err(err) => {
                                    // Set server errors
                                    set_errors.set(err);
                                }
                            }
                            set_submitting.set(false);
                        }
                    });
                }
                Err(validation_errors) => {
                    set_errors.set(validation_errors);
                    
                    // Focus first error field
                    if let Some(first_error_field) = validation_errors.field_errors.keys().next() {
                        focus_field(first_error_field);
                    }
                }
            }
        })
    };
    
    FormHandle {
        values: values.into(),
        errors: errors.into(),
        touched: touched.into(),
        dirty_fields: dirty_fields.into(),
        is_valid: create_memo(move |_| errors.get().is_empty()).into(),
        is_dirty: create_memo(move |_| !dirty_fields.get().is_empty()).into(),
        is_submitting: is_submitting.into(),
        submit_count: submit_count.into(),
        set_value: set_values,
        set_field_value,
        // ... other methods
        register,
        handle_submit,
        // ... rest of the handle
    }
}
4. Validation System
rust// leptos-forms/src/validation/mod.rs

use std::collections::HashMap;

/// Validation errors container
#[derive(Debug, Clone, Default)]
pub struct ValidationErrors {
    pub field_errors: HashMap<String, String>,
    pub form_errors: Vec<String>,
}

/// Built-in validators
#[derive(Debug, Clone)]
pub enum Validator {
    Required,
    Email,
    Url,
    MinLength(usize),
    MaxLength(usize),
    Min(f64),
    Max(f64),
    Pattern(String),
    Custom(String), // Function name
    AsyncCustom(String), // Async function name
}

/// Validation rule builder for fluent API
pub struct ValidationRules {
    rules: Vec<Validator>,
}

impl ValidationRules {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
    
    pub fn required(mut self) -> Self {
        self.rules.push(Validator::Required);
        self
    }
    
    pub fn email(mut self) -> Self {
        self.rules.push(Validator::Email);
        self
    }
    
    pub fn min_length(mut self, min: usize) -> Self {
        self.rules.push(Validator::MinLength(min));
        self
    }
    
    pub fn pattern(mut self, regex: &str) -> Self {
        self.rules.push(Validator::Pattern(regex.to_string()));
        self
    }
    
    pub fn custom<F>(mut self, validator: F) -> Self 
    where
        F: Fn(&FieldValue) -> Result<(), String> + 'static
    {
        // Store custom validator
        self
    }
    
    pub fn async_custom<F, Fut>(mut self, validator: F) -> Self
    where
        F: Fn(FieldValue) -> Fut + 'static,
        Fut: Future<Output = Result<(), String>> + 'static,
    {
        // Store async validator
        self
    }
}

/// Schema validation using a builder pattern
pub struct FormSchema<T> {
    field_rules: HashMap<String, ValidationRules>,
    form_validators: Vec<Box<dyn Fn(&T) -> Result<(), String>>>,
}

impl<T: Form> FormSchema<T> {
    pub fn field(mut self, name: &str, rules: ValidationRules) -> Self {
        self.field_rules.insert(name.to_string(), rules);
        self
    }
    
    pub fn refine<F>(mut self, validator: F) -> Self
    where
        F: Fn(&T) -> Result<(), String> + 'static
    {
        self.form_validators.push(Box::new(validator));
        self
    }
    
    pub fn validate(&self, form: &T) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::default();
        
        // Validate individual fields
        for (field_name, rules) in &self.field_rules {
            if let Some(value) = form.get_field(field_name) {
                for rule in &rules.rules {
                    if let Err(error) = validate_field_value(&value, rule) {
                        errors.field_errors.insert(field_name.clone(), error);
                        break; // Stop at first error for this field
                    }
                }
            }
        }
        
        // Run form-level validators
        for validator in &self.form_validators {
            if let Err(error) = validator(form) {
                errors.form_errors.push(error);
            }
        }
        
        if errors.field_errors.is_empty() && errors.form_errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
5. Field Components Integration
rust// leptos-forms/src/components/field.rs

use leptos::*;

/// Generic field component that handles error display
#[component]
pub fn FormField(
    #[prop(into)] name: String,
    #[prop(into)] label: String,
    #[prop(optional)] description: Option<String>,
    #[prop(optional)] required: bool,
    children: Children,
) -> impl IntoView {
    let form = use_context::<FormContext>()
        .expect("FormField must be used within a Form");
    
    let field = form.register(&name);
    
    view! {
        <div class="form-field">
            <label for={&field.props.id} class="form-label">
                {label}
                {required.then(|| view! { <span class="required">"*"</span> })}
            </label>
            
            {description.map(|desc| view! {
                <p class="form-description">{desc}</p>
            })}
            
            // Render children with field props injected
            {children()}
            
            <Show when=move || field.error.get().is_some()>
                <p class="form-error" id={format!("{}-error", field.props.id)}>
                    {move || field.error.get()}
                </p>
            </Show>
        </div>
    }
}

/// Text input with form integration
#[component]
pub fn TextInput(
    #[prop(into)] name: String,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] autocomplete: Option<String>,
    #[prop(optional)] input_type: Option<String>,
) -> impl IntoView {
    let form = use_context::<FormContext>()
        .expect("TextInput must be used within a Form");
    
    let field = form.register(&name);
    
    view! {
        <input
            type={input_type.unwrap_or_else(|| "text".to_string())}
            id={&field.props.id}
            name={&field.props.name}
            value={move || field.value.get().to_string()}
            on:input={field.on_change}
            on:blur={field.on_blur}
            placeholder={placeholder}
            autocomplete={autocomplete}
            aria-invalid={move || field.error.get().is_some()}
            aria-describedby={move || {
                field.error.get().is_some()
                    .then(|| format!("{}-error", field.props.id))
            }}
            class="form-input"
            class:error={move || field.error.get().is_some()}
        />
    }
}

/// Integration with your radix-leptos components
#[component]
pub fn RadixTextField(
    #[prop(into)] name: String,
    #[prop(into)] label: String,
) -> impl IntoView {
    let form = use_context::<FormContext>().unwrap();
    let field = form.register(&name);
    
    view! {
        <TextField
            value={field.value}
            on_change={field.on_change}
            error={field.error}
            label={label}
        />
    }
}
6. Advanced Features
Array Fields
rust// leptos-forms/src/fields/array.rs

#[component]
pub fn FieldArray<T: Clone + Default + 'static>(
    #[prop(into)] name: String,
    render_item: Rc<dyn Fn(usize, T, ArrayHelpers<T>) -> View>,
) -> impl IntoView {
    let form = use_context::<FormContext>().unwrap();
    let (items, set_items) = create_signal(Vec::<T>::new());
    
    let helpers = ArrayHelpers {
        append: Rc::new(move |item| {
            set_items.update(|items| items.push(item));
        }),
        prepend: Rc::new(move |item| {
            set_items.update(|items| items.insert(0, item));
        }),
        remove: Rc::new(move |index| {
            set_items.update(|items| {
                if index < items.len() {
                    items.remove(index);
                }
            });
        }),
        move_item: Rc::new(move |from, to| {
            set_items.update(|items| {
                if from < items.len() && to < items.len() {
                    let item = items.remove(from);
                    items.insert(to, item);
                }
            });
        }),
        swap: Rc::new(move |index_a, index_b| {
            set_items.update(|items| items.swap(index_a, index_b));
        }),
    };
    
    view! {
        <div class="field-array">
            <For
                each=move || items.get().into_iter().enumerate()
                key=|(index, _)| *index
                children=move |(index, item)| {
                    render_item(index, item, helpers.clone())
                }
            />
        </div>
    }
}
File Upload
rust// leptos-forms/src/fields/file.rs

#[derive(Clone)]
pub struct FileConstraints {
    pub max_size: Option<usize>,
    pub accept: Option<Vec<String>>,
    pub multiple: bool,
}

#[component]
pub fn FileInput(
    #[prop(into)] name: String,
    #[prop(optional)] constraints: Option<FileConstraints>,
    #[prop(optional)] on_progress: Option<Callback<f32>>,
) -> impl IntoView {
    let form = use_context::<FormContext>().unwrap();
    let (uploading, set_uploading) = create_signal(false);
    let (progress, set_progress) = create_signal(0.0);
    
    let handle_change = move |ev: Event| {
        let input: HtmlInputElement = event_target(&ev);
        if let Some(files) = input.files() {
            set_uploading.set(true);
            
            spawn_local(async move {
                for i in 0..files.length() {
                    if let Some(file) = files.item(i) {
                        // Validate file constraints
                        if let Some(ref constraints) = constraints {
                            if let Some(max_size) = constraints.max_size {
                                if file.size() as usize > max_size {
                                    form.set_field_error(
                                        &name,
                                        format!("File exceeds maximum size of {} bytes", max_size)
                                    );
                                    continue;
                                }
                            }
                        }
                        
                        // Upload file
                        let form_data = web_sys::FormData::new().unwrap();
                        form_data.append_with_blob("file", &file).unwrap();
                        
                        // Make upload request with progress tracking
                        let response = upload_with_progress(
                            form_data,
                            move |percent| {
                                set_progress.set(percent);
                                if let Some(callback) = &on_progress {
                                    callback.call(percent);
                                }
                            }
                        ).await;
                        
                        // Store file reference in form
                        if let Ok(file_data) = response {
                            form.set_field_value(&name, FieldValue::File(file_data));
                        }
                    }
                }
                set_uploading.set(false);
            });
        }
    };
    
    view! {
        <div class="file-input-container">
            <input
                type="file"
                accept={constraints.as_ref().and_then(|c| {
                    c.accept.as_ref().map(|a| a.join(","))
                })}
                multiple={constraints.as_ref().map(|c| c.multiple).unwrap_or(false)}
                on:change={handle_change}
                disabled={move || uploading.get()}
            />
            
            <Show when=move || uploading.get()>
                <div class="upload-progress">
                    <div class="progress-bar" style={move || {
                        format!("width: {}%", progress.get() * 100.0)
                    }} />
                </div>
            </Show>
        </div>
    }
}
Conditional Fields
rust// leptos-forms/src/fields/conditional.rs

#[component]
pub fn ConditionalField<T: Form>(
    #[prop(into)] when: Signal<bool>,
    #[prop(optional)] unmount_on_hide: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <Show
            when=move || when.get()
            fallback=|| {
                if unmount_on_hide {
                    view! {}.into_view()
                } else {
                    view! { <div style="display: none">{children()}</div> }.into_view()
                }
            }
        >
            {children()}
        </Show>
    }
}

// Usage example
#[component]
fn ConditionalFormExample() -> impl IntoView {
    let form = use_form::<RegistrationForm>(None, Default::default());
    
    view! {
        <FormField name="account_type" label="Account Type">
            <Select {...form.register("account_type")}>
                <option value="personal">"Personal"</option>
                <option value="business">"Business"</option>
            </Select>
        </FormField>
        
        <ConditionalField when={move || {
            form.values.get().account_type == "business"
        }}>
            <FormField name="company_name" label="Company Name">
                <TextInput name="company_name" />
            </FormField>
            
            <FormField name="tax_id" label="Tax ID">
                <TextInput name="tax_id" />
            </FormField>
        </ConditionalField>
    }
}
7. Form Wizard/Multi-Step Forms
rust// leptos-forms/src/wizard/mod.rs

#[derive(Clone)]
pub struct WizardStep<T> {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub fields: Vec<String>,
    pub validate: Option<Rc<dyn Fn(&T) -> Result<(), ValidationErrors>>>,
}

#[component]
pub fn FormWizard<T: Form>(
    steps: Vec<WizardStep<T>>,
    #[prop(optional)] on_complete: Option<Callback<T>>,
) -> impl IntoView {
    let (current_step, set_current_step) = create_signal(0);
    let (completed_steps, set_completed_steps) = create_signal(HashSet::new());
    let form = use_form::<T>(None, Default::default());
    
    let can_proceed = create_memo(move |_| {
        let step = &steps[current_step.get()];
        // Validate only fields in current step
        step.fields.iter().all(|field| {
            form.errors.get().field_errors.get(field).is_none()
        })
    });
    
    let next_step = move || {
        if can_proceed.get() {
            set_completed_steps.update(|steps| {
                steps.insert(current_step.get());
            });
            set_current_step.update(|step| *step = (*step + 1).min(steps.len() - 1));
        }
    };
    
    let prev_step = move || {
        set_current_step.update(|step| *step = step.saturating_sub(1));
    };
    
    view! {
        <div class="form-wizard">
            // Step indicator
            <div class="wizard-steps">
                <For
                    each=move || steps.iter().enumerate()
                    key=|(index, step)| step.id.clone()
                    children=move |(index, step)| {
                        let is_active = move || current_step.get() == index;
                        let is_completed = move || completed_steps.get().contains(&index);
                        
                        view! {
                            <div
                                class="wizard-step"
                                class:active={is_active}
                                class:completed={is_completed}
                            >
                                <div class="step-number">{index + 1}</div>
                                <div class="step-title">{&step.title}</div>
                            </div>
                        }
                    }
                />
            </div>
            
            // Current step content
            <div class="wizard-content">
                {move || {
                    let step = &steps[current_step.get()];
                    view! {
                        <div class="step-content">
                            <h2>{&step.title}</h2>
                            {step.description.as_ref().map(|desc| {
                                view! { <p>{desc}</p> }
                            })}
                            
                            // Render only fields for current step
                            <For
                                each=move || step.fields.clone()
                                key=|field| field.clone()
                                children=move |field| {
                                    // Render appropriate field component
                                    view! {
                                        <FormField name={&field} label={&field}>
                                            <TextInput name={&field} />
                                        </FormField>
                                    }
                                }
                            />
                        </div>
                    }
                }}
            </div>
            
            // Navigation
            <div class="wizard-navigation">
                <Button
                    on:click=move |_| prev_step()
                    disabled=move || current_step.get() == 0
                >
                    "Previous"
                </Button>
                
                <Show
                    when=move || current_step.get() < steps.len() - 1
                    fallback=move || {
                        view! {
                            <Button
                                on:click=move |_| {
                                    if let Some(callback) = &on_complete {
                                        callback.call(form.values.get());
                                    }
                                }
                                disabled=move || !can_proceed.get()
                            >
                                "Complete"
                            </Button>
                        }
                    }
                >
                    <Button
                        on:click=move |_| next_step()
                        disabled=move || !can_proceed.get()
                    >
                        "Next"
                    </Button>
                </Show>
            </div>
        </div>
    }
}
8. Testing Utilities
rust// leptos-forms/src/testing/mod.rs

/// Testing utilities for forms
pub mod test_utils {
    use super::*;
    
    /// Create a test form context
    pub fn create_test_form<T: Form>(initial: T) -> TestFormContext<T> {
        TestFormContext {
            form: use_form(Some(initial), Default::default()),
            submitted_values: Rc::new(RefCell::new(None)),
        }
    }
    
    /// Helper to simulate form field changes
    pub fn simulate_change<T: Form>(
        form: &FormHandle<T>,
        field: &str,
        value: impl Into<FieldValue>,
    ) {
        form.set_field_value(field, value.into());
    }
    
    /// Helper to simulate form submission
    pub async fn simulate_submit<T: Form>(form: &FormHandle<T>) -> Result<T, ValidationErrors> {
        let values = form.values.get();
        match values.validate() {
            Ok(()) => Ok(values),
            Err(errors) => Err(errors),
        }
    }
    
    /// Assert that a field has an error
    pub fn assert_field_error(form: &FormHandle<impl Form>, field: &str) {
        assert!(
            form.errors.get().field_errors.contains_key(field),
            "Expected field '{}' to have an error",
            field
        );
    }
    
    /// Assert that form is valid
    pub fn assert_form_valid(form: &FormHandle<impl Form>) {
        assert!(
            form.is_valid.get(),
            "Expected form to be valid, but found errors: {:?}",
            form.errors.get()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_form_validation() {
        // Test your form validation
        let form = create_test_form(LoginForm {
            email: "invalid".to_string(),
            password: "123".to_string(),
        });
        
        simulate_submit(&form).await;
        
        assert_field_error(&form, "email");
        assert_field_error(&form, "password");
    }
}
Usage Examples
Basic Form
rustuse leptos_forms::*;

#[derive(Form, Clone, Debug, Serialize, Deserialize)]
struct ContactForm {
    #[form(validators(required, min_length = 2))]
    name: String,
    
    #[form(validators(required, email))]
    email: String,
    
    #[form(validators(required, min_length = 10))]
    message: String,
}

#[component]
fn ContactFormComponent() -> impl IntoView {
    let form = use_form::<ContactForm>(
        None,
        FormOptions {
            mode: ValidationMode::OnBlur,
            on_submit: |data| async move {
                // Send to API
                api::send_contact(data).await
            },
        },
    );
    
    view! {
        <form on:submit=form.handle_submit>
            <FormField name="name" label="Your Name" required=true>
                <TextInput name="name" placeholder="John Doe" />
            </FormField>
            
            <FormField name="email" label="Email Address" required=true>
                <TextInput name="email" type="email" placeholder="john@example.com" />
            </FormField>
            
            <FormField name="message" label="Message" required=true>
                <TextArea name="message" rows=5 />
            </FormField>
            
            <Button 
                type="submit"
                disabled=move || form.is_submitting.get() || !form.is_valid.get()
            >
                {move || if form.is_submitting.get() { "Sending..." } else { "Send Message" }}
            </Button>
        </form>
    }
}
Complex Form with All Features
rust#[derive(Form, Clone, Debug, Serialize, Deserialize)]
struct ApplicationForm {
    // Personal Information
    #[form(validators(required))]
    first_name: String,
    
    #[form(validators(required))]
    last_name: String,
    
    #[form(validators(required, email))]
    email: String,
    
    #[form(validators(required, phone))]
    phone: String,
    
    // File Upload
    #[form(validators(required))]
    resume: FileData,
    
    #[form(validators(max_size = 5_000_000))]
    portfolio: Option<FileData>,
    
    // Dynamic Fields
    #[form(validators(min_items = 1, max_items = 5))]
    references: Vec<Reference>,
    
    // Conditional Fields
    #[form(validators(required_if = "has_experience"))]
    years_experience: Option<u32>,
    
    #[form(validators(required_if = "has_experience"))]
    previous_companies: Option<Vec<String>>,
}

#[component]
fn ApplicationFormComponent() -> impl IntoView {
    let form = use_form::<ApplicationForm>(None, Default::default());
    
    view! {
        <FormWizard
            steps=vec![
                WizardStep {
                    id: "personal".to_string(),
                    title: "Personal Information".to_string(),
                    fields: vec!["first_name", "last_name", "email", "phone"],
                    validate: None,
                },
                WizardStep {
                    id: "documents".to_string(),
                    title: "Documents".to_string(),
                    fields: vec!["resume", "portfolio"],
                    validate: None,
                },
                WizardStep {
                    id: "references".to_string(),
                    title: "References".to_string(),
                    fields: vec!["references"],
                    validate: None,
                },
                WizardStep {
                    id: "experience".to_string(),
                    title: "Experience".to_string(),
                    fields: vec!["years_experience", "previous_companies"],
                    validate: None,
                },
            ]
            on_complete=move |data| {
                // Submit application
                spawn_local(async move {
                    api::submit_application(data).await;
                });
            }
        />
    }
}
Integration with Your Existing Libraries
rust// Integration with radix-leptos and shadcn-ui
use radix_leptos::*;
use shadcn_ui::*;

#[component]
fn StyledForm() -> impl IntoView {
    let form = use_form::<MyForm>(None, Default::default());
    
    view! {
        <Card>
            <CardHeader>
                <CardTitle>"User Registration"</CardTitle>
            </CardHeader>
            <CardContent>
                <form on:submit=form.handle_submit>
                    // Use your shadcn components with form integration
                    <FormField name="email">
                        <Label>"Email"</Label>
                        <Input {...form.register("email")} />
                        <FormMessage />
                    </FormField>
                    
                    <FormField name="country">
                        <Label>"Country"</Label>
                        <Select {...form.register("country")}>
                            <SelectTrigger>
                                <SelectValue placeholder="Select a country" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="us">"United States"</SelectItem>
                                <SelectItem value="uk">"United Kingdom"</SelectItem>
                            </SelectContent>
                        </Select>
                    </FormField>
                    
                    <Button type="submit" class="w-full">
                        "Register"
                    </Button>
                </form>
            </CardContent>
        </Card>
    }
}
This comprehensive form library design provides:

Type safety throughout
Zero-cost abstractions using Rust's ownership system
Compile-time validation where possible
Rich runtime validation with custom validators
Seamless integration with your existing UI components
Advanced features like file uploads, multi-step forms, and conditional fields
Testing utilities for reliable form testing
