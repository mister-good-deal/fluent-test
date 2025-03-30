use crate::expectation::Expectation;

/// Not modifier trait for negating expectations
pub trait NotModifier<T> {
    /// Creates a negated expectation
    fn not(self) -> Self;
}

impl<T: Clone> NotModifier<T> for Expectation<T> {
    /// Creates a negated expectation
    /// This provides a fluent API for negated expectations:
    /// expect(value).not().to_equal(x)
    fn not(self) -> Self {
        Self {
            value: self.value.clone(),
            expr_str: self.expr_str,
            negated: !self.negated,
            chain: self.chain.clone(),
            in_chain: self.in_chain, // Preserve chain status
            is_final: self.is_final, // Preserve finality status
        }
    }
}
