use fluent_test::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

// Setup a new test module focused just on attribute-style fixtures
mod attribute_test_module {
    use super::*;

    // Counter for tracking fixture calls
    static SETUP_COUNT: AtomicUsize = AtomicUsize::new(0);
    static TEARDOWN_COUNT: AtomicUsize = AtomicUsize::new(0);

    // Print the module path to help debug
    fn print_module_path() {
        println!("Current module path: {}", module_path!());
    }

    // Setup function using attribute
    #[setup]
    fn setup() {
        println!("Running setup in {}", module_path!());
        SETUP_COUNT.fetch_add(1, Ordering::SeqCst);
        let count = SETUP_COUNT.load(Ordering::SeqCst);
        println!("Setup count is now: {}", count);
    }

    // Teardown function using attribute
    #[tear_down]
    fn teardown() {
        println!("Running teardown in {}", module_path!());
        TEARDOWN_COUNT.fetch_add(1, Ordering::SeqCst);
        let count = TEARDOWN_COUNT.load(Ordering::SeqCst);
        println!("Teardown count is now: {}", count);
    }

    // Test using with_fixtures attribute
    #[test]
    #[with_fixtures]
    fn test_with_attribute_fixtures() {
        print_module_path();

        // Get the current counts
        let setup_count = SETUP_COUNT.load(Ordering::SeqCst);
        let teardown_count = TEARDOWN_COUNT.load(Ordering::SeqCst);

        println!("In test: setup_count={}, teardown_count={}", setup_count, teardown_count);

        // Verify setup was called
        expect!(setup_count).to_equal(1);
    }
}
