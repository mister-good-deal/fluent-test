use crate::expectation::Expectation;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::ops::Rem;

pub trait NumericMatchers<T> {
    fn to_be_greater_than(self, expected: T) -> Self;
    fn to_be_less_than(self, expected: T) -> Self;
    fn to_be_even(self) -> Self
    where
        T: Rem<Output = T> + From<u8> + PartialEq;
    fn to_be_odd(self) -> Self
    where
        T: Rem<Output = T> + From<u8> + PartialEq;
    fn to_be_divisible_by(self, divisor: T) -> Self
    where
        T: Rem<Output = T> + From<u8> + PartialEq;
    fn to_be_positive(self) -> Self
    where
        T: PartialOrd + From<u8>;
    fn to_be_negative(self) -> Self
    where
        T: PartialOrd + From<u8>;
    fn to_be_in_range<R>(self, range: R) -> Self
    where
        R: std::ops::RangeBounds<T> + std::fmt::Debug;
}

impl<T> NumericMatchers<T> for Expectation<T>
where
    T: Debug + PartialOrd + Copy,
{
    fn to_be_greater_than(self, expected: T) -> Self {
        let result = self.value > expected;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} greater than {:?}", expected);

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }

    fn to_be_less_than(self, expected: T) -> Self {
        let result = self.value < expected;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} less than {:?}", expected);

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }

    fn to_be_even(self) -> Self
    where
        T: Rem<Output = T> + From<u8> + PartialEq,
    {
        let zero = T::from(0);
        let two = T::from(2);
        let result = self.value % two == zero;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} even");

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }

    fn to_be_odd(self) -> Self
    where
        T: Rem<Output = T> + From<u8> + PartialEq,
    {
        let zero = T::from(0);
        let two = T::from(2);
        let result = self.value % two != zero;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} odd");

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }

    fn to_be_divisible_by(self, divisor: T) -> Self
    where
        T: Rem<Output = T> + From<u8> + PartialEq,
    {
        let zero = T::from(0);
        let result = self.value % divisor == zero;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} divisible by {:?}", divisor);

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }

    fn to_be_positive(self) -> Self
    where
        T: PartialOrd + From<u8>,
    {
        let zero = T::from(0);
        let result = self.value > zero;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} positive");

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }

    fn to_be_negative(self) -> Self
    where
        T: PartialOrd + From<u8>,
    {
        let zero = T::from(0);
        let result = self.value < zero;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} negative");

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }

    fn to_be_in_range<R>(self, range: R) -> Self
    where
        R: std::ops::RangeBounds<T> + std::fmt::Debug,
    {
        use std::ops::Bound;

        let result = match (range.start_bound(), range.end_bound()) {
            (Bound::Included(start), Bound::Included(end)) => self.value >= *start && self.value <= *end,
            (Bound::Included(start), Bound::Excluded(end)) => self.value >= *start && self.value < *end,
            (Bound::Excluded(start), Bound::Included(end)) => self.value > *start && self.value <= *end,
            (Bound::Excluded(start), Bound::Excluded(end)) => self.value > *start && self.value < *end,
            (Bound::Included(start), Bound::Unbounded) => self.value >= *start,
            (Bound::Excluded(start), Bound::Unbounded) => self.value > *start,
            (Bound::Unbounded, Bound::Included(end)) => self.value <= *end,
            (Bound::Unbounded, Bound::Excluded(end)) => self.value < *end,
            (Bound::Unbounded, Bound::Unbounded) => true,
        };

        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };
        let range_str = format!("{:?}", range);

        let description = format!("is{not} in range {}", range_str);

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_greater_than() {
        crate::Reporter::disable_deduplication();
        
        expect!(42).to_be_greater_than(30);
        expect!(42).not().to_be_greater_than(50);
    }

    #[test]
    #[should_panic(expected = "is greater than")]
    fn test_greater_than_fails() {
        let _assertion = expect!(30).to_be_greater_than(42);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "NOT is greater than")]
    fn test_not_greater_than_fails() {
        let _assertion = expect!(42).not().to_be_greater_than(30);
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_less_than() {
        crate::Reporter::disable_deduplication();
        
        expect!(30).to_be_less_than(42);
        expect!(42).not().to_be_less_than(30);
    }

    #[test]
    #[should_panic(expected = "is less than")]
    fn test_less_than_fails() {
        let _assertion = expect!(42).to_be_less_than(30);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "NOT is less than")]
    fn test_not_less_than_fails() {
        let _assertion = expect!(30).not().to_be_less_than(42);
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_even_odd() {
        crate::Reporter::disable_deduplication();
        
        // Even tests
        expect!(42).to_be_even();
        expect!(43).not().to_be_even();
        
        // Odd tests
        expect!(43).to_be_odd();
        expect!(42).not().to_be_odd();
    }

    #[test]
    #[should_panic(expected = "is even")]
    fn test_even_fails() {
        let _assertion = expect!(43).to_be_even();
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "NOT is even")]
    fn test_not_even_fails() {
        let _assertion = expect!(42).not().to_be_even();
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "is odd")]
    fn test_odd_fails() {
        let _assertion = expect!(42).to_be_odd();
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "NOT is odd")]
    fn test_not_odd_fails() {
        let _assertion = expect!(43).not().to_be_odd();
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_divisible_by() {
        crate::Reporter::disable_deduplication();
        
        expect!(42).to_be_divisible_by(7);
        expect!(42).not().to_be_divisible_by(5);
    }

    #[test]
    #[should_panic(expected = "is divisible by")]
    fn test_divisible_by_fails() {
        let _assertion = expect!(42).to_be_divisible_by(5);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "NOT is divisible by")]
    fn test_not_divisible_by_fails() {
        let _assertion = expect!(42).not().to_be_divisible_by(7);
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_positive_negative() {
        crate::Reporter::disable_deduplication();
        
        // Positive tests
        expect!(42).to_be_positive();
        expect!(-42).not().to_be_positive();
        
        // Negative tests
        expect!(-42).to_be_negative();
        expect!(42).not().to_be_negative();
    }

    #[test]
    #[should_panic(expected = "is positive")]
    fn test_positive_fails() {
        let _assertion = expect!(-42).to_be_positive();
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "NOT is positive")]
    fn test_not_positive_fails() {
        let _assertion = expect!(42).not().to_be_positive();
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "is negative")]
    fn test_negative_fails() {
        let _assertion = expect!(42).to_be_negative();
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "NOT is negative")]
    fn test_not_negative_fails() {
        let _assertion = expect!(-42).not().to_be_negative();
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_in_range() {
        crate::Reporter::disable_deduplication();
        
        // Test different range types
        expect!(42).to_be_in_range(40..=50); // Inclusive range
        expect!(42).to_be_in_range(40..51);  // Exclusive range
        expect!(42).to_be_in_range(40..);    // Open-ended range
        expect!(42).to_be_in_range(..51);    // From zero range
        
        // Test negations
        expect!(30).not().to_be_in_range(40..50);
    }

    #[test]
    #[should_panic(expected = "is in range")]
    fn test_in_range_fails() {
        let _assertion = expect!(30).to_be_in_range(40..50);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "NOT is in range")]
    fn test_not_in_range_fails() {
        let _assertion = expect!(42).not().to_be_in_range(40..50);
        std::hint::black_box(_assertion);
    }
}
