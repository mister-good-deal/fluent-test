use crate::expectation::Expectation;

pub trait BooleanMatchers {
    fn to_be_true(self) -> Self;
    fn to_be_false(self) -> Self;
}

impl BooleanMatchers for Expectation<bool> {
    fn to_be_true(self) -> Self {
        let result = self.value;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} true");

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_be_false(self) -> Self {
        let result = !self.value;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} false");

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_boolean_true() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // This should pass
        expect!(true).to_be_true();
        expect!(false).not().to_be_true();
    }

    #[test]
    #[should_panic(expected = "is not true")]
    fn test_not_true_fails() {
        // This will evaluate and panic when the Expectation is dropped
        let _assertion = expect!(true).not().to_be_true();
        // Force the value to be dropped at the end of the function
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "is true")]
    fn test_false_to_be_true_fails() {
        // This will evaluate and panic when the Expectation is dropped
        let _assertion = expect!(false).to_be_true();
        // Force the value to be dropped at the end of the function
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_boolean_false() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // This should pass
        expect!(false).to_be_false();
        expect!(true).not().to_be_false();
    }

    #[test]
    #[should_panic(expected = "is not false")]
    fn test_not_false_fails() {
        // This will evaluate and panic when the Expectation is dropped
        let _assertion = expect!(false).not().to_be_false();
        // Force the value to be dropped at the end of the function
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "is false")]
    fn test_true_to_be_false_fails() {
        // This will evaluate and panic when the Expectation is dropped
        let _assertion = expect!(true).to_be_false();
        // Force the value to be dropped at the end of the function
        std::hint::black_box(_assertion);
    }
}
