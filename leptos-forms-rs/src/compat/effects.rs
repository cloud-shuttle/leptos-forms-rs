//! Effects compatibility layer for Leptos 0.6 and 0.8
//! 
//! Handles the different effect return types and lifecycle management.

use std::marker::PhantomData;

/// Version-agnostic effect wrapper
pub struct EffectCompat {
    #[cfg(feature = "leptos-0-6")]
    inner: Option<leptos_06::Effect<()>>,
    
    #[cfg(feature = "leptos-0-8")]
    inner: Option<leptos_08::prelude::Effect>,
}

impl EffectCompat {
    /// Create a new effect
    pub fn new<F>(f: F) -> Self
    where F: Fn(Option<()>) + 'static {
        #[cfg(feature = "leptos-0-6")]
        {
            let effect = leptos_06::create_effect(move |_| f(None));
            Self { inner: Some(effect) }
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            let effect = leptos_08::prelude::Effect::new(move |_| f(None));
            Self { inner: Some(effect) }
        }
    }
    
    /// Create an effect with a dependency
    pub fn new_with_deps<F, D>(f: F, deps: D) -> Self
    where 
        F: Fn(Option<()>) + 'static,
        D: Clone + 'static,
    {
        #[cfg(feature = "leptos-0-6")]
        {
            let effect = leptos_06::create_effect(move |_| f(None));
            Self { inner: Some(effect) }
        }
        
        #[cfg(feature = "leptos-0-8")]
        {
            let effect = leptos_08::prelude::Effect::new_with_deps(move |_| f(None), deps);
            Self { inner: Some(effect) }
        }
    }
    
    /// Stop the effect (0.8 only, 0.6 effects are automatically cleaned up)
    pub fn stop(mut self) {
        #[cfg(feature = "leptos-0-8")]
        {
            if let Some(effect) = self.inner.take() {
                effect.stop();
            }
        }
        // 0.6 effects are automatically cleaned up with scope
    }
    
    /// Check if the effect is still active
    pub fn is_active(&self) -> bool {
        self.inner.is_some()
    }
}

impl Drop for EffectCompat {
    fn drop(&mut self) {
        #[cfg(feature = "leptos-0-8")]
        {
            if let Some(effect) = self.inner.take() {
                effect.stop();
            }
        }
    }
}

/// Create a conditional effect with cleanup
pub fn create_conditional_effect<T, F, C>(
    condition: impl Fn() -> bool + 'static,
    effect_fn: F,
    cleanup_fn: C,
) -> EffectCompat
where
    T: 'static,
    F: Fn() -> T + 'static,
    C: Fn(T) + 'static,
{
    let mut cleanup_handle: Option<T> = None;
    
    EffectCompat::new(move |_| {
        if condition() {
            if let Some(handle) = cleanup_handle.take() {
                cleanup_fn(handle);
            }
            cleanup_handle = Some(effect_fn());
        }
    })
}

/// Create an effect that runs only once
pub fn create_once_effect<F>(f: F) -> EffectCompat
where F: FnOnce() + 'static {
    let mut has_run = false;
    EffectCompat::new(move |_| {
        if !has_run {
            f();
            has_run = true;
        }
    })
}

/// Create an effect with debouncing
pub fn create_debounced_effect<F>(
    delay_ms: u32,
    f: F,
) -> EffectCompat
where F: Fn() + 'static {
    #[cfg(feature = "leptos-0-6")]
    {
        use gloo_timers::callback::Timeout;
        
        let mut timeout_handle: Option<Timeout> = None;
        EffectCompat::new(move |_| {
            if let Some(handle) = timeout_handle.take() {
                handle.cancel();
            }
            
            let timeout = Timeout::new(delay_ms, move || {
                f();
            });
            timeout_handle = Some(timeout);
        })
    }
    
    #[cfg(feature = "leptos-0-8")]
    {
        use gloo_timers::callback::Timeout;
        
        let mut timeout_handle: Option<Timeout> = None;
        EffectCompat::new(move |_| {
            if let Some(handle) = timeout_handle.take() {
                handle.cancel();
            }
            
            let timeout = Timeout::new(delay_ms, move || {
                f();
            });
            timeout_handle = Some(timeout);
        })
    }
}

/// Create an effect with throttling
pub fn create_throttled_effect<F>(
    interval_ms: u32,
    f: F,
) -> EffectCompat
where F: Fn() + 'static {
    #[cfg(feature = "leptos-0-6")]
    {
        use gloo_timers::callback::Timeout;
        use std::cell::RefCell;
        use std::rc::Rc;
        
        let last_run = Rc::new(RefCell::new(0u64));
        let last_run_clone = last_run.clone();
        
        EffectCompat::new(move |_| {
            let now = web_sys::window()
                .and_then(|w| w.performance())
                .map(|p| p.now())
                .unwrap_or(0.0) as u64;
            
            let last = *last_run_clone.borrow();
            if now - last >= interval_ms as u64 {
                f();
                *last_run_clone.borrow_mut() = now;
            }
        })
    }
    
    #[cfg(feature = "leptos-0-8")]
    {
        use gloo_timers::callback::Timeout;
        use std::cell::RefCell;
        use std::rc::Rc;
        
        let last_run = Rc::new(RefCell::new(0u64));
        let last_run_clone = last_run.clone();
        
        EffectCompat::new(move |_| {
            let now = web_sys::window()
                .and_then(|w| w.performance())
                .map(|p| p.now())
                .unwrap_or(0.0) as u64;
            
            let last = *last_run_clone.borrow();
            if now - last >= interval_ms as u64 {
                f();
                *last_run_clone.borrow_mut() = now;
            }
        })
    }
}

/// Create an effect that tracks multiple dependencies
pub fn create_multi_deps_effect<F, D>(f: F, deps: Vec<D>) -> EffectCompat
where
    F: Fn() + 'static,
    D: Clone + PartialEq + 'static,
{
    #[cfg(feature = "leptos-0-6")]
    {
        // For 0.6, we'll create a simple effect that always runs
        // In practice, you might want to implement more sophisticated dependency tracking
        EffectCompat::new(move |_| f())
    }
    
    #[cfg(feature = "leptos-0-8")]
    {
        // For 0.8, we can use the built-in dependency tracking
        let effect = leptos_08::prelude::Effect::new_with_deps(move |_| f(), deps);
        EffectCompat { inner: Some(effect) }
    }
}

/// Effect lifecycle manager for complex scenarios
pub struct EffectManager {
    effects: Vec<EffectCompat>,
}

impl EffectManager {
    /// Create a new effect manager
    pub fn new() -> Self {
        Self { effects: Vec::new() }
    }
    
    /// Add an effect to the manager
    pub fn add_effect(&mut self, effect: EffectCompat) {
        self.effects.push(effect);
    }
    
    /// Stop all effects
    pub fn stop_all(&mut self) {
        for effect in self.effects.drain(..) {
            effect.stop();
        }
    }
    
    /// Get the number of active effects
    pub fn active_count(&self) -> usize {
        self.effects.iter().filter(|e| e.is_active()).count()
    }
}

impl Drop for EffectManager {
    fn drop(&mut self) {
        self.stop_all();
    }
}

impl Default for EffectManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_effect_creation() {
        let effect = EffectCompat::new(|_| {
            // Test effect
        });
        assert!(effect.is_active());
    }
    
    #[test]
    fn test_conditional_effect() {
        let effect = create_conditional_effect(
            || true,
            || "test",
            |_| {},
        );
        assert!(effect.is_active());
    }
    
    #[test]
    fn test_effect_manager() {
        let mut manager = EffectManager::new();
        assert_eq!(manager.active_count(), 0);
        
        let effect = EffectCompat::new(|_| {});
        manager.add_effect(effect);
        assert_eq!(manager.active_count(), 1);
        
        manager.stop_all();
        assert_eq!(manager.active_count(), 0);
    }
}
