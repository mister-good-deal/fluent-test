use crate::reporter::report_failure;
use crate::reporter::report_success;

/// The core struct that holds a value being tested and provides
/// the fluent API for assertions
pub struct Expectation<T> {
    pub(crate) value: T,
    pub(crate) expr_str: &'static str,
    pub(crate) negated: bool,
}

/// A negated version of the expectation that allows for chaining methods
/// This creates a nice Jest-like API: expect(value).not().to_equal(x)
pub struct NotExpectation<T> {
    pub(crate) value: T,
    pub(crate) expr_str: &'static str,
    pub(crate) negated: bool,
}

// Implement the necessary methods for NotExpectation
impl<T> NotExpectation<T> {
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

// Add the not() method for negated expectations
impl<T: Clone> Expectation<T> {
    /// Creates a negated expectation
    /// This provides a fluent API for negated expectations:
    /// expect(value).not().to_equal(x)
    pub fn not(&self) -> NotExpectation<T> {
        NotExpectation { value: self.value.clone(), expr_str: self.expr_str, negated: !self.negated }
    }
}
