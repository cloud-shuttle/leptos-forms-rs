//! Performance Monitor Module - DevTools performance monitoring
//!
//! This module provides the PerformanceMonitor for tracking form performance
//! metrics including operation times and memory usage.

pub mod performance_monitor;

// Re-export public API
pub use performance_monitor::*;
