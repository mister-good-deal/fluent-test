use crate::expectation::Expectation;
use std::fmt::Debug;

pub trait ResultMatchers<T, E> {
    fn to_be_ok(self) -> Self;
    fn to_be_err(self) -> Self;
    fn to_contain_ok<U: PartialEq<T> + Debug>(self, expected: U) -> Self;
    fn to_contain_err<U: PartialEq<E> + Debug>(self, expected: U) -> Self;
}

impl<T: Debug + Clone, E: Debug + Clone> ResultMatchers<T, E> for Expectation<&Result<T, E>> {
    fn to_be_ok(self) -> Self {
        let result = self.value.is_ok();
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} Ok");

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }

    fn to_be_err(self) -> Self {
        let result = self.value.is_err();
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} Err");

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }

    fn to_contain_ok<U: PartialEq<T> + Debug>(self, expected: U) -> Self {
        let result = match &self.value {
            Ok(v) => expected == *v,
            Err(_) => false,
        };
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} contain Ok value {:?}", expected);

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }

    fn to_contain_err<U: PartialEq<E> + Debug>(self, expected: U) -> Self {
        let result = match &self.value {
            Ok(_) => false,
            Err(e) => expected == *e,
        };
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} contain Err value {:?}", expected);

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_result_to_be_ok() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let ok_result: Result<i32, &str> = Ok(42);
        let err_result: Result<i32, &str> = Err("error");

        // Positive test cases
        expect!(&ok_result).to_be_ok();
        expect!(&err_result).not().to_be_ok();
    }

    #[test]
    #[should_panic]
    fn test_err_to_be_ok_fails() {
        let err_result: Result<i32, &str> = Err("error");
        expect!(&err_result).to_be_ok();
    }

    #[test]
    #[should_panic]
    fn test_ok_not_to_be_ok_fails() {
        let ok_result: Result<i32, &str> = Ok(42);
        expect!(&ok_result).not().to_be_ok();
    }

    #[test]
    fn test_result_to_be_err() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let ok_result: Result<i32, &str> = Ok(42);
        let err_result: Result<i32, &str> = Err("error");

        // Positive test cases
        expect!(&err_result).to_be_err();
        expect!(&ok_result).not().to_be_err();
    }

    #[test]
    #[should_panic]
    fn test_ok_to_be_err_fails() {
        let ok_result: Result<i32, &str> = Ok(42);
        expect!(&ok_result).to_be_err();
    }

    #[test]
    #[should_panic]
    fn test_err_not_to_be_err_fails() {
        let err_result: Result<i32, &str> = Err("error");
        expect!(&err_result).not().to_be_err();
    }

    #[test]
    fn test_result_to_contain_ok() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let ok_result: Result<i32, &str> = Ok(42);
        let different_ok: Result<i32, &str> = Ok(100);
        let err_result: Result<i32, &str> = Err("error");

        // Positive test cases
        expect!(&ok_result).to_contain_ok(42);
        expect!(&different_ok).not().to_contain_ok(42);
        expect!(&err_result).not().to_contain_ok(42);
    }

    #[test]
    #[should_panic]
    fn test_wrong_ok_value_fails() {
        let ok_result: Result<i32, &str> = Ok(42);
        expect!(&ok_result).to_contain_ok(100);
    }

    #[test]
    #[should_panic]
    fn test_right_ok_value_not_fails() {
        let ok_result: Result<i32, &str> = Ok(42);
        expect!(&ok_result).not().to_contain_ok(42);
    }

    #[test]
    fn test_result_to_contain_err() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let err_result: Result<i32, &str> = Err("network error");
        let different_err: Result<i32, &str> = Err("authentication error");
        let ok_result: Result<i32, &str> = Ok(42);

        // Positive test cases
        expect!(&err_result).to_contain_err("network error");
        expect!(&different_err).not().to_contain_err("network error");
        expect!(&ok_result).not().to_contain_err("network error");
    }

    #[test]
    #[should_panic]
    fn test_wrong_err_value_fails() {
        let err_result: Result<i32, &str> = Err("network error");
        expect!(&err_result).to_contain_err("authentication error");
    }

    #[test]
    #[should_panic]
    fn test_right_err_value_not_fails() {
        let err_result: Result<i32, &str> = Err("network error");
        expect!(&err_result).not().to_contain_err("network error");
    }
}
