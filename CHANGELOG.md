# Changelog

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
