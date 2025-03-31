use crate::expectation::Expectation;
use std::fmt::Debug;

pub trait OptionMatchers<T> {
    fn to_be_some(self) -> Self;
    fn to_be_none(self) -> Self;
    fn to_contain_value<U: PartialEq<T> + Debug>(self, expected: U) -> Self;
}

impl<T: Debug + Clone> OptionMatchers<T> for Expectation<&Option<T>> {
    fn to_be_some(self) -> Self {
        let result = self.value.is_some();
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} Some");

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }

    fn to_be_none(self) -> Self {
        let result = self.value.is_none();
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} None");

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }

    fn to_contain_value<U: PartialEq<T> + Debug>(self, expected: U) -> Self {
        let result = match &self.value {
            Some(v) => expected == *v,
            None => false,
        };
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} contain value {:?}", expected);

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
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

        // Positive test cases
        expect!(&some_value).to_be_some();
        expect!(&none_value).not().to_be_some();
    }

    #[test]
    #[should_panic]
    fn test_none_to_be_some_fails() {
        let none_value: Option<i32> = None;
        expect!(&none_value).to_be_some();
    }

    #[test]
    #[should_panic]
    fn test_some_not_to_be_some_fails() {
        let some_value: Option<i32> = Some(42);
        expect!(&some_value).not().to_be_some();
    }

    #[test]
    fn test_option_to_be_none() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let some_value: Option<i32> = Some(42);
        let none_value: Option<i32> = None;

        // Positive test cases
        expect!(&none_value).to_be_none();
        expect!(&some_value).not().to_be_none();
    }

    #[test]
    #[should_panic]
    fn test_some_to_be_none_fails() {
        let some_value: Option<i32> = Some(42);
        expect!(&some_value).to_be_none();
    }

    #[test]
    #[should_panic]
    fn test_none_not_to_be_none_fails() {
        let none_value: Option<i32> = None;
        expect!(&none_value).not().to_be_none();
    }

    #[test]
    fn test_option_to_contain_value() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let some_value: Option<i32> = Some(42);
        let other_value: Option<i32> = Some(100);
        let none_value: Option<i32> = None;

        // Positive test cases
        expect!(&some_value).to_contain_value(42);
        expect!(&other_value).not().to_contain_value(42);
        expect!(&none_value).not().to_contain_value(42);
    }

    #[test]
    #[should_panic]
    fn test_wrong_value_fails() {
        let some_value: Option<i32> = Some(42);
        expect!(&some_value).to_contain_value(100);
    }

    #[test]
    #[should_panic]
    fn test_right_value_not_fails() {
        let some_value: Option<i32> = Some(42);
        expect!(&some_value).not().to_contain_value(42);
    }
}
