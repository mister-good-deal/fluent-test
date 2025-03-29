use std::fmt::Debug;
use std::ops::{Add, Sub, Div, Rem};
use std::cmp::PartialOrd;
use crate::expectation::Expectation;

pub trait NumericMatchers<T> {
    fn to_be_greater_than(self, expected: T);
    fn to_be_less_than(self, expected: T);
    fn to_be_even(self);
    fn to_be_odd(self);
    fn to_be_divisible_by(self, divisor: T);
    fn to_be_positive(self);
    fn to_be_negative(self);
    fn to_be_in_range<R>(self, range: R) where R: std::ops::RangeBounds<T>;
}

impl<T> NumericMatchers<T> for Expectation<T>
where
    T: Debug + PartialOrd + Copy,
{
    fn to_be_greater_than(self, expected: T) {
        let result = self.value > expected;
        let success = if self.negated { !result } else { result };

        if success {
            let msg = if self.negated {
                "is not greater than"
            } else {
                "is greater than"
            };
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

    // Implement other numeric matchers...
    fn to_be_less_than(self, expected: T) {
        // Similar implementation to to_be_greater_than
        unimplemented!()
    }

    fn to_be_even(self) where T: Rem<Output = T> + From<u8> + PartialEq {
        unimplemented!()
    }

    fn to_be_odd(self) where T: Rem<Output = T> + From<u8> + PartialEq {
        unimplemented!()
    }

    fn to_be_divisible_by(self, divisor: T) where T: Rem<Output = T> + From<u8> + PartialEq {
        unimplemented!()
    }

    fn to_be_positive(self) where T: PartialOrd + From<u8> {
        unimplemented!()
    }

    fn to_be_negative(self) where T: PartialOrd + From<u8> {
        unimplemented!()
    }

    fn to_be_in_range<R>(self, range: R) where R: std::ops::RangeBounds<T> {
        unimplemented!()
    }
}
