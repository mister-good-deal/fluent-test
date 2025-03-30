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
    let my_number = 5;
    let my_string = "hello world";
    let my_vec = vec![1, 2, 3];
    
    expect!(my_number).to_be_greater_than(3);
    expect!(my_string).to_contain("world");
    expect!(my_vec).to_have_length(3);
}
```

## Available Matchers

FluentTest provides a comprehensive set of matchers for various types. All matchers support negation through either the
`not()` method or the `expect_not!` macro.

### Boolean

- [**to_be_true**](#to_be_true) - Checks if a boolean is true
- [**to_be_false**](#to_be_false) - Checks if a boolean is false

### Equality

- [**to_equal**](#to_equal) - Checks if a value equals another value

### Numeric

- [**to_be_greater_than**](#to_be_greater_than) - Checks if a number is greater than another
- [**to_be_less_than**](#to_be_less_than) - Checks if a number is less than another
- [**to_be_even**](#to_be_even) - Checks if a number is even
- [**to_be_odd**](#to_be_odd) - Checks if a number is odd
- [**to_be_divisible_by**](#to_be_divisible_by) - Checks if a number is divisible by another
- [**to_be_positive**](#to_be_positive) - Checks if a number is positive
- [**to_be_negative**](#to_be_negative) - Checks if a number is negative
- [**to_be_in_range**](#to_be_in_range) - Checks if a number is within a specified range

### String

- [**to_be_empty**](#to_be_empty) - Checks if a string is empty
- [**to_contain**](#to_contain) - Checks if a string contains a substring
- [**to_start_with**](#to_start_with) - Checks if a string starts with a prefix
- [**to_end_with**](#to_end_with) - Checks if a string ends with a suffix
- [**to_match_regex**](#to_match_regex) - Checks if a string matches a regex pattern
- [**to_have_length**](#to_have_length) - Checks if a string has a specific length
- [**to_have_length_greater_than**](#to_have_length_greater_than) - Checks if a string length is greater than a value
- [**to_have_length_less_than**](#to_have_length_less_than) - Checks if a string length is less than a value

### Collection

- [**to_be_empty_collection**](#to_be_empty_collection) - Checks if a collection is empty
- [**to_have_length_collection**](#to_have_length_collection) - Checks if a collection has a specific length
- [**to_contain_collection**](#to_contain_collection) - Checks if a collection contains a specific element
- [**to_contain_all_of**](#to_contain_all_of) - Checks if a collection contains all specified elements
- [**to_equal_collection**](#to_equal_collection) - Compares two collections for element-wise equality

### HashMap

- [**to_be_empty_hashmap**](#to_be_empty_hashmap) - Checks if a HashMap is empty
- [**to_have_length_hashmap**](#to_have_length_hashmap) - Checks if a HashMap has a specific length
- [**to_contain_key**](#to_contain_key) - Checks if a HashMap contains a specific key
- [**to_contain_entry**](#to_contain_entry) - Checks if a HashMap contains a specific key-value pair

### Option

- [**to_be_some**](#to_be_some) - Checks if an Option contains a value
- [**to_be_none**](#to_be_none) - Checks if an Option is None
- [**to_contain_value**](#to_contain_value) - Checks if an Option contains a specific value

### Result

- [**to_be_ok**](#to_be_ok) - Checks if a Result is Ok
- [**to_be_err**](#to_be_err) - Checks if a Result is Err
- [**to_contain_ok**](#to_contain_ok) - Checks if a Result contains a specific Ok value
- [**to_contain_err**](#to_contain_err) - Checks if a Result contains a specific Err value

## Using Modifiers

### Not Modifier

FluentTest supports two ways to negate expectations:

```rust
#[test]
fn test_not_modifiers() {
    let value = 42;
    let name = "Arthur";
    
    // Two ways to use negated expectations
    
    // 1. Using the .not() method (fluent API)
    expect!(value).not().to_equal(100);
    expect!(name).not().to_contain("Bob");
    
    // 2. Using the expect_not! macro
    expect_not!(value).to_equal(100);
}
```

### Logical Chain Modifiers (AND/OR)

FluentTest allows chaining multiple assertions with logical operators:

```rust
#[test]
fn test_chain_modifiers() {
    let number = 42;
    
    // Using AND to require all conditions to pass
    expect!(number).to_be_greater_than(30)
                 .and().to_be_less_than(50)
                 .and().to_be_even();
    
    // Using OR where at least one condition must pass
    expect!(number).to_be_greater_than(100)  // This fails
                 .or().to_be_less_than(30)   // This fails
                 .or().to_equal(42);         // This passes, so overall expression passes
    
    // Combining NOT with logical modifiers
    expect!(number).not().to_be_less_than(30)
                 .and().not().to_be_greater_than(50);
}
```

The output from logical chains is clean and concise, showing just the final result with properly indented details for failing chains.

## Matcher Documentation

### Boolean Matchers

#### to_be_true

Checks if a boolean is true.

```rust
fn test_boolean_true() {
    let is_enabled = true;
    let is_disabled = false;
    
    expect!(is_enabled).to_be_true();     // Passes
    expect!(is_disabled).not().to_be_true(); // Passes
}
```

#### to_be_false

Checks if a boolean is false.

```rust
fn test_boolean_false() {
    let is_enabled = true;
    let is_disabled = false;
    
    expect!(is_disabled).to_be_false();     // Passes
    expect!(is_enabled).not().to_be_false(); // Passes
}
```

### Equality Matchers

#### to_equal

Checks if a value equals another value.

```rust
fn test_equality() {
    let value = 42;
    let name = "Arthur";
    
    expect!(value).to_equal(42);         // Passes
    expect!(value).not().to_equal(13);   // Passes
    expect!(name).to_equal("Arthur");    // Passes
}
```

### Numeric Matchers

#### to_be_greater_than

Checks if a number is greater than another number.

```rust
fn test_greater_than() {
    let value = 42;
    
    expect!(value).to_be_greater_than(30);       // Passes
    expect!(value).not().to_be_greater_than(50); // Passes
}
```

#### to_be_less_than

Checks if a number is less than another number.

```rust
fn test_less_than() {
    let value = 42;
    
    expect!(value).to_be_less_than(50);       // Passes
    expect!(value).not().to_be_less_than(30); // Passes
}
```

#### to_be_even

Checks if a number is even.

```rust
fn test_even() {
    let even = 42;
    let odd = 43;
    
    expect!(even).to_be_even();       // Passes
    expect!(odd).not().to_be_even();  // Passes
}
```

#### to_be_odd

Checks if a number is odd.

```rust
fn test_odd() {
    let even = 42;
    let odd = 43;
    
    expect!(odd).to_be_odd();        // Passes
    expect!(even).not().to_be_odd(); // Passes
}
```

#### to_be_divisible_by

Checks if a number is divisible by another number.

```rust
fn test_divisible_by() {
    let value = 42;
    
    expect!(value).to_be_divisible_by(7);   // Passes (42 is divisible by 7)
    expect!(value).not().to_be_divisible_by(5); // Not divisible by 5
}
```

#### to_be_positive

Checks if a number is positive (greater than zero).

```rust
fn test_positive() {
    let positive = 42;
    let negative = -42;
    
    expect!(positive).to_be_positive();       // Passes
    expect!(negative).not().to_be_positive(); // Passes
}
```

#### to_be_negative

Checks if a number is negative (less than zero).

```rust
fn test_negative() {
    let positive = 42;
    let negative = -42;
    
    expect!(negative).to_be_negative();       // Passes
    expect!(positive).not().to_be_negative(); // Passes
}
```

#### to_be_in_range

Checks if a number is within a specified range.

```rust
fn test_in_range() {
    let value = 42;
    
    expect!(value).to_be_in_range(40..=45);  // Inclusive range 40 to 45
    expect!(value).not().to_be_in_range(50..60);  // Not in range 50 to 60
    
    // Different range types
    expect!(value).to_be_in_range(40..46);  // Half-open range
    expect!(value).to_be_in_range(30..);  // Range from 30 upwards
    expect!(value).to_be_in_range(..50);  // Range up to but not including 50
}
```

### String Matchers

#### to_be_empty

Checks if a string is empty.

```rust
fn test_empty_string() {
    let empty = "";
    let not_empty = "hello";
    
    expect!(empty).to_be_empty();        // Passes
    expect!(not_empty).not().to_be_empty(); // Passes
}
```

#### to_contain

Checks if a string contains a specified substring.

```rust
fn test_string_contains() {
    let greeting = "Hello, world!";
    
    expect!(greeting).to_contain("world");       // Passes
    expect!(greeting).not().to_contain("moon");  // Passes
}
```

#### to_start_with

Checks if a string starts with a specified prefix.

```rust
fn test_string_starts_with() {
    let greeting = "Hello, world!";
    
    expect!(greeting).to_start_with("Hello");       // Passes
    expect!(greeting).not().to_start_with("Goodbye"); // Passes
}
```

#### to_end_with

Checks if a string ends with a specified suffix.

```rust
fn test_string_ends_with() {
    let greeting = "Hello, world!";
    
    expect!(greeting).to_end_with("world!");        // Passes
    expect!(greeting).not().to_end_with("universe"); // Passes
}
```

#### to_match_regex

Checks if a string matches a regular expression pattern.

```rust
fn test_string_matches_regex() {
    let greeting = "Hello, world!";
    
    expect!(greeting).to_match_regex(r"^Hello.*!$");        // Passes
    expect!(greeting).not().to_match_regex(r"^Goodbye.*\?$"); // Passes
}
```

#### to_have_length

Checks if a string has a specific length.

```rust
fn test_string_length() {
    let greeting = "Hello, world!";
    
    expect!(greeting).to_have_length(13);        // Passes
    expect!(greeting).not().to_have_length(10);  // Passes
}
```

#### to_have_length_greater_than

Checks if a string length is greater than a specified value.

```rust
fn test_string_length_greater_than() {
    let greeting = "Hello, world!";
    
    expect!(greeting).to_have_length_greater_than(10);        // Passes
    expect!(greeting).not().to_have_length_greater_than(15);  // Passes
}
```

#### to_have_length_less_than

Checks if a string length is less than a specified value.

```rust
fn test_string_length_less_than() {
    let greeting = "Hello, world!";
    
    expect!(greeting).to_have_length_less_than(20);        // Passes
    expect!(greeting).not().to_have_length_less_than(10);  // Passes
}
```

### Collection Matchers

#### to_be_empty_collection

Checks if a collection is empty.

```rust
fn test_empty_collection() {
    let empty_vec: Vec<i32> = vec![];
    let non_empty_vec = vec![1, 2, 3];
    
    expect!(empty_vec.as_slice()).to_be_empty();             // Passes
    expect!(non_empty_vec.as_slice()).not().to_be_empty();   // Passes
}
```

#### to_have_length_collection

Checks if a collection has a specific length.

```rust
fn test_collection_length() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    expect!(numbers.as_slice()).to_have_length(5);         // Passes
    expect!(numbers.as_slice()).not().to_have_length(3);   // Passes
}
```

#### to_contain_collection

Checks if a collection contains a specific element.

```rust
fn test_collection_contains() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    expect!(numbers.as_slice()).to_contain(3);           // Passes
    expect!(numbers.as_slice()).not().to_contain(10);    // Passes
}
```

#### to_contain_all_of

Checks if a collection contains all of the specified elements.

```rust
fn test_collection_contains_all() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    expect!(numbers.as_slice()).to_contain_all_of(&[1, 3, 5]);        // Passes
    expect!(numbers.as_slice()).not().to_contain_all_of(&[1, 6, 7]);  // Passes
}
```

#### to_equal_collection

Compares two collections for element-wise equality.

```rust
fn test_equal_collection() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    expect!(numbers.as_slice()).to_equal_collection(vec![1, 2, 3, 4, 5]);         // Passes
    expect!(numbers.as_slice()).not().to_equal_collection(vec![5, 4, 3, 2, 1]);   // Passes
}
```

### HashMap Matchers

#### to_be_empty_hashmap

Checks if a HashMap is empty.

```rust
fn test_empty_hashmap() {
    use std::collections::HashMap;
    
    let empty_map: HashMap<&str, i32> = HashMap::new();
    let mut non_empty_map = HashMap::new();
    non_empty_map.insert("Alice", 100);
    
    expect!(&empty_map).to_be_empty();             // Passes
    expect!(&non_empty_map).not().to_be_empty();   // Passes
}
```

#### to_have_length_hashmap

Checks if a HashMap has a specific number of entries.

```rust
fn test_hashmap_length() {
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    scores.insert("Alice", 100);
    scores.insert("Bob", 85);
    
    expect!(&scores).to_have_length(2);         // Passes
    expect!(&scores).not().to_have_length(1);   // Passes
}
```

#### to_contain_key

Checks if a HashMap contains a specific key.

```rust
fn test_hashmap_contains_key() {
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    scores.insert("Alice", 100);
    scores.insert("Bob", 85);
    
    expect!(&scores).to_contain_key("Alice");          // Passes
    expect!(&scores).not().to_contain_key("Charlie");  // Passes
}
```

#### to_contain_entry

Checks if a HashMap contains a specific key-value pair.

```rust
fn test_hashmap_contains_entry() {
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    scores.insert("Alice", 100);
    scores.insert("Bob", 85);
    
    expect!(&scores).to_contain_entry("Alice", &100);         // Passes
    expect!(&scores).not().to_contain_entry("Alice", &50);    // Passes
}
```

### Option Matchers

#### to_be_some

Checks if an Option contains a value.

```rust
fn test_option_some() {
    let maybe_value: Option<i32> = Some(42);
    let empty_option: Option<i32> = None;
    
    expect!(&maybe_value).to_be_some();         // Passes
    expect!(&empty_option).not().to_be_some();  // Passes
}
```

#### to_be_none

Checks if an Option is None.

```rust
fn test_option_none() {
    let maybe_value: Option<i32> = Some(42);
    let empty_option: Option<i32> = None;
    
    expect!(&empty_option).to_be_none();        // Passes
    expect!(&maybe_value).not().to_be_none();   // Passes
}
```

#### to_contain_value

Checks if an Option contains a specific value.

```rust
fn test_option_contains_value() {
    let maybe_value: Option<i32> = Some(42);
    let other_value: Option<i32> = Some(100);
    
    expect!(&maybe_value).to_contain_value(42);        // Passes
    expect!(&other_value).not().to_contain_value(42);  // Passes
}
```

### Result Matchers

#### to_be_ok

Checks if a Result is Ok.

```rust
fn test_result_ok() {
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("failed");
    
    expect!(&success).to_be_ok();         // Passes
    expect!(&failure).not().to_be_ok();  // Passes
}
```

#### to_be_err

Checks if a Result is Err.

```rust
fn test_result_err() {
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("failed");
    
    expect!(&failure).to_be_err();        // Passes
    expect!(&success).not().to_be_err();  // Passes
}
```

#### to_contain_ok

Checks if a Result contains a specific Ok value.

```rust
fn test_result_contains_ok() {
    let success: Result<i32, &str> = Ok(42);
    let other_success: Result<i32, &str> = Ok(100);
    
    expect!(&success).to_contain_ok(42);           // Passes
    expect!(&other_success).not().to_contain_ok(42); // Passes
}
```

#### to_contain_err

Checks if a Result contains a specific error value.

```rust
fn test_result_contains_err() {
    let network_err: Result<i32, &str> = Err("network error");
    let auth_err: Result<i32, &str> = Err("authentication error");
    
    expect!(&network_err).to_contain_err("network error");            // Passes
    expect!(&auth_err).not().to_contain_err("network error");         // Passes
}
```

## Custom Matchers

You can easily extend FluentTest with your own matchers:

```rust
// Define a custom matcher for your domain
trait UserMatchers<T> {
    fn to_be_admin(self);
}

// Implement it for the Expectation type
impl<T: AsRef<User>> UserMatchers<T> for Expectation<T> {
    fn to_be_admin(self) {
        let user = self.value.as_ref();
        let success = user.role == Role::Admin;
        let not = if self.negated { " not" } else { "" };
        
        if (success && !self.negated) || (!success && self.negated) {
            self.report_success(&format!("is{not} an admin"));
        } else {
            let expected_msg = format!("Expected {}{not} to be an admin", self.expr_str);
            self.report_failure(&expected_msg, &format!("Found role: {:?}", user.role));
        }
    }
}

// Use it in your tests
#[test]
fn test_user_permissions() {
    let admin_user = get_admin_user();
    let regular_user = get_regular_user();
    
    expect!(admin_user).to_be_admin();           // Passes
    expect!(regular_user).not().to_be_admin();   // Passes
}
```

## Output Formatting

FluentTest enhances the standard test output with colors, symbols, and improved formatting:

- **Color Coding**: Green for passing tests, red for failing tests
- **Unicode Symbols**: Check (✓) marks for passing conditions, cross (✗) for failing ones
- **Clean Variable Names**: Reference symbols (`&`) are automatically removed from output
- **Consistent Indentation**: Multi-line output is properly indented for readability
- **Logical Assertions**: AND/OR chains produce concise, readable output

You can customize the output for CI environments or other special cases:

```rust
// In your test module or test helper file
#[test]
fn setup() {
    fluent_test::config()
        .use_colors(true)
        .use_unicode_symbols(true)
        .show_success_details(false)
        .apply();
}
```

## How It Works

FluentTest is built around a few core components:

1. The `expect!` macro which captures both values and their textual representation
2. The `expect_not!` macro which creates negated expectations
3. The `Expectation<T>` struct which holds the value and provides the fluent API
4. Trait implementations for different types of assertions
5. A custom test reporter that enhances the standard output

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
rustup toolchain install nightly --component llvm-tools-preview
cargo install grcov
cargo install rustfilt

# Build with coverage instrumentation and run tests
RUSTFLAGS="-C instrument-coverage" cargo +nightly test

# Generate HTML report
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./coverage/
```

Then open `./coverage/index.html` in your browser.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
