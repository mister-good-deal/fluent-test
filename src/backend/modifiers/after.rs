use crate::backend::assertions::Assertion;
use std::time::Duration;

/// Trait for adding an 'after' time constraint to an assertion
pub trait AfterModifier<T> {
    /// Asserts that the operation completed after the specified duration
    #[allow(dead_code)]
    fn after(self, duration: Duration) -> Self;
}

impl<T> AfterModifier<T> for Assertion<T>
where
    T: Clone + std::fmt::Debug,
{
    fn after(self, duration: Duration) -> Self {
        // Only applies if we have an elapsed time
        if let Some(elapsed) = self.elapsed {
            if elapsed <= duration {
                // Add a failing step
                let message = format!("Expected operation to take more than {:?}, but it took {:?}", duration, elapsed);
                return self.with_custom_message(&message);
            }
        }

        // If no elapsed time or constraint satisfied, return unmodified
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_after_modifier_passing() {
        // Set up an assertion with an elapsed time greater than the constraint
        let assertion = Assertion::with_elapsed(42, "test_value", Duration::from_millis(150));

        // Apply the after modifier
        let result = assertion.after(Duration::from_millis(100));

        // Should not add any steps since the constraint is satisfied
        assert_eq!(result.steps.len(), 0);
    }

    #[test]
    #[should_panic(expected = "Expected operation to take more than")]
    fn test_after_modifier_failing() {
        // Set up an assertion with an elapsed time less than the constraint
        let assertion = Assertion::with_elapsed(42, "test_value", Duration::from_millis(50));

        // Apply the after modifier - this creates a result with a failing step
        let result = assertion.after(Duration::from_millis(100));

        // This should cause a panic with our error message
        result.assert_success();
    }

    #[test]
    fn test_after_modifier_no_elapsed() {
        // Set up a normal assertion without elapsed time
        let assertion = Assertion::new(42, "test_value");

        // Apply the after modifier
        let result = assertion.after(Duration::from_millis(100));

        // Should not add any steps since there's no elapsed time
        assert_eq!(result.steps.len(), 0);
    }
}
