# Performance Testing Plan - Leptos Forms
**Project**: Leptos Forms Library  
**Version**: 1.0  
**Date**: 2025-01-02  
**Status**: Performance Strategy  

## 1. Performance Testing Strategy

### 1.1 Performance Objectives
| Metric | Target | Threshold | Measurement Method |
|--------|--------|-----------|-------------------|
| Bundle Size | ≤15KB gzipped | 20KB gzipped | Webpack Bundle Analyzer |
| Field Update Latency | ≤1ms average | 2ms 95th percentile | JavaScript Performance API |
| Form Validation Time | ≤2ms average | 5ms 95th percentile | Custom benchmarking |
| Memory Usage | ≤200MB/100 forms | 300MB/100 forms | Browser Memory Profiler |
| Initial Load Time | ≤500ms | 1000ms | Lighthouse CI |
| Time to Interactive | ≤1000ms | 2000ms | Core Web Vitals |

### 1.2 Testing Pyramid for Performance
```
        /\
       /  \     Load Testing (10%)
      /____\    - Stress testing
     /      \   - Scalability limits
    /________\  - Resource exhaustion

      /\     Integration Performance (30%)
     /  \    - Component interactions
    /____\   - End-to-end workflows
   /______\  - Browser compatibility

       /\  Unit Performance (60%)
      /  \ - Function benchmarks
     /____\- Algorithm efficiency
    /______\- Memory allocation
   /________\- CPU utilization
```

## 2. Benchmark Suite Design

### 2.1 Core Performance Benchmarks

#### Form Creation Benchmarks
```rust
// benches/form_creation.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use leptos_forms::*;

fn bench_form_creation(c: &mut Criterion) {
    c.bench_function("form_creation_simple", |b| {
        b.iter(|| {
            let form = use_form::<SimpleForm>(
                black_box(None),
                black_box(FormOptions::default())
            );
            black_box(form)
        });
    });
    
    c.bench_function("form_creation_complex", |b| {
        b.iter(|| {
            let form = use_form::<ComplexForm>(
                black_box(None),
                black_box(FormOptions::default())
            );
            black_box(form)
        });
    });
    
    c.bench_function("form_creation_with_100_fields", |b| {
        b.iter(|| {
            let form = use_form::<MassiveForm>(
                black_box(None),
                black_box(FormOptions::default())
            );
            black_box(form)
        });
    });
}

fn bench_field_registration(c: &mut Criterion) {
    let form = use_form::<SimpleForm>(None, FormOptions::default());
    
    c.bench_function("field_registration", |b| {
        b.iter_batched(
            || format!("field_{}", fastrand::u32(0..1000)),
            |field_name| {
                let registration = form.register.call(black_box(field_name));
                black_box(registration)
            },
            BatchSize::SmallInput
        );
    });
    
    c.bench_function("bulk_field_registration", |b| {
        b.iter(|| {
            for i in 0..100 {
                let field_name = format!("field_{}", i);
                let registration = form.register.call(black_box(field_name));
                black_box(registration);
            }
        });
    });
}

criterion_group!(form_benches, bench_form_creation, bench_field_registration);
criterion_main!(form_benches);
```

#### Field Update Benchmarks
```rust
// benches/field_updates.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leptos_forms::*;

fn bench_field_updates(c: &mut Criterion) {
    let form = use_form::<TestForm>(None, FormOptions::default());
    
    c.bench_function("single_field_update", |b| {
        b.iter(|| {
            form.set_field_value.call(black_box((
                "test_field".to_string(),
                FieldValue::String("test_value".to_string())
            )));
        });
    });
    
    c.bench_function("multiple_field_updates", |b| {
        b.iter(|| {
            for i in 0..10 {
                form.set_field_value.call(black_box((
                    format!("field_{}", i),
                    FieldValue::String(format!("value_{}", i))
                )));
            }
        });
    });
    
    c.bench_function("rapid_field_updates", |b| {
        b.iter(|| {
            // Simulate rapid typing
            let base_value = "test_value";
            for i in 0..20 {
                let value = format!("{}{}", base_value, "x".repeat(i));
                form.set_field_value.call(black_box((
                    "rapid_field".to_string(),
                    FieldValue::String(value)
                )));
            }
        });
    });
}

fn bench_field_value_conversion(c: &mut Criterion) {
    c.bench_function("string_to_field_value", |b| {
        b.iter(|| {
            let value = FieldValue::String(black_box("test string".to_string()));
            black_box(value)
        });
    });
    
    c.bench_function("number_to_field_value", |b| {
        b.iter(|| {
            let value = FieldValue::Number(black_box(42.5));
            black_box(value)
        });
    });
    
    c.bench_function("complex_object_to_field_value", |b| {
        b.iter(|| {
            let mut object = std::collections::HashMap::new();
            object.insert("key1".to_string(), FieldValue::String("value1".to_string()));
            object.insert("key2".to_string(), FieldValue::Number(123.0));
            let value = FieldValue::Object(black_box(object));
            black_box(value)
        });
    });
}

criterion_group!(field_benches, bench_field_updates, bench_field_value_conversion);
criterion_main!(field_benches);
```

#### Validation Benchmarks
```rust
// benches/validation.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leptos_forms::*;

fn bench_validation(c: &mut Criterion) {
    let valid_form = create_valid_test_form();
    let invalid_form = create_invalid_test_form();
    
    c.bench_function("validate_valid_form", |b| {
        b.iter(|| {
            let result = black_box(&valid_form).validate();
            black_box(result)
        });
    });
    
    c.bench_function("validate_invalid_form", |b| {
        b.iter(|| {
            let result = black_box(&invalid_form).validate();
            black_box(result)
        });
    });
    
    c.bench_function("validate_large_form", |b| {
        let large_form = create_form_with_100_fields();
        b.iter(|| {
            let result = black_box(&large_form).validate();
            black_box(result)
        });
    });
}

fn bench_individual_validators(c: &mut Criterion) {
    c.bench_function("email_validator_valid", |b| {
        let email = FieldValue::String("test@example.com".to_string());
        b.iter(|| {
            let result = validators::email(black_box(&email));
            black_box(result)
        });
    });
    
    c.bench_function("email_validator_invalid", |b| {
        let email = FieldValue::String("invalid-email".to_string());
        b.iter(|| {
            let result = validators::email(black_box(&email));
            black_box(result)
        });
    });
    
    c.bench_function("regex_validator", |b| {
        let value = FieldValue::String("abc123".to_string());
        let pattern_validator = validators::pattern(r"^[a-z]+\d+$").unwrap();
        b.iter(|| {
            let result = pattern_validator(black_box(&value));
            black_box(result)
        });
    });
    
    c.bench_function("custom_validator", |b| {
        let value = FieldValue::String("test_string".to_string());
        let custom_validator = validators::custom(|v: &FieldValue| {
            match v {
                FieldValue::String(s) if s.len() > 5 => Ok(()),
                _ => Err("String too short".to_string()),
            }
        });
        b.iter(|| {
            let result = custom_validator(black_box(&value));
            black_box(result)
        });
    });
}

criterion_group!(validation_benches, bench_validation, bench_individual_validators);
criterion_main!(validation_benches);
```

### 2.2 Cache Performance Benchmarks

```rust
// benches/cache_performance.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use leptos_forms::*;
use tokio::runtime::Runtime;

fn bench_cache_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let cache = FormCache::new(CacheConfig::default());
    
    c.bench_function("cache_store_small_form", |b| {
        b.to_async(&rt).iter_batched(
            || (FormId::new("test_form"), create_small_form_state()),
            |(form_id, form_state)| async {
                cache.store_form_state(&form_id, &form_state).await.unwrap();
            },
            BatchSize::SmallInput
        );
    });
    
    c.bench_function("cache_retrieve_existing", |b| {
        // Pre-populate cache
        rt.block_on(async {
            let form_state = create_small_form_state();
            cache.store_form_state(&FormId::new("existing_form"), &form_state).await.unwrap();
        });
        
        b.to_async(&rt).iter(|| async {
            let result = cache.get_form_state(&FormId::new("existing_form")).await.unwrap();
            black_box(result)
        });
    });
    
    c.bench_function("cache_miss", |b| {
        b.to_async(&rt).iter(|| async {
            let result = cache.get_form_state(&FormId::new("nonexistent")).await.unwrap();
            black_box(result)
        });
    });
}

fn bench_cache_eviction(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("lru_eviction_under_pressure", |b| {
        b.iter_batched(
            || {
                let cache = FormCache::new(CacheConfig {
                    max_forms: 10,
                    max_memory: 1024 * 1024, // 1MB
                    validation_ttl: std::time::Duration::from_secs(300),
                });
                cache
            },
            |cache| {
                rt.block_on(async {
                    // Fill cache beyond capacity
                    for i in 0..20 {
                        let form_state = create_small_form_state();
                        let form_id = FormId::new(&format!("form_{}", i));
                        cache.store_form_state(&form_id, &form_state).await.unwrap();
                    }
                    
                    // Verify eviction occurred
                    let result = cache.get_form_state(&FormId::new("form_0")).await.unwrap();
                    black_box(result)
                });
            },
            BatchSize::LargeInput
        );
    });
}

criterion_group!(cache_benches, bench_cache_operations, bench_cache_eviction);
criterion_main!(cache_benches);
```

## 3. Memory Usage Testing

### 3.1 Memory Leak Detection
```rust
// tests/memory_tests.rs
use std::mem;
use leptos_forms::*;

#[cfg(test)]
mod memory_tests {
    use super::*;
    
    #[test]
    fn test_form_handle_memory_footprint() {
        let form = use_form::<SimpleForm>(None, FormOptions::default());
        let size = mem::size_of_val(&form);
        
        // FormHandle should have reasonable memory footprint
        assert!(size < 1024, "FormHandle size: {} bytes", size);
        
        println!("FormHandle memory footprint: {} bytes", size);
    }
    
    #[test]
    fn test_field_registration_memory() {
        let form = use_form::<SimpleForm>(None, FormOptions::default());
        let field = form.register.call("test_field".to_string());
        let size = mem::size_of_val(&field);
        
        assert!(size < 512, "FieldRegistration size: {} bytes", size);
        
        println!("FieldRegistration memory footprint: {} bytes", size);
    }
    
    #[test]
    fn test_memory_scaling() {
        let initial_memory = get_current_memory_usage();
        
        let mut forms = Vec::new();
        for i in 0..100 {
            let form = use_form::<SimpleForm>(None, FormOptions::default());
            // Register 10 fields per form
            for j in 0..10 {
                let field_name = format!("field_{}_{}", i, j);
                form.register.call(field_name);
            }
            forms.push(form);
        }
        
        let final_memory = get_current_memory_usage();
        let memory_per_form = (final_memory - initial_memory) / 100;
        
        println!("Memory per form (10 fields): {} bytes", memory_per_form);
        
        // Should scale reasonably
        assert!(memory_per_form < 50_000, "Memory per form: {} bytes", memory_per_form);
    }
    
    #[test]
    fn test_form_cleanup() {
        let initial_memory = get_current_memory_usage();
        
        {
            let _forms: Vec<_> = (0..100).map(|_| {
                use_form::<SimpleForm>(None, FormOptions::default())
            }).collect();
            
            // Forms exist in this scope
        } // Forms dropped here
        
        // Force garbage collection
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        let final_memory = get_current_memory_usage();
        let memory_growth = final_memory - initial_memory;
        
        // Should have minimal memory growth after cleanup
        assert!(memory_growth < 10_000, "Memory growth after cleanup: {} bytes", memory_growth);
    }
    
    fn get_current_memory_usage() -> usize {
        // Platform-specific memory usage measurement
        #[cfg(target_arch = "wasm32")]
        {
            // WASM memory pages * page size
            web_sys::js_sys::WebAssembly::Memory::new(
                &web_sys::js_sys::WebAssembly::MemoryDescriptor::new(1)
            ).unwrap().buffer().byte_length() as usize
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Use platform-specific memory measurement
            0 // Placeholder - implement based on platform
        }
    }
}
```

### 3.2 Memory Pressure Testing
```javascript
// tests/memory-pressure.js
// Browser-based memory pressure testing

async function testMemoryPressure() {
    const forms = [];
    const memoryGrowthLimit = 100 * 1024 * 1024; // 100MB
    
    let initialMemory = await measureMemoryUsage();
    
    try {
        // Create forms until memory pressure
        for (let i = 0; i < 1000; i++) {
            const form = createComplexForm();
            
            // Register many fields
            for (let j = 0; j < 50; j++) {
                form.register(`field_${i}_${j}`);
            }
            
            forms.push(form);
            
            // Check memory every 10 forms
            if (i % 10 === 0) {
                const currentMemory = await measureMemoryUsage();
                const memoryGrowth = currentMemory - initialMemory;
                
                console.log(`Forms: ${i}, Memory growth: ${memoryGrowth / 1024 / 1024}MB`);
                
                if (memoryGrowth > memoryGrowthLimit) {
                    console.log(`Memory limit reached at ${i} forms`);
                    break;
                }
            }
        }
        
    } catch (error) {
        console.error('Memory pressure test failed:', error);
    } finally {
        // Cleanup
        forms.length = 0;
        
        // Force garbage collection
        if (window.gc) {
            window.gc();
        }
        
        // Measure final memory
        const finalMemory = await measureMemoryUsage();
        console.log(`Final memory usage: ${finalMemory / 1024 / 1024}MB`);
    }
}

async function measureMemoryUsage() {
    if ('memory' in performance) {
        return performance.memory.usedJSHeapSize;
    } else {
        // Fallback measurement
        return 0;
    }
}
```

## 4. Load Testing and Stress Testing

### 4.1 Concurrent User Simulation
```rust
// tests/load_tests.rs
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use leptos_forms::*;

#[tokio::test]
async fn test_concurrent_form_operations() {
    let num_concurrent_users = 100;
    let operations_per_user = 50;
    
    let mut handles = Vec::new();
    
    for user_id in 0..num_concurrent_users {
        let handle = tokio::spawn(async move {
            let form = use_form::<TestForm>(None, FormOptions::default());
            
            // Simulate user interactions
            for i in 0..operations_per_user {
                // Random field updates
                let field_name = format!("field_{}", i % 10);
                let value = FieldValue::String(format!("user_{}_value_{}", user_id, i));
                
                form.set_field_value.call((field_name, value));
                
                // Validate occasionally
                if i % 10 == 0 {
                    let _ = form.values.get().validate();
                }
                
                // Small delay to simulate thinking time
                sleep(Duration::from_millis(1)).await;
            }
            
            // Final form submission
            form.submit.call(());
        });
        
        handles.push(handle);
    }
    
    // Wait for all users to complete
    let start_time = std::time::Instant::now();
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    let total_time = start_time.elapsed();
    let operations_per_second = (num_concurrent_users * operations_per_user) as f64 
        / total_time.as_secs_f64();
    
    println!("Concurrent load test completed:");
    println!("  Users: {}", num_concurrent_users);
    println!("  Operations per user: {}", operations_per_user);
    println!("  Total time: {:?}", total_time);
    println!("  Operations per second: {:.2}", operations_per_second);
    
    // Performance assertions
    assert!(operations_per_second > 1000.0, "Performance below threshold");
    assert!(total_time < Duration::from_secs(30), "Test took too long");
}
```

### 4.2 Stress Testing
```bash
#!/bin/bash
# scripts/stress-test.sh

echo "🔥 Running stress tests..."

# Memory stress test
echo "📊 Memory stress test..."
node tests/memory-stress.js

# CPU stress test
echo "⚡ CPU stress test..."
cargo test --release test_cpu_intensive_operations

# Concurrent access stress test
echo "👥 Concurrent access stress test..."
cargo test --release test_concurrent_form_operations

# Long-running stability test
echo "⏰ Long-running stability test..."
timeout 300s cargo test --release test_long_running_stability

# Resource exhaustion test
echo "💾 Resource exhaustion test..."
cargo test --release test_resource_exhaustion

echo "✅ Stress tests completed!"
```

## 5. Browser Performance Testing

### 5.1 Core Web Vitals Monitoring
```javascript
// tests/web-vitals.js
import { getCLS, getFID, getFCP, getLCP, getTTFB } from 'web-vitals';

function measureWebVitals() {
    const metrics = {};
    
    getCLS(metric => {
        metrics.cls = metric.value;
        console.log('Cumulative Layout Shift:', metric.value);
        
        // Assert CLS < 0.1 (good)
        if (metric.value > 0.1) {
            console.warn('CLS above threshold:', metric.value);
        }
    });
    
    getFID(metric => {
        metrics.fid = metric.value;
        console.log('First Input Delay:', metric.value);
        
        // Assert FID < 100ms (good)
        if (metric.value > 100) {
            console.warn('FID above threshold:', metric.value);
        }
    });
    
    getFCP(metric => {
        metrics.fcp = metric.value;
        console.log('First Contentful Paint:', metric.value);
        
        // Assert FCP < 1.8s (good)
        if (metric.value > 1800) {
            console.warn('FCP above threshold:', metric.value);
        }
    });
    
    getLCP(metric => {
        metrics.lcp = metric.value;
        console.log('Largest Contentful Paint:', metric.value);
        
        // Assert LCP < 2.5s (good)
        if (metric.value > 2500) {
            console.warn('LCP above threshold:', metric.value);
        }
    });
    
    getTTFB(metric => {
        metrics.ttfb = metric.value;
        console.log('Time to First Byte:', metric.value);
        
        // Assert TTFB < 800ms (good)
        if (metric.value > 800) {
            console.warn('TTFB above threshold:', metric.value);
        }
    });
    
    return metrics;
}
```

### 5.2 Performance Observer Integration
```javascript
// tests/performance-observer.js
function setupPerformanceMonitoring() {
    // Monitor long tasks
    if ('PerformanceObserver' in window) {
        const longTaskObserver = new PerformanceObserver(list => {
            list.getEntries().forEach(entry => {
                if (entry.duration > 50) { // Tasks > 50ms
                    console.warn('Long task detected:', {
                        duration: entry.duration,
                        startTime: entry.startTime,
                        name: entry.name
                    });
                }
            });
        });
        
        longTaskObserver.observe({ entryTypes: ['longtask'] });
        
        // Monitor layout shifts
        const layoutShiftObserver = new PerformanceObserver(list => {
            list.getEntries().forEach(entry => {
                if (entry.value > 0.1) { // Significant layout shifts
                    console.warn('Layout shift detected:', {
                        value: entry.value,
                        startTime: entry.startTime,
                        sources: entry.sources
                    });
                }
            });
        });
        
        layoutShiftObserver.observe({ entryTypes: ['layout-shift'] });
        
        // Monitor navigation timing
        const navigationObserver = new PerformanceObserver(list => {
            list.getEntries().forEach(entry => {
                const loadTime = entry.loadEventEnd - entry.fetchStart;
                console.log('Page load time:', loadTime);
                
                if (loadTime > 3000) { // > 3 seconds
                    console.warn('Slow page load:', loadTime);
                }
            });
        });
        
        navigationObserver.observe({ entryTypes: ['navigation'] });
    }
}
```

## 6. Performance Regression Testing

### 6.1 Automated Performance CI
```yaml
# .github/workflows/performance.yml
name: Performance Testing

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  performance-benchmarks:
    name: Performance Benchmarks
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        
      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-perf-cargo-${{ hashFiles('**/Cargo.lock') }}
          
      - name: Run performance benchmarks
        run: |
          cargo bench --bench form_creation > bench-results.txt
          cargo bench --bench field_updates >> bench-results.txt
          cargo bench --bench validation >> bench-results.txt
          
      - name: Store benchmark results
        uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: 'cargo'
          output-file-path: bench-results.txt
          github-token: ${{ secrets.GITHUB_TOKEN }}
          auto-push: true
          comment-on-alert: true
          alert-threshold: '110%'  # Alert if 10% slower
          fail-on-alert: true
          
      - name: Bundle size check
        run: |
          wasm-pack build --release --target web
          BUNDLE_SIZE=$(wc -c < pkg/leptos_forms_bg.wasm)
          echo "Bundle size: $BUNDLE_SIZE bytes"
          
          # Check against target (15KB = 15360 bytes)
          if [ $BUNDLE_SIZE -gt 15360 ]; then
            echo "❌ Bundle size $BUNDLE_SIZE exceeds 15KB target"
            exit 1
          fi
          
          # Store bundle size for trending
          echo "$BUNDLE_SIZE" > bundle-size.txt
          
      - name: Upload performance artifacts
        uses: actions/upload-artifact@v3
        with:
          name: performance-results
          path: |
            bench-results.txt
            bundle-size.txt
```

### 6.2 Performance Regression Detection
```rust
// tests/performance_regression.rs
use std::time::{Duration, Instant};
use leptos_forms::*;

#[cfg(test)]
mod performance_regression_tests {
    use super::*;
    
    #[test]
    fn test_field_update_performance_regression() {
        let form = use_form::<SimpleForm>(None, FormOptions::default());
        let mut durations = Vec::new();
        
        // Warm up
        for _ in 0..10 {
            form.set_field_value.call(("warm_up".to_string(), FieldValue::String("value".to_string())));
        }
        
        // Measure performance
        for i in 0..100 {
            let start = Instant::now();
            form.set_field_value.call((
                format!("field_{}", i),
                FieldValue::String(format!("value_{}", i))
            ));
            durations.push(start.elapsed());
        }
        
        let avg_duration = durations.iter().sum::<Duration>() / durations.len() as u32;
        let max_duration = durations.iter().max().unwrap();
        let p95_duration = {
            let mut sorted = durations.clone();
            sorted.sort();
            sorted[(sorted.len() * 95 / 100).min(sorted.len() - 1)]
        };
        
        println!("Field update performance:");
        println!("  Average: {:?}", avg_duration);
        println!("  Maximum: {:?}", max_duration);
        println!("  95th percentile: {:?}", p95_duration);
        
        // Performance assertions
        assert!(avg_duration < Duration::from_millis(1), "Average field update too slow: {:?}", avg_duration);
        assert!(p95_duration < Duration::from_millis(2), "95th percentile too slow: {:?}", p95_duration);
        assert!(max_duration < Duration::from_millis(5), "Maximum field update too slow: {:?}", max_duration);
    }
    
    #[test]
    fn test_validation_performance_regression() {
        let form_data = create_complex_form_data();
        let mut durations = Vec::new();
        
        // Measure validation performance
        for _ in 0..1000 {
            let start = Instant::now();
            let _ = form_data.validate();
            durations.push(start.elapsed());
        }
        
        let avg_duration = durations.iter().sum::<Duration>() / durations.len() as u32;
        let p95_duration = {
            let mut sorted = durations.clone();
            sorted.sort();
            sorted[(sorted.len() * 95 / 100).min(sorted.len() - 1)]
        };
        
        println!("Validation performance:");
        println!("  Average: {:?}", avg_duration);
        println!("  95th percentile: {:?}", p95_duration);
        
        // Performance assertions
        assert!(avg_duration < Duration::from_millis(2), "Average validation too slow: {:?}", avg_duration);
        assert!(p95_duration < Duration::from_millis(5), "95th percentile validation too slow: {:?}", p95_duration);
    }
}
```

## 7. Performance Monitoring in Production

### 7.1 Real User Monitoring (RUM)
```javascript
// src/monitoring/rum.js
class FormPerformanceMonitor {
    constructor() {
        this.metrics = new Map();
        this.setupObservers();
    }
    
    setupObservers() {
        // Monitor form operations
        this.observeFormOperations();
        
        // Monitor memory usage
        this.observeMemoryUsage();
        
        // Monitor user interactions
        this.observeUserInteractions();
    }
    
    observeFormOperations() {
        const originalSetFieldValue = FormHandle.prototype.setFieldValue;
        
        FormHandle.prototype.setFieldValue = function(fieldName, value) {
            const start = performance.now();
            const result = originalSetFieldValue.call(this, fieldName, value);
            const duration = performance.now() - start;
            
            this.recordMetric('field_update_duration', duration);
            
            return result;
        };
    }
    
    observeMemoryUsage() {
        if ('memory' in performance) {
            setInterval(() => {
                this.recordMetric('memory_usage', performance.memory.usedJSHeapSize);
            }, 10000); // Every 10 seconds
        }
    }
    
    observeUserInteractions() {
        document.addEventListener('input', (event) => {
            if (event.target.hasAttribute('data-form-field')) {
                this.recordMetric('user_input_frequency', 1);
            }
        });
    }
    
    recordMetric(name, value) {
        if (!this.metrics.has(name)) {
            this.metrics.set(name, []);
        }
        
        this.metrics.get(name).push({
            value,
            timestamp: Date.now()
        });
        
        // Limit metric history
        const metrics = this.metrics.get(name);
        if (metrics.length > 1000) {
            metrics.splice(0, metrics.length - 1000);
        }
        
        this.reportIfNeeded(name, value);
    }
    
    reportIfNeeded(name, value) {
        // Report performance issues
        if (name === 'field_update_duration' && value > 10) {
            this.reportSlowFieldUpdate(value);
        }
        
        if (name === 'memory_usage' && value > 100 * 1024 * 1024) { // 100MB
            this.reportHighMemoryUsage(value);
        }
    }
    
    reportSlowFieldUpdate(duration) {
        if (this.canReport('slow_field_update')) {
            console.warn('Slow field update detected:', duration);
            
            // Send to analytics service
            this.sendMetric('performance_issue', {
                type: 'slow_field_update',
                duration,
                userAgent: navigator.userAgent,
                url: location.href
            });
        }
    }
    
    reportHighMemoryUsage(usage) {
        if (this.canReport('high_memory_usage')) {
            console.warn('High memory usage detected:', usage);
            
            this.sendMetric('performance_issue', {
                type: 'high_memory_usage',
                usage,
                userAgent: navigator.userAgent,
                url: location.href
            });
        }
    }
    
    canReport(type) {
        const lastReport = this.lastReported?.get(type);
        const now = Date.now();
        
        if (!lastReport || now - lastReport > 60000) { // 1 minute throttle
            if (!this.lastReported) this.lastReported = new Map();
            this.lastReported.set(type, now);
            return true;
        }
        
        return false;
    }
    
    async sendMetric(event, data) {
        try {
            await fetch('/api/metrics', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ event, data, timestamp: Date.now() })
            });
        } catch (error) {
            console.error('Failed to send metric:', error);
        }
    }
    
    getMetricSummary(name) {
        const values = this.metrics.get(name) || [];
        if (values.length === 0) return null;
        
        const sorted = values.map(v => v.value).sort((a, b) => a - b);
        
        return {
            count: values.length,
            min: sorted[0],
            max: sorted[sorted.length - 1],
            mean: sorted.reduce((sum, v) => sum + v, 0) / sorted.length,
            p50: sorted[Math.floor(sorted.length * 0.5)],
            p95: sorted[Math.floor(sorted.length * 0.95)],
            p99: sorted[Math.floor(sorted.length * 0.99)]
        };
    }
}

// Initialize monitoring
const formMonitor = new FormPerformanceMonitor();
```

This comprehensive performance testing plan ensures the Leptos Forms library meets all performance targets through systematic benchmarking, regression testing, and real-world monitoring.

---

**Document Control**
- **Created**: 2025-01-02
- **Last Modified**: 2025-01-02
- **Next Review**: Monthly during development
- **Version**: 1.0
- **Classification**: Performance Strategy