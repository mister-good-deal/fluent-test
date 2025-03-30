use crate::expectation::Expectation;

pub trait BooleanMatchers {
    fn to_be_true(self);
    fn to_be_false(self);
}

impl BooleanMatchers for Expectation<bool> {
    fn to_be_true(self) {
        let result = self.value;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("is{not} true"));
        } else {
            let expected_msg = format!("Expected {}{not} to be true", self.expr_str);
            self.report_failure(&expected_msg, &format!("Received: {}", self.value));
        }
    }

    fn to_be_false(self) {
        let result = !self.value;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("is{not} false"));
        } else {
            let expected_msg = format!("Expected {}{not} to be false", self.expr_str);
            self.report_failure(&expected_msg, &format!("Received: {}", self.value));
        }
    }
}
