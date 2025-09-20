# Remediation Plan 05: Dependency Updates & Modernization

**Priority**: MEDIUM
**Estimated Effort**: 1-2 days
**Blockers**: None (can run in parallel)
**Dependencies**: None

## Overview

Update all dependencies to latest stable versions as of September 2025 and ensure compatibility.

## Current Dependency Status

### Rust Version

- **Current**: 1.89+
- **Latest**: 1.90.0 (Sept 2025)
- **Action**: Update MSRV to 1.90.0

### Core Dependencies

#### Leptos Framework

- **Current**: 0.8
- **Status**: Appears to be latest stable
- **Action**: Verify and pin to specific patch version

#### Build Tools

```toml
# Current versions to verify/update
wasm-pack = "0.12"           # Check for 0.13+
wasm-bindgen = "0.2"         # Check for latest 0.2.x
web-sys = "0.3"              # Check for latest 0.3.x
```

## Dependency Audit Plan

### 1. Security Audit

```bash
# Check for known vulnerabilities
cargo audit

# Check for unused dependencies
cargo machete

# Check for outdated dependencies
cargo outdated
```

### 2. Version Updates

**High Priority Updates**:

- Update Rust to 1.90.0
- Update all security-flagged crates
- Update build tools (wasm-pack, etc.)

**Medium Priority Updates**:

- Update dev dependencies (Playwright, etc.)
- Update procedural macro dependencies
- Update serde ecosystem

**Low Priority Updates**:

- Update utility crates that don't affect API

### 3. Breaking Change Analysis

**For each major dependency update**:

- Review changelog for breaking changes
- Update code to handle API changes
- Run full test suite
- Update documentation if API changes

## Specific Updates Needed

### Cargo.toml Updates

```toml
# Root Cargo.toml
[workspace.dependencies]
serde = { version = "1.0.210" }        # Update to latest 1.0.x
serde_json = "1.0.127"                 # Update to latest
chrono = { version = "0.4.38", features = ["serde"] }
regex = "1.10.6"                       # Update to latest
wasm-bindgen = "0.2.93"                # Update to latest 0.2.x
web-sys = "0.3.70"                     # Update to latest 0.3.x
```

### Development Dependencies

```toml
# package.json updates
"@playwright/test": "^1.47.0"          # Update from 1.55.0
"typescript": "^5.5.4"                 # Update from 5.3.0
"vite": "^5.4.6"                       # Update from 7.1.4 (major version change)
```

### MSRV Updates

```toml
# Update minimum supported Rust version
rust-version = "1.90.0"

# Update CI matrix to test against:
# - 1.90.0 (MSRV)
# - stable
# - nightly (allow failures)
```

## Compatibility Testing

### Test Matrix

- **Rust versions**: 1.90.0, stable, nightly
- **Leptos versions**: 0.8.x (current), future versions
- **Target platforms**:
  - native (x86_64, aarch64)
  - wasm32-unknown-unknown
  - wasm32-wasi

### Validation Steps

1. Update dependencies incrementally
2. Run `cargo check` after each update
3. Run full test suite
4. Test WASM compilation
5. Test examples still work
6. Update documentation

## Breaking Changes to Handle

### Potential Issues

- **Leptos API changes**: Signal API, component macros
- **wasm-bindgen changes**: WASM interface updates
- **serde changes**: Serialization behavior
- **proc-macro changes**: Macro expansion differences

### Mitigation Strategy

- Use feature flags for breaking changes when possible
- Maintain compatibility layers during transition
- Document migration path for users
- Provide clear error messages for unsupported versions

## Documentation Updates

### Files to Update

- `README.md` - Update version requirements
- `AGENTS.md` - Update build commands if needed
- `Cargo.toml` - Update package metadata
- `docs/` - Update getting started guide

### Version Compatibility Matrix

```markdown
| Leptos Forms Version | Rust MSRV | Leptos | WASM Support |
| -------------------- | --------- | ------ | ------------ |
| 1.2.x                | 1.90.0    | 0.8.x  | ✅           |
| 1.1.x                | 1.89.0    | 0.8.x  | ✅           |
```

## CI/CD Updates

### GitHub Actions

- Update Rust toolchain action to 1.90.0
- Add dependency vulnerability scanning
- Add automated dependency update PRs (Dependabot)
- Test against multiple Leptos versions

### Example workflow update

```yaml
- name: Install Rust
  uses: actions-rs/toolchain@v1
  with:
    toolchain: 1.90.0
    override: true
    components: rustfmt, clippy
```

## Acceptance Criteria

### Functionality

- [ ] All code compiles with Rust 1.90.0
- [ ] No cargo audit security warnings
- [ ] All tests pass with updated dependencies
- [ ] WASM compilation works
- [ ] Examples run correctly

### Documentation

- [ ] README reflects actual requirements
- [ ] Migration guide for breaking changes (if any)
- [ ] Updated installation instructions
- [ ] Version compatibility matrix

### Automation

- [ ] CI tests multiple Rust versions
- [ ] Automated security scanning
- [ ] Dependency update automation

## Implementation Steps

### Day 1: Analysis & Planning

- Run dependency audit tools
- Identify breaking changes
- Plan update order
- Set up testing branches

### Day 2: Implementation

- Update dependencies incrementally
- Fix compilation issues
- Update documentation
- Test all functionality

## Risk Mitigation

### Rollback Plan

- Keep current working versions in git tags
- Use feature flags for risky updates
- Test thoroughly before merging
- Document any breaking changes

### Communication

- Update CHANGELOG.md with all changes
- Communicate breaking changes to users
- Provide migration assistance

## Next Steps

After completion, proceed to `06_DOCUMENTATION_FIXES.md`
