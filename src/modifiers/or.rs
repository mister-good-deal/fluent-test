use crate::assertion::LogicalOp;
use crate::expectation::Expectation;

/// OR modifier trait for chaining expectations
pub trait OrModifier<T> {
    /// Creates an OR-chained expectation that allows for multiple assertions on the same value
    /// This provides a fluent API for alternative assertions:
    /// expect(value).to_be_greater_than(100).or().to_be_less_than(0)
    fn or(self) -> Self;
}

impl<T: Clone> OrModifier<T> for Expectation<T> {
    /// Returns a new Expectation with the same value, allowing for OR chaining assertions
    fn or(self) -> Self {
        // The previous expectation was intermediate (not final)
        let mut result = self;
        result.mark_as_intermediate();

        let mut new_chain = result.chain.clone();
        new_chain.set_last_logic(LogicalOp::Or);

        Self {
            value: result.value.clone(),
            expr_str: result.expr_str,
            negated: result.negated,
            chain: new_chain,
            in_chain: true,  // Always mark as part of a chain
            is_final: false, // This is not the final step - there will be more after 'or()'
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_or_modifier() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let value = 42;

        // Create a variable to hold the chain so it doesn't get dropped immediately
        let chain = expect!(value)
            .to_be_greater_than(100) // This fails
            .or()
            .to_be_less_than(100); // This passes

        // Manually evaluate the chain - should not panic because OR with a passing condition
        // It should return true since one of the conditions passes
        let result = chain.evaluate();
        assert!(result, "OR chain with one passing condition should evaluate to true");
    }

    #[test]
    fn test_or_modifier_failing() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();
        
        let value = 42;
        // Both fail - should return false, not panic
        let chain = expect!(value).to_be_greater_than(100).or().to_be_less_than(10);
        // Manually evaluate the chain - should return false because both conditions fail
        let result = chain.evaluate();
        assert!(!result, "OR chain with all failing conditions should evaluate to false");
    }
}
