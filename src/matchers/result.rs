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
