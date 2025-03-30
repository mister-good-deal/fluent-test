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

        let description = format!("is{not} equal to {:?}", expected);

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }
}
