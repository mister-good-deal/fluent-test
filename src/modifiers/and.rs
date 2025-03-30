use crate::expectation::Expectation;

/// AND modifier trait for chaining expectations
pub trait AndModifier<T> {
    /// Creates an AND-chained expectation that allows for multiple assertions on the same value
    /// This provides a fluent API for multiple assertions:
    /// expect(value).to_be_greater_than(5).and().to_be_less_than(10)
    fn and(&self) -> Self;
}

impl<T: Clone> AndModifier<T> for Expectation<T> {
    /// Returns a new Expectation with the same value, allowing for chaining assertions
    fn and(&self) -> Self {
        Self { value: self.value.clone(), expr_str: self.expr_str, negated: self.negated }
    }
}
