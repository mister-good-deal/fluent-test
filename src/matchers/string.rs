use crate::expectation::Expectation;
use regex::Regex;
use std::fmt::Debug;

pub trait StringMatchers {
    fn to_contain(self, substring: &str) -> Self;
    fn to_start_with(self, prefix: &str) -> Self;
    fn to_end_with(self, suffix: &str) -> Self;
    fn to_match_regex(self, pattern: &str) -> Self;
    fn to_have_length(self, length: usize) -> Self;
    fn to_have_length_greater_than(self, length: usize) -> Self;
    fn to_have_length_less_than(self, length: usize) -> Self;
}

// Add function to check if a string is empty
pub trait StringExt {
    fn to_be_empty(self) -> Self;
}

impl<T: AsRef<str> + Debug + Clone> StringExt for Expectation<T> {
    fn to_be_empty(self) -> Self {
        let value = self.value.as_ref();
        let result = value.is_empty();
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let success_msg = format!("is{not} empty");
        let expected_msg = format!("Expected {}{not} to be empty", self.expr_str);
        let received_msg = format!("Received: {:?}", value);

        self.handle_assertion_result(success, &success_msg, &expected_msg, &received_msg)
    }
}

// Implement StringMatchers for regular Expectation
impl<T: AsRef<str> + Debug + Clone> StringMatchers for Expectation<T> {
    fn to_contain(self, substring: &str) -> Self {
        let value = self.value.as_ref();
        let result = value.contains(substring);
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let success_msg = format!("does{not} contain \"{}\"", substring);
        let expected_msg = format!("Expected {}{not} to contain \"{}\"", self.expr_str, substring);
        let received_msg = format!("Received: {:?}", value);

        self.handle_assertion_result(success, &success_msg, &expected_msg, &received_msg)
    }

    fn to_start_with(self, prefix: &str) -> Self {
        let value = self.value.as_ref();
        let result = value.starts_with(prefix);
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let success_msg = format!("does{not} start with \"{}\"", prefix);
        let expected_msg = format!("Expected {}{not} to start with \"{}\"", self.expr_str, prefix);
        let received_msg = format!("Received: {:?}", value);

        self.handle_assertion_result(success, &success_msg, &expected_msg, &received_msg)
    }

    fn to_end_with(self, suffix: &str) -> Self {
        let value = self.value.as_ref();
        let result = value.ends_with(suffix);
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let success_msg = format!("does{not} end with \"{}\"", suffix);
        let expected_msg = format!("Expected {}{not} to end with \"{}\"", self.expr_str, suffix);
        let received_msg = format!("Received: {:?}", value);

        self.handle_assertion_result(success, &success_msg, &expected_msg, &received_msg)
    }

    fn to_match_regex(self, pattern: &str) -> Self {
        let value = self.value.as_ref();
        let regex = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                self.report_failure(&format!("Invalid regex pattern: {}", pattern), &format!("Error: {}", e));
                panic!("Invalid regex pattern"); // Panic to stop execution
            }
        };

        let result = regex.is_match(value);
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let success_msg = format!("does{not} match pattern \"{}\"", pattern);
        let expected_msg = format!("Expected {}{not} to match regex \"{}\"", self.expr_str, pattern);
        let received_msg = format!("Received: {:?}", value);

        self.handle_assertion_result(success, &success_msg, &expected_msg, &received_msg)
    }

    fn to_have_length(self, length: usize) -> Self {
        let value = self.value.as_ref();
        let actual_length = value.len();
        let result = actual_length == length;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let success_msg = format!("does{not} have length {}", length);
        let expected_msg = format!("Expected {}{not} to have length {}", self.expr_str, length);
        let received_msg = format!("Actual length: {}", actual_length);

        self.handle_assertion_result(success, &success_msg, &expected_msg, &received_msg)
    }

    fn to_have_length_greater_than(self, length: usize) -> Self {
        let value = self.value.as_ref();
        let actual_length = value.len();
        let result = actual_length > length;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let success_msg = format!("does{not} have length greater than {}", length);
        let expected_msg = format!("Expected {}{not} to have length greater than {}", self.expr_str, length);
        let received_msg = format!("Actual length: {}", actual_length);

        self.handle_assertion_result(success, &success_msg, &expected_msg, &received_msg)
    }

    fn to_have_length_less_than(self, length: usize) -> Self {
        let value = self.value.as_ref();
        let actual_length = value.len();
        let result = actual_length < length;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let success_msg = format!("does{not} have length less than {}", length);
        let expected_msg = format!("Expected {}{not} to have length less than {}", self.expr_str, length);
        let received_msg = format!("Actual length: {}", actual_length);

        self.handle_assertion_result(success, &success_msg, &expected_msg, &received_msg)
    }
}
