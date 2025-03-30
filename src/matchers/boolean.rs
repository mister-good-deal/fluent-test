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
