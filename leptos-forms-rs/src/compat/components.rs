//! Component compatibility layer for Leptos 0.6 and 0.8
//! 
//! Handles the Scope parameter differences and component definition patterns.

use std::marker::PhantomData;

/// Version-agnostic component wrapper
pub struct ComponentCompat<T: Clone + 'static> {
    #[cfg(feature = "leptos-0-6")]
    inner: leptos_06::Component<T>,
    
    #[cfg(feature = "leptos-0-8")]
    inner: leptos_08::prelude::Component<T>,
    
    _phantom: PhantomData<T>,
}

/// Macro for creating version-agnostic components
#[macro_export]
macro_rules! compat_component {
    (
        fn $name:ident($($prop:ident: $type:ty),*) -> impl IntoView $body:block
    ) => {
        #[cfg(feature = "leptos-0-6")]
        #[leptos_06::component]
        fn $name(cx: leptos_06::Scope, $($prop: $type),*) -> impl leptos_06::IntoView {
            use leptos_06::*;
            let view_result = { $body };
            view_result
        }
        
        #[cfg(feature = "leptos-0-8")]
        #[leptos_08::prelude::component]
        fn $name($($prop: $type),*) -> impl leptos_08::prelude::IntoView {
            use leptos_08::prelude::*;
            let view_result = { $body };
            view_result
        }
    };
}

/// Macro for creating version-agnostic components with explicit scope handling
#[macro_export]
macro_rules! compat_component_with_scope {
    (
        fn $name:ident($($prop:ident: $type:ty),*) -> impl IntoView $body:block
    ) => {
        #[cfg(feature = "leptos-0-6")]
        #[leptos_06::component]
        fn $name(cx: leptos_06::Scope, $($prop: $type),*) -> impl leptos_06::IntoView {
            use leptos_06::*;
            let view_result = { $body };
            view_result
        }
        
        #[cfg(feature = "leptos-0-8")]
        #[leptos_08::prelude::component]
        fn $name($($prop: $type),*) -> impl leptos_08::prelude::IntoView {
            use leptos_08::prelude::*;
            // Create a dummy scope for compatibility
            let _cx = ();
            let view_result = { $body };
            view_result
        }
    };
}

/// Macro for creating version-agnostic components that need scope access
#[macro_export]
macro_rules! compat_component_scope_aware {
    (
        fn $name:ident($($prop:ident: $type:ty),*) -> impl IntoView $body:block
    ) => {
        #[cfg(feature = "leptos-0-6")]
        #[leptos_06::component]
        fn $name(cx: leptos_06::Scope, $($prop: $type),*) -> impl leptos_06::IntoView {
            use leptos_06::*;
            let view_result = { $body };
            view_result
        }
        
        #[cfg(feature = "leptos-0-8")]
        #[leptos_08::prelude::component]
        fn $name($($prop: $type),*) -> impl leptos_08::prelude::IntoView {
            use leptos_08::prelude::*;
            // In 0.8, we don't need scope, but we can create a dummy for compatibility
            let _cx = ();
            let view_result = { $body };
            view_result
        }
    };
}

/// Trait for version-agnostic component mounting
pub trait MountCompat {
    /// Mount the component to the DOM
    fn mount_to_body<F>(f: F) 
    where F: FnOnce() -> impl IntoView + 'static;
    
    /// Mount the component to a specific element
    fn mount_to_element<F>(element_id: &str, f: F) -> Result<(), String>
    where F: FnOnce() -> impl IntoView + 'static;
}

impl MountCompat for () {
    fn mount_to_body<F>(f: F) 
    where F: FnOnce() -> impl IntoView + 'static {
        #[cfg(feature = "leptos-0-6")]
        {
            leptos_06::mount_to_body(f);
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            leptos_08::prelude::mount_to_body(f);
        }
    }
    
    fn mount_to_element<F>(element_id: &str, f: F) -> Result<(), String>
    where F: FnOnce() -> impl IntoView + 'static {
        #[cfg(feature = "leptos-0-6")]
        {
            if let Some(element) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id(element_id)) {
                leptos_06::mount_to(element, f);
                Ok(())
            } else {
                Err(format!("Element with id '{}' not found", element_id))
            }
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            if let Some(element) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id(element_id)) {
                leptos_08::prelude::mount_to(element, f);
                Ok(())
            } else {
                Err(format!("Element with id '{}' not found", element_id))
            }
        }
    }
}

/// Version-agnostic component builder
pub struct ComponentBuilder<T> {
    props: T,
}

impl<T> ComponentBuilder<T> {
    /// Create a new component builder
    pub fn new(props: T) -> Self {
        Self { props }
    }
    
    /// Build the component with version-agnostic mounting
    pub fn mount_to_body<F>(self, f: F) 
    where F: FnOnce(T) -> impl IntoView + 'static {
        #[cfg(feature = "leptos-0-6")]
        {
            leptos_06::mount_to_body(move || f(self.props));
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            leptos_08::prelude::mount_to_body(move || f(self.props));
        }
    }
    
    /// Build the component and mount to a specific element
    pub fn mount_to_element<F>(self, element_id: &str, f: F) -> Result<(), String>
    where F: FnOnce(T) -> impl IntoView + 'static {
        #[cfg(feature = "leptos-0-6")]
        {
            if let Some(element) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id(element_id)) {
                leptos_06::mount_to(element, move || f(self.props));
                Ok(())
            } else {
                Err(format!("Element with id '{}' not found", element_id))
            }
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            if let Some(element) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id(element_id)) {
                leptos_08::prelude::mount_to(element, move || f(self.props));
                Ok(())
            } else {
                Err(format!("Element with id '{}' not found", element_id))
            }
        }
    }
}

/// Create a component builder
pub fn component<T>(props: T) -> ComponentBuilder<T> {
    ComponentBuilder::new(props)
}

/// Version-agnostic component registration
pub trait ComponentRegistry {
    /// Register a component with the given name
    fn register_component<F>(&mut self, name: &str, component: F) -> Result<(), String>
    where F: Fn() -> impl IntoView + 'static;
    
    /// Get a component by name
    fn get_component(&self, name: &str) -> Option<Box<dyn Fn() -> impl IntoView + 'static>>;
}

/// Simple component registry implementation
pub struct SimpleComponentRegistry {
    components: std::collections::HashMap<String, Box<dyn std::any::Any + 'static>>,
}

impl SimpleComponentRegistry {
    /// Create a new component registry
    pub fn new() -> Self {
        Self {
            components: std::collections::HashMap::new(),
        }
    }
}

impl ComponentRegistry for SimpleComponentRegistry {
    fn register_component<F>(&mut self, name: &str, component: F) -> Result<(), String>
    where F: Fn() -> impl IntoView + 'static {
        // Note: This is a simplified implementation
        // In practice, you'd want to handle the IntoView trait object properly
        self.components.insert(name.to_string(), Box::new(component));
        Ok(())
    }
    
    fn get_component(&self, name: &str) -> Option<Box<dyn Fn() -> impl IntoView + 'static>> {
        // Note: This is a simplified implementation
        // In practice, you'd want to handle the IntoView trait object properly
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_component_builder() {
        let builder = ComponentBuilder::new(42);
        // This would need a proper test environment to actually mount
        assert_eq!(builder.props, 42);
    }
    
    #[test]
    fn test_component_registry() {
        let mut registry = SimpleComponentRegistry::new();
        let result = registry.register_component("test", || {
            // Dummy component
            leptos::view! { <div>"Test"</div> }
        });
        assert!(result.is_ok());
    }
}
