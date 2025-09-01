//! Compatibility layer for Leptos 0.6 and 0.8
//! 
//! This module provides version-agnostic abstractions that work with both
//! Leptos 0.6 and 0.8, allowing gradual migration between versions.

pub mod signals;
pub mod components;
pub mod resources;
pub mod effects;
pub mod context;
pub mod views;

pub use signals::*;
pub use components::*;
pub use resources::*;
pub use effects::*;
pub use context::*;
pub use views::*;

/// Re-export common types for convenience
pub mod prelude {
    pub use super::{
        SignalCompat, MemoCompat, EffectCompat, ComponentCompat,
        ResourceCompat, ContextCompat, ViewCompat
    };
}

/// Version detection and feature flag utilities
pub mod version {
    #[cfg(feature = "leptos-0-6")]
    pub const LEPTOS_VERSION: &str = "0.6";
    
    #[cfg(feature = "leptos-0-8")]
    pub const LEPTOS_VERSION: &str = "0.8";
    
    #[cfg(feature = "leptos-0-6")]
    pub fn is_leptos_06() -> bool { true }
    
    #[cfg(feature = "leptos-0-6")]
    pub fn is_leptos_08() -> bool { false }
    
    #[cfg(feature = "leptos-0-8")]
    pub fn is_leptos_06() -> bool { false }
    
    #[cfg(feature = "leptos-0-8")]
    pub fn is_leptos_08() -> bool { true }
    
    /// Check if we're running on Leptos 0.8+
    pub fn supports_tachys() -> bool {
        is_leptos_08()
    }
    
    /// Check if we need to handle Scope parameters
    pub fn requires_scope() -> bool {
        is_leptos_06()
    }
}
