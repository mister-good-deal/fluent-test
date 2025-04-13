//! Module for test fixtures support with setup and teardown capabilities
//!
//! This module provides the runtime functionality for test fixtures using attributes.
//! It works with procedural macros to provide a clean API for setting up and tearing
//! down test environments.

use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::collections::HashMap;
use std::panic::{self, AssertUnwindSafe};
use std::sync::Mutex;

/// Simple fixture registration system that uses a global hashmap instead of inventory
pub type FixtureFunc = Box<dyn Fn() + Send + Sync + 'static>;

static SETUP_FIXTURES: Lazy<Mutex<HashMap<&'static str, Vec<FixtureFunc>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

static TEARDOWN_FIXTURES: Lazy<Mutex<HashMap<&'static str, Vec<FixtureFunc>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Register a setup function for a module
///
/// This is automatically called by the `#[setup]` attribute macro.
pub fn register_setup(module_path: &'static str, func: FixtureFunc) {
    let mut fixtures = SETUP_FIXTURES.lock().unwrap();
    fixtures.entry(module_path).or_default().push(func);
}

/// Register a teardown function for a module
///
/// This is automatically called by the `#[tear_down]` attribute macro.
pub fn register_teardown(module_path: &'static str, func: FixtureFunc) {
    let mut fixtures = TEARDOWN_FIXTURES.lock().unwrap();
    fixtures.entry(module_path).or_default().push(func);
}

thread_local! {
    /// Indicator of whether we're currently in a fixture-wrapped test
    static IN_FIXTURE_TEST: RefCell<bool> = const { RefCell::new(false) };
}

/// Run a test function with appropriate setup and teardown
///
/// This is automatically called by the `#[with_fixtures]` attribute macro.
pub fn run_test_with_fixtures<F>(module_path: &'static str, test_fn: AssertUnwindSafe<F>)
where
    F: FnOnce(),
{
    // Set the fixture test flag
    IN_FIXTURE_TEST.with(|flag| {
        *flag.borrow_mut() = true;
    });

    // Run setup functions for this module if any exist
    if let Ok(fixtures) = SETUP_FIXTURES.lock() {
        if let Some(setup_funcs) = fixtures.get(module_path) {
            for setup_fn in setup_funcs {
                setup_fn();
            }
        }
    }

    // Run the test function, capturing any panics
    let result = panic::catch_unwind(test_fn);

    // Always run teardown, even if the test panics
    if let Ok(fixtures) = TEARDOWN_FIXTURES.lock() {
        if let Some(teardown_funcs) = fixtures.get(module_path) {
            for teardown_fn in teardown_funcs {
                teardown_fn();
            }
        }
    }

    // Reset the fixture test flag
    IN_FIXTURE_TEST.with(|flag| {
        *flag.borrow_mut() = false;
    });

    // Re-throw any panic that occurred during the test
    if let Err(err) = result {
        panic::resume_unwind(err);
    }
}

/// Check if we're running inside a fixture-wrapped test
pub fn is_in_fixture_test() -> bool {
    return IN_FIXTURE_TEST.with(|flag| *flag.borrow());
}
