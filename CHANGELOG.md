# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2025-01-05

### Added
- Full compatibility with Leptos 0.8
- Modern validation engine with sophisticated validation rules
- Comprehensive test suite with 97 passing tests
- Enhanced form field types and validation
- Real-time validation hooks
- Form persistence capabilities
- Performance benchmarking tools
- Rich text, markdown, and code input components
- File upload handling with validation
- Conditional validation based on field dependencies

### Changed
- **BREAKING**: Updated to Leptos 0.8 API
  - `ValidatorConfig` → `Validator` enum
  - `FormSchema` structure simplified
  - `use_form` hook now returns tuple `(FormHandle, Callback, Callback)`
  - `FormHandle` methods updated for new API
- **BREAKING**: Form trait methods renamed
  - `get_field` → `get_field_value`
  - `set_field` → `set_field_value`
- Updated validation error handling
- Improved reactive context handling
- Enhanced field metadata structure

### Fixed
- Fixed pattern validator to use provided regex patterns
- Resolved reactive context errors in FormHandle methods
- Fixed field access and serialization issues
- Corrected validation rules engine implementation
- Fixed test data consistency across all test suites
- Resolved compilation errors for Leptos 0.8 compatibility

### Technical Improvements
- Modernized library for September 2025 standards
- Implemented comprehensive TDD approach
- Added extensive error handling and edge case coverage
- Improved performance and memory usage
- Enhanced type safety and compile-time guarantees

## [0.3.0] - Previous Release

### Added
- Initial form handling capabilities
- Basic validation system
- Form components and hooks

### Changed
- Various API improvements and bug fixes

## [0.2.0] - Previous Release

### Added
- Procedural macro support
- Enhanced form field types

## [0.1.0] - Initial Release

### Added
- Basic form handling functionality
- Initial validation system
- Core form components