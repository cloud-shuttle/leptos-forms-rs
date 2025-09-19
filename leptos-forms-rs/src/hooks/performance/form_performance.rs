//! Form performance hook
//!
//! This module provides the `use_form_performance` hook for
//! monitoring form performance and collecting metrics.

use crate::core::traits::Form;
use crate::core::FormHandle;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Hook for form performance monitoring
pub fn use_form_performance<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
) -> (
    ReadSignal<crate::core::performance::FormPerformanceMetrics>,
    Callback<()>,
) {
    let metrics = RwSignal::new(crate::core::performance::FormPerformanceMetrics::new());

    let form_clone = form_handle.clone();
    let metrics_clone = metrics;

    // Benchmark callback for measuring performance
    let benchmark = Callback::new(move |_| {
        let form_clone = form_clone.clone();
        let metrics_clone = metrics_clone;

        spawn_local(async move {
            let start = std::time::Instant::now();

            // Measure form creation time
            let _form = FormHandle::new(form_clone.values().get());
            let creation_time = start.elapsed();

            // Update metrics
            metrics_clone.update(|m| {
                m.record_form_creation(creation_time);
            });
        });
    });

    (metrics.read_only(), benchmark)
}
