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

        let description = format!("is{not} empty");

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }
}

// Implement StringMatchers for regular Expectation
impl<T: AsRef<str> + Debug + Clone> StringMatchers for Expectation<T> {
    fn to_contain(self, substring: &str) -> Self {
        let value = self.value.as_ref();
        let result = value.contains(substring);
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} contain \"{}\"", substring);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_start_with(self, prefix: &str) -> Self {
        let value = self.value.as_ref();
        let result = value.starts_with(prefix);
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} start with \"{}\"", prefix);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_end_with(self, suffix: &str) -> Self {
        let value = self.value.as_ref();
        let result = value.ends_with(suffix);
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} end with \"{}\"", suffix);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_match_regex(self, pattern: &str) -> Self {
        let value = self.value.as_ref();
        let regex = match Regex::new(pattern) {
            Ok(r) => r,
            Err(_e) => {
                // For invalid regex, we still want to add it to the chain
                // but mark it as failed
                let description = format!("match regex \"{}\" (INVALID REGEX)", pattern);
                return self.add_assertion_step(description, false);
            }
        };

        let result = regex.is_match(value);
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} match pattern \"{}\"", pattern);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_have_length(self, length: usize) -> Self {
        let value = self.value.as_ref();
        let actual_length = value.len();
        let result = actual_length == length;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} have length {}", length);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_have_length_greater_than(self, length: usize) -> Self {
        let value = self.value.as_ref();
        let actual_length = value.len();
        let result = actual_length > length;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} have length greater than {}", length);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_have_length_less_than(self, length: usize) -> Self {
        let value = self.value.as_ref();
        let actual_length = value.len();
        let result = actual_length < length;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} have length less than {}", length);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }
}
