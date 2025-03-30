use crate::expectation::Expectation;
use std::fmt::Debug;

pub trait EqualityMatchers<T> {
    fn to_equal(self, expected: T);
}

impl<T: Debug + PartialEq> EqualityMatchers<T> for Expectation<T> {
    fn to_equal(self, expected: T) {
        let result = self.value == expected;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("is{not} equal to {:?}", expected));
        } else {
            let expected_msg = format!("Expected {}{not} to equal {:?}", self.expr_str, expected);
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }
}
