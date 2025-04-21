//! Async Matchers Example
//!
//! This example shows how to use the async matchers in fluent-test.
//! Run with: `cargo run --example async_matchers --features async`

use fluent_test::expect;
use std::time::Duration;
use tokio::time::sleep;

// Helper function that returns a future which resolves to a value
async fn delayed_value(delay_ms: u64) -> i32 {
    sleep(Duration::from_millis(delay_ms)).await;
    return 42;
}

// Helper function that returns a future which resolves to an error
async fn delayed_error(delay_ms: u64) -> Result<i32, String> {
    sleep(Duration::from_millis(delay_ms)).await;
    return Err("operation failed".to_string());
}

#[tokio::main]
async fn main() {
    println!("Async Matchers Example");
    println!("=====================\n");

    // Basic async value example
    demo_basic_async().await;

    // Time constraints example
    demo_time_constraints().await;

    // Error handling example
    demo_error_handling().await;

    println!("\nAll async examples completed!");
}

/// Demonstrates basic async value assertions
async fn demo_basic_async() {
    println!("1. Basic Async Assertions");
    println!("-----------------------");

    // Create a future that will resolve to 42 after 100ms
    let future = delayed_value(100);

    // When working with futures, await them before making assertions
    expect!(future).to_resolve_with(42);
    println!("✓ Future resolved to: {}", result);
}

/// Demonstrates time constraint assertions
async fn demo_time_constraints() {
    println!("2. Time Constraint Assertions");
    println!("---------------------------");

    // In tests, you can verify timing constraints

    // Example 1: Operation takes a minimum amount of time
    let start = std::time::Instant::now();
    let future = delayed_value(10);

    expect!(future).to_resolve_with(42).after(Duration::from_millis(10));
    let elapsed = start.elapsed();
    println!("✓ Operation completed in {:?}", elapsed);

    // Example 2: Operation takes a maximum amount of time
    let start = std::time::Instant::now();
    let future = delayed_value(10);

    expect!(future).to_resolve_with(42).before(Duration::from_millis(20));
    let elapsed = start.elapsed();
    println!("✓ Operation completed in {:?}", elapsed);

    println!("Time constraint assertions completed successfully!\n");
}

/// Demonstrates error handling assertions
async fn demo_error_handling() {
    println!("3. Error Handling Assertions");
    println!("---------------------------");

    // Create a future that will resolve to an error
    let future = delayed_error(100);

    // Example 1: Expect the future to fail with a specific error
    expect!(future).to_fail_with("operation failed");
    println!("✓ Future failed with operation failed");

    // Example 2: Expect the future to fail with a specific error and a constraint
    let future = delayed_error(100);
    expect!(future).to_fail_with("operation failed").after(Duration::from_millis(50));
    println!("✓ Future failed with operation failed and took more than 50ms");
    // Example 3: Expect the future to fail with a specific error and a constraint
    let future = delayed_error(100);
    expect!(future).to_fail_with("operation failed").before(Duration::from_millis(200));
    println!("✓ Future failed with operation failed and took less than 200ms");

    println!("Error handling assertions completed successfully!");
}
