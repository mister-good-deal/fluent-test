use fluent_test::prelude::*;
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};

// Counter for tracking setup and teardown calls
static SETUP_COUNTER: AtomicUsize = AtomicUsize::new(0);
static TEARDOWN_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Test state shared between tests
thread_local! {
    static TEST_VALUE: RefCell<u32> = RefCell::new(0);
}

// Reset helper for the test value - this is called by the setup function
fn reset_test_value() {
    TEST_VALUE.with(|v| {
        *v.borrow_mut() = 0;
    });
}

// Helper to set the test value
fn set_test_value(value: u32) {
    TEST_VALUE.with(|v| {
        *v.borrow_mut() = value;
    });
}

// Helper to get the test value
fn get_test_value() -> u32 {
    TEST_VALUE.with(|v| *v.borrow())
}

// Main test module for fixtures tests
mod main_fixtures {
    use super::*;

    // Setup function using the attribute style
    #[setup]
    fn setup_function() {
        SETUP_COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    // Teardown function using the attribute style
    #[tear_down]
    fn teardown_function() {
        TEARDOWN_COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    // Add a setup function that resets the test value
    #[setup]
    fn reset_test_state() {
        reset_test_value();
    }

    #[test]
    #[with_fixtures]
    fn test_fixtures_are_called() {
        // Setup functions should have been called
        // Since we have two setup functions (setup_function and reset_test_state),
        // the counter will reflect that
        let setup_count = SETUP_COUNTER.load(Ordering::SeqCst);
        expect!(setup_count).to_be_greater_than(0);

        // Do something in the test
        set_test_value(42);
        expect!(get_test_value()).to_equal(42);
    }

    #[test]
    #[with_fixtures]
    fn test_fixtures_run_for_each_test() {
        // The counter continues from previous test
        let _prev_count = SETUP_COUNTER.load(Ordering::SeqCst);

        // Verify that test state was reset (should be 0 from our setup function)
        expect!(get_test_value()).to_equal(0);

        // Set a value that should be cleared by teardown and next setup
        set_test_value(123);

        // After this test completes, the Teardown function will run,
        // which we'll verify in the next test
    }

    #[test]
    #[with_fixtures]
    fn test_teardown_ran_from_previous_test() {
        // Verify setup ran again
        let setup_count = SETUP_COUNTER.load(Ordering::SeqCst);
        expect!(setup_count).to_be_greater_than(0);

        // The value should have been reset by our reset_test_state setup function
        expect!(get_test_value()).to_equal(0);

        // Teardown should have been called from the previous test
        let teardown_count = TEARDOWN_COUNTER.load(Ordering::SeqCst);
        expect!(teardown_count).to_be_greater_than(0);
    }
}
