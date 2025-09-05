use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Performance metrics for form operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormPerformanceMetrics {
    /// Time taken to create the form
    pub form_creation_time: Duration,
    /// Number of field operations (set, get, update)
    pub field_operations: u64,
    /// Number of validation operations
    pub validation_operations: u64,
    /// Number of submission operations
    pub submission_operations: u64,
    /// Total number of operations
    pub total_operations: u64,
    /// Memory usage in bytes (approximate)
    pub memory_usage_bytes: u64,
    /// Average time per field operation
    pub avg_field_operation_time: Duration,
    /// Average time per validation operation
    pub avg_validation_time: Duration,
    /// Average time per submission operation
    pub avg_submission_time: Duration,
    /// Peak memory usage in bytes
    pub peak_memory_usage_bytes: u64,
    /// Number of re-renders
    pub re_render_count: u64,
    /// Time spent in validation
    pub total_validation_time: Duration,
    /// Time spent in field operations
    pub total_field_operation_time: Duration,
    /// Time spent in submissions
    pub total_submission_time: Duration,
}

impl Default for FormPerformanceMetrics {
    fn default() -> Self {
        Self {
            form_creation_time: Duration::ZERO,
            field_operations: 0,
            validation_operations: 0,
            submission_operations: 0,
            total_operations: 0,
            memory_usage_bytes: 0,
            avg_field_operation_time: Duration::ZERO,
            avg_validation_time: Duration::ZERO,
            avg_submission_time: Duration::ZERO,
            peak_memory_usage_bytes: 0,
            re_render_count: 0,
            total_validation_time: Duration::ZERO,
            total_field_operation_time: Duration::ZERO,
            total_submission_time: Duration::ZERO,
        }
    }
}

impl FormPerformanceMetrics {
    /// Create new performance metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record form creation time
    pub fn record_form_creation(&mut self, duration: Duration) {
        self.form_creation_time = duration;
    }

    /// Record a field operation
    pub fn record_field_operation(&mut self, duration: Duration) {
        self.field_operations += 1;
        self.total_field_operation_time += duration;
        self.avg_field_operation_time = if self.field_operations > 0 {
            Duration::from_nanos(
                (self.total_field_operation_time.as_nanos() as u64) / self.field_operations,
            )
        } else {
            Duration::ZERO
        };
        self.total_operations += 1;
    }

    /// Record a validation operation
    pub fn record_validation(&mut self, duration: Duration) {
        self.validation_operations += 1;
        self.total_validation_time += duration;
        self.avg_validation_time = if self.validation_operations > 0 {
            Duration::from_nanos(
                (self.total_validation_time.as_nanos() as u64) / self.validation_operations,
            )
        } else {
            Duration::ZERO
        };
        self.total_operations += 1;
    }

    /// Record a submission operation
    pub fn record_submission(&mut self, duration: Duration) {
        self.submission_operations += 1;
        self.total_submission_time += duration;
        self.avg_submission_time = if self.submission_operations > 0 {
            Duration::from_nanos(
                (self.total_submission_time.as_nanos() as u64) / self.submission_operations,
            )
        } else {
            Duration::ZERO
        };
        self.total_operations += 1;
    }

    /// Record memory usage
    pub fn record_memory_usage(&mut self, bytes: u64) {
        self.memory_usage_bytes = bytes;
        if bytes > self.peak_memory_usage_bytes {
            self.peak_memory_usage_bytes = bytes;
        }
    }

    /// Record a re-render
    pub fn record_re_render(&mut self) {
        self.re_render_count += 1;
    }

    /// Get performance summary as a string
    pub fn summary(&self) -> String {
        format!(
            "Form Performance Summary:\n\
             - Form Creation: {:?}\n\
             - Field Operations: {} (avg: {:?})\n\
             - Validations: {} (avg: {:?})\n\
             - Submissions: {} (avg: {:?})\n\
             - Total Operations: {}\n\
             - Memory Usage: {} bytes (peak: {} bytes)\n\
             - Re-renders: {}\n\
             - Total Field Time: {:?}\n\
             - Total Validation Time: {:?}\n\
             - Total Submission Time: {:?}",
            self.form_creation_time,
            self.field_operations,
            self.avg_field_operation_time,
            self.validation_operations,
            self.avg_validation_time,
            self.submission_operations,
            self.avg_submission_time,
            self.total_operations,
            self.memory_usage_bytes,
            self.peak_memory_usage_bytes,
            self.re_render_count,
            self.total_field_operation_time,
            self.total_validation_time,
            self.total_submission_time
        )
    }

    /// Check if performance is within acceptable thresholds
    pub fn is_performance_acceptable(&self, thresholds: &PerformanceThresholds) -> bool {
        self.form_creation_time <= thresholds.max_form_creation_time
            && self.avg_field_operation_time <= thresholds.max_field_operation_time
            && self.avg_validation_time <= thresholds.max_validation_time
            && self.avg_submission_time <= thresholds.max_submission_time
            && self.memory_usage_bytes <= thresholds.max_memory_usage_bytes
    }
}

/// Performance thresholds for acceptable performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum acceptable form creation time
    pub max_form_creation_time: Duration,
    /// Maximum acceptable field operation time
    pub max_field_operation_time: Duration,
    /// Maximum acceptable validation time
    pub max_validation_time: Duration,
    /// Maximum acceptable submission time
    pub max_submission_time: Duration,
    /// Maximum acceptable memory usage in bytes
    pub max_memory_usage_bytes: u64,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_form_creation_time: Duration::from_millis(100),
            max_field_operation_time: Duration::from_millis(10),
            max_validation_time: Duration::from_millis(50),
            max_submission_time: Duration::from_millis(200),
            max_memory_usage_bytes: 1024 * 1024, // 1MB
        }
    }
}

/// Performance benchmark results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResults {
    /// Performance metrics
    pub metrics: FormPerformanceMetrics,
    /// Whether performance meets thresholds
    pub meets_thresholds: bool,
    /// Performance score (0-100)
    pub performance_score: f64,
    /// Recommendations for improvement
    pub recommendations: Vec<String>,
}

impl BenchmarkResults {
    /// Create new benchmark results
    pub fn new(metrics: FormPerformanceMetrics, thresholds: &PerformanceThresholds) -> Self {
        let meets_thresholds = metrics.is_performance_acceptable(thresholds);
        let performance_score = Self::calculate_performance_score(&metrics, thresholds);
        let recommendations = Self::generate_recommendations(&metrics, thresholds);

        Self {
            metrics,
            meets_thresholds,
            performance_score,
            recommendations,
        }
    }

    /// Calculate performance score (0-100)
    fn calculate_performance_score(
        metrics: &FormPerformanceMetrics,
        thresholds: &PerformanceThresholds,
    ) -> f64 {
        let mut score: f64 = 100.0;

        // Deduct points for exceeding thresholds
        if metrics.form_creation_time > thresholds.max_form_creation_time {
            score -= 20.0;
        }
        if metrics.avg_field_operation_time > thresholds.max_field_operation_time {
            score -= 15.0;
        }
        if metrics.avg_validation_time > thresholds.max_validation_time {
            score -= 15.0;
        }
        if metrics.avg_submission_time > thresholds.max_submission_time {
            score -= 20.0;
        }
        if metrics.memory_usage_bytes > thresholds.max_memory_usage_bytes {
            score -= 10.0;
        }

        score.max(0.0_f64)
    }

    /// Generate performance improvement recommendations
    fn generate_recommendations(
        metrics: &FormPerformanceMetrics,
        thresholds: &PerformanceThresholds,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        if metrics.form_creation_time > thresholds.max_form_creation_time {
            recommendations.push("Form creation is slow. Consider lazy initialization or reducing initial field count.".to_string());
        }

        if metrics.avg_field_operation_time > thresholds.max_field_operation_time {
            recommendations.push(
                "Field operations are slow. Consider batching updates or optimizing field access."
                    .to_string(),
            );
        }

        if metrics.avg_validation_time > thresholds.max_field_operation_time {
            recommendations.push(
                "Validation is slow. Consider async validation or reducing validation complexity."
                    .to_string(),
            );
        }

        if metrics.avg_submission_time > thresholds.max_submission_time {
            recommendations.push("Form submission is slow. Consider optimizing submission logic or using background processing.".to_string());
        }

        if metrics.memory_usage_bytes > thresholds.max_memory_usage_bytes {
            recommendations.push("Memory usage is high. Consider reducing field count or optimizing data structures.".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push(
                "Performance is within acceptable thresholds. No immediate improvements needed."
                    .to_string(),
            );
        }

        recommendations
    }
}
