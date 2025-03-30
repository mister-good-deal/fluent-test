# FluentTest

A fluent, Jest-like testing library for Rust that builds upon the standard testing infrastructure. FluentTest provides
expressive assertions with readable error messages while maintaining compatibility with Rust's built-in testing functionality.

## Features

- **Fluent, Expressive API**: Write tests in a readable, chainable style similar to Jest.
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

## Using Not Modifiers

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

## Matcher Documentation

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

## Output Customization

FluentTest enhances the standard test output with colors, symbols, and improved formatting.

For CI environments or other special cases, you can customize the output:

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

## License

This project is licensed under the MIT License - see the LICENSE file for details.
