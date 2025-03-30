use crate::expectation::Expectation;
use regex::Regex;
use std::fmt::Debug;

pub trait StringMatchers {
    fn to_contain(self, substring: &str);
    fn to_start_with(self, prefix: &str);
    fn to_end_with(self, suffix: &str);
    fn to_match_regex(self, pattern: &str);
    fn to_have_length(self, length: usize);
    fn to_have_length_greater_than(self, length: usize);
    fn to_have_length_less_than(self, length: usize);
}

// Add function to check if a string is empty
pub trait StringExt {
    fn to_be_empty(self);
}

impl<T: AsRef<str> + Debug> StringExt for Expectation<T> {
    fn to_be_empty(self) {
        let value = self.value.as_ref();
        let result = value.is_empty();
        let success = if self.negated { !result } else { result };

        if success {
            let msg = if self.negated { "is not empty" } else { "is empty" };
            self.report_success(msg);
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to be empty", self.expr_str)
            } else {
                format!("Expected {} to be empty", self.expr_str)
            };
            self.report_failure(&expected_msg, &format!("Received: {:?}", value));
        }
    }
}

// Implement StringMatchers for regular Expectation
impl<T: AsRef<str> + Debug> StringMatchers for Expectation<T> {
    fn to_contain(self, substring: &str) {
        let value = self.value.as_ref();
        let result = value.contains(substring);
        let success = if self.negated { !result } else { result };

        if success {
            let msg = if self.negated { "does not contain" } else { "contains" };
            self.report_success(&format!("{} \"{}\"", msg, substring));
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to contain \"{}\"", self.expr_str, substring)
            } else {
                format!("Expected {} to contain \"{}\"", self.expr_str, substring)
            };
            self.report_failure(&expected_msg, &format!("Received: {:?}", value));
        }
    }

    fn to_start_with(self, prefix: &str) {
        let value = self.value.as_ref();
        let result = value.starts_with(prefix);
        let success = if self.negated { !result } else { result };

        if success {
            let msg = if self.negated { "does not start with" } else { "starts with" };
            self.report_success(&format!("{} \"{}\"", msg, prefix));
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to start with \"{}\"", self.expr_str, prefix)
            } else {
                format!("Expected {} to start with \"{}\"", self.expr_str, prefix)
            };
            self.report_failure(&expected_msg, &format!("Received: {:?}", value));
        }
    }

    fn to_end_with(self, suffix: &str) {
        let value = self.value.as_ref();
        let result = value.ends_with(suffix);
        let success = if self.negated { !result } else { result };

        if success {
            let msg = if self.negated { "does not end with" } else { "ends with" };
            self.report_success(&format!("{} \"{}\"", msg, suffix));
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to end with \"{}\"", self.expr_str, suffix)
            } else {
                format!("Expected {} to end with \"{}\"", self.expr_str, suffix)
            };
            self.report_failure(&expected_msg, &format!("Received: {:?}", value));
        }
    }

    fn to_match_regex(self, pattern: &str) {
        let value = self.value.as_ref();
        let regex = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                self.report_failure(&format!("Invalid regex pattern: {}", pattern), &format!("Error: {}", e));
                return;
            }
        };

        let result = regex.is_match(value);
        let success = if self.negated { !result } else { result };

        if success {
            let msg = if self.negated { "does not match pattern" } else { "matches pattern" };
            self.report_success(&format!("{} \"{}\"", msg, pattern));
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to match regex \"{}\"", self.expr_str, pattern)
            } else {
                format!("Expected {} to match regex \"{}\"", self.expr_str, pattern)
            };
            self.report_failure(&expected_msg, &format!("Received: {:?}", value));
        }
    }

    fn to_have_length(self, length: usize) {
        let value = self.value.as_ref();
        let actual_length = value.len();
        let result = actual_length == length;
        let success = if self.negated { !result } else { result };

        if success {
            let msg = if self.negated { "does not have length" } else { "has length" };
            self.report_success(&format!("{} {}", msg, length));
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to have length {}", self.expr_str, length)
            } else {
                format!("Expected {} to have length {}", self.expr_str, length)
            };
            self.report_failure(&expected_msg, &format!("Actual length: {}", actual_length));
        }
    }

    fn to_have_length_greater_than(self, length: usize) {
        let value = self.value.as_ref();
        let actual_length = value.len();
        let result = actual_length > length;
        let success = if self.negated { !result } else { result };

        if success {
            let msg = if self.negated { "does not have length greater than" } else { "has length greater than" };
            self.report_success(&format!("{} {}", msg, length));
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to have length greater than {}", self.expr_str, length)
            } else {
                format!("Expected {} to have length greater than {}", self.expr_str, length)
            };
            self.report_failure(&expected_msg, &format!("Actual length: {}", actual_length));
        }
    }

    fn to_have_length_less_than(self, length: usize) {
        let value = self.value.as_ref();
        let actual_length = value.len();
        let result = actual_length < length;
        let success = if self.negated { !result } else { result };

        if success {
            let msg = if self.negated { "does not have length less than" } else { "has length less than" };
            self.report_success(&format!("{} {}", msg, length));
        } else {
            let expected_msg = if self.negated {
                format!("Expected {} not to have length less than {}", self.expr_str, length)
            } else {
                format!("Expected {} to have length less than {}", self.expr_str, length)
            };
            self.report_failure(&expected_msg, &format!("Actual length: {}", actual_length));
        }
    }
}
