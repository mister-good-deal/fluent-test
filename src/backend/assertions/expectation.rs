use super::chain::AssertionChain;
use super::sentence::AssertionSentence;

/// The core struct that holds a value being tested and provides
/// the fluent API for assertions
pub struct Expectation<T> {
    pub(crate) value: T,
    pub(crate) expr_str: &'static str,
    pub(crate) negated: bool,
    pub(crate) chain: AssertionChain,
    pub(crate) in_chain: bool, // Flag to track if this is part of a chain
    pub(crate) is_final: bool, // Flag to mark the final step in a chain
}

impl<T> Expectation<T> {
    /// Creates a new expectation
    pub fn new(value: T, expr_str: &'static str) -> Self {
        return Self {
            value,
            expr_str,
            negated: false,
            chain: AssertionChain::new(expr_str),
            in_chain: false,
            is_final: true, // By default, single-step assertions are final
        };
    }

    /// Add an assertion step using an AssertionSentence and get back a cloned Expectation for chaining
    pub fn add_assertion_step(&self, mut sentence: AssertionSentence, result: bool) -> Self
    where
        T: Clone,
    {
        let mut new_chain = self.chain.clone();

        // Set the negation from the expectation
        sentence = sentence.with_negation(self.negated);

        // Set the subject from the chain
        sentence.subject = self.expr_str.to_string();

        // Calculate the final pass/fail result with negation applied
        let passed = if self.negated { !result } else { result };

        // Add to the chain
        new_chain.steps.push(super::chain::AssertionStep { sentence, passed, logical_op: None });

        return Self {
            value: self.value.clone(),
            expr_str: self.expr_str,
            negated: false, // Reset negation after using it
            chain: new_chain,
            in_chain: true, // Mark this as part of a chain to prevent premature evaluation
            is_final: true, // This step is final until a modifier makes it non-final
        };
    }

    /// Mark this expectation as non-final (intermediate step in a chain)
    pub fn mark_as_intermediate(&mut self) {
        self.is_final = false;
    }

    /// Mark this expectation as final (last step in a chain)
    pub fn mark_as_final(&mut self) {
        self.is_final = true;
    }

    /// Explicitly evaluate the assertion chain
    /// This will be called automatically when the Expectation drops
    /// Returns true if the assertion passed, false otherwise
    pub fn evaluate(self) -> bool
    where
        T: Clone,
    {
        // In tests with #[should_panic], we need to evaluate regardless of finality
        let in_test = std::thread::current().name().unwrap_or("").starts_with("test_");
        let force_evaluate = in_test && !self.chain.steps.is_empty();

        // Only evaluate non-final assertions in test context
        if !self.is_final && !force_evaluate {
            return true; // Non-final assertions don't report on their own
        }

        // Final assertions or test assertions always evaluate
        return self.chain.clone().evaluate();
    }
}

thread_local! {
    static EVALUATION_IN_PROGRESS: std::cell::RefCell<bool> = const { std::cell::RefCell::new(false) };
}

/// For automatic evaluation of assertions when the Expectation drops
impl<T> Drop for Expectation<T> {
    fn drop(&mut self) {
        // We can't move out of self in drop, but we can clone the chain
        // Skip if the chain is empty or if we're dropping during a panic
        // (to avoid double-panics)
        if !self.chain.steps.is_empty() && !std::thread::panicking() {
            // Only evaluate final assertions, not intermediate steps in a chain
            if !self.is_final {
                return;
            }

            // Only evaluate if we're not already in the middle of an evaluation
            let should_evaluate = EVALUATION_IN_PROGRESS.with(|flag| {
                let is_evaluating = *flag.borrow();
                if !is_evaluating {
                    *flag.borrow_mut() = true;
                    return true;
                } else {
                    return false;
                }
            });

            if should_evaluate {
                let chain = self.chain.clone();
                // We don't need to check the result in the Drop implementation
                let _ = chain.evaluate();

                // Reset the flag
                EVALUATION_IN_PROGRESS.with(|flag| {
                    *flag.borrow_mut() = false;
                });
            }
        }
    }
}
