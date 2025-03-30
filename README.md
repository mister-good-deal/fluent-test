# FluentTest

A fluent, Jest-like testing library for Rust that builds upon the standard testing infrastructure. FluentTest provides expressive assertions with readable error messages while maintaining compatibility with Rust's built-in testing functionality.

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

## Detailed Examples

### Basic Assertions

```rust
#[test]
fn test_basic_assertions() {
    let value = 42;
    let name = "Arthur";
    let items = vec![1, 2, 3, 4];
    
    expect!(value).to_equal(42);
    expect!(value).not_to_equal(13);
    expect!(value).to_be_greater_than(30);
    expect!(value).to_be_less_than(50);
    
    expect!(name).to_equal("Arthur");
    expect!(name).to_contain("rt");
    expect!(name).to_start_with("Ar");
    expect!(name).to_end_with("ur");
    
    expect!(items).to_have_length(4);
    expect!(items).to_contain(3);
    expect!(items).not_to_contain(5);
}
```

### Using Jest-like Not Syntax

FluentTest supports several ways to negate expectations, similar to Jest's `.not` property:

```rust
#[test]
fn test_not_modifiers() {
    let value = 42;
    let name = "Arthur";
    
    // Four different ways to use negated expectations
    
    // 1. Traditional method chaining
    expect!(value).not_method().to_equal(100);
    
    // 2. Alternative method
    expect!(value).not_prop().to_equal(100);
    
    // 3. We implement the Not trait, but it's not practical for direct use
    // let expectation = expect!(value);
    // let negated = !expectation; // Works but isn't useful directly
    
    // 4. Using the not! macro
    not!(value).to_equal(100);
    
    // 5. Jest-style .not() method (recommended)
    expect!(value).not().to_equal(100);
    expect!(name).not().to_contain("Bob");
}
```

### Numeric Testing

```rust
#[test]
fn test_numbers() {
    let value = 42;
    
    expect!(value).to_be_even();
    expect!(value).not_to_be_odd();
    expect!(value).to_be_divisible_by(7);
    expect!(value).to_be_positive();
    expect!(value).to_be_in_range(40..=45);
}
```

### String Testing

```rust
#[test]
fn test_strings() {
    let message = "Hello, world!";
    
    expect!(message).to_match_regex(r"^Hello.*!$");
    expect!(message).to_be_lowercase_at(7); // character 'w'
    expect!(message).to_have_length_greater_than(10);
}
```

### Error Testing

```rust
#[test]
fn test_results_and_options() {
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("something went wrong");
    let some_value: Option<i32> = Some(42);
    let no_value: Option<i32> = None;
    
    expect!(success).to_be_ok().and_to_equal(42);
    expect!(failure).to_be_err().and_to_contain("wrong");
    expect!(some_value).to_be_some().and_to_equal(42);
    expect!(no_value).to_be_none();
}
```

## Grouping Tests

FluentTest works with Rust's standard test organization:

```rust
mod tests {
    use super::*;
    use fluent_test::prelude::*;
    
    #[test]
    fn test_addition() {
        let result = add(2, 3);
        expect!(result).to_equal(5);
    }
    
    #[test]
    fn test_subtraction() {
        let result = subtract(5, 3);
        expect!(result).to_equal(2);
    }
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
        if user.role == Role::Admin {
            self.report_success("is an admin");
        } else {
            self.report_failure(
                &format!("Expected {} to be an admin", self.expr_str),
                &format!("Found role: {:?}", user.role)
            );
        }
    }
}

// Use it in your tests
#[test]
fn test_user_permissions() {
    let user = get_user(1);
    expect!(user).to_be_admin();
}
```

## How It Works

FluentTest uses Rust's macro system to capture both the value and the expression text at the test site. This allows it to provide clear, context-aware error messages without requiring you to manually specify variable names.

The library seamlessly integrates with Rust's test framework, so you can run your tests with the standard `cargo test` command.

## Output Customization

FluentTest enhances the standard test output with colors, symbols, and improved formatting, making it easier to quickly understand test results.

For CI environments or other special cases, you can customize the output format:

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

## Compatibility

FluentTest is designed to work alongside Rust's standard testing functionality. You can:

- Mix and match FluentTest assertions with standard `assert!` macros
- Use FluentTest in some test modules and not others
- Integrate with other testing utilities like `proptest` or `quickcheck`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/fluent-test.git
   cd fluent-test
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the tests:
   ```bash
   cargo test
   ```

## Implementation Details

FluentTest is built around a few core components:

1. The `expect!` macro which captures both values and their textual representation
2. The `Expectation<T>` struct which holds the value and provides the fluent API
3. Trait implementations for different types of assertions
4. A custom test reporter that enhances the standard output

## Roadmap

- [ ] More built-in matchers for common Rust types
- [ ] Snapshot testing support
- [ ] Async test support
- [ ] Property-based testing integration
- [ ] Custom test runner with improved reporting
- [ ] IDE integration

## License

This project is licensed under the MIT License - see the LICENSE file for details.