# Test Fixtures in FluentTest

FluentTest provides a fixture system that allows you to set up and tear down test environments before and after each test. This is useful for:

- Preparing test data
- Setting up mock objects
- Cleaning up resources after tests
- Ensuring tests start with a clean environment

## Using Fixtures

Fixtures are defined using attribute macros and are tied to the module they're defined in. Tests that use the `#[with_fixtures]` attribute will automatically run the setup and teardown functions for that module. You can also apply fixtures to all test functions in a module using the `#[with_fixtures_module]` attribute on the module itself.

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

## Module-Level Fixtures

You can apply fixtures to all test functions in a module by using the `#[with_fixtures_module]` attribute:

```rust
use fluent_test::prelude::*;

#[with_fixtures_module]
mod user_tests {
    use super::*;
    
    #[setup]
    fn setup() {
        // Setup specific to user tests
        println!("Setting up test environment");
    }
    
    #[tear_down]
    fn tear_down() {
        // Teardown specific to user tests
        println!("Cleaning up test environment");
    }
    
    // No need for #[with_fixtures] on each test
    #[test]
    fn test_one() {
        // Fixtures will be automatically applied
        expect!(2 + 2).to_equal(4);
    }
    
    #[test]
    fn test_two() {
        // Fixtures will also be applied here
        expect!(3 + 3).to_equal(6);
    }
}
```

This eliminates the need to add `#[with_fixtures]` to each test function, making your tests more concise.

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

## Lifecycle Fixtures

FluentTest supports four types of fixtures to handle different phases of the test lifecycle:

1. **#[before_all]** - Runs once before any test in the module
2. **#[setup]** - Runs before each test in the module
3. **#[tear_down]** - Runs after each test in the module
4. **#[after_all]** - Runs once after all tests in the module have completed

Here's an example using all lifecycle fixtures:

```rust
use fluent_test::prelude::*;

#[with_fixtures_module]
mod database_tests {
    use super::*;
    
    // Run once before any test in this module
    #[before_all]
    fn initialize_database() {
        println!("Creating test database...");
        // Create test database, load schema, etc.
    }
    
    // Run before each test
    #[setup]
    fn setup_test_data() {
        println!("Setting up test data...");
        // Insert test data for each test
    }
    
    #[test]
    fn test_user_creation() {
        // Test code that uses the database
    }
    
    #[test]
    fn test_user_deletion() {
        // Another test using the database
    }
    
    // Run after each test
    #[tear_down]
    fn clean_test_data() {
        println!("Cleaning up test data...");
        // Remove test data after each test
    }
    
    // Run once after all tests in this module
    #[after_all]
    fn drop_database() {
        println!("Dropping test database...");
        // Clean up the test database
    }
}
```

The execution order is:
1. `initialize_database()` (#[before_all]) - Run once at the start
2. For each test:
   a. `setup_test_data()` (#[setup]) - Run before the test
   b. Test function - The actual test
   c. `clean_test_data()` (#[tear_down]) - Run after the test
3. `drop_database()` (#[after_all]) - Run once at the end

## Implementation Details

Fixtures are collected and associated with their module paths. When a test marked with `#[with_fixtures]` is run, it:

1. Runs before_all functions for the module if they haven't run yet
2. Runs setup functions for the current module
3. Executes the test code
4. Runs teardown functions for the module, even if the test code panics
5. Registers after_all functions to run when testing is complete

### Notes about After All

The #[after_all] fixture relies on Rust's ctor crate to register a global exit handler. This ensures that after_all fixtures run even if tests are executed in parallel or if some tests fail. However, there are some limitations:

- In cargo test with parallel execution, the execution order is not guaranteed
- When running tests individually, after_all fixtures will still run at process exit