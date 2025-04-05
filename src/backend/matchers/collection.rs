use crate::backend::Expectation;
use crate::backend::assertions::sentence::AssertionSentence;
use std::fmt::Debug;

pub trait CollectionMatchers<T> {
    fn to_be_empty(self) -> Self;
    fn to_have_length(self, expected: usize) -> Self;
    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) -> Self;
    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self;
    fn to_equal_collection<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self;
}

impl<T: Debug + Clone + PartialEq> CollectionMatchers<T> for Expectation<&[T]> {
    fn to_be_empty(self) -> Self {
        let result = self.value.is_empty();
        let sentence = AssertionSentence::new("be", "empty");

        return self.add_assertion_step(sentence, result);
    }

    fn to_have_length(self, expected: usize) -> Self {
        let actual_length = self.value.len();
        let result = actual_length == expected;
        let sentence = AssertionSentence::new("have", format!("length {}", expected));

        return self.add_assertion_step(sentence, result);
    }

    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) -> Self {
        let result = self.value.iter().any(|item| expected == *item);
        let sentence = AssertionSentence::new("contain", format!("{:?}", expected));

        return self.add_assertion_step(sentence, result);
    }

    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        let result = expected.iter().all(|exp| self.value.iter().any(|item| *exp == *item));
        let sentence = AssertionSentence::new("contain", format!("all of {:?}", expected));

        return self.add_assertion_step(sentence, result);
    }

    fn to_equal_collection<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        // Convert expected to Vec<&T> for comparison
        let expected_vec: Vec<&U> = expected.iter().collect();

        // Quick length check first
        if self.value.len() != expected_vec.len() {
            let result = false;
            let sentence = AssertionSentence::new("equal", format!("collection {:?} (different lengths)", expected_vec));

            return self.add_assertion_step(sentence, result);
        }

        // Check element by element
        let result = self.value.iter().zip(expected_vec.iter()).all(|(a, b)| *b == a);
        let sentence = AssertionSentence::new("equal", format!("collection {:?}", expected_vec));

        return self.add_assertion_step(sentence, result);
    }
}

impl<T: Debug + Clone + PartialEq> CollectionMatchers<T> for Expectation<&Vec<T>> {
    fn to_be_empty(self) -> Self {
        let result = self.value.is_empty();
        let sentence = AssertionSentence::new("be", "empty");

        return self.add_assertion_step(sentence, result);
    }

    fn to_have_length(self, expected: usize) -> Self {
        let actual_length = self.value.len();
        let result = actual_length == expected;
        let sentence = AssertionSentence::new("have", format!("length {}", expected));

        return self.add_assertion_step(sentence, result);
    }

    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) -> Self {
        let result = self.value.iter().any(|item| expected == *item);
        let sentence = AssertionSentence::new("contain", format!("{:?}", expected));

        return self.add_assertion_step(sentence, result);
    }

    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        let result = expected.iter().all(|exp| self.value.iter().any(|item| *exp == *item));
        let sentence = AssertionSentence::new("contain", format!("all of {:?}", expected));

        return self.add_assertion_step(sentence, result);
    }

    fn to_equal_collection<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        // Convert expected to Vec<&T> for comparison
        let expected_vec: Vec<&U> = expected.iter().collect();

        // Quick length check first
        if self.value.len() != expected_vec.len() {
            let result = false;
            let sentence = AssertionSentence::new("equal", format!("collection {:?} (different lengths)", expected_vec));

            return self.add_assertion_step(sentence, result);
        }

        // Check element by element
        let result = self.value.iter().zip(expected_vec.iter()).all(|(a, b)| *b == a);
        let sentence = AssertionSentence::new("equal", format!("collection {:?}", expected_vec));

        return self.add_assertion_step(sentence, result);
    }
}

impl<T: Debug + Clone + PartialEq> CollectionMatchers<T> for Expectation<Vec<T>> {
    fn to_be_empty(self) -> Self {
        let result = self.value.is_empty();
        let sentence = AssertionSentence::new("be", "empty");

        return self.add_assertion_step(sentence, result);
    }

    fn to_have_length(self, expected: usize) -> Self {
        let actual_length = self.value.len();
        let result = actual_length == expected;
        let sentence = AssertionSentence::new("have", format!("length {}", expected));

        return self.add_assertion_step(sentence, result);
    }

    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) -> Self {
        let result = self.value.iter().any(|item| expected == *item);
        let sentence = AssertionSentence::new("contain", format!("{:?}", expected));

        return self.add_assertion_step(sentence, result);
    }

    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        let result = expected.iter().all(|exp| self.value.iter().any(|item| *exp == *item));
        let sentence = AssertionSentence::new("contain", format!("all of {:?}", expected));

        return self.add_assertion_step(sentence, result);
    }

    fn to_equal_collection<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        // Convert expected to Vec<&T> for comparison
        let expected_vec: Vec<&U> = expected.iter().collect();

        // Quick length check first
        if self.value.len() != expected_vec.len() {
            let result = false;
            let sentence = AssertionSentence::new("equal", format!("collection {:?} (different lengths)", expected_vec));

            return self.add_assertion_step(sentence, result);
        }

        // Check element by element
        let result = self.value.iter().zip(expected_vec.iter()).all(|(a, b)| *b == a);
        let sentence = AssertionSentence::new("equal", format!("collection {:?}", expected_vec));

        return self.add_assertion_step(sentence, result);
    }
}

// Implement for array references
impl<T: Debug + Clone + PartialEq, const N: usize> CollectionMatchers<T> for Expectation<&[T; N]> {
    fn to_be_empty(self) -> Self {
        let result = self.value.is_empty();
        let sentence = AssertionSentence::new("be", "empty");

        return self.add_assertion_step(sentence, result);
    }

    fn to_have_length(self, expected: usize) -> Self {
        let actual_length = self.value.len();
        let result = actual_length == expected;
        let sentence = AssertionSentence::new("have", format!("length {}", expected));

        return self.add_assertion_step(sentence, result);
    }

    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) -> Self {
        let result = self.value.iter().any(|item| expected == *item);
        let sentence = AssertionSentence::new("contain", format!("{:?}", expected));

        return self.add_assertion_step(sentence, result);
    }

    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        let result = expected.iter().all(|exp| self.value.iter().any(|item| *exp == *item));
        let sentence = AssertionSentence::new("contain", format!("all of {:?}", expected));

        return self.add_assertion_step(sentence, result);
    }

    fn to_equal_collection<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        // Convert expected to Vec<&T> for comparison
        let expected_vec: Vec<&U> = expected.iter().collect();

        // Quick length check first
        if self.value.len() != expected_vec.len() {
            let result = false;
            let sentence = AssertionSentence::new("equal", format!("collection {:?} (different lengths)", expected_vec));

            return self.add_assertion_step(sentence, result);
        }

        // Check element by element
        let result = self.value.iter().zip(expected_vec.iter()).all(|(a, b)| *b == a);
        let sentence = AssertionSentence::new("equal", format!("collection {:?}", expected_vec));

        return self.add_assertion_step(sentence, result);
    }
}

// Legacy method aliases for backward compatibility
pub trait CollectionExtensions<T: Debug + Clone + PartialEq> {
    fn to_equal<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self;
}

// Fully qualified syntax to disambiguate
impl<T: Debug + Clone + PartialEq> CollectionExtensions<T> for Expectation<&[T]> {
    fn to_equal<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        return <Expectation<&[T]> as CollectionMatchers<T>>::to_equal_collection(self, expected);
    }
}

impl<T: Debug + Clone + PartialEq> CollectionExtensions<T> for Expectation<&Vec<T>> {
    fn to_equal<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        return <Expectation<&Vec<T>> as CollectionMatchers<T>>::to_equal_collection(self, expected);
    }
}

impl<T: Debug + Clone + PartialEq> CollectionExtensions<T> for Expectation<Vec<T>> {
    fn to_equal<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        return <Expectation<Vec<T>> as CollectionMatchers<T>>::to_equal_collection(self, expected);
    }
}

// Also implement for array references
impl<T: Debug + Clone + PartialEq, const N: usize> CollectionExtensions<T> for Expectation<&[T; N]> {
    fn to_equal<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        return <Expectation<&[T; N]> as CollectionMatchers<T>>::to_equal_collection(self, expected);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    // Import the specific traits we need for these tests
    use super::{CollectionExtensions, CollectionMatchers};

    #[test]
    fn test_collection_equality() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should pass with the new method name
        expect!(&[1, 2, 3]).to_equal_collection(&[1, 2, 3]);
        expect!(&vec![1, 2, 3]).to_equal_collection(&[1, 2, 3]);
        expect!(vec![1, 2, 3]).to_equal_collection(&[1, 2, 3]);

        // These should not be equal with the new method name
        expect!(&[1, 2, 3]).not().to_equal_collection(&[3, 2, 1]);
        expect!(&vec![1, 2, 3]).not().to_equal_collection(&[3, 2, 1]);
        expect!(vec![1, 2, 3]).not().to_equal_collection(&[3, 2, 1]);

        // Test backward compatibility with the old method name, using fully qualified syntax to disambiguate
        <Expectation<&[i32; 3]> as CollectionExtensions<i32>>::to_equal(expect!(&[1, 2, 3]), &[1, 2, 3]);
        <Expectation<&Vec<i32>> as CollectionExtensions<i32>>::to_equal(expect!(&vec![1, 2, 3]), &[1, 2, 3]);
        <Expectation<Vec<i32>> as CollectionExtensions<i32>>::to_equal(expect!(vec![1, 2, 3]), &[1, 2, 3]);
    }

    #[test]
    #[should_panic(expected = "equal collection")]
    fn test_same_collection_not_fails() {
        // This should fail because [1, 2, 3] == [1, 2, 3]
        let _assertion = expect!(&[1, 2, 3]).not().to_equal_collection(&[1, 2, 3]);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "equal collection")]
    fn test_different_collection_fails() {
        // This should fail because [1, 2, 3] != [3, 2, 1]
        let _assertion = expect!(&[1, 2, 3]).to_equal_collection(&[3, 2, 1]);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "different lengths")]
    fn test_shorter_collection_fails() {
        // This should fail because lengths don't match
        let _assertion = expect!(&[1, 2]).to_equal_collection(&[1, 2, 3]);
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_collection_length() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should pass
        expect!(&[1, 2, 3]).to_have_length(3);
        expect!(&vec![1, 2, 3]).to_have_length(3);
        expect!(vec![1, 2, 3]).to_have_length(3);

        // These should also pass
        expect!(&[1, 2, 3]).not().to_have_length(4);
        expect!(&vec![1, 2, 3]).not().to_have_length(4);
        expect!(vec![1, 2, 3]).not().to_have_length(4);
    }

    #[test]
    #[should_panic(expected = "have length")]
    fn test_wrong_length_fails() {
        // This should fail because length is 3, not 4
        let _assertion = expect!(&[1, 2, 3]).to_have_length(4);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "not have length")]
    fn test_right_length_not_fails() {
        // This should fail because length is indeed 3
        let _assertion = expect!(&[1, 2, 3]).not().to_have_length(3);
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_collection_contains() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should pass
        expect!(&[1, 2, 3]).to_contain(2);
        expect!(&vec![1, 2, 3]).to_contain(2);
        expect!(vec![1, 2, 3]).to_contain(2);

        // These should also pass (absence)
        expect!(&[1, 2, 3]).not().to_contain(4);
        expect!(&vec![1, 2, 3]).not().to_contain(4);
        expect!(vec![1, 2, 3]).not().to_contain(4);
    }

    #[test]
    #[should_panic(expected = "contain")]
    fn test_present_value_not_fails() {
        // This should fail because 2 is present
        let _assertion = expect!(&[1, 2, 3]).not().to_contain(2);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "contain 4")]
    fn test_missing_value_fails() {
        // This should fail because 4 is not present
        let _assertion = expect!(&[1, 2, 3]).to_contain(4);
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_collection_contains_all() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should pass
        expect!(&[1, 2, 3]).to_contain_all_of(&[1, 2]);
        expect!(&vec![1, 2, 3]).to_contain_all_of(&[1, 2]);
        expect!(vec![1, 2, 3]).to_contain_all_of(&[1, 2]);

        // These should also pass (some values missing)
        expect!(&[1, 2, 3]).not().to_contain_all_of(&[1, 4]);
        expect!(&vec![1, 2, 3]).not().to_contain_all_of(&[1, 4]);
        expect!(vec![1, 2, 3]).not().to_contain_all_of(&[1, 4]);
    }

    #[test]
    #[should_panic(expected = "contain all of")]
    fn test_present_values_not_fails() {
        // This should fail because all values are present
        let _assertion = expect!(&[1, 2, 3]).not().to_contain_all_of(&[1, 2]);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "contain all of")]
    fn test_missing_values_fails() {
        // This should fail because 4 is not present
        let _assertion = expect!(&[1, 2, 3]).to_contain_all_of(&[1, 4]);
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_empty_collection() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should all be empty
        expect!(&[] as &[i32]).to_be_empty();
        expect!(&Vec::<i32>::new()).to_be_empty();
        expect!(Vec::<i32>::new()).to_be_empty();

        // These should not be empty
        expect!(&[1]).not().to_be_empty();
        expect!(&vec![1]).not().to_be_empty();
        expect!(vec![1]).not().to_be_empty();
    }

    #[test]
    #[should_panic(expected = "be empty")]
    fn test_non_empty_to_be_empty_fails() {
        // This should fail because the array is not empty
        let _assertion = expect!(&[1]).to_be_empty();
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "not be empty")]
    fn test_empty_not_to_be_empty_fails() {
        // This should fail because the array is empty
        let _assertion = expect!(&[] as &[i32]).not().to_be_empty();
        std::hint::black_box(_assertion);
    }
}
