//! Resources compatibility layer for Leptos 0.6 and 0.8
//! 
//! Handles the different resource APIs and serialization requirements.

use std::marker::PhantomData;
use serde::{Serialize, Deserialize};

/// Version-agnostic resource wrapper
pub struct ResourceCompat<T: Clone + 'static> {
    #[cfg(feature = "leptos-0-6")]
    inner: leptos_06::Resource<(), T>,
    
    #[cfg(feature = "leptos-0-8")]
    inner: leptos_08::prelude::Resource<(), T>,
    
    _phantom: PhantomData<T>,
}

impl<T: Clone + 'static> ResourceCompat<T> {
    /// Create a new resource
    pub fn new<F>(fetcher: F) -> Self
    where F: Fn() -> futures::future::BoxFuture<'static, T> + 'static {
        #[cfg(feature = "leptos-0-6")]
        {
            let inner = leptos_06::create_resource(|| (), move |_| fetcher());
            Self { inner, _phantom: PhantomData }
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            let inner = leptos_08::prelude::Resource::new(|| (), move |_| fetcher());
            Self { inner, _phantom: PhantomData }
        }
    }
    
    /// Get the current resource value
    pub fn get(&self) -> Option<T> {
        #[cfg(feature = "leptos-0-6")]
        {
            self.inner.read()
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            self.inner.get()
        }
    }
    
    /// Check if the resource is loading
    pub fn is_loading(&self) -> bool {
        #[cfg(feature = "leptos-0-6")]
        {
            self.inner.loading().get()
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            self.inner.loading().get()
        }
    }
    
    /// Check if the resource has an error
    pub fn has_error(&self) -> bool {
        #[cfg(feature = "leptos-0-6")]
        {
            self.inner.error().get().is_some()
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            self.inner.error().get().is_some()
        }
    }
    
    /// Get the error if any
    pub fn error(&self) -> Option<String> {
        #[cfg(feature = "leptos-0-6")]
        {
            self.inner.error().get().map(|e| e.to_string())
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            self.inner.error().get().map(|e| e.to_string())
        }
    }
    
    /// Refresh the resource
    pub fn refresh(&self) {
        #[cfg(feature = "leptos-0-6")]
        {
            self.inner.refetch();
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            self.inner.refetch();
        }
    }
}

impl<T: Clone + 'static> Clone for ResourceCompat<T> {
    fn clone(&self) -> Self {
        #[cfg(feature = "leptos-0-6")]
        {
            Self {
                inner: self.inner.clone(),
                _phantom: PhantomData,
            }
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            Self {
                inner: self.inner.clone(),
                _phantom: PhantomData,
            }
        }
    }
}

/// Version-agnostic local resource wrapper (client-only)
pub struct LocalResourceCompat<T: Clone + 'static> {
    #[cfg(feature = "leptos-0-6")]
    inner: leptos_06::Resource<(), T>,
    
    #[cfg(feature = "leptos-0-8")]
    inner: leptos_08::prelude::LocalResource<(), T>,
    
    _phantom: PhantomData<T>,
}

impl<T: Clone + 'static> LocalResourceCompat<T> {
    /// Create a new local resource
    pub fn new<F>(fetcher: F) -> Self
    where F: Fn() -> futures::future::BoxFuture<'static, T> + 'static {
        #[cfg(feature = "leptos-0-6")]
        {
            let inner = leptos_06::create_resource(|| (), move |_| fetcher());
            Self { inner, _phantom: PhantomData }
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            let inner = leptos_08::prelude::LocalResource::new(|| (), move |_| fetcher());
            Self { inner, _phantom: PhantomData }
        }
    }
    
    /// Get the current resource value
    pub fn get(&self) -> Option<T> {
        #[cfg(feature = "leptos-0-6")]
        {
            self.inner.read()
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            self.inner.get()
        }
    }
    
    /// Check if the resource is loading
    pub fn is_loading(&self) -> bool {
        #[cfg(feature = "leptos-0-6")]
        {
            self.inner.loading().get()
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            self.inner.loading().get()
        }
    }
    
    /// Check if the resource has an error
    pub fn has_error(&self) -> bool {
        #[cfg(feature = "leptos-0-6")]
        {
            self.inner.error().get().is_some()
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            self.inner.error().get().is_some()
        }
    }
    
    /// Get the error if any
    pub fn error(&self) -> Option<String> {
        #[cfg(feature = "leptos-0-6")]
        {
            self.inner.error().get().map(|e| e.to_string())
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            self.inner.error().get().map(|e| e.to_string())
        }
    }
    
    /// Refresh the resource
    pub fn refresh(&self) {
        #[cfg(feature = "leptos-0-6")]
        {
            self.inner.refetch();
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            self.inner.refetch();
        }
    }
}

impl<T: Clone + 'static> Clone for LocalResourceCompat<T> {
    fn clone(&self) -> Self {
        #[cfg(feature = "leptos-0-6")]
        {
            Self {
                inner: self.inner.clone(),
                _phantom: PhantomData,
            }
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            Self {
                inner: self.inner.clone(),
                _phantom: PhantomData,
            }
        }
    }
}

/// Create a resource with version-agnostic API
pub fn create_resource<T, F>(fetcher: F) -> ResourceCompat<T>
where
    T: Clone + 'static,
    F: Fn() -> futures::future::BoxFuture<'static, T> + 'static,
{
    ResourceCompat::new(fetcher)
}

/// Create a local resource with version-agnostic API
pub fn create_local_resource<T, F>(fetcher: F) -> LocalResourceCompat<T>
where
    T: Clone + 'static,
    F: Fn() -> futures::future::BoxFuture<'static, T> + 'static,
{
    LocalResourceCompat::new(fetcher)
}

/// Resource with serialization support for SSR (0.8+)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "leptos-0-8", derive(Serialize, Deserialize))]
pub struct SerializableResource<T>
where
    T: Clone + 'static,
{
    #[cfg_attr(feature = "leptos-0-8", serde(skip_serializing_if = "Option::is_none"))]
    pub data: Option<T>,
    #[cfg_attr(feature = "leptos-0-8", serde(skip_serializing_if = "Option::is_none"))]
    pub error: Option<String>,
    pub loading: bool,
}

impl<T: Clone + 'static> SerializableResource<T> {
    /// Create a new serializable resource
    pub fn new() -> Self {
        Self {
            data: None,
            error: None,
            loading: false,
        }
    }
    
    /// Create from a resource compat
    pub fn from_resource(resource: &ResourceCompat<T>) -> Self {
        Self {
            data: resource.get(),
            error: resource.error(),
            loading: resource.is_loading(),
        }
    }
    
    /// Set the data
    pub fn set_data(&mut self, data: T) {
        self.data = Some(data);
        self.loading = false;
        self.error = None;
    }
    
    /// Set an error
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
        self.loading = false;
        self.data = None;
    }
    
    /// Set loading state
    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
        if loading {
            self.error = None;
        }
    }
}

impl<T: Clone + 'static> Default for SerializableResource<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Resource manager for handling multiple resources
pub struct ResourceManager {
    resources: std::collections::HashMap<String, Box<dyn std::any::Any + 'static>>,
}

impl ResourceManager {
    /// Create a new resource manager
    pub fn new() -> Self {
        Self {
            resources: std::collections::HashMap::new(),
        }
    }
    
    /// Add a resource to the manager
    pub fn add_resource<T: Clone + 'static>(&mut self, name: &str, resource: ResourceCompat<T>) {
        self.resources.insert(name.to_string(), Box::new(resource));
    }
    
    /// Get a resource by name
    pub fn get_resource<T: Clone + 'static>(&self, name: &str) -> Option<&ResourceCompat<T>> {
        self.resources.get(name)?.downcast_ref()
    }
    
    /// Refresh all resources
    pub fn refresh_all(&self) {
        for resource in self.resources.values() {
            // This is a simplified implementation
            // In practice, you'd want to handle the different resource types properly
        }
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_serializable_resource() {
        let mut resource = SerializableResource::<String>::new();
        assert!(resource.data.is_none());
        assert!(resource.error.is_none());
        assert!(!resource.loading);
        
        resource.set_data("test".to_string());
        assert_eq!(resource.data, Some("test".to_string()));
        assert!(!resource.loading);
        assert!(resource.error.is_none());
        
        resource.set_error("error".to_string());
        assert_eq!(resource.error, Some("error".to_string()));
        assert!(!resource.loading);
        assert!(resource.data.is_none());
        
        resource.set_loading(true);
        assert!(resource.loading);
        assert!(resource.error.is_none());
    }
    
    #[test]
    fn test_resource_manager() {
        let mut manager = ResourceManager::new();
        let resource = create_resource(|| {
            Box::pin(async { "test".to_string() })
        });
        
        manager.add_resource("test", resource);
        assert!(manager.get_resource::<String>("test").is_some());
        assert!(manager.get_resource::<String>("nonexistent").is_none());
    }
}
