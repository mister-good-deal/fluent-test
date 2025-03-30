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
