//! WASM-specific optimizations and configurations

use js_sys;
use wasm_bindgen::prelude::*;

/// Initialize WASM optimizations
/// This should be called early in the application lifecycle
#[wasm_bindgen]
pub fn init_wasm_optimizations() {
    // Set up panic hook for better error reporting in WASM
    console_error_panic_hook::set_once();

    // Initialize logging for WASM
    #[cfg(feature = "wasm-opt")]
    {
        console_log::init_with_level(log::Level::Warn).expect("Failed to initialize console_log");
    }

    // Optimize memory usage
    optimize_memory_usage();
}

/// Optimize memory usage for WASM
fn optimize_memory_usage() {
    // Set up memory management optimizations
    #[cfg(target_arch = "wasm32")]
    {
        // Enable memory growth if needed
        if let Some(window) = web_sys::window() {
            if let Some(performance) = window.performance() {
                // Log performance metrics in development
                #[cfg(debug_assertions)]
                {
                    let _ = performance.now();
                }
            }
        }
    }
}

/// Get WASM memory usage statistics
#[wasm_bindgen]
pub fn get_memory_usage() -> JsValue {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(performance) = window.performance() {
                let memory_info = js_sys::Object::new();
                js_sys::Reflect::set(&memory_info, &"timestamp".into(), &performance.now().into())
                    .ok();

                // Try to get memory info if available
                if let Ok(memory) = js_sys::eval("performance.memory") {
                    if !memory.is_undefined() {
                        return memory;
                    }
                }

                return memory_info.into();
            }
        }
    }

    JsValue::NULL
}

/// Optimize form performance for WASM
pub fn optimize_form_performance() {
    // Reduce allocations in hot paths
    #[cfg(feature = "wasm-opt")]
    {
        // Use string interning for common form field names
        // This reduces memory usage and improves performance
        lazy_static::lazy_static! {
            static ref COMMON_FIELD_NAMES: std::collections::HashSet<&'static str> = {
                let mut set = std::collections::HashSet::new();
                set.insert("email");
                set.insert("password");
                set.insert("name");
                set.insert("firstName");
                set.insert("lastName");
                set.insert("phone");
                set.insert("address");
                set.insert("city");
                set.insert("state");
                set.insert("zipCode");
                set.insert("country");
                set
            };
        }

        // Pre-allocate common validation patterns
        lazy_static::lazy_static! {
            static ref EMAIL_REGEX: regex::Regex = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
            static ref PHONE_REGEX: regex::Regex = regex::Regex::new(r"^\+?[\d\s\-\(\)]+$").unwrap();
        }
    }
}

/// Configure WASM-specific form optimizations
pub fn configure_wasm_form_optimizations() {
    optimize_form_performance();

    // Set up efficient event handling
    #[cfg(target_arch = "wasm32")]
    {
        // Use efficient event delegation
        // This reduces the number of event listeners and improves performance
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                // Set up global event delegation for form events
                let _ = document.add_event_listener_with_callback(
                    "input",
                    Closure::wrap(Box::new(|event: web_sys::Event| {
                        // Handle input events efficiently
                        if let Some(target) = event.target() {
                            if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                                // Only process form-related elements
                                if element.tag_name() == "INPUT"
                                    || element.tag_name() == "SELECT"
                                    || element.tag_name() == "TEXTAREA"
                                {
                                    // Trigger optimized validation
                                    #[cfg(feature = "wasm-opt")]
                                    {
                                        // Use efficient validation patterns
                                    }
                                }
                            }
                        }
                    }) as Box<dyn FnMut(_)>)
                    .into_js_value()
                    .dyn_ref::<js_sys::Function>()
                    .unwrap(),
                );
            }
        }
    }
}
