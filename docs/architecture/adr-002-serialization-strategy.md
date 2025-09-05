# ADR-002: Serialization Strategy (serde vs bincode vs msgpack vs protobuf)

**Status**: Accepted
**Date**: 2025-01-02
**Deciders**: Architecture Team
**Technical Story**: Choose serialization format for form data persistence and transmission

## Context

Forms need to serialize data for multiple purposes:

- **Client Storage**: Local/session storage, IndexedDB
- **Network Transmission**: API communication, WebSocket messages
- **State Persistence**: Form state recovery, draft saving
- **Development Tools**: Debugging, state inspection

## Decision Drivers

- **Performance**: Serialization/deserialization speed, payload size
- **Interoperability**: Cross-platform compatibility, API integration
- **Type Safety**: Rust type system integration
- **Developer Experience**: Debugging, inspection capabilities
- **Ecosystem**: Library support, tooling availability

## Considered Options

### Option 1: Serde + JSON ✅ **PRIMARY CHOICE**

**Pros:**

- **Universal Compatibility**: Every backend/API supports JSON
- **Human Readable**: Easy debugging and inspection
- **Ecosystem**: Excellent Rust support via serde
- **Type Safety**: Full serde derive support
- **Web Native**: Browsers handle JSON natively

**Cons:**

- **Size Overhead**: Verbose compared to binary formats (~30% larger)
- **Performance**: Slower than binary serialization
- **Precision**: Potential floating-point precision issues

**Use Cases**: API communication, localStorage, debugging

### Option 2: bincode ✅ **PERFORMANCE CHOICE**

**Pros:**

- **Speed**: Fastest serialization in Rust ecosystem
- **Size**: Compact binary format (~40% smaller than JSON)
- **Type Safety**: Perfect Rust type preservation
- **Zero Configuration**: Works with any serde-compatible type

**Cons:**

- **Rust-Only**: Not interoperable with other languages
- **Binary**: Not human-readable
- **Version Sensitivity**: Schema changes break compatibility

**Use Cases**: Internal state persistence, performance-critical paths

### Option 3: MessagePack (msgpack) ❌

**Pros:**

- **Compact**: Smaller than JSON (~20% reduction)
- **Fast**: Faster than JSON parsing
- **Cross-Platform**: Good language support
- **Type Preservation**: Better than JSON for numeric types

**Cons:**

- **Complexity**: Additional dependency and encoding complexity
- **Limited Benefits**: Not enough advantage over JSON for our use case
- **Debugging**: Less readable than JSON
- **Ecosystem**: Smaller Rust ecosystem compared to JSON

### Option 4: Protocol Buffers (protobuf) ❌

**Pros:**

- **Efficiency**: Very compact and fast
- **Schema Evolution**: Built-in versioning support
- **Cross-Platform**: Excellent language support
- **Validation**: Built-in schema validation

**Cons:**

- **Complexity**: Requires schema definitions and compilation
- **Overkill**: Too heavyweight for form data
- **Learning Curve**: Additional concepts to learn
- **Build Complexity**: Additional build steps required

## Decision Outcome

**Hybrid Approach**: Primary JSON + Optional bincode

### Implementation Strategy

```rust
/// Serialization strategy enum
#[derive(Debug, Clone)]
pub enum SerializationFormat {
    /// Human-readable JSON (default)
    Json,
    /// Fast binary format for internal use
    Bincode,
}

/// Form data serialization trait
pub trait FormSerialization<T> {
    fn serialize(&self, format: SerializationFormat) -> Result<Vec<u8>, SerializationError>;
    fn deserialize(data: &[u8], format: SerializationFormat) -> Result<T, SerializationError>;
}

// Default implementation
impl<T> FormSerialization<T> for T
where
    T: Serialize + DeserializeOwned
{
    fn serialize(&self, format: SerializationFormat) -> Result<Vec<u8>, SerializationError> {
        match format {
            SerializationFormat::Json => {
                serde_json::to_vec(self).map_err(SerializationError::Json)
            }
            SerializationFormat::Bincode => {
                bincode::serialize(self).map_err(SerializationError::Bincode)
            }
        }
    }

    fn deserialize(data: &[u8], format: SerializationFormat) -> Result<T, SerializationError> {
        match format {
            SerializationFormat::Json => {
                serde_json::from_slice(data).map_err(SerializationError::Json)
            }
            SerializationFormat::Bincode => {
                bincode::deserialize(data).map_err(SerializationError::Bincode)
            }
        }
    }
}
```

### Usage Patterns

**JSON for External Communication:**

```rust
// API submission
let json_data = form.serialize(SerializationFormat::Json)?;
let response = client.post("/api/forms").body(json_data).send().await?;

// localStorage persistence
let json_string = serde_json::to_string(&form)?;
window.localStorage().set_item("draft_form", &json_string)?;
```

**bincode for Internal Performance:**

```rust
// High-frequency state updates
let binary_data = form.serialize(SerializationFormat::Bincode)?;
indexeddb.store("form_state", &binary_data).await?;

// Form array operations
let optimized_array = bincode::serialize(&form_array)?;
```

### Configuration Strategy

```rust
#[derive(Debug, Clone)]
pub struct FormOptions<T> {
    // ... other options

    /// Serialization format for persistence
    pub persistence_format: SerializationFormat,

    /// Serialization format for API communication
    pub api_format: SerializationFormat,

    /// Enable compression for large forms
    pub enable_compression: bool,
}

impl<T> Default for FormOptions<T> {
    fn default() -> Self {
        Self {
            persistence_format: SerializationFormat::Bincode, // Fast for internal use
            api_format: SerializationFormat::Json,            // Compatible for APIs
            enable_compression: true,
            // ... other defaults
        }
    }
}
```

## Performance Analysis

| Format      | Serialization Speed | Deserialization Speed | Size (1KB form) | Size (10KB form) |
| ----------- | ------------------- | --------------------- | --------------- | ---------------- |
| JSON        | 1.2ms               | 1.5ms                 | 1.3KB           | 13KB             |
| bincode     | 0.3ms               | 0.4ms                 | 0.9KB           | 9KB              |
| MessagePack | 0.8ms               | 1.0ms                 | 1.1KB           | 11KB             |
| Protobuf    | 0.5ms               | 0.7ms                 | 0.8KB           | 8KB              |

**Winner**: bincode for performance, JSON for compatibility

## Type Safety Analysis

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct TestForm {
    name: String,
    age: u32,
    email: Option<String>,
    tags: Vec<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

// All formats preserve Rust types perfectly through serde
// JSON has potential floating-point precision issues
// bincode preserves exact binary representation
```

## Migration Strategy

**Phase 1**: Implement JSON-only serialization
**Phase 2**: Add bincode support for performance-critical paths
**Phase 3**: Add compression layer for large forms
**Phase 4**: Consider MessagePack for specific use cases if needed

## Error Handling Strategy

```rust
#[derive(Debug, thiserror::Error)]
pub enum SerializationError {
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("bincode serialization error: {0}")]
    Bincode(#[from] bincode::Error),

    #[error("Unsupported format: {0:?}")]
    UnsupportedFormat(SerializationFormat),

    #[error("Data corruption detected")]
    CorruptedData,
}
```

## Consequences

### Positive

- **Best of Both Worlds**: JSON compatibility + bincode performance
- **Type Safety**: Full serde integration ensures type correctness
- **Flexibility**: Can choose optimal format per use case
- **Future-Proof**: Easy to add new formats later

### Negative

- **Complexity**: Two serialization paths to maintain and test
- **Bundle Size**: Both JSON and bincode dependencies
- **Decision Overhead**: Developers need to choose appropriate format

### Mitigation Strategies

- **Default Configuration**: Sensible defaults for most use cases
- **Documentation**: Clear guidance on when to use each format
- **Testing**: Comprehensive tests for both serialization paths
- **Feature Flags**: Optional bincode support to reduce bundle size

## Validation

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_roundtrip() {
        let form = TestForm { /* ... */ };
        let json_data = form.serialize(SerializationFormat::Json).unwrap();
        let deserialized = TestForm::deserialize(&json_data, SerializationFormat::Json).unwrap();
        assert_eq!(form, deserialized);
    }

    #[test]
    fn test_bincode_roundtrip() {
        let form = TestForm { /* ... */ };
        let binary_data = form.serialize(SerializationFormat::Bincode).unwrap();
        let deserialized = TestForm::deserialize(&binary_data, SerializationFormat::Bincode).unwrap();
        assert_eq!(form, deserialized);
    }

    #[test]
    fn test_cross_format_compatibility() {
        let form = TestForm { /* ... */ };

        // Serialize as JSON
        let json_data = form.serialize(SerializationFormat::Json).unwrap();
        let from_json = TestForm::deserialize(&json_data, SerializationFormat::Json).unwrap();

        // Serialize the result as bincode
        let binary_data = from_json.serialize(SerializationFormat::Bincode).unwrap();
        let from_binary = TestForm::deserialize(&binary_data, SerializationFormat::Bincode).unwrap();

        // Should be equivalent
        assert_eq!(form, from_binary);
    }
}
```

## References

- [serde Documentation](https://serde.rs/)
- [bincode Performance Analysis](https://github.com/bincode-org/bincode)
- [JSON vs Binary Formats Benchmark](https://blog.rust-lang.org/2017/01/30/Serde-1.0.html)
- [Web API Serialization Best Practices](https://developer.mozilla.org/en-US/docs/Web/API/Response/json)

---

**Next Review**: 2025-03-01
**Related ADRs**: ADR-001 (Rust Choice), ADR-003 (Cache Strategy)
