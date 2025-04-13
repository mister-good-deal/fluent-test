# Test Fixtures in FluentTest

FluentTest provides a fixture system that allows you to set up and tear down test environments before and after each test. This is useful for:

- Preparing test data
- Setting up mock objects
- Cleaning up resources after tests
- Ensuring tests start with a clean environment

## Using Fixtures

Fixtures are defined using attribute macros and are tied to the module they're defined in. Tests that use the `#[with_fixtures]` attribute will automatically run the setup and teardown functions for that module.

### Basic Example

```rust
use fluent_test::prelude::*;

// Define setup and teardown functions
#[setup]
fn setup() {
    // Code to run before each test
    println!("Setting up test environment");
}

#[tear_down]
fn tear_down() {
    // Code to run after each test
    println!("Cleaning up test environment");
}

#[test]
#[with_fixtures]
fn my_test() {
    // This will run setup, then the test code, then teardown
    // Your test code here
    expect!(2 + 2).to_equal(4);
}
```

## Module Scoping

Fixtures are scoped to the module they're defined in. Each module can have its own setup and teardown functions:

```rust
mod user_tests {
    use super::*;
    
    #[setup]
    fn setup() {
        // Setup specific to user tests
        create_test_user();
    }
    
    #[tear_down]
    fn tear_down() {
        // Teardown specific to user tests
        remove_test_user();
    }
    
    #[test]
    #[with_fixtures]
    fn test_user_creation() {
        // Test code here
    }
}

mod product_tests {
    use super::*;
    
    #[setup]
    fn setup() {
        // Setup specific to product tests
        create_test_product();
    }
    
    // Tests in this module use product fixtures
}
```

## Attribute-Style Fixtures

FluentTest uses attribute-style syntax for fixtures:

```rust
use fluent_test::prelude::*;

// Use attributes to define fixtures
#[setup]
fn my_setup() {
    // Setup code
}

#[tear_down]
fn my_teardown() {
    // Teardown code
}

// Run tests with fixtures using an attribute
#[with_fixtures]
fn test_something() {
    // Test code - fixtures will be automatically applied
    expect!(2 + 2).to_equal(4);
}
```

## Best Practices

1. **Keep fixtures simple**: Focus on the minimum setup needed for your tests
2. **Clean up resources**: Ensure teardown functions properly release any resources created during setup
3. **Module organization**: Group related tests in modules with appropriate fixtures
4. **Reuse fixtures**: Define common setup/teardown logic in shared modules
5. **Test isolation**: Ensure tests don't interfere with each other by resetting state in setup

## Error Handling

If a test panics, the teardown function will still be called, ensuring resources are properly cleaned up. This is similar to how RAII (Resource Acquisition Is Initialization) works in Rust.

```rust
#[with_fixtures]
fn test_with_panic() {
    // Even if this test panics...
    panic!("Test failure");
    // ...the teardown function will still run
}
```

## Implementation Details

Fixtures are collected and associated with their module paths. When a test marked with `#[with_fixtures]` is run, it:

1. Looks up setup functions for the current module path
2. Runs those setup functions
3. Executes the test code
4. Runs any teardown functions for the module, even if the test code panics