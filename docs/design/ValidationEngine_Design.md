# Validation Engine Design

**Files**: `leptos-forms-rs/src/validation/engine/` (multiple files < 300 lines each)
**Priority**: HIGH
**Current Issues**: Performance problems, monolithic structure (419 lines)

## Problem Statement

The current validation engine has critical performance issues:

- Creates new `ValidationRuleEngine` on every validation call
- No caching of compiled validators (especially regex patterns)
- Excessive cloning and allocation
- Monolithic 419-line file that's hard to maintain and test

## Solution Architecture

### File Structure (Split into focused modules)

```
leptos-forms-rs/src/validation/
├── engine/
│   ├── mod.rs (< 100 lines) - Public API and re-exports
│   ├── sync_validator.rs (< 200 lines) - Synchronous validation
│   ├── async_validator.rs (< 200 lines) - Asynchronous validation
│   ├── cache.rs (< 150 lines) - Validation caching system
│   └── performance.rs (< 100 lines) - Performance monitoring
├── rules/
│   ├── mod.rs (< 50 lines) - Rule exports
│   ├── built_in.rs (< 250 lines) - Standard validators
│   ├── custom.rs (< 150 lines) - Custom validator support
│   └── cross_field.rs (< 200 lines) - Multi-field validation
├── errors.rs (< 200 lines) - Error types and handling
└── mod.rs (< 100 lines) - Module exports
```

## Core Design Principles

### 1. Performance-First

- Cache compiled validators (regex, complex rules)
- Reuse validation engine instances
- Minimize allocations during validation
- Lazy loading of expensive validators

### 2. Composability

- Validators can be combined and chained
- Custom validators integrate seamlessly
- Cross-field validation support
- Conditional validation rules

### 3. Type Safety

- Compile-time validation rule checking
- Strong typing for all validation results
- Generic over form types

## Cached Validation Engine

### Main Engine API (`sync_validator.rs`)

```rust
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct CachedValidationEngine {
    // Cached compiled rules to avoid recompilation
    rule_cache: Arc<RwLock<HashMap<String, CompiledRule>>>,
    // Regex cache for pattern validators
    regex_cache: Arc<RwLock<HashMap<String, Regex>>>,
    // Performance metrics
    metrics: Arc<ValidationMetrics>,
}

impl CachedValidationEngine {
    pub fn new() -> Self {
        Self {
            rule_cache: Arc::new(RwLock::new(HashMap::new())),
            regex_cache: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(ValidationMetrics::new()),
        }
    }

    /// Validate a single field with caching
    pub fn validate_field(
        &self,
        field_name: &str,
        value: &FieldValue,
        rules: &[ValidationRule]
    ) -> ValidationResult {
        let start_time = web_sys::window().unwrap().performance().unwrap().now();

        let rule_key = self.generate_rule_cache_key(field_name, rules);

        let compiled_rule = {
            let cache = self.rule_cache.read().unwrap();
            if let Some(cached) = cache.get(&rule_key) {
                cached.clone()
            } else {
                drop(cache); // Release read lock
                let compiled = self.compile_rules(rules);
                let mut cache = self.rule_cache.write().unwrap();
                cache.insert(rule_key.clone(), compiled.clone());
                compiled
            }
        };

        let result = compiled_rule.validate(value, &self.regex_cache);

        // Record performance metrics
        let duration = web_sys::window().unwrap().performance().unwrap().now() - start_time;
        self.metrics.record_validation(field_name, duration, result.is_ok());

        result
    }

    /// Validate entire form
    pub fn validate_form<T>(&self, form: &T) -> FormValidationResult
    where T: Form {
        let mut results = FormValidationResult::new();
        let form_data = form.to_field_map();

        // First pass: individual field validation
        for (field_name, field_value) in &form_data {
            if let Some(field_meta) = T::get_field_metadata(field_name) {
                let result = self.validate_field(
                    field_name,
                    field_value,
                    &field_meta.validators
                );
                results.add_field_result(field_name.clone(), result);
            }
        }

        // Second pass: cross-field validation
        let cross_field_rules = T::get_cross_field_validators();
        for rule in cross_field_rules {
            let result = rule.validate(&form_data);
            results.add_cross_field_result(rule.name(), result);
        }

        results
    }
}
```

## Compiled Rule System

### CompiledRule (`cache.rs`)

```rust
#[derive(Clone)]
pub enum CompiledRule {
    Required,
    Email {
        regex: String, // Store regex pattern, compile lazily
    },
    Length {
        min: Option<usize>,
        max: Option<usize>
    },
    Pattern {
        pattern: String,
        flags: RegexFlags,
    },
    Custom {
        validator: Arc<dyn Fn(&FieldValue) -> ValidationResult + Send + Sync>,
    },
    Chain {
        rules: Vec<CompiledRule>,
        mode: ChainMode, // AllMustPass, FirstError, etc.
    },
}

impl CompiledRule {
    pub fn validate(
        &self,
        value: &FieldValue,
        regex_cache: &Arc<RwLock<HashMap<String, Regex>>>
    ) -> ValidationResult {
        match self {
            CompiledRule::Required => {
                if value.is_empty() {
                    Err(ValidationError::Required)
                } else {
                    Ok(())
                }
            },
            CompiledRule::Email { regex } => {
                let compiled_regex = {
                    let cache = regex_cache.read().unwrap();
                    if let Some(cached) = cache.get(regex) {
                        cached.clone()
                    } else {
                        drop(cache);
                        let compiled = Regex::new(regex)
                            .map_err(|_| ValidationError::Internal)?;
                        let mut cache = regex_cache.write().unwrap();
                        cache.insert(regex.clone(), compiled.clone());
                        compiled
                    }
                };

                if compiled_regex.is_match(&value.as_string()) {
                    Ok(())
                } else {
                    Err(ValidationError::Email)
                }
            },
            // ... other rule implementations
        }
    }
}
```

## Async Validation Support

### Async Engine (`async_validator.rs`)

```rust
use futures::future::BoxFuture;

#[async_trait(?Send)]
pub trait AsyncValidator {
    async fn validate(&self, value: &FieldValue) -> ValidationResult;

    /// Timeout for async validation (default: 5 seconds)
    fn timeout(&self) -> Duration {
        Duration::from_secs(5)
    }
}

pub struct AsyncValidationEngine {
    sync_engine: CachedValidationEngine,
    async_validators: HashMap<String, Box<dyn AsyncValidator>>,
}

impl AsyncValidationEngine {
    pub async fn validate_field_async(
        &self,
        field_name: &str,
        value: &FieldValue,
        rules: &[ValidationRule]
    ) -> ValidationResult {
        // First run sync validation (fast)
        let sync_result = self.sync_engine.validate_field(field_name, value, rules);
        if sync_result.is_err() {
            return sync_result; // Fail fast on sync validation
        }

        // Then run async validation if needed
        if let Some(async_validator) = self.async_validators.get(field_name) {
            let timeout_duration = async_validator.timeout();

            match timeout(timeout_duration, async_validator.validate(value)).await {
                Ok(result) => result,
                Err(_) => Err(ValidationError::Timeout),
            }
        } else {
            sync_result
        }
    }
}
```

## Cross-Field Validation

### Cross-Field Rules (`cross_field.rs`)

```rust
pub trait CrossFieldValidator {
    fn name(&self) -> &str;
    fn validate(&self, form_data: &HashMap<String, FieldValue>) -> ValidationResult;
}

// Example: Password confirmation
pub struct PasswordMatchValidator {
    password_field: String,
    confirm_field: String,
}

impl CrossFieldValidator for PasswordMatchValidator {
    fn name(&self) -> &str { "password_match" }

    fn validate(&self, form_data: &HashMap<String, FieldValue>) -> ValidationResult {
        let password = form_data.get(&self.password_field);
        let confirm = form_data.get(&self.confirm_field);

        match (password, confirm) {
            (Some(p), Some(c)) if p == c => Ok(()),
            (Some(_), Some(_)) => Err(ValidationError::PasswordMismatch),
            _ => Err(ValidationError::MissingFields),
        }
    }
}
```

## Performance Monitoring

### Metrics Collection (`performance.rs`)

```rust
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

pub struct ValidationMetrics {
    total_validations: AtomicUsize,
    total_time_ms: AtomicU64,
    cache_hits: AtomicUsize,
    cache_misses: AtomicUsize,
    error_count: AtomicUsize,
}

impl ValidationMetrics {
    pub fn record_validation(&self, field: &str, duration_ms: f64, success: bool) {
        self.total_validations.fetch_add(1, Ordering::Relaxed);
        self.total_time_ms.fetch_add(duration_ms as u64, Ordering::Relaxed);

        if !success {
            self.error_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn cache_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_stats(&self) -> ValidationStats {
        ValidationStats {
            total_validations: self.total_validations.load(Ordering::Relaxed),
            average_time_ms: self.total_time_ms.load(Ordering::Relaxed) as f64
                / self.total_validations.load(Ordering::Relaxed) as f64,
            cache_hit_rate: self.cache_hits.load(Ordering::Relaxed) as f64
                / (self.cache_hits.load(Ordering::Relaxed) + self.cache_misses.load(Ordering::Relaxed)) as f64,
            error_rate: self.error_count.load(Ordering::Relaxed) as f64
                / self.total_validations.load(Ordering::Relaxed) as f64,
        }
    }
}
```

## Usage Examples

### Basic Usage

```rust
let engine = CachedValidationEngine::new();

// Validation is cached after first call
let result1 = engine.validate_field("email", &email_value, &email_rules);
let result2 = engine.validate_field("email", &email_value, &email_rules); // Uses cache

// Form validation
let form_result = engine.validate_form(&my_form);
```

### Custom Validators

```rust
let custom_validator = CompiledRule::Custom {
    validator: Arc::new(|value| {
        if value.as_string().contains("banned_word") {
            Err(ValidationError::Custom("Contains banned content".to_string()))
        } else {
            Ok(())
        }
    }),
};

// Add to validation chain
let rules = vec![
    CompiledRule::Required,
    CompiledRule::Length { min: Some(5), max: Some(100) },
    custom_validator,
];
```

## Performance Targets

### Benchmarks

- **Simple validation** (required, length): <0.1ms
- **Complex validation** (regex, custom): <1ms
- **Form validation** (10 fields): <5ms
- **Cache hit**: <0.01ms
- **Memory usage**: <1MB for 1000 cached rules

### Optimization Strategies

- Compile validation rules once, reuse many times
- Cache regex compilation results
- Use atomic operations for metrics (low overhead)
- Minimize string allocations during validation
- Lazy loading of expensive validators

## Testing Strategy

### Performance Tests

```rust
#[cfg(test)]
mod perf_tests {
    #[test]
    fn validation_performance_under_load() {
        let engine = CachedValidationEngine::new();
        let start = std::time::Instant::now();

        for i in 0..1000 {
            let value = FieldValue::String(format!("test{}@example.com", i));
            engine.validate_field("email", &value, &email_rules);
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Validation too slow: {}ms", duration.as_millis());
    }
}
```

## Success Criteria

### Performance

- [ ] Validation rules cached and reused
- [ ] <1ms validation time for complex fields
- [ ] <5ms validation time for entire forms
- [ ] > 90% cache hit rate after warm-up
- [ ] Memory usage stable under load

### Architecture

- [ ] All files under 300 lines
- [ ] Clear separation of sync/async validation
- [ ] Extensible custom validator system
- [ ] Thread-safe caching implementation

### Functionality

- [ ] All existing validators work
- [ ] Cross-field validation support
- [ ] Async validation with timeout
- [ ] Performance metrics collection
- [ ] Comprehensive error handling
