use crate::expectation::Expectation;
use std::fmt::Debug;

pub trait OptionMatchers<T> {
    fn to_be_some(self);
    fn to_be_none(self);
    fn to_contain_value<U: PartialEq<T> + Debug>(self, expected: U);
}

impl<T: Debug + Clone> OptionMatchers<T> for Expectation<&Option<T>> {
    fn to_be_some(self) {
        let result = self.value.is_some();
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("is{not} Some"));
        } else {
            let expected_msg = format!("Expected {}{not} to be Some", self.expr_str);
            let received = match &self.value {
                Some(v) => format!("Some({:?})", v),
                None => "None".to_string(),
            };
            self.report_failure(&expected_msg, &format!("Received: {}", received));
        }
    }

    fn to_be_none(self) {
        let result = self.value.is_none();
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("is{not} None"));
        } else {
            let expected_msg = format!("Expected {}{not} to be None", self.expr_str);
            let received = match &self.value {
                Some(v) => format!("Some({:?})", v),
                None => "None".to_string(),
            };
            self.report_failure(&expected_msg, &format!("Received: {}", received));
        }
    }

    fn to_contain_value<U: PartialEq<T> + Debug>(self, expected: U) {
        let result = match &self.value {
            Some(v) => expected == *v,
            None => false,
        };
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("does{not} contain value {:?}", expected));
        } else {
            let expected_msg = format!("Expected {}{not} to contain value {:?}", self.expr_str, expected);
            let received = match &self.value {
                Some(v) => format!("Some({:?})", v),
                None => "None".to_string(),
            };
            self.report_failure(&expected_msg, &format!("Received: {}", received));
        }
    }
}
