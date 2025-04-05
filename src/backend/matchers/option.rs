use crate::backend::Expectation;
use crate::backend::assertions::sentence::AssertionSentence;
use std::fmt::Debug;

/// Trait for Option<T> assertions
pub trait OptionMatchers<T: Debug> {
    fn to_be_some(self) -> Self;
    fn to_be_none(self) -> Self;
    fn to_contain(self, expected: &T) -> Self
    where
        T: PartialEq;
}

// Implementation for owned Option<T>
impl<T: Debug + Clone + PartialEq> OptionMatchers<T> for Expectation<Option<T>> {
    fn to_be_some(self) -> Self {
        let result = self.value.is_some();
        let sentence = AssertionSentence::new("be", "some");

        return self.add_assertion_step(sentence, result);
    }

    fn to_be_none(self) -> Self {
        let result = self.value.is_none();
        let sentence = AssertionSentence::new("be", "none");

        return self.add_assertion_step(sentence, result);
    }

    fn to_contain(self, expected: &T) -> Self
    where
        T: PartialEq,
    {
        let result = match &self.value {
            Some(actual) => actual == expected,
            None => false,
        };
        let sentence = AssertionSentence::new("contain", format!("{:?}", expected));

        return self.add_assertion_step(sentence, result);
    }
}

// Implementation for references to Option<T>
impl<T: Debug + Clone + PartialEq> OptionMatchers<T> for Expectation<&Option<T>> {
    fn to_be_some(self) -> Self {
        let result = self.value.is_some();
        let sentence = AssertionSentence::new("be", "some");

        return self.add_assertion_step(sentence, result);
    }

    fn to_be_none(self) -> Self {
        let result = self.value.is_none();
        let sentence = AssertionSentence::new("be", "none");

        return self.add_assertion_step(sentence, result);
    }

    fn to_contain(self, expected: &T) -> Self
    where
        T: PartialEq,
    {
        let result = match self.value {
            Some(actual) => actual == expected,
            None => false,
        };
        let sentence = AssertionSentence::new("contain", format!("{:?}", expected));

        return self.add_assertion_step(sentence, result);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_option_to_be_some() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let some_value: Option<i32> = Some(42);
        let none_value: Option<i32> = None;

        // These should pass
        expect!(some_value.clone()).to_be_some();
        expect!(none_value.clone()).to_be_none();
        expect!(none_value.clone()).not().to_be_some();
        expect!(some_value.clone()).not().to_be_none();
    }

    #[test]
    #[should_panic(expected = "be some")]
    fn test_none_to_be_some_fails() {
        let value: Option<i32> = None;
        let _assertion = expect!(value).to_be_some();
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "be none")]
    fn test_some_to_be_none_fails() {
        let value: Option<i32> = Some(42);
        let _assertion = expect!(value).to_be_none();
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_option_to_contain() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let value: Option<i32> = Some(42);

        // These should pass
        expect!(value.clone()).to_contain(&42);
        expect!(value.clone()).not().to_contain(&43);

        let none_value: Option<i32> = None;
        expect!(none_value).not().to_contain(&42);
    }

    #[test]
    #[should_panic(expected = "contain")]
    fn test_wrong_value_not_fails() {
        let value: Option<i32> = Some(42);
        let _assertion = expect!(value).not().to_contain(&42);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "contain 43")]
    fn test_missing_value_fails() {
        let value: Option<i32> = Some(42);
        let _assertion = expect!(value).to_contain(&43);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "contain 42")]
    fn test_none_value_fails() {
        let value: Option<i32> = None;
        let _assertion = expect!(value).to_contain(&42);
        std::hint::black_box(_assertion);
    }
}
