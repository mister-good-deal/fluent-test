use crate::backend::assertions::Assertion;
use std::time::Instant;

/// Trait for adding a 'by_time' constraint to an assertion
pub trait ByTimeModifier<T> {
    /// Asserts that the operation completed by the specified instant
    #[allow(dead_code)]
    fn by_time(self, deadline: Instant) -> Self;
}

impl<T> ByTimeModifier<T> for Assertion<T>
where
    T: Clone + std::fmt::Debug,
{
    fn by_time(self, deadline: Instant) -> Self {
        // Get the current time
        let now = Instant::now();

        if now > deadline {
            // Add a failing step
            let message = format!("Expected operation to complete by {:?}, but it completed at {:?}", deadline, now);
            return self.with_custom_message(&message);
        }

        // If deadline not passed, return unmodified
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn test_by_time_modifier_passing() {
        // Set up an assertion with a future deadline
        let deadline = Instant::now() + Duration::from_millis(100);
        let assertion = Assertion::new(42, "test_value");

        // Apply the by_time modifier
        let result = assertion.by_time(deadline);

        // Should not add any steps since the deadline hasn't passed
        assert_eq!(result.steps.len(), 0);
    }

    #[test]
    #[should_panic(expected = "Expected operation to complete by")]
    fn test_by_time_modifier_failing() {
        // Set up an assertion with a past deadline
        let deadline = Instant::now() - Duration::from_millis(100);
        let assertion = Assertion::new(42, "test_value");

        // Apply the by_time modifier - this creates a result with a failing step
        let result = assertion.by_time(deadline);

        // This should cause a panic with our error message
        result.assert_success();
    }
}
