use crate::backend::assertions::Assertion;
use std::time::Duration;

/// Trait for adding a 'before' time constraint to an assertion
pub trait BeforeModifier<T> {
    /// Asserts that the operation completed before the specified duration
    #[allow(dead_code)]
    fn before(self, duration: Duration) -> Self;
}

impl<T> BeforeModifier<T> for Assertion<T>
where
    T: Clone + std::fmt::Debug,
{
    fn before(self, duration: Duration) -> Self {
        // Only applies if we have an elapsed time
        if let Some(elapsed) = self.elapsed {
            if elapsed >= duration {
                // Add a failing step
                let message = format!("Expected operation to complete in less than {:?}, but it took {:?}", duration, elapsed);
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
    fn test_before_modifier_passing() {
        // Set up an assertion with an elapsed time less than the constraint
        let assertion = Assertion::with_elapsed(42, "test_value", Duration::from_millis(50));

        // Apply the before modifier
        let result = assertion.before(Duration::from_millis(100));

        // Should not add any steps since the constraint is satisfied
        assert_eq!(result.steps.len(), 0);
    }

    #[test]
    #[should_panic(expected = "Expected operation to complete in less than")]
    fn test_before_modifier_failing() {
        // Set up an assertion with an elapsed time greater than the constraint
        let assertion = Assertion::with_elapsed(42, "test_value", Duration::from_millis(150));

        // Apply the before modifier - this creates a result with a failing step
        let result = assertion.before(Duration::from_millis(100));

        // This should cause a panic with our error message
        result.assert_success();
    }

    #[test]
    fn test_before_modifier_no_elapsed() {
        // Set up a normal assertion without elapsed time
        let assertion = Assertion::new(42, "test_value");

        // Apply the before modifier
        let result = assertion.before(Duration::from_millis(100));

        // Should not add any steps since there's no elapsed time
        assert_eq!(result.steps.len(), 0);
    }
}
