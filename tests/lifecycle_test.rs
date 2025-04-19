use fluent_test::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

// Setup counters to track execution
static BEFORE_ALL_COUNTER: AtomicUsize = AtomicUsize::new(0);
static SETUP_COUNTER: AtomicUsize = AtomicUsize::new(0);
static TEARDOWN_COUNTER: AtomicUsize = AtomicUsize::new(0);
static AFTER_ALL_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Module testing the before_all and after_all attributes
#[with_fixtures_module]
mod lifecycle_fixtures {
    use super::*;

    // Runs once before any test in this module
    #[before_all]
    fn setup_module() {
        BEFORE_ALL_COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    // Runs before each test in this module
    #[setup]
    fn setup_test() {
        SETUP_COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    // Runs after each test in this module
    #[tear_down]
    fn teardown_test() {
        TEARDOWN_COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    // Runs once after all tests in this module
    #[after_all]
    fn teardown_module() {
        AFTER_ALL_COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    #[test]
    fn test_before_all_executed_once() {
        // before_all should have been called once
        let before_all_count = BEFORE_ALL_COUNTER.load(Ordering::SeqCst);
        expect!(before_all_count).to_equal(1);

        // setup should have been called for this test
        let setup_count = SETUP_COUNTER.load(Ordering::SeqCst);
        expect!(setup_count).to_be_greater_than(0);
    }

    #[test]
    fn test_setup_executed_for_each_test() {
        // before_all should still be 1
        let before_all_count = BEFORE_ALL_COUNTER.load(Ordering::SeqCst);
        expect!(before_all_count).to_equal(1);

        // setup should be called once per test
        let setup_count = SETUP_COUNTER.load(Ordering::SeqCst);
        expect!(setup_count).to_be_greater_than(1); // More than 1 because of previous test

        // teardown should have been called for the previous test
        let teardown_count = TEARDOWN_COUNTER.load(Ordering::SeqCst);
        expect!(teardown_count).to_be_greater_than(0);
    }
}

// After all fixtures can't be tested here directly because they run at process exit
// But we can register a test-specific fixture and verify it works
mod verify_after_all {
    use super::*;
    use once_cell::sync::Lazy;
    use std::cell::RefCell;
    use std::sync::Mutex;

    // A thread-safe cell to store test results
    static VERIFICATION_CELL: Lazy<Mutex<RefCell<bool>>> = Lazy::new(|| Mutex::new(RefCell::new(false)));

    // Register an after_all handler that will update our verification cell
    #[after_all]
    fn verify_after_all_runs() {
        if let Ok(lock) = VERIFICATION_CELL.lock() {
            *lock.borrow_mut() = true;
        }
    }

    // Function to check if our after_all was executed
    pub fn was_after_all_executed() -> bool {
        if let Ok(lock) = VERIFICATION_CELL.lock() {
            return *lock.borrow();
        }
        return false;
    }

    // We need at least one test to trigger the fixture setup
    #[test]
    #[with_fixtures]
    fn dummy_test_to_register_fixtures() {
        // Just a dummy test to ensure our fixtures are registered
        expect!(true).to_be_true();
    }
}
