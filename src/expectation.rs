use crate::reporter::report_failure;
use crate::reporter::report_success;

/// The core struct that holds a value being tested and provides
/// the fluent API for assertions
pub struct Expectation<T> {
    pub(crate) value: T,
    pub(crate) expr_str: &'static str,
    pub(crate) negated: bool,
}

impl<T> Expectation<T> {
    /// Creates a new expectation
    pub fn new(value: T, expr_str: &'static str) -> Self {
        Self { value, expr_str, negated: false }
    }

    /// Helper to report a successful test
    pub(crate) fn report_success(&self, message: &str) {
        report_success(&format!("{} {}", self.expr_str, message));
    }

    /// Helper to report a failed test
    pub(crate) fn report_failure(&self, expected: &str, received: &str) {
        report_failure(expected, received);
        // Panic to support #[should_panic] test cases
        panic!("Assertion failed: {}", expected);
    }
}

impl<T: Clone> Expectation<T> {
    /// Helper to handle assertion success/failure and return a new expectation
    /// for chaining additional assertions
    pub(crate) fn handle_assertion_result(&self, success: bool, success_msg: &str, expected_msg: &str, received_msg: &str) -> Self {
        if success {
            self.report_success(success_msg);
        } else {
            self.report_failure(expected_msg, received_msg);
        }

        // Clone self to allow chaining further assertions
        Self { value: self.value.clone(), expr_str: self.expr_str, negated: self.negated }
    }
}
