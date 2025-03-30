use crate::expectation::Expectation;
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
            let msg = if self.negated {
                "is not equal to"
            } else {
                "is equal to"
            };
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
        Expectation {
            value: self.value,
            expr_str: self.expr_str,
            negated: true,
        }
        .to_equal(expected);
    }
}
