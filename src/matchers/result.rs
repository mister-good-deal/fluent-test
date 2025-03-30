use crate::expectation::Expectation;
use std::fmt::Debug;

pub trait ResultMatchers<T, E> {
    fn to_be_ok(self);
    fn to_be_err(self);
    fn to_contain_ok<U: PartialEq<T> + Debug>(self, expected: U);
    fn to_contain_err<U: PartialEq<E> + Debug>(self, expected: U);
}

impl<T: Debug + Clone, E: Debug + Clone> ResultMatchers<T, E> for Expectation<&Result<T, E>> {
    fn to_be_ok(self) {
        let result = self.value.is_ok();
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("is{not} Ok"));
        } else {
            let expected_msg = format!("Expected {}{not} to be Ok", self.expr_str);
            let received = match &self.value {
                Ok(v) => format!("Ok({:?})", v),
                Err(e) => format!("Err({:?})", e),
            };
            self.report_failure(&expected_msg, &format!("Received: {}", received));
        }
    }

    fn to_be_err(self) {
        let result = self.value.is_err();
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("is{not} Err"));
        } else {
            let expected_msg = format!("Expected {}{not} to be Err", self.expr_str);
            let received = match &self.value {
                Ok(v) => format!("Ok({:?})", v),
                Err(e) => format!("Err({:?})", e),
            };
            self.report_failure(&expected_msg, &format!("Received: {}", received));
        }
    }

    fn to_contain_ok<U: PartialEq<T> + Debug>(self, expected: U) {
        let result = match &self.value {
            Ok(v) => expected == *v,
            Err(_) => false,
        };
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("does{not} contain Ok value {:?}", expected));
        } else {
            let expected_msg = format!("Expected {}{not} to contain Ok value {:?}", self.expr_str, expected);
            let received = match &self.value {
                Ok(v) => format!("Ok({:?})", v),
                Err(e) => format!("Err({:?})", e),
            };
            self.report_failure(&expected_msg, &format!("Received: {}", received));
        }
    }

    fn to_contain_err<U: PartialEq<E> + Debug>(self, expected: U) {
        let result = match &self.value {
            Ok(_) => false,
            Err(e) => expected == *e,
        };
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("does{not} contain Err value {:?}", expected));
        } else {
            let expected_msg = format!("Expected {}{not} to contain Err value {:?}", self.expr_str, expected);
            let received = match &self.value {
                Ok(v) => format!("Ok({:?})", v),
                Err(e) => format!("Err({:?})", e),
            };
            self.report_failure(&expected_msg, &format!("Received: {}", received));
        }
    }
}
