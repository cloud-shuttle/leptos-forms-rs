# ADR-003: Cache Storage Strategy (in-memory vs persistent)

**Status**: Accepted
**Date**: 2025-01-02
**Deciders**: Architecture Team
**Technical Story**: Choose storage strategy for form state caching and persistence

## Context

Forms require various types of data storage for optimal user experience:

- **Transient State**: Field values, validation errors, touch/dirty state
- **Draft Persistence**: Auto-save functionality, recovery after refresh
- **Performance Cache**: Validation results, computed values, expensive operations
- **User Preferences**: Form configuration, validation settings
- **Offline Support**: Form data when network is unavailable

## Decision Drivers

- **Performance**: Access speed, memory efficiency, cache hit rates
- **Persistence**: Data survival across sessions, browser restarts
- **Capacity**: Storage limits, garbage collection
- **Security**: Sensitive data handling, encryption requirements
- **User Experience**: Seamless recovery, offline functionality

## Storage Hierarchy Strategy

### Tier 1: In-Memory (Primary) ✅

```rust
/// High-speed in-memory cache for active forms
pub struct MemoryCache<T> {
    /// Form state cache with LRU eviction
    states: LruCache<FormId, FormState<T>>,
    /// Validation results cache
    validations: LruCache<ValidationKey, ValidationResult>,
    /// Computed values cache
    computed: LruCache<ComputationKey, ComputedValue>,
    /// Configuration
    config: CacheConfig,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum memory usage (default: 50MB)
    max_memory: usize,
    /// Maximum cached forms (default: 100)
    max_forms: usize,
    /// Validation cache TTL (default: 5 minutes)
    validation_ttl: Duration,
}
```

**Use Cases:**

- Active form state management
- Real-time validation caching
- Computed property memoization
- Component render optimization

**Advantages:**

- Fastest possible access (nanosecond retrieval)
- Zero serialization overhead
- Perfect for reactive updates
- Automatic cleanup with LRU

**Limitations:**

- Lost on page refresh
- Memory usage grows with form complexity
- Limited by available RAM

### Tier 2: Session Storage (Secondary) ✅

```rust
/// Session-based persistence for current tab
pub struct SessionCache<T> {
    storage: web_sys::Storage,
    key_prefix: String,
    serialization_format: SerializationFormat,
}

impl<T> SessionCache<T>
where
    T: Serialize + DeserializeOwned
{
    pub async fn store_form(&self, id: &FormId, form: &FormState<T>) -> Result<(), CacheError> {
        let key = format!("{}_form_{}", self.key_prefix, id);
        let data = form.serialize(self.serialization_format)?;
        self.storage.set_item(&key, &base64::encode(data))?;
        Ok(())
    }
}
```

**Use Cases:**

- Auto-save functionality
- Form recovery after accidental refresh
- Tab-specific form state
- Temporary draft storage

**Advantages:**

- Survives page refresh
- Automatic cleanup on tab close
- ~5-10MB storage capacity
- Synchronous API

**Limitations:**

- Lost when tab closes
- Limited storage space
- Single-tab scope

### Tier 3: Local Storage (Tertiary) ✅

```rust
/// Persistent local storage for user preferences and drafts
pub struct LocalCache<T> {
    storage: web_sys::Storage,
    key_prefix: String,
    encryption: Option<EncryptionKey>,
}

impl<T> LocalCache<T> {
    pub async fn store_draft(&self, id: &FormId, form: &FormState<T>) -> Result<(), CacheError> {
        let key = format!("{}_draft_{}", self.key_prefix, id);
        let mut data = form.serialize(SerializationFormat::Json)?;

        // Encrypt sensitive data if configured
        if let Some(key) = &self.encryption {
            data = key.encrypt(&data)?;
        }

        self.storage.set_item(&key, &base64::encode(data))?;
        Ok(())
    }
}
```

**Use Cases:**

- Long-term draft persistence
- User preferences storage
- Form templates and defaults
- Cross-session continuity

**Advantages:**

- Survives browser restart
- ~5-10MB capacity per origin
- Cross-tab accessibility
- User-controlled retention

**Limitations:**

- Synchronous API (blocking)
- Limited storage space
- Vulnerable to user clearing

### Tier 4: IndexedDB (Advanced) ✅

```rust
/// Advanced persistent storage for large forms and offline support
pub struct IndexedDBCache<T> {
    db: IdbDatabase,
    store_name: String,
    version: u32,
}

impl<T> IndexedDBCache<T> {
    pub async fn store_large_form(&self, id: &FormId, form: &FormState<T>) -> Result<(), CacheError> {
        let transaction = self.db
            .transaction_on_one_with_mode(&self.store_name, IdbTransactionMode::Readwrite)?;
        let store = transaction.object_store(&self.store_name)?;

        let data = form.serialize(SerializationFormat::Bincode)?;
        let js_value = serde_wasm_bindgen::to_value(&CacheEntry {
            id: id.clone(),
            data,
            created_at: js_sys::Date::now(),
            expires_at: js_sys::Date::now() + (7 * 24 * 60 * 60 * 1000.0), // 7 days
        })?;

        store.put_key_val(&id.to_js_value(), &js_value)?;
        transaction.await.into_result()?;
        Ok(())
    }
}
```

**Use Cases:**

- Large form data (>1MB)
- Offline form support
- Complex form attachments
- Long-term data archival

**Advantages:**

- Large storage capacity (>1GB)
- Asynchronous API (non-blocking)
- Transactional consistency
- Advanced indexing and querying

**Limitations:**

- Complex API
- Async overhead for small operations
- Browser support variations

## Unified Cache Interface

```rust
/// Unified caching interface that coordinates all storage tiers
pub struct FormCache<T> {
    memory: MemoryCache<T>,
    session: SessionCache<T>,
    local: LocalCache<T>,
    indexed_db: Option<IndexedDBCache<T>>,
    config: CacheConfiguration,
}

impl<T> FormCache<T>
where
    T: Form + Serialize + DeserializeOwned + Clone
{
    /// Store form state with automatic tier selection
    pub async fn store_form_state(&self, id: &FormId, form: &FormState<T>) -> Result<(), CacheError> {
        // Always cache in memory for immediate access
        self.memory.store(id, form.clone()).await?;

        // Determine persistence tier based on form characteristics
        let persistence_tier = self.determine_persistence_tier(form);

        match persistence_tier {
            PersistenceTier::Session => {
                self.session.store_form(id, form).await?;
            }
            PersistenceTier::Local => {
                self.session.store_form(id, form).await?;
                self.local.store_draft(id, form).await?;
            }
            PersistenceTier::IndexedDB => {
                if let Some(idb) = &self.indexed_db {
                    idb.store_large_form(id, form).await?;
                }
            }
            PersistenceTier::Memory => {
                // Memory only - no persistence
            }
        }

        Ok(())
    }

    /// Retrieve form state with tier fallback
    pub async fn get_form_state(&self, id: &FormId) -> Result<Option<FormState<T>>, CacheError> {
        // Try memory cache first (fastest)
        if let Some(form) = self.memory.get(id).await? {
            return Ok(Some(form));
        }

        // Try session storage
        if let Some(form) = self.session.get_form(id).await? {
            // Warm memory cache for future access
            self.memory.store(id, form.clone()).await?;
            return Ok(Some(form));
        }

        // Try local storage
        if let Some(form) = self.local.get_draft(id).await? {
            // Warm both memory and session caches
            self.memory.store(id, form.clone()).await?;
            self.session.store_form(id, &form).await?;
            return Ok(Some(form));
        }

        // Try IndexedDB
        if let Some(idb) = &self.indexed_db {
            if let Some(form) = idb.get_large_form(id).await? {
                // Warm memory cache only (too large for session storage)
                self.memory.store(id, form.clone()).await?;
                return Ok(Some(form));
            }
        }

        Ok(None)
    }

    /// Determine optimal persistence tier based on form characteristics
    fn determine_persistence_tier(&self, form: &FormState<T>) -> PersistenceTier {
        let serialized_size = form.estimate_serialized_size();

        match serialized_size {
            size if size > 1_000_000 => PersistenceTier::IndexedDB,  // >1MB
            size if size > 100_000 => PersistenceTier::Local,        // >100KB
            size if size > 10_000 => PersistenceTier::Session,       // >10KB
            _ => PersistenceTier::Memory,                             // <10KB
        }
    }
}

#[derive(Debug, Clone)]
pub enum PersistenceTier {
    Memory,
    Session,
    Local,
    IndexedDB,
}
```

## Cache Eviction Strategy

```rust
/// Intelligent cache eviction based on usage patterns
pub struct CacheEvictionManager<T> {
    /// Track access patterns for smart eviction
    access_tracker: AccessTracker,
    /// Memory pressure detector
    memory_monitor: MemoryMonitor,
}

impl<T> CacheEvictionManager<T> {
    /// Evict caches based on memory pressure and access patterns
    pub async fn evict_if_needed(&self, cache: &mut FormCache<T>) -> Result<(), CacheError> {
        let memory_pressure = self.memory_monitor.get_pressure_level();

        match memory_pressure {
            MemoryPressure::Low => {
                // No action needed
            }
            MemoryPressure::Medium => {
                // Evict least recently used items from memory cache
                cache.memory.evict_lru(0.25).await?; // Remove 25% of items
            }
            MemoryPressure::High => {
                // Aggressive eviction
                cache.memory.evict_lru(0.5).await?; // Remove 50% of items
                cache.session.cleanup_expired().await?;
            }
            MemoryPressure::Critical => {
                // Emergency cleanup
                cache.memory.clear().await?;
                cache.session.cleanup_all().await?;
            }
        }

        Ok(())
    }
}
```

## Security Considerations

```rust
/// Security layer for sensitive form data
pub struct SecureCache<T> {
    inner: FormCache<T>,
    encryption: EncryptionManager,
    security_policy: SecurityPolicy,
}

#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    /// Fields that should never be cached persistently
    sensitive_fields: HashSet<String>,
    /// Enable encryption for local/IndexedDB storage
    encrypt_persistent: bool,
    /// Auto-clear sensitive data after timeout
    auto_clear_timeout: Option<Duration>,
}

impl<T> SecureCache<T> {
    pub async fn store_form_state(&self, id: &FormId, form: &FormState<T>) -> Result<(), CacheError> {
        // Filter sensitive fields before caching
        let filtered_form = self.filter_sensitive_fields(form)?;

        // Store with appropriate security measures
        self.inner.store_form_state(id, &filtered_form).await
    }

    fn filter_sensitive_fields(&self, form: &FormState<T>) -> Result<FormState<T>, CacheError> {
        let mut filtered = form.clone();

        for sensitive_field in &self.security_policy.sensitive_fields {
            filtered.clear_field(sensitive_field);
        }

        Ok(filtered)
    }
}
```

## Performance Optimization

### Cache Warming Strategy

```rust
/// Proactive cache warming for better performance
pub struct CacheWarmer<T> {
    cache: Arc<FormCache<T>>,
    predictor: AccessPredictor,
}

impl<T> CacheWarmer<T> {
    /// Warm cache based on usage patterns
    pub async fn warm_likely_forms(&self) -> Result<(), CacheError> {
        let likely_forms = self.predictor.predict_next_access().await?;

        for form_id in likely_forms {
            // Pre-load into memory cache
            if let Some(form) = self.cache.get_form_state(&form_id).await? {
                self.cache.memory.store(&form_id, form).await?;
            }
        }

        Ok(())
    }
}
```

### Compression Strategy

```rust
/// Compression layer for storage optimization
pub struct CompressedCache<T> {
    inner: FormCache<T>,
    compression: CompressionAlgorithm,
}

impl<T> CompressedCache<T> {
    pub async fn store_form_state(&self, id: &FormId, form: &FormState<T>) -> Result<(), CacheError> {
        // Compress before storing to persistent layers
        let compressed = self.compression.compress(form)?;
        self.inner.store_form_state(id, &compressed).await
    }
}
```

## Decision Consequences

### Positive

- **Optimal Performance**: Multi-tier architecture provides best possible access speed
- **Robust Persistence**: Multiple fallback layers ensure data reliability
- **Scalable**: Handles everything from simple forms to complex enterprise applications
- **Secure**: Built-in security measures for sensitive data

### Negative

- **Complexity**: Multiple storage layers increase implementation complexity
- **Memory Usage**: In-memory caching uses additional RAM
- **Storage Limits**: Browser storage limitations may affect large forms

### Risk Mitigation

- **Comprehensive Testing**: Test all cache tiers and fallback scenarios
- **Memory Monitoring**: Implement memory pressure detection and eviction
- **Storage Quotas**: Handle storage limit exceptions gracefully
- **Security Audits**: Regular security reviews of caching mechanisms

## Implementation Timeline

**Week 1**: Basic in-memory cache with LRU eviction
**Week 2**: Session storage integration with auto-save
**Week 3**: Local storage for drafts and preferences
**Week 4**: IndexedDB for large forms and offline support
**Week 5**: Security layer and encryption
**Week 6**: Performance optimization and compression

## References

- [Web Storage API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API)
- [IndexedDB API](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API)
- [Browser Storage Limits](https://web.dev/storage-for-the-web/)
- [Cache Eviction Strategies](https://en.wikipedia.org/wiki/Cache_replacement_policies)

---

**Next Review**: 2025-03-01
**Related ADRs**: ADR-002 (Serialization), ADR-004 (Type System)
