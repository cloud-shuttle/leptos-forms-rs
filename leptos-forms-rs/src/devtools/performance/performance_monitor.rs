//! Performance Monitor implementation
//!
//! This module provides the PerformanceMonitor for tracking form performance metrics.

use crate::core::{Form, FormHandle};
use std::time::Duration;

use super::super::types::PerformanceMetrics;

/// Performance Monitor for DevTools
pub struct PerformanceMonitor<T: Form + Send + Sync + PartialEq> {
    _form_handle: FormHandle<T>,
    metrics: PerformanceMetrics,
}

impl<T: Form + Send + Sync + PartialEq> PerformanceMonitor<T> {
    pub fn new(form_handle: &FormHandle<T>) -> Result<Self, String> {
        Ok(Self {
            _form_handle: form_handle.clone(),
            metrics: PerformanceMetrics {
                form_creation_time: Some(Duration::from_millis(1)),
                total_field_operations: 0,
                validation_operations: 0,
                average_field_operation_time: None,
                average_validation_time: None,
                memory_usage: Some(1024),
            },
        })
    }

    pub fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.clone()
    }

    pub fn track_field_operation(&mut self, operation_time: Duration) {
        self.metrics.total_field_operations += 1;

        // Update average time calculation
        if let Some(ref mut avg) = self.metrics.average_field_operation_time {
            // Simple running average calculation
            let total_time = avg.as_nanos() * (self.metrics.total_field_operations - 1) as u128
                + operation_time.as_nanos();
            *avg = Duration::from_nanos(
                (total_time / self.metrics.total_field_operations as u128) as u64,
            );
        } else {
            self.metrics.average_field_operation_time = Some(operation_time);
        }
    }

    pub fn track_validation_operation(&mut self, operation_time: Duration) {
        self.metrics.validation_operations += 1;

        // Update average time calculation
        if let Some(ref mut avg) = self.metrics.average_validation_time {
            // Simple running average calculation
            let total_time = avg.as_nanos() * (self.metrics.validation_operations - 1) as u128
                + operation_time.as_nanos();
            *avg = Duration::from_nanos(
                (total_time / self.metrics.validation_operations as u128) as u64,
            );
        } else {
            self.metrics.average_validation_time = Some(operation_time);
        }
    }
}
