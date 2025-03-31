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

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_string_to_be_empty() {
        crate::Reporter::disable_deduplication();

        let empty_string = "";
        let non_empty_string = "hello";

        expect!(empty_string).to_be_empty();
        expect!(non_empty_string).not().to_be_empty();
    }

    #[test]
    #[should_panic(expected = "is empty")]
    fn test_non_empty_to_be_empty_fails() {
        let _assertion = expect!("hello").to_be_empty();
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "is not empty")]
    fn test_empty_not_to_be_empty_fails() {
        let _assertion = expect!("").not().to_be_empty();
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_string_contains() {
        crate::Reporter::disable_deduplication();

        let text = "Hello, world!";

        expect!(text).to_contain("world");
        expect!(text).not().to_contain("universe");
    }

    #[test]
    #[should_panic(expected = "does contain")]
    fn test_string_contains_fails() {
        let _assertion = expect!("Hello, world!").to_contain("universe");
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "does not contain")]
    fn test_string_not_contains_fails() {
        let _assertion = expect!("Hello, world!").not().to_contain("world");
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_string_starts_with() {
        crate::Reporter::disable_deduplication();

        let text = "Hello, world!";

        expect!(text).to_start_with("Hello");
        expect!(text).not().to_start_with("world");
    }

    #[test]
    #[should_panic(expected = "does start with")]
    fn test_string_starts_with_fails() {
        let _assertion = expect!("Hello, world!").to_start_with("world");
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "does not start with")]
    fn test_string_not_starts_with_fails() {
        let _assertion = expect!("Hello, world!").not().to_start_with("Hello");
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_string_ends_with() {
        crate::Reporter::disable_deduplication();

        let text = "Hello, world!";

        expect!(text).to_end_with("world!");
        expect!(text).not().to_end_with("Hello");
    }

    #[test]
    #[should_panic(expected = "does end with")]
    fn test_string_ends_with_fails() {
        let _assertion = expect!("Hello, world!").to_end_with("Hello");
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "does not end with")]
    fn test_string_not_ends_with_fails() {
        let _assertion = expect!("Hello, world!").not().to_end_with("world!");
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_string_match_regex() {
        crate::Reporter::disable_deduplication();

        let text = "Hello, world!";

        expect!(text).to_match_regex(r"Hello,\s\w+!");
        expect!(text).not().to_match_regex(r"Goodbye,\s\w+!");
    }

    #[test]
    #[should_panic(expected = "does match pattern")]
    fn test_string_match_regex_fails() {
        let _assertion = expect!("Hello, world!").to_match_regex(r"Goodbye,\s\w+!");
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "does not match pattern")]
    fn test_string_not_match_regex_fails() {
        let _assertion = expect!("Hello, world!").not().to_match_regex(r"Hello,\s\w+!");
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_string_length() {
        crate::Reporter::disable_deduplication();

        let text = "Hello";

        expect!(text).to_have_length(5);
        expect!(text).not().to_have_length(10);
        expect!(text).to_have_length_greater_than(3);
        expect!(text).to_have_length_less_than(10);
    }

    #[test]
    #[should_panic(expected = "does have length")]
    fn test_string_length_fails() {
        let _assertion = expect!("Hello").to_have_length(10);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "does not have length")]
    fn test_string_not_length_fails() {
        let _assertion = expect!("Hello").not().to_have_length(5);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "does have length greater than")]
    fn test_string_length_greater_than_fails() {
        let _assertion = expect!("Hello").to_have_length_greater_than(10);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "does not have length greater than")]
    fn test_string_not_length_greater_than_fails() {
        let _assertion = expect!("Hello").not().to_have_length_greater_than(3);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "does have length less than")]
    fn test_string_length_less_than_fails() {
        let _assertion = expect!("Hello").to_have_length_less_than(3);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "does not have length less than")]
    fn test_string_not_length_less_than_fails() {
        let _assertion = expect!("Hello").not().to_have_length_less_than(10);
        std::hint::black_box(_assertion);
    }
}
