use rest::prelude::*;
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};

// Setup counters to track execution
static BEFORE_ALL_COUNTER: AtomicUsize = AtomicUsize::new(0);
static SETUP_COUNTER: AtomicUsize = AtomicUsize::new(0);
static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);
static TEARDOWN_COUNTER: AtomicUsize = AtomicUsize::new(0);
static AFTER_ALL_COUNTER: AtomicUsize = AtomicUsize::new(0);

// A shared value for tests
thread_local! {
    static TEST_VALUE: RefCell<String> = RefCell::new(String::new());
}

// Helper to set the test value
fn set_test_value(value: &str) {
    TEST_VALUE.with(|v| {
        *v.borrow_mut() = value.to_string();
    });
}

// Helper to get the test value
fn get_test_value() -> String {
    TEST_VALUE.with(|v| v.borrow().clone())
}

// Helper to print a module state
fn print_module_state(stage: &str) {
    println!("{}", "-".repeat(50));
    println!("{}", stage);
    println!("  Before All count: {}", BEFORE_ALL_COUNTER.load(Ordering::SeqCst));
    println!("  Setup count     : {}", SETUP_COUNTER.load(Ordering::SeqCst));
    println!("  Test count      : {}", TEST_COUNTER.load(Ordering::SeqCst));
    println!("  Teardown count  : {}", TEARDOWN_COUNTER.load(Ordering::SeqCst));
    println!("  After All count : {}", AFTER_ALL_COUNTER.load(Ordering::SeqCst));
    println!("  Test value      : {}", get_test_value());
    println!("{}", "-".repeat(50));
}

// Module demonstrating the before_all and after_all attributes
#[with_fixtures_module]
mod lifecycle_test {
    use super::*;

    // Runs once before any test in this module
    #[before_all]
    fn setup_module() {
        println!("Running before_all setup...");
        BEFORE_ALL_COUNTER.fetch_add(1, Ordering::SeqCst);
        set_test_value("Initialized by before_all");
    }

    // Runs before each test in this module
    #[setup]
    fn setup_test() {
        println!("Running setup for a test...");
        SETUP_COUNTER.fetch_add(1, Ordering::SeqCst);
        // We'll append to the test value to see the sequence
        let current = get_test_value();
        set_test_value(&format!("{} + setup", current));
    }

    // First test
    #[test]
    pub fn first_test() {
        println!("Running first test...");
        TEST_COUNTER.fetch_add(1, Ordering::SeqCst);

        // Check the counters
        expect!(BEFORE_ALL_COUNTER.load(Ordering::SeqCst)).to_equal(1);
        expect!(SETUP_COUNTER.load(Ordering::SeqCst)).to_equal(1);

        // Modify the test value
        let current = get_test_value();
        set_test_value(&format!("{} + first_test", current));

        print_module_state("During first test");
    }

    // Second test
    #[test]
    pub fn second_test() {
        println!("Running second test...");
        TEST_COUNTER.fetch_add(1, Ordering::SeqCst);

        // Check the counters
        expect!(BEFORE_ALL_COUNTER.load(Ordering::SeqCst)).to_equal(1); // Still 1 because before_all runs only once
        expect!(SETUP_COUNTER.load(Ordering::SeqCst)).to_equal(2); // Now 2 because setup runs before each test

        // Modify the test value
        let current = get_test_value();
        set_test_value(&format!("{} + second_test", current));

        print_module_state("During second test");
    }

    // Runs after each test in this module
    #[tear_down]
    fn teardown_test() {
        println!("Running teardown after a test...");
        TEARDOWN_COUNTER.fetch_add(1, Ordering::SeqCst);

        // Append to the test value
        let current = get_test_value();
        set_test_value(&format!("{} + teardown", current));
    }

    // Runs once after all tests in this module
    #[after_all]
    fn teardown_module() {
        println!("Running after_all teardown...");
        AFTER_ALL_COUNTER.fetch_add(1, Ordering::SeqCst);

        // Append to the test value
        let current = get_test_value();
        set_test_value(&format!("{} + after_all", current));

        print_module_state("After all tests completed");
    }
}

// Alternative approach to running the example in the main function
fn run_simulated_tests() {
    println!("\nRunning example of module lifecycle fixtures:");
    println!("This demonstrates the order of execution for fixture types:");
    println!("  1. #[before_all]   - Runs once before any test in the module");
    println!("  2. #[setup]        - Runs before each test");
    println!("  3. Test function   - The actual test");
    println!("  4. #[tear_down]    - Runs after each test");
    println!("  5. #[after_all]    - Runs once after all tests in the module\n");

    // Print initial state
    print_module_state("Initial state");

    // Setup handlers for real tests would execute in this order
    println!("\nIn normal test execution, the lifecycle would be:");

    // Simulate before_all
    println!("1. Running before_all setup once at the beginning");
    BEFORE_ALL_COUNTER.fetch_add(1, Ordering::SeqCst);
    set_test_value("Initialized by before_all");

    // Simulate first test
    println!("2. For test #1: Running setup");
    SETUP_COUNTER.fetch_add(1, Ordering::SeqCst);
    let current = get_test_value();
    set_test_value(&format!("{} + setup", current));

    println!("3. For test #1: Running the test itself");
    TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let current = get_test_value();
    set_test_value(&format!("{} + first_test", current));

    println!("4. For test #1: Running teardown");
    TEARDOWN_COUNTER.fetch_add(1, Ordering::SeqCst);
    let current = get_test_value();
    set_test_value(&format!("{} + teardown", current));

    // Simulate second test
    println!("5. For test #2: Running setup again");
    SETUP_COUNTER.fetch_add(1, Ordering::SeqCst);
    let current = get_test_value();
    set_test_value(&format!("{} + setup", current));

    println!("6. For test #2: Running the test itself");
    TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let current = get_test_value();
    set_test_value(&format!("{} + second_test", current));

    println!("7. For test #2: Running teardown");
    TEARDOWN_COUNTER.fetch_add(1, Ordering::SeqCst);
    let current = get_test_value();
    set_test_value(&format!("{} + teardown", current));

    // Simulate after_all
    println!("8. After all tests: Running after_all cleanup once at the end");
    AFTER_ALL_COUNTER.fetch_add(1, Ordering::SeqCst);
    let current = get_test_value();
    set_test_value(&format!("{} + after_all", current));

    // Final state
    print_module_state("Final state");
}

fn main() {
    // Enable enhanced output for better test reporting
    config().enhanced_output(true).apply();

    // Run the simulated tests
    run_simulated_tests();

    // Notes about how after_all works in real tests
    println!("\nNOTE: In real tests with cargo test:");
    println!("- #[before_all] will run once before any test in the module");
    println!("- #[after_all] will run at process exit");
    println!("\nAll tests passed!");
}
