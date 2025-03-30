use crate::expectation::Expectation;

/// OR modifier trait for chaining expectations
pub trait OrModifier<T> {
    /// Creates an OR-chained expectation that allows for multiple assertions on the same value
    /// This provides a fluent API for alternative assertions:
    /// expect(value).to_be_greater_than(100).or().to_be_less_than(0)
    fn or(&self) -> Self;
}

impl<T: Clone> OrModifier<T> for Expectation<T> {
    /// Returns a new Expectation with the same value, allowing for OR chaining assertions
    /// Note: This simply returns a clone of the expectation with the same properties.
    /// The actual OR logic would be implemented at the assertion level.
    fn or(&self) -> Self {
        Self { value: self.value.clone(), expr_str: self.expr_str, negated: self.negated }
    }
}
