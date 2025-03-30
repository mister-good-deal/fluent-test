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
