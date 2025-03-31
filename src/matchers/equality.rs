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

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_equality() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // Test with integers
        expect!(42).to_equal(42);
        expect!(42).not().to_equal(100);

        // Test with strings
        expect!("hello").to_equal("hello");
        expect!("hello").not().to_equal("world");

        // Test with floating point
        expect!(3.14).to_equal(3.14);
        expect!(3.14).not().to_equal(2.71);

        // Test with boolean
        expect!(true).to_equal(true);
        expect!(true).not().to_equal(false);
    }

    #[test]
    #[should_panic(expected = "is equal to")]
    fn test_equality_fails() {
        // This should fail because 42 != 43
        let _assertion = expect!(42).to_equal(43);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "NOT is equal to")]
    fn test_equality_not_fails() {
        // This should fail because !(42 != 42)
        let _assertion = expect!(42).not().to_equal(42);
        std::hint::black_box(_assertion);
    }
}
