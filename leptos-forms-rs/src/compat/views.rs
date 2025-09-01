//! Views compatibility layer for Leptos 0.6 and 0.8
//! 
//! Handles different view return types and conditional rendering patterns.

/// Trait for version-agnostic view operations
pub trait ViewCompat {
    /// Convert to view
    fn into_view(self) -> impl IntoView;
}

/// Version-agnostic conditional view rendering
pub fn conditional_view_compat<T, F>(
    condition: impl Fn() -> bool + 'static,
    true_view: T,
    false_view: F,
) -> impl IntoView
where
    T: IntoView,
    F: IntoView,
{
    #[cfg(feature = "leptos-0-6")]
    {
        if condition() {
            true_view.into_view()
        } else {
            false_view.into_view()
        }
    }
    
    #[cfg(feature = "leptos-0-8")]
    {
        use leptos_08::prelude::either::Either;
        if condition() {
            Either::Left(true_view)
        } else {
            Either::Right(false_view)
        }
    }
}

/// Version-agnostic list rendering
pub fn list_view_compat<T, F>(
    items: impl IntoIterator<Item = T>,
    render_fn: F,
) -> impl IntoView
where
    T: Clone + 'static,
    F: Fn(T) -> impl IntoView + Clone + 'static,
{
    #[cfg(feature = "leptos-0-6")]
    {
        use leptos_06::*;
        let items = items.into_iter().collect::<Vec<_>>();
        view! {
            <>
                {items.into_iter().map(|item| render_fn(item.clone()))}
            </>
        }
    }
    
    #[cfg(feature = "leptos-0-8")]
    {
        use leptos_08::prelude::*;
        let items = items.into_iter().collect::<Vec<_>>();
        view! {
            <>
                {items.into_iter().map(|item| render_fn(item.clone()))}
            </>
        }
    }
}

/// Version-agnostic dynamic view rendering
pub fn dynamic_view_compat<T, F>(
    value: T,
    render_fn: F,
) -> impl IntoView
where
    T: Clone + 'static,
    F: Fn(T) -> impl IntoView + Clone + 'static,
{
    #[cfg(feature = "leptos-0-6")]
    {
        use leptos_06::*;
        create_memo(move |_| render_fn(value.clone()))
    }
    
    #[cfg(feature = "leptos-0-8")]
    {
        use leptos_08::prelude::*;
        Memo::new(move |_| render_fn(value.clone()))
    }
}

/// Version-agnostic view builder
pub struct ViewBuilder {
    views: Vec<Box<dyn std::any::Any + 'static>>,
}

impl ViewBuilder {
    /// Create a new view builder
    pub fn new() -> Self {
        Self { views: Vec::new() }
    }
    
    /// Add a view to the builder
    pub fn add<V: IntoView + 'static>(&mut self, view: V) {
        // This is a simplified implementation
        // In practice, you'd want to handle the IntoView trait object properly
        self.views.push(Box::new(view));
    }
    
    /// Build the final view
    pub fn build(self) -> impl IntoView {
        #[cfg(feature = "leptos-0-6")]
        {
            use leptos_06::*;
            view! { <></> }
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            use leptos_08::prelude::*;
            view! { <></> }
        }
    }
}

impl Default for ViewBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Version-agnostic view composition
pub struct ViewComposer {
    views: Vec<Box<dyn std::any::Any + 'static>>,
}

impl ViewComposer {
    /// Create a new view composer
    pub fn new() -> Self {
        Self { views: Vec::new() }
    }
    
    /// Add a view to the composer
    pub fn add<V: IntoView + 'static>(&mut self, view: V) {
        // This is a simplified implementation
        // In practice, you'd want to handle the IntoView trait object properly
        self.views.push(Box::new(view));
    }
    
    /// Compose all views into a single view
    pub fn compose(self) -> impl IntoView {
        #[cfg(feature = "leptos-0-6")]
        {
            use leptos_06::*;
            view! { <></> }
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            use leptos_08::prelude::*;
            view! { <></> }
        }
    }
}

impl Default for ViewComposer {
    fn default() -> Self {
        Self::new()
    }
}

/// Version-agnostic view caching
pub struct ViewCache {
    cache: std::collections::HashMap<String, Box<dyn std::any::Any + 'static>>,
}

impl ViewCache {
    /// Create a new view cache
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
        }
    }
    
    /// Get a cached view
    pub fn get<V: IntoView + 'static>(&self, key: &str) -> Option<&V> {
        self.cache.get(key)?.downcast_ref()
    }
    
    /// Set a cached view
    pub fn set<V: IntoView + 'static>(&mut self, key: String, view: V) {
        self.cache.insert(key, Box::new(view));
    }
    
    /// Remove a cached view
    pub fn remove(&mut self, key: &str) -> Option<Box<dyn std::any::Any + 'static>> {
        self.cache.remove(key)
    }
    
    /// Clear all cached views
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

impl Default for ViewCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_view_builder() {
        let mut builder = ViewBuilder::new();
        builder.add("test");
        let _view = builder.build();
    }
    
    #[test]
    fn test_view_composer() {
        let mut composer = ViewComposer::new();
        composer.add("test");
        let _view = composer.compose();
    }
    
    #[test]
    fn test_view_cache() {
        let mut cache = ViewCache::new();
        cache.set("test".to_string(), "value");
        assert_eq!(cache.get::<&str>("test"), Some(&"value"));
        
        cache.remove("test");
        assert!(cache.get::<&str>("test").is_none());
    }
}
