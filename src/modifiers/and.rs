use crate::assertion::LogicalOp;
use crate::expectation::Expectation;

/// AND modifier trait for chaining expectations
pub trait AndModifier<T> {
    /// Creates an AND-chained expectation that allows for multiple assertions on the same value
    /// This provides a fluent API for multiple assertions:
    /// expect(value).to_be_greater_than(5).and().to_be_less_than(10)
    fn and(self) -> Self;
}

impl<T: Clone> AndModifier<T> for Expectation<T> {
    /// Returns a new Expectation with the same value, allowing for chaining assertions
    fn and(self) -> Self {
        // The previous expectation was intermediate (not final)
        let mut result = self;
        result.mark_as_intermediate();

        let mut new_chain = result.chain.clone();
        new_chain.set_last_logic(LogicalOp::And);

        Self {
            value: result.value.clone(),
            expr_str: result.expr_str,
            negated: result.negated,
            chain: new_chain,
            in_chain: true,  // Always mark as part of a chain
            is_final: false, // This is not the final step - there will be more after 'and()'
        }
    }
}
