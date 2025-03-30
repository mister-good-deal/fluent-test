use crate::expectation::Expectation;
use std::fmt::Debug;

pub trait EqualityMatchers<T> {
    fn to_equal(self, expected: T) -> Self;
}

impl<T: Debug + PartialEq + Clone> EqualityMatchers<T> for Expectation<T> {
    fn to_equal(self, expected: T) -> Self {
        let result = self.value == expected;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let success_msg = format!("is{not} equal to {:?}", expected);
        let expected_msg = format!("Expected {}{not} to equal {:?}", self.expr_str, expected);
        let received_msg = format!("Received: {:?}", self.value);

        self.handle_assertion_result(success, &success_msg, &expected_msg, &received_msg)
    }
}
