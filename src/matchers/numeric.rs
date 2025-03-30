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

        if success {
            let msg = if self.negated { "is not greater than" } else { "is greater than" };
            self.report_success(&format!("{} {:?}", msg, expected));
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to be greater than {:?}", self.expr_str, expected)
            } else {
                format!("Expected {} to be greater than {:?}", self.expr_str, expected)
            };
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }

    fn to_be_less_than(self, expected: T) {
        let result = self.value < expected;
        let success = if self.negated { !result } else { result };

        if success {
            let msg = if self.negated { "is not less than" } else { "is less than" };
            self.report_success(&format!("{} {:?}", msg, expected));
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to be less than {:?}", self.expr_str, expected)
            } else {
                format!("Expected {} to be less than {:?}", self.expr_str, expected)
            };
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

        if success {
            let msg = if self.negated { "is not even" } else { "is even" };
            self.report_success(msg);
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to be even", self.expr_str)
            } else {
                format!("Expected {} to be even", self.expr_str)
            };
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

        if success {
            let msg = if self.negated { "is not odd" } else { "is odd" };
            self.report_success(msg);
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to be odd", self.expr_str)
            } else {
                format!("Expected {} to be odd", self.expr_str)
            };
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

        if success {
            let msg = if self.negated { "is not divisible by" } else { "is divisible by" };
            self.report_success(&format!("{} {:?}", msg, divisor));
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to be divisible by {:?}", self.expr_str, divisor)
            } else {
                format!("Expected {} to be divisible by {:?}", self.expr_str, divisor)
            };
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

        if success {
            let msg = if self.negated { "is not positive" } else { "is positive" };
            self.report_success(msg);
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to be positive", self.expr_str)
            } else {
                format!("Expected {} to be positive", self.expr_str)
            };
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

        if success {
            let msg = if self.negated { "is not negative" } else { "is negative" };
            self.report_success(msg);
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to be negative", self.expr_str)
            } else {
                format!("Expected {} to be negative", self.expr_str)
            };
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

        if success {
            let msg = if self.negated { "is not in range" } else { "is in range" };
            self.report_success(msg);
        } else {
            let range_str = format!("{:?}", range);
            let expected_msg = if self.negated {
                format!("Expected {} not to be in range {}", self.expr_str, range_str)
            } else {
                format!("Expected {} to be in range {}", self.expr_str, range_str)
            };
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }
}
