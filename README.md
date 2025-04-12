# FluentTest

[![Crates.io](https://img.shields.io/crates/v/fluent-test.svg)](https://crates.io/crates/fluent-test)
[![Build Status](https://github.com/mister-good-deal/fluent-test/workflows/CI/badge.svg)](https://github.com/mister-good-deal/fluent-test/actions)
[![codecov](https://codecov.io/gh/mister-good-deal/fluent-test/graph/badge.svg?token=W5L8E2CQ9M)](https://codecov.io/gh/mister-good-deal/fluent-test)
[![License](https://img.shields.io/crates/l/fluent-test.svg)](https://github.com/mister-good-deal/fluent-test/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/fluent-test.svg)](https://crates.io/crates/fluent-test)

A fluent, Jest-like testing library for Rust that builds upon the standard testing infrastructure. FluentTest provides
expressive assertions with readable error messages while maintaining compatibility with Rust's built-in testing functionality.

## Features

- **Fluent, Expressive API**: Write tests in a readable, chainable style similar to Jest.
- **Logical Chain Modifiers**: Combine assertions with `.and()` and `.or()` operators.
- **Helpful Error Messages**: Get clear error messages that include variable names and expressions.
- **Seamless Integration**: Works alongside Rust's standard testing infrastructure.
- **Beautiful Test Output**: Enhanced test reporting with visual cues and better organization.
- **Type-Safe Assertions**: Leverages Rust's type system for compile-time safety.

## Roadmap

- [x] Basic matchers for primitive types
- [x] Matchers for collections (Vec, HashMap, etc.)
- [x] Matchers for Option and Result types
- [x] Logical chain modifiers (AND/OR)
- [x] Custom matcher support
- [x] Improved test output formatting
- [x] CI/CD integration for automatic releases
- [x] Documentation and examples
- [ ] More advanced matchers (e.g., regex, custom types)
- [ ] Performance optimizations
- [ ] Support for async testing
- [ ] Custom console client (e.g., for CI/CD or nice terminal output)
- [ ] Support for code coverage reporting
- [ ] Support for mocking and stubbing
- [ ] Integration with other testing frameworks (e.g., mockito, assert_eq)
- [ ] Community contributions and feedback
- [ ] Continuous improvement based on user feedback
- [ ] Enhanced error reporting and debugging tools
- [ ] Integration with popular CI/CD tools

## Quick Start

Add FluentTest to your project:

```bash
cargo add fluent-test --dev
```

Write your first test:

```rust
use fluent_test::prelude::*;

#[test]
fn should_check_values() {
    // By default, FluentTest behaves like standard Rust assertions
    // To enable enhanced output, configure it:
    config().enhanced_output(true).apply();
    
    let my_number = 5;
    let my_string = "hello world";
    let my_vec = vec![1, 2, 3];
    
    expect!(my_number).to_be_greater_than(3);
    expect!(my_string).to_contain("world");
    expect!(my_vec).to_have_length(3);
}
```

You can also enable enhanced output globally by setting the environment variable:

```bash
FLUENT_TEST_ENHANCED_OUTPUT=true cargo test
```

## Available Matchers

FluentTest provides a comprehensive set of matchers for various types. All matchers support negation through either the
`not()` method or the `expect_not!` macro.

For complete documentation of all matchers, please see the [Wiki documentation](https://github.com/mister-good-deal/fluent-test/wiki).

### Boolean Matchers

- **to_be_true** - Checks if a boolean is true
- **to_be_false** - Checks if a boolean is false

[View Boolean Matchers documentation](https://github.com/mister-good-deal/fluent-test/wiki/Boolean-Matchers)

### Equality Matchers

- **to_equal** - Checks if a value equals another value

[View Equality Matchers documentation](https://github.com/mister-good-deal/fluent-test/wiki/Equality-Matchers)

### Numeric Matchers

- **to_be_greater_than** - Checks if a number is greater than another
- **to_be_less_than** - Checks if a number is less than another
- **to_be_even** - Checks if a number is even
- **to_be_odd** - Checks if a number is odd
- **to_be_divisible_by** - Checks if a number is divisible by another
- **to_be_positive** - Checks if a number is positive
- **to_be_negative** - Checks if a number is negative
- **to_be_in_range** - Checks if a number is within a specified range

[View Numeric Matchers documentation](https://github.com/mister-good-deal/fluent-test/wiki/Numeric-Matchers)

### String Matchers

- **to_be_empty** - Checks if a string is empty
- **to_contain** - Checks if a string contains a substring
- **to_start_with** - Checks if a string starts with a prefix
- **to_end_with** - Checks if a string ends with a suffix
- **to_match_regex** - Checks if a string matches a regex pattern
- **to_have_length** - Checks if a string has a specific length
- **to_have_length_greater_than** - Checks if a string length is greater than a value
- **to_have_length_less_than** - Checks if a string length is less than a value

[View String Matchers documentation](https://github.com/mister-good-deal/fluent-test/wiki/String-Matchers)

### Collection Matchers

- **to_be_empty** - Checks if a collection is empty
- **to_have_length** - Checks if a collection has a specific length
- **to_contain** - Checks if a collection contains a specific element
- **to_contain_all_of** - Checks if a collection contains all specified elements
- **to_equal_collection** - Compares two collections for element-wise equality

[View Collection Matchers documentation](https://github.com/mister-good-deal/fluent-test/wiki/Collection-Matchers)

### HashMap Matchers

- **to_be_empty** - Checks if a HashMap is empty
- **to_have_length** - Checks if a HashMap has a specific length
- **to_contain_key** - Checks if a HashMap contains a specific key
- **to_contain_entry** - Checks if a HashMap contains a specific key-value pair

[View HashMap Matchers documentation](https://github.com/mister-good-deal/fluent-test/wiki/HashMap-Matchers)

### Option Matchers

- **to_be_some** - Checks if an Option contains a value
- **to_be_none** - Checks if an Option is None
- **to_contain_value** - Checks if an Option contains a specific value

[View Option Matchers documentation](https://github.com/mister-good-deal/fluent-test/wiki/Option-Matchers)

### Result Matchers

- **to_be_ok** - Checks if a Result is Ok
- **to_be_err** - Checks if a Result is Err
- **to_contain_ok** - Checks if a Result contains a specific Ok value
- **to_contain_err** - Checks if a Result contains a specific Err value

[View Result Matchers documentation](https://github.com/mister-good-deal/fluent-test/wiki/Result-Matchers)

## Using Modifiers

FluentTest provides powerful modifiers to create complex assertions, including:

- Negation with the `.not()` method or `expect_not!` macro
- Logical chaining with `.and()` and `.or()` operators
- Combining negation with logical operators

```rust
// Example of chained assertions
expect!(number).to_be_greater_than(30)
             .and().to_be_less_than(50)
             .and().to_be_even();

// Example of negation
expect!(value).not().to_equal(100);
```

[View Using Modifiers documentation](https://github.com/mister-good-deal/fluent-test/wiki/Using-Modifiers)

## Custom Matchers

FluentTest is designed to be easily extensible. You can create your own custom matchers to make your tests more expressive and domain-specific.

[View Custom Matchers documentation](https://github.com/mister-good-deal/fluent-test/wiki/Custom-Matchers)

## Output Formatting

FluentTest enhances the standard test output with colors, symbols, and improved formatting:

- **Color Coding**: Green for passing tests, red for failing tests
- **Unicode Symbols**: Check (✓) marks for passing conditions, cross (✗) for failing ones
- **Clean Variable Names**: Reference symbols (`&`) are automatically removed from output
- **Consistent Indentation**: Multi-line output is properly indented for readability

[View Output Formatting documentation](https://github.com/mister-good-deal/fluent-test/wiki/Output-Formatting)

## Architecture

FluentTest uses a modular, event-driven architecture:

- **Backend Layer** - Core assertion evaluation logic
- **Config System** - Controls the library's behavior
- **Event System** - Decouples assertion execution from reporting
- **Frontend Layer** - Reporting and user interface

[View Architecture documentation](https://github.com/mister-good-deal/fluent-test/wiki/Architecture)

## Releases

This project is automatically published to [crates.io](https://crates.io/crates/fluent-test) when:

1. The version in Cargo.toml is increased beyond the latest git tag
2. The code is merged to the master branch
3. All CI checks pass (tests, examples, linting)

The publishing workflow will:

1. Create a git tag for the new version (vX.Y.Z)
2. Publish the package to crates.io
3. Generate a GitHub release using notes from CHANGELOG.md
4. Fall back to auto-generated notes if no CHANGELOG entry exists

## Code Coverage

This project uses Rust's official LLVM-based code coverage instrumentation to track test coverage. The coverage workflow:

1. Compiles the project with coverage instrumentation using Rust nightly's `-C instrument-coverage` flag
2. Runs the test suite to generate raw coverage data in LLVM's profraw format
3. Uses grcov to convert the raw coverage data to:
   - LCOV format for uploading to Codecov.io
   - HTML reports for local viewing

Coverage reports are automatically generated on each push to the master branch and for pull requests.

### Viewing Coverage Reports

- The latest coverage report is always available on [Codecov.io](https://codecov.io/gh/mister-good-deal/fluent-test)
- Each CI run also produces an HTML coverage report available as a downloadable artifact

### Running Code Coverage Locally

To generate coverage reports locally:

```bash
# Install required components
rustup component add llvm-tools-preview
cargo install grcov
cargo install rustfilt

# Build with coverage instrumentation and run tests
RUSTFLAGS="-C instrument-coverage" cargo test

# Generate HTML report
grcov . --binary-path ./target/debug/ -s . -t html --branch --keep-only "src/**" -o ./coverage

# Generate Markdown report
grcov . --binary-path ./target/debug/ -s . -t markdown --branch --keep-only "src/**" -o ./coverage/coverage.md
```

Then open `./coverage/index.html` in your browser or view the Markdown report in your favorite editor.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
