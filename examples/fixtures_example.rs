use fluent_test::prelude::*;
use std::cell::RefCell;

// A shared counter for demonstration
thread_local! {
    static TEST_COUNTER: RefCell<u32> = RefCell::new(0);
}

// Define a module for the first test example with the run_test method
mod test1 {
    use super::*;

    // Setup function with attribute style
    #[setup]
    fn setup() {
        println!("Setting up test environment for test1...");
        TEST_COUNTER.with(|counter| {
            *counter.borrow_mut() = 0;
        });
    }

    // Teardown function with attribute style
    #[tear_down]
    fn tear_down() {
        println!("Cleaning up test environment for test1...");
        TEST_COUNTER.with(|counter| {
            println!("Final counter value: {}", *counter.borrow());
        });
    }

    // Add the run_test function to the test1 module with attribute style
    #[with_fixtures]
    pub fn run_test() {
        // Initial value should be 0
        expect!(get_counter()).to_equal(0);

        // Increment and check
        increment_counter();
        expect!(get_counter()).to_equal(1);

        // Increment again and check
        increment_counter();
        expect!(get_counter()).to_equal(2);
    }
}

// Define a module for the second test example with the run_test method
mod test2 {
    use super::*;

    // Setup function with attribute style
    #[setup]
    fn setup() {
        println!("Setting up test environment for test2...");
        TEST_COUNTER.with(|counter| {
            *counter.borrow_mut() = 0;
        });
    }

    // Teardown function with attribute style
    #[tear_down]
    fn tear_down() {
        println!("Cleaning up test environment for test2...");
        TEST_COUNTER.with(|counter| {
            println!("Final counter value: {}", *counter.borrow());
        });
    }

    // Add the run_test function to the test2 module with attribute style
    #[with_fixtures]
    pub fn run_test() {
        // Each test gets a fresh environment
        expect!(get_counter()).to_equal(0);
    }
}

// Helper to increment the counter - used by both modules
fn increment_counter() {
    TEST_COUNTER.with(|counter| {
        *counter.borrow_mut() += 1;
    });
}

// Helper to get the counter value - used by both modules
fn get_counter() -> u32 {
    // Reset counter after getting the value outside of test scope
    let value = TEST_COUNTER.with(|counter| *counter.borrow());
    return value;
}

fn main() {
    // Enable enhanced output for better test reporting
    config().enhanced_output(true).apply();

    println!("Running first test with fixtures:");
    test1::run_test();

    println!("\nRunning second test with fixtures:");
    test2::run_test();

    println!("\nBoth tests passed!");
}
