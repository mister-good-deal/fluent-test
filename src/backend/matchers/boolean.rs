use crate::backend::Expectation;
use crate::backend::assertions::sentence::AssertionSentence;

pub trait BooleanMatchers {
    fn to_be_true(self) -> Self;
    fn to_be_false(self) -> Self;
}

impl BooleanMatchers for Expectation<bool> {
    fn to_be_true(self) -> Self {
        let result = self.value;
        let sentence = AssertionSentence::new("be", "true");

        return self.add_assertion_step(sentence, result);
    }

    fn to_be_false(self) -> Self {
        let result = !self.value;
        let sentence = AssertionSentence::new("be", "false");

        return self.add_assertion_step(sentence, result);
    }
}

// Implementation for references to bool
impl BooleanMatchers for Expectation<&bool> {
    fn to_be_true(self) -> Self {
        let result = *self.value;
        let sentence = AssertionSentence::new("be", "true");

        return self.add_assertion_step(sentence, result);
    }

    fn to_be_false(self) -> Self {
        let result = !*self.value;
        let sentence = AssertionSentence::new("be", "false");

        return self.add_assertion_step(sentence, result);
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
    #[should_panic(expected = "not be true")]
    fn test_not_true_fails() {
        // This will evaluate and panic when the Expectation is dropped
        let _assertion = expect!(true).not().to_be_true();
        // Force the value to be dropped at the end of the function
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "be true")]
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
    #[should_panic(expected = "not be false")]
    fn test_not_false_fails() {
        // This will evaluate and panic when the Expectation is dropped
        let _assertion = expect!(false).not().to_be_false();
        // Force the value to be dropped at the end of the function
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "be false")]
    fn test_true_to_be_false_fails() {
        // This will evaluate and panic when the Expectation is dropped
        let _assertion = expect!(true).to_be_false();
        // Force the value to be dropped at the end of the function
        std::hint::black_box(_assertion);
    }
}
