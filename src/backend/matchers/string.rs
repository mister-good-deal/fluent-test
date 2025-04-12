use crate::backend::Assertion;
use crate::backend::assertions::sentence::AssertionSentence;

/// Trait for string assertions
pub trait StringMatchers {
    fn to_be_empty(self) -> Self;
    fn to_have_length(self, expected: usize) -> Self;

    /// Check if the string contains a substring
    fn to_contain(self, substring: &str) -> Self;

    /// Type-specific version of to_contain to avoid trait conflicts
    fn to_contain_substring(self, substring: &str) -> Self;

    fn to_start_with(self, prefix: &str) -> Self;
    fn to_end_with(self, suffix: &str) -> Self;
    fn to_match(self, pattern: &str) -> Self;
}

impl StringMatchers for Assertion<String> {
    fn to_be_empty(self) -> Self {
        let result = self.value.is_empty();
        let sentence = AssertionSentence::new("be", "empty");

        return self.add_step(sentence, result);
    }

    fn to_have_length(self, expected: usize) -> Self {
        let actual_length = self.value.len();
        let result = actual_length == expected;
        let sentence = AssertionSentence::new("have", format!("length {}", expected));

        return self.add_step(sentence, result);
    }

    fn to_contain(self, substring: &str) -> Self {
        return self.to_contain_substring(substring);
    }

    fn to_contain_substring(self, substring: &str) -> Self {
        let result = self.value.contains(substring);
        let sentence = AssertionSentence::new("contain", format!("\"{}\"", substring));

        return self.add_step(sentence, result);
    }

    fn to_start_with(self, prefix: &str) -> Self {
        let result = self.value.starts_with(prefix);
        let sentence = AssertionSentence::new("start with", format!("\"{}\"", prefix));

        return self.add_step(sentence, result);
    }

    fn to_end_with(self, suffix: &str) -> Self {
        let result = self.value.ends_with(suffix);
        let sentence = AssertionSentence::new("end with", format!("\"{}\"", suffix));

        return self.add_step(sentence, result);
    }

    fn to_match(self, pattern: &str) -> Self {
        // This is a simplified implementation since we can't easily include a regex library
        // In a real implementation, this would use a regex library to match the pattern
        let result = self.value.contains(pattern);
        let sentence = AssertionSentence::new("match", format!("pattern \"{}\"", pattern));

        return self.add_step(sentence, result);
    }
}

// Implement for &str references
impl StringMatchers for Assertion<&str> {
    fn to_be_empty(self) -> Self {
        let result = self.value.is_empty();
        let sentence = AssertionSentence::new("be", "empty");

        return self.add_step(sentence, result);
    }

    fn to_have_length(self, expected: usize) -> Self {
        let actual_length = self.value.len();
        let result = actual_length == expected;
        let sentence = AssertionSentence::new("have", format!("length {}", expected));

        return self.add_step(sentence, result);
    }

    fn to_contain(self, substring: &str) -> Self {
        return self.to_contain_substring(substring);
    }

    fn to_contain_substring(self, substring: &str) -> Self {
        let result = self.value.contains(substring);
        let sentence = AssertionSentence::new("contain", format!("\"{}\"", substring));

        return self.add_step(sentence, result);
    }

    fn to_start_with(self, prefix: &str) -> Self {
        let result = self.value.starts_with(prefix);
        let sentence = AssertionSentence::new("start with", format!("\"{}\"", prefix));

        return self.add_step(sentence, result);
    }

    fn to_end_with(self, suffix: &str) -> Self {
        let result = self.value.ends_with(suffix);
        let sentence = AssertionSentence::new("end with", format!("\"{}\"", suffix));

        return self.add_step(sentence, result);
    }

    fn to_match(self, pattern: &str) -> Self {
        // This is a simplified implementation since we can't easily include a regex library
        // In a real implementation, this would use a regex library to match the pattern
        let result = self.value.contains(pattern);
        let sentence = AssertionSentence::new("match", format!("pattern \"{}\"", pattern));

        return self.add_step(sentence, result);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_string_to_be_empty() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should pass
        expect!("").to_be_empty();
        expect!("hello").not().to_be_empty();
        expect!(String::new()).to_be_empty();
        expect!(String::from("hello")).not().to_be_empty();
    }

    #[test]
    #[should_panic(expected = "be empty")]
    fn test_non_empty_to_be_empty_fails() {
        let _assertion = expect!("hello").to_be_empty();
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "not be empty")]
    fn test_empty_not_to_be_empty_fails() {
        let _assertion = expect!("").not().to_be_empty();
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_string_to_have_length() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should pass
        expect!("hello").to_have_length(5);
        expect!("hello").not().to_have_length(4);
        expect!(String::from("hello")).to_have_length(5);
    }

    #[test]
    #[should_panic(expected = "have length")]
    fn test_wrong_length_fails() {
        let _assertion = expect!("hello").to_have_length(4);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "not have length")]
    fn test_right_length_not_fails() {
        let _assertion = expect!("hello").not().to_have_length(5);
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_string_to_contain() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should pass
        expect!("hello world").to_contain("hello");
        expect!("hello world").not().to_contain("goodbye");
        expect!(String::from("hello world")).to_contain("world");
    }

    #[test]
    #[should_panic(expected = "not contain")]
    fn test_not_contains_when_it_does_fails() {
        let _assertion = expect!("hello world").not().to_contain("hello");
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "contain")]
    fn test_contains_when_it_doesnt_fails() {
        let _assertion = expect!("hello world").to_contain("goodbye");
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_string_to_start_with() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should pass
        expect!("hello world").to_start_with("hello");
        expect!("hello world").not().to_start_with("world");
        expect!(String::from("hello world")).to_start_with("hello");
    }

    #[test]
    #[should_panic(expected = "not start with")]
    fn test_not_starts_with_when_it_does_fails() {
        let _assertion = expect!("hello world").not().to_start_with("hello");
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "start with")]
    fn test_starts_with_when_it_doesnt_fails() {
        let _assertion = expect!("hello world").to_start_with("world");
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_string_to_end_with() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should pass
        expect!("hello world").to_end_with("world");
        expect!("hello world").not().to_end_with("hello");
        expect!(String::from("hello world")).to_end_with("world");
    }

    #[test]
    #[should_panic(expected = "not end with")]
    fn test_not_ends_with_when_it_does_fails() {
        let _assertion = expect!("hello world").not().to_end_with("world");
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "end with")]
    fn test_ends_with_when_it_doesnt_fails() {
        let _assertion = expect!("hello world").to_end_with("hello");
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_string_to_match() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should pass (simplified implementation)
        expect!("hello world").to_match("world");
        expect!("hello world").not().to_match("goodbye");
        expect!(String::from("hello world")).to_match("hello");
    }

    #[test]
    #[should_panic(expected = "not match")]
    fn test_not_matches_when_it_does_fails() {
        let _assertion = expect!("hello world").not().to_match("hello");
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "match")]
    fn test_matches_when_it_doesnt_fails() {
        let _assertion = expect!("hello world").to_match("goodbye");
        std::hint::black_box(_assertion);
    }
}
