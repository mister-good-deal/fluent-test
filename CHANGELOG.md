# Changelog

## 0.3.1 (2024-04-13)

### Added

- Added comprehensive unit tests:
  - Tests for configuration options and environment variable handling in `config.rs`
  - Tests for event emission and handler registration in `events.rs`
  - Tests for reporter functionality including session tracking and message deduplication in `reporter.rs`

### Fixed

- Fixed format macro warnings in event tests

## 0.3.0 (2024-04-13)

### Added

- Added enhanced configuration system:
  - Simplified API with `config().enhanced_output(true).apply()`
  - Support for environment variable `FLUENT_TEST_ENHANCED_OUTPUT`
  - New example files demonstrating configuration options
- Added new example files:
  - `config_example.rs` - Shows configuration options
  - `enhanced_output.rs` - Demonstrates enhanced output formatting

### Changed

- Implemented event-based architecture:
  - Decoupled assertions from reporting using an event system
  - Created a clean separation between backend and frontend components
  - Added EventEmitter for publishing assertion events
  - Implemented customizable event handlers for flexible output formatting
- Improved internal architecture:
  - Eliminated duplicate code in matcher implementations using helper traits
  - Created consistent abstraction pattern across all matchers
  - Reduced code by ~30% while maintaining the same public API
- Improved test output:
  - Better grammar with smart verb conjugation based on variable names
  - Removed reference symbols (`&`) from variable names in assertion messages
- Updated all examples to use the modern configuration approach
- Marked `initialize_event_system` as deprecated (hidden from docs)

### Removed

- Removed backward compatibility code
- Removed unused test utilities
- Removed `REFACTORING.md` as it's no longer needed

## 0.2.0 (2024-03-30)

### Added

- Added support for logical chain modifiers:
  - `.and()` - Chain multiple assertions that must all pass
  - `.or()` - Chain multiple assertions where at least one must pass
- Added support for combining `.not()` with logical chain operators

### Changed

- Improved output formatting:
  - Removed ampersands (`&`) from variable names in output for cleaner display
  - Added check (✓) and cross (✗) signs for passed/failed conditions
  - Applied proper coloring (green for passed, red for failed conditions)
  - Improved indentation in failure details
- Modified assertion chain behavior to avoid duplicate output in chained assertions
- Assertions no longer panic outside of test contexts, allowing for use in examples and production code

### Fixed

- Fixed issue with duplicate output in logical chain assertions
- Fixed the output formatting in Reporter's summary section

## 0.1.1 (2024-03-30)

### Added

- Boolean matchers:
  - `to_be_true` - Check if a boolean is true
  - `to_be_false` - Check if a boolean is false
- Option matchers:
  - `to_be_some` - Check if an Option contains a value
  - `to_be_none` - Check if an Option is None
  - `to_contain_value` - Check if an Option contains a specific value
- Result matchers:
  - `to_be_ok` - Check if a Result is Ok
  - `to_be_err` - Check if a Result is Err
  - `to_contain_ok` - Check if a Result contains a specific Ok value
  - `to_contain_err` - Check if a Result contains a specific Err value
- Collection matchers:
  - `to_be_empty` - Check if a collection is empty
  - `to_have_length` - Check if a collection has a specific length
  - `to_contain` - Check if a collection contains a specific element
  - `to_contain_all_of` - Check if a collection contains all specified elements
  - `to_equal_collection` - Compare two collections for element-wise equality
- HashMap matchers:
  - `to_be_empty` - Check if a HashMap is empty
  - `to_have_length` - Check if a HashMap has a specific length
  - `to_contain_key` - Check if a HashMap contains a specific key
  - `to_contain_entry` - Check if a HashMap contains a specific key-value pair
- Comprehensive documentation for all matchers
- Unit tests for all new matchers

### Fixed

- Fixed clippy warnings in collection matchers
- Fixed markdown linting issues in README

## 0.1.0 (2024-03-30)

### Added

- Initial implementation of FluentTest, a Jest-like testing library for Rust
- Core expectation and matcher system
- Fluent assertion API with `expect!` macro
- Negation support with `.not()` method and `expect_not!` macro
- Numeric matchers:
  - `to_be_greater_than` - Check if a number is greater than another
  - `to_be_less_than` - Check if a number is less than another
  - `to_be_even` - Check if a number is even
  - `to_be_odd` - Check if a number is odd
  - `to_be_divisible_by` - Check if a number is divisible by another
  - `to_be_positive` - Check if a number is positive
  - `to_be_negative` - Check if a number is negative
  - `to_be_in_range` - Check if a number is within a specified range
- String matchers:
  - `to_be_empty` - Check if a string is empty
  - `to_contain` - Check if a string contains a substring
  - `to_start_with` - Check if a string starts with a prefix
  - `to_end_with` - Check if a string ends with a suffix
  - `to_match_regex` - Check if a string matches a regex pattern
  - `to_have_length` - Check if a string has a specific length
  - `to_have_length_greater_than` - Check if a string length is greater than a value
  - `to_have_length_less_than` - Check if a string length is less than a value
- Equality matcher (`to_equal`) for comparing values
- Consistent error message formatting
- Test reporting with colorized output
- Custom test configuration options
- Comprehensive documentation with examples
- Automated CI/CD setup with GitHub Actions
- Automated release and publishing process
