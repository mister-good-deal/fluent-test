use crate::expectation::{Expectation, NotExpectation};
use std::fmt::Debug;

pub trait EqualityMatchers<T> {
    fn to_equal(self, expected: T);
    fn not_to_equal(self, expected: T);
}

impl<T: Debug + PartialEq> EqualityMatchers<T> for Expectation<T> {
    fn to_equal(self, expected: T) {
        let result = self.value == expected;
        let success = if self.negated { !result } else { result };

        if success {
            let msg = if self.negated { "is not equal to" } else { "is equal to" };
            self.report_success(&format!("{} {:?}", msg, expected));
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to equal {:?}", self.expr_str, expected)
            } else {
                format!("Expected {} to equal {:?}", self.expr_str, expected)
            };
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }

    fn not_to_equal(self, expected: T) {
        Expectation { value: self.value, expr_str: self.expr_str, negated: true }.to_equal(expected);
    }
}

// Implement matchers for NotExpectation to support jest-like syntax (expect(x).not.to_equal(y))
impl<T: Debug + PartialEq> EqualityMatchers<T> for NotExpectation<T> {
    fn to_equal(self, expected: T) {
        // Reuse the main implementation but with the negated flag
        let result = self.value == expected;
        let success = if self.negated { !result } else { result };

        if success {
            let msg = if self.negated { "is not equal to" } else { "is equal to" };
            self.report_success(&format!("{} {:?}", msg, expected));
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to equal {:?}", self.expr_str, expected)
            } else {
                format!("Expected {} to equal {:?}", self.expr_str, expected)
            };
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }

    fn not_to_equal(self, expected: T) {
        // This would be a double negative, so it's just a normal to_equal
        NotExpectation {
            value: self.value,
            expr_str: self.expr_str,
            negated: !self.negated, // Toggle the negated flag
        }
        .to_equal(expected);
    }
}
