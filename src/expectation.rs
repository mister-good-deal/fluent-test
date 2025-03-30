use crate::reporter::report_failure;
use crate::reporter::report_success;
use std::ops::Not;

/// The core struct that holds a value being tested and provides
/// the fluent API for assertions
pub struct Expectation<T> {
    pub(crate) value: T,
    pub(crate) expr_str: &'static str,
    pub(crate) negated: bool,
}

/// A negated version of the expectation that allows for chaining methods
/// This creates a nice Jest-like API: expect(value).not.to_equal(x)
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

    /// Creates a negated version of this expectation (for consumers who prefer method chaining)
    /// This is the traditional way to negate an expectation
    pub fn not_method(mut self) -> Self {
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
        // Panic to support #[should_panic] test cases
        panic!("Assertion failed: {}", expected);
    }
}

/// Implementation of the Not operator (!) for Expectation
/// This allows using !expect(value) as a shorthand for expect(value).not_method()
impl<T> Not for Expectation<T> {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        self.negated = !self.negated;
        self
    }
}

// For Jest-style .not property, provide access to a NotExpectation
impl<T: Clone> Expectation<T> {
    /// Returns a negated expectation, mimicking Jest's `.not` property
    /// This is an alternative to using the `.not` property directly
    pub fn not_prop(&self) -> Expectation<T> {
        Expectation { value: self.value.clone(), expr_str: self.expr_str, negated: !self.negated }
    }
}

// Add a property for the Jest-style API using the not field
impl<T: Clone> Expectation<T> {
    /// Simulates the Jest-style .not property
    /// This allows for the syntax: expect(value).not.to_equal(x)
    #[allow(clippy::should_implement_trait)]
    pub fn not(&self) -> NotExpectation<T> {
        NotExpectation { value: self.value.clone(), expr_str: self.expr_str, negated: !self.negated }
    }
}
