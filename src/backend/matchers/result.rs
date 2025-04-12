use crate::backend::Assertion;
use crate::backend::assertions::sentence::AssertionSentence;
use std::fmt::Debug;

/// Trait for Result<T, E> assertions
pub trait ResultMatchers<T: Debug, E: Debug> {
    fn to_be_ok(self) -> Self;
    fn to_be_err(self) -> Self;
    fn to_contain_ok<U: PartialEq<T> + Debug>(self, expected: &U) -> Self;
    fn to_contain_err<U: PartialEq<E> + Debug>(self, expected: &U) -> Self;
}

impl<T: Debug + Clone, E: Debug + Clone> ResultMatchers<T, E> for Assertion<Result<T, E>> {
    fn to_be_ok(self) -> Self {
        let result = self.value.is_ok();
        let sentence = AssertionSentence::new("be", "ok");

        return self.add_step(sentence, result);
    }

    fn to_be_err(self) -> Self {
        let result = self.value.is_err();
        let sentence = AssertionSentence::new("be", "err");

        return self.add_step(sentence, result);
    }

    fn to_contain_ok<U: PartialEq<T> + Debug>(self, expected: &U) -> Self {
        let result = match &self.value {
            Ok(actual) => expected == actual,
            Err(_) => false,
        };

        let sentence = AssertionSentence::new("contain", format!("ok value {:?}", expected));

        return self.add_step(sentence, result);
    }

    fn to_contain_err<U: PartialEq<E> + Debug>(self, expected: &U) -> Self {
        let result = match &self.value {
            Ok(_) => false,
            Err(actual) => expected == actual,
        };

        let sentence = AssertionSentence::new("contain", format!("err value {:?}", expected));

        return self.add_step(sentence, result);
    }
}

// Implementation for references to Result<T, E>
impl<T: Debug + Clone, E: Debug + Clone> ResultMatchers<T, E> for Assertion<&Result<T, E>> {
    fn to_be_ok(self) -> Self {
        let result = self.value.is_ok();
        let sentence = AssertionSentence::new("be", "ok");

        return self.add_step(sentence, result);
    }

    fn to_be_err(self) -> Self {
        let result = self.value.is_err();
        let sentence = AssertionSentence::new("be", "err");

        return self.add_step(sentence, result);
    }

    fn to_contain_ok<U: PartialEq<T> + Debug>(self, expected: &U) -> Self {
        let result = match &self.value {
            Ok(actual) => expected == actual,
            Err(_) => false,
        };

        let sentence = AssertionSentence::new("contain", format!("ok value {:?}", expected));

        return self.add_step(sentence, result);
    }

    fn to_contain_err<U: PartialEq<E> + Debug>(self, expected: &U) -> Self {
        let result = match &self.value {
            Ok(_) => false,
            Err(actual) => expected == actual,
        };

        let sentence = AssertionSentence::new("contain", format!("err value {:?}", expected));

        return self.add_step(sentence, result);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_result_to_be_ok() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let ok_value: Result<i32, &str> = Ok(42);
        let err_value: Result<i32, &str> = Err("error");

        // These should pass
        expect!(ok_value).to_be_ok();
        expect!(err_value).to_be_err();
        expect!(err_value).not().to_be_ok();
        expect!(ok_value).not().to_be_err();
    }

    #[test]
    #[should_panic(expected = "be ok")]
    fn test_err_to_be_ok_fails() {
        let value: Result<i32, &str> = Err("error");
        let _assertion = expect!(value).to_be_ok();
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "be err")]
    fn test_ok_to_be_err_fails() {
        let value: Result<i32, &str> = Ok(42);
        let _assertion = expect!(value).to_be_err();
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_result_contain_values() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let ok_value: Result<i32, &str> = Ok(42);
        let err_value: Result<i32, &str> = Err("error");

        // These should pass
        expect!(ok_value).to_contain_ok(&42);
        expect!(ok_value).not().to_contain_ok(&43);
        expect!(err_value).to_contain_err(&"error");
        expect!(err_value).not().to_contain_err(&"different");
    }

    #[test]
    #[should_panic(expected = "contain ok value")]
    fn test_ok_wrong_value_fails() {
        let value: Result<i32, &str> = Ok(42);
        let _assertion = expect!(value).to_contain_ok(&43);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "not contain ok value")]
    fn test_ok_right_value_not_fails() {
        let value: Result<i32, &str> = Ok(42);
        let _assertion = expect!(value).not().to_contain_ok(&42);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "contain err value")]
    fn test_err_wrong_value_fails() {
        let value: Result<i32, &str> = Err("error");
        let _assertion = expect!(value).to_contain_err(&"different");
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "not contain err value")]
    fn test_err_right_value_not_fails() {
        let value: Result<i32, &str> = Err("error");
        let _assertion = expect!(value).not().to_contain_err(&"error");
        std::hint::black_box(_assertion);
    }
}
