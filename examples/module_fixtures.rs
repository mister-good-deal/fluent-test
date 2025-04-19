use fluent_test::prelude::*;
use std::cell::RefCell;

// A shared counter for demonstration
thread_local! {
    static TEST_COUNTER: RefCell<u32> = RefCell::new(0);
}

// Helper to increment the counter
fn increment_counter() {
    TEST_COUNTER.with(|counter| {
        *counter.borrow_mut() += 1;
    });
}

// Helper to get the counter value
fn get_counter() -> u32 {
    return TEST_COUNTER.with(|counter| *counter.borrow());
}

// Define functions to demonstrate module fixtures
fn setup_module_fixtures() {
    println!("\nExample of using module-level fixtures:");
    println!("In a real test, you would use:");
    println!("#[with_fixtures_module]");
    println!("mod my_test_module {{");
    println!("    #[setup]");
    println!("    fn setup() {{ /* setup code */ }}");
    println!("");
    println!("    #[tear_down]");
    println!("    fn tear_down() {{ /* cleanup code */ }}");
    println!("");
    println!("    #[test]");
    println!("    fn test_something() {{ /* fixtures automatically applied */ }}");
    println!("}}\n");
}

// Module fixture setup example
fn run_fixture_setup() {
    println!("Setting up test environment...");
    TEST_COUNTER.with(|counter| {
        *counter.borrow_mut() = 0;
    });
}

// Module fixture teardown example
fn run_fixture_teardown() {
    println!("Cleaning up test environment...");
    TEST_COUNTER.with(|counter| {
        println!("Final counter value: {}", *counter.borrow());
    });
}

// Simulate a test
fn run_simulated_test() {
    // Run a simulated setup
    run_fixture_setup();

    println!("Running test...");
    // Test code
    expect!(get_counter()).to_equal(0);

    // Do something in the test
    increment_counter();
    expect!(get_counter()).to_equal(1);

    // Run a simulated teardown
    run_fixture_teardown();
}

fn main() {
    // Enable enhanced output for better test reporting
    config().enhanced_output(true).apply();

    // Explain the module fixture concept
    setup_module_fixtures();

    // Run a simulated test
    run_simulated_test();

    println!("\nAll tests passed!");
}
