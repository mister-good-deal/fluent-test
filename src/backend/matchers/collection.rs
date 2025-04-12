use crate::backend::Assertion;
use crate::backend::assertions::sentence::AssertionSentence;
use std::fmt::Debug;

pub trait CollectionMatchers<T> {
    fn to_be_empty(self) -> Self;
    fn to_have_length(self, expected: usize) -> Self;
    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) -> Self;
    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self;
    fn to_equal_collection<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self;
}

impl<T: Debug + Clone + PartialEq> CollectionMatchers<T> for Assertion<&[T]> {
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

    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) -> Self {
        let result = self.value.iter().any(|item| expected == *item);
        let sentence = AssertionSentence::new("contain", format!("{:?}", expected));

        return self.add_step(sentence, result);
    }

    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        let result = expected.iter().all(|exp| self.value.iter().any(|item| *exp == *item));
        let sentence = AssertionSentence::new("contain", format!("all of {:?}", expected));

        return self.add_step(sentence, result);
    }

    fn to_equal_collection<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        // Convert expected to Vec<&T> for comparison
        let expected_vec: Vec<&U> = expected.iter().collect();

        // Quick length check first
        if self.value.len() != expected_vec.len() {
            let result = false;
            let sentence = AssertionSentence::new("equal", format!("collection {:?} (different lengths)", expected_vec));
            return self.add_step(sentence, result);
        }

        // Check if all elements match in order
        let result = self.value.iter().zip(expected_vec.iter()).all(|(a, b)| **b == *a);
        let sentence = AssertionSentence::new("equal", format!("collection {:?}", expected_vec));

        return self.add_step(sentence, result);
    }
}

// Add support for Vec<T> as reference
impl<T: Debug + Clone + PartialEq> CollectionMatchers<T> for Assertion<&Vec<T>> {
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

    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) -> Self {
        let result = self.value.iter().any(|item| expected == *item);
        let sentence = AssertionSentence::new("contain", format!("{:?}", expected));

        return self.add_step(sentence, result);
    }

    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        let result = expected.iter().all(|exp| self.value.iter().any(|item| *exp == *item));
        let sentence = AssertionSentence::new("contain", format!("all of {:?}", expected));

        return self.add_step(sentence, result);
    }

    fn to_equal_collection<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        // Quick length check first
        if self.value.len() != expected.len() {
            let result = false;
            let sentence = AssertionSentence::new("equal", format!("collection {:?} (different lengths)", expected));
            return self.add_step(sentence, result);
        }

        // Check if all elements match in order
        let result = self.value.iter().zip(expected.iter()).all(|(a, b)| *b == *a);
        let sentence = AssertionSentence::new("equal", format!("collection {:?}", expected));

        return self.add_step(sentence, result);
    }
}

// Also add support for Vec<T> directly
impl<T: Debug + Clone + PartialEq> CollectionMatchers<T> for Assertion<Vec<T>> {
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

    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) -> Self {
        let result = self.value.iter().any(|item| expected == *item);
        let sentence = AssertionSentence::new("contain", format!("{:?}", expected));

        return self.add_step(sentence, result);
    }

    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        let result = expected.iter().all(|exp| self.value.iter().any(|item| *exp == *item));
        let sentence = AssertionSentence::new("contain", format!("all of {:?}", expected));

        return self.add_step(sentence, result);
    }

    fn to_equal_collection<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        // Quick length check first
        if self.value.len() != expected.len() {
            let result = false;
            let sentence = AssertionSentence::new("equal", format!("collection {:?} (different lengths)", expected));
            return self.add_step(sentence, result);
        }

        // Check if all elements match in order
        let result = self.value.iter().zip(expected.iter()).all(|(a, b)| *b == *a);
        let sentence = AssertionSentence::new("equal", format!("collection {:?}", expected));

        return self.add_step(sentence, result);
    }
}

// Also add support for array references
impl<T: Debug + Clone + PartialEq, const N: usize> CollectionMatchers<T> for Assertion<&[T; N]> {
    fn to_be_empty(self) -> Self {
        let result = N == 0;
        let sentence = AssertionSentence::new("be", "empty");

        return self.add_step(sentence, result);
    }

    fn to_have_length(self, expected: usize) -> Self {
        let actual_length = N;
        let result = actual_length == expected;
        let sentence = AssertionSentence::new("have", format!("length {}", expected));

        return self.add_step(sentence, result);
    }

    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) -> Self {
        let result = self.value.iter().any(|item| expected == *item);
        let sentence = AssertionSentence::new("contain", format!("{:?}", expected));

        return self.add_step(sentence, result);
    }

    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        let result = expected.iter().all(|exp| self.value.iter().any(|item| *exp == *item));
        let sentence = AssertionSentence::new("contain", format!("all of {:?}", expected));

        return self.add_step(sentence, result);
    }

    fn to_equal_collection<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        // Quick length check first
        if N != expected.len() {
            let result = false;
            let sentence = AssertionSentence::new("equal", format!("collection {:?} (different lengths)", expected));
            return self.add_step(sentence, result);
        }

        // Check if all elements match in order
        let result = self.value.iter().zip(expected.iter()).all(|(a, b)| *b == *a);
        let sentence = AssertionSentence::new("equal", format!("collection {:?}", expected));

        return self.add_step(sentence, result);
    }
}

/// Extension trait for adding helper methods to collections
pub trait CollectionExtensions<T> {
    fn first(&self) -> Option<&T>;
    fn last(&self) -> Option<&T>;
}

impl<T> CollectionExtensions<T> for Vec<T> {
    fn first(&self) -> Option<&T> {
        return <[T]>::first(self);
    }

    fn last(&self) -> Option<&T> {
        return <[T]>::last(self);
    }
}

impl<T> CollectionExtensions<T> for &[T] {
    fn first(&self) -> Option<&T> {
        return <[T]>::first(self);
    }

    fn last(&self) -> Option<&T> {
        return <[T]>::last(self);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_collection_length() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();

        expect!(slice).to_have_length(5);
        expect!(&collection).to_have_length(5);
    }

    #[test]
    #[should_panic(expected = "have length 6")]
    fn test_wrong_length_fails() {
        // This should fail
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).to_have_length(6);
    }

    #[test]
    #[should_panic(expected = "not have length 5")]
    fn test_right_length_not_fails() {
        // This should fail
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).not().to_have_length(5);
    }

    #[test]
    fn test_collection_contains() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();

        expect!(slice).to_contain(3);
        expect!(&collection).to_contain(3);
    }

    #[test]
    #[should_panic(expected = "contain 6")]
    fn test_missing_value_fails() {
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).to_contain(6);
    }

    #[test]
    #[should_panic(expected = "not contain 3")]
    fn test_present_value_not_fails() {
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).not().to_contain(3);
    }

    #[test]
    fn test_collection_contains_all() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();

        expect!(slice).to_contain_all_of(&[1, 3, 5]);
        expect!(&collection).to_contain_all_of(&[1, 3, 5]);
    }

    #[test]
    #[should_panic(expected = "contain all of")]
    fn test_missing_values_fails() {
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).to_contain_all_of(&[1, 6, 7]);
    }

    #[test]
    #[should_panic(expected = "not contain all of")]
    fn test_present_values_not_fails() {
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).not().to_contain_all_of(&[1, 3, 5]);
    }

    #[test]
    fn test_collection_equality() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();

        expect!(slice).to_equal_collection(&[1, 2, 3, 4, 5]);
        expect!(&collection).to_equal_collection(&[1, 2, 3, 4, 5]);
    }

    #[test]
    #[should_panic(expected = "equal collection")]
    fn test_different_collection_fails() {
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).to_equal_collection(&[5, 4, 3, 2, 1]);
    }

    #[test]
    #[should_panic(expected = "different lengths")]
    fn test_shorter_collection_fails() {
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).to_equal_collection(&[1, 2, 3]);
    }

    #[test]
    #[should_panic(expected = "not equal collection")]
    fn test_same_collection_not_fails() {
        let collection = vec![1, 2, 3, 4, 5];
        let slice = collection.as_slice();
        expect!(slice).not().to_equal_collection(&[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_empty_collection() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let empty: Vec<i32> = vec![];
        let slice = empty.as_slice();

        expect!(slice).to_be_empty();
        expect!(&empty).to_be_empty();
    }

    #[test]
    #[should_panic(expected = "be empty")]
    fn test_non_empty_to_be_empty_fails() {
        let collection = vec![1, 2, 3];
        let slice = collection.as_slice();
        expect!(slice).to_be_empty();
    }

    #[test]
    #[should_panic(expected = "not be empty")]
    fn test_empty_not_to_be_empty_fails() {
        let empty: Vec<i32> = vec![];
        let slice = empty.as_slice();
        expect!(slice).not().to_be_empty();
    }
}
