//! Context compatibility layer for Leptos 0.6 and 0.8
//! 
//! Handles the Scope parameter differences in context APIs.

/// Trait for version-agnostic context operations
pub trait ContextCompat {
    /// Provide a context value
    fn provide<T: Clone + 'static>(&self, value: T);
    
    /// Use a context value
    fn use_context<T: Clone + 'static>(&self) -> Option<T>;
}

/// Version-agnostic context provider
pub struct ContextProvider;

impl ContextCompat for ContextProvider {
    #[cfg(feature = "leptos-0-6")]
    fn provide<T: Clone + 'static>(&self, value: T) {
        leptos_06::provide_context(value);
    }
    
    #[cfg(feature = "leptos-0-8")]
    fn provide<T: Clone + 'static>(&self, value: T) {
        leptos_08::prelude::provide_context(value);
    }
    
    #[cfg(feature = "leptos-0-6")]
    fn use_context<T: Clone + 'static>(&self) -> Option<T> {
        leptos_06::use_context()
    }
    
    #[cfg(feature = "leptos-0-8")]
    fn use_context<T: Clone + 'static>(&self) -> Option<T> {
        leptos_08::prelude::use_context()
    }
}

/// Provide context with version-agnostic API
pub fn provide_context<T: Clone + 'static>(value: T) {
    ContextProvider.provide(value);
}

/// Use context with version-agnostic API
pub fn use_context<T: Clone + 'static>() -> Option<T> {
    ContextProvider.use_context()
}

/// Context manager for handling multiple context values
pub struct ContextManager {
    values: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + 'static>>,
}

impl ContextManager {
    /// Create a new context manager
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
        }
    }
    
    /// Set a context value
    pub fn set<T: 'static>(&mut self, value: T) {
        self.values.insert(std::any::TypeId::of::<T>(), Box::new(value));
    }
    
    /// Get a context value
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.values.get(&std::any::TypeId::of::<T>())?.downcast_ref()
    }
    
    /// Get a mutable reference to a context value
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.values.get_mut(&std::any::TypeId::of::<T>())?.downcast_mut()
    }
    
    /// Remove a context value
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.values.remove(&std::any::TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast().ok().map(|b| *b))
    }
    
    /// Check if a context value exists
    pub fn has<T: 'static>(&self) -> bool {
        self.values.contains_key(&std::any::TypeId::of::<T>())
    }
    
    /// Clear all context values
    pub fn clear(&mut self) {
        self.values.clear();
    }
}

impl Default for ContextManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_context_manager() {
        let mut manager = ContextManager::new();
        
        // Test setting and getting values
        manager.set(42i32);
        assert_eq!(manager.get::<i32>(), Some(&42));
        assert!(manager.has::<i32>());
        
        // Test getting mutable reference
        if let Some(value) = manager.get_mut::<i32>() {
            *value = 100;
        }
        assert_eq!(manager.get::<i32>(), Some(&100));
        
        // Test removing values
        let removed = manager.remove::<i32>();
        assert_eq!(removed, Some(100));
        assert!(!manager.has::<i32>());
        
        // Test clearing
        manager.set("test".to_string());
        manager.clear();
        assert!(!manager.has::<String>());
    }
}
