use std::fmt::Debug;
use crate::reporter::report_success;
use crate::reporter::report_failure;

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
        Self {
            value,
            expr_str,
            negated: false,
        }
    }

    /// Negates the next assertion
    pub fn not(mut self) -> Self {
        self.negated = !self.negated;
        self
    }

    /// Helper to report a successful test
    pub(crate) fn report_success(&self, message: &str) {
        report_success(&format!("{} {}", self.expr_str, message));
    }

    /// Helper to report a failed test
    pub(crate) fn report_failure(&self, expected: &str, received: &str) {
        report_failure(expected, received);
    }
}
