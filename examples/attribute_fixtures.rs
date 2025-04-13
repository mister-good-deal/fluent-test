use fluent_test::prelude::*;
use std::cell::RefCell;

// A shared counter for demonstration
thread_local! {
    static TEST_COUNTER: RefCell<u32> = RefCell::new(0);
}

// Module to contain the fixture functions with attribute-style macros
mod test_module {
    use super::*;

    // Setup function with the attribute style
    #[setup]
    fn setup() {
        println!("Setting up test environment with attribute macro...");
        TEST_COUNTER.with(|counter| {
            *counter.borrow_mut() = 0;
        });
    }

    // Teardown function with the attribute style
    #[tear_down]
    fn tear_down() {
        println!("Cleaning up test environment with attribute macro...");
        TEST_COUNTER.with(|counter| {
            println!("Final counter value: {}", *counter.borrow());
        });
    }

    // Test function with the attribute style
    #[with_fixtures]
    pub fn run_test() {
        // Initial value should be 0 (set by the setup function)
        expect!(get_counter()).to_equal(0);

        // Increment and check
        increment_counter();
        expect!(get_counter()).to_equal(1);
    }
}

// Helper to increment the counter
fn increment_counter() {
    TEST_COUNTER.with(|counter| {
        *counter.borrow_mut() += 1;
    });
}

// Helper to get the counter value
fn get_counter() -> u32 {
    TEST_COUNTER.with(|counter| *counter.borrow())
}

fn main() {
    // Enable enhanced output for better test reporting
    config().enhanced_output(true).apply();

    println!("Running test with attribute-style fixtures:");
    test_module::run_test();

    println!("\nTest passed!");
}
