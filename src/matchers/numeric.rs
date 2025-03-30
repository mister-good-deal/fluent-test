use crate::expectation::Expectation;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::ops::Rem;

pub trait NumericMatchers<T> {
    fn to_be_greater_than(self, expected: T);
    fn to_be_less_than(self, expected: T);
    fn to_be_even(self)
    where
        T: Rem<Output = T> + From<u8> + PartialEq;
    fn to_be_odd(self)
    where
        T: Rem<Output = T> + From<u8> + PartialEq;
    fn to_be_divisible_by(self, divisor: T)
    where
        T: Rem<Output = T> + From<u8> + PartialEq;
    fn to_be_positive(self)
    where
        T: PartialOrd + From<u8>;
    fn to_be_negative(self)
    where
        T: PartialOrd + From<u8>;
    fn to_be_in_range<R>(self, range: R)
    where
        R: std::ops::RangeBounds<T> + std::fmt::Debug;
}

impl<T> NumericMatchers<T> for Expectation<T>
where
    T: Debug + PartialOrd + Copy,
{
    fn to_be_greater_than(self, expected: T) {
        let result = self.value > expected;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("is{not} greater than {:?}", expected));
        } else {
            let expected_msg = format!("Expected {}{not} to be greater than {:?}", self.expr_str, expected);
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }

    fn to_be_less_than(self, expected: T) {
        let result = self.value < expected;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("is{not} less than {:?}", expected));
        } else {
            let expected_msg = format!("Expected {}{not} to be less than {:?}", self.expr_str, expected);
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }

    fn to_be_even(self)
    where
        T: Rem<Output = T> + From<u8> + PartialEq,
    {
        let zero = T::from(0);
        let two = T::from(2);
        let result = self.value % two == zero;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("is{not} even"));
        } else {
            let expected_msg = format!("Expected {}{not} to be even", self.expr_str);
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }

    fn to_be_odd(self)
    where
        T: Rem<Output = T> + From<u8> + PartialEq,
    {
        let zero = T::from(0);
        let two = T::from(2);
        let result = self.value % two != zero;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("is{not} odd"));
        } else {
            let expected_msg = format!("Expected {}{not} to be odd", self.expr_str);
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }

    fn to_be_divisible_by(self, divisor: T)
    where
        T: Rem<Output = T> + From<u8> + PartialEq,
    {
        let zero = T::from(0);
        let result = self.value % divisor == zero;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("is{not} divisible by {:?}", divisor));
        } else {
            let expected_msg = format!("Expected {}{not} to be divisible by {:?}", self.expr_str, divisor);
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }

    fn to_be_positive(self)
    where
        T: PartialOrd + From<u8>,
    {
        let zero = T::from(0);
        let result = self.value > zero;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("is{not} positive"));
        } else {
            let expected_msg = format!("Expected {}{not} to be positive", self.expr_str);
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }

    fn to_be_negative(self)
    where
        T: PartialOrd + From<u8>,
    {
        let zero = T::from(0);
        let result = self.value < zero;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("is{not} negative"));
        } else {
            let expected_msg = format!("Expected {}{not} to be negative", self.expr_str);
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }

    fn to_be_in_range<R>(self, range: R)
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

        if success {
            self.report_success(&format!("is{not} in range {}", range_str));
        } else {
            let expected_msg = format!("Expected {}{not} to be in range {}", self.expr_str, range_str);

            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }
}
