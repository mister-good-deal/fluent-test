use crate::expectation::Expectation;
use std::fmt::Debug;

pub trait CollectionMatchers<T> {
    fn to_be_empty(self) -> Self;
    fn to_have_length(self, expected: usize) -> Self;
    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) -> Self;
    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self;
    fn to_equal_collection<U: IntoIterator<Item = T>>(self, expected: U) -> Self
    where
        T: PartialEq;
}

// Implementation for slices which cover arrays
impl<T: Debug + Clone + PartialEq> CollectionMatchers<T> for Expectation<&[T]> {
    fn to_be_empty(self) -> Self {
        let result = self.value.is_empty();
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} empty");

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_have_length(self, expected: usize) -> Self {
        let actual_length = self.value.len();
        let result = actual_length == expected;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} have length {}", expected);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) -> Self {
        let result = self.value.iter().any(|item| expected == *item);
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} contain {:?}", expected);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        let result = expected.iter().all(|expected_item| self.value.iter().any(|actual_item| *expected_item == *actual_item));
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} contain all of {:?}", expected);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_equal_collection<U: IntoIterator<Item = T>>(self, expected: U) -> Self
    where
        T: PartialEq,
    {
        // Convert expected into Vec for easier comparison
        let expected_vec: Vec<T> = expected.into_iter().collect();

        // Check if lengths match first
        if self.value.len() != expected_vec.len() {
            let success = self.negated;
            let not = if self.negated { " not" } else { "" };

            let description = format!("does{not} equal collection {:?} (different lengths)", expected_vec);
            return self.add_assertion_step(description, success);
        }

        // Check each element
        let result = self.value.iter().zip(expected_vec.iter()).all(|(actual, expected)| actual == expected);

        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} equal collection {:?}", expected_vec);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }
}

// Implementation for Vec references
impl<T: Debug + Clone + PartialEq> CollectionMatchers<T> for Expectation<&Vec<T>> {
    fn to_be_empty(self) -> Self {
        let result = self.value.is_empty();
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} empty");

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_have_length(self, expected: usize) -> Self {
        let actual_length = self.value.len();
        let result = actual_length == expected;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} have length {}", expected);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) -> Self {
        let result = self.value.iter().any(|item| expected == *item);
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} contain {:?}", expected);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        let result = expected.iter().all(|expected_item| self.value.iter().any(|actual_item| *expected_item == *actual_item));
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} contain all of {:?}", expected);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_equal_collection<U: IntoIterator<Item = T>>(self, expected: U) -> Self
    where
        T: PartialEq,
    {
        // Convert expected into Vec for easier comparison
        let expected_vec: Vec<T> = expected.into_iter().collect();

        // Check if lengths match first
        if self.value.len() != expected_vec.len() {
            let success = self.negated;
            let not = if self.negated { " not" } else { "" };

            let description = format!("does{not} equal collection {:?} (different lengths)", expected_vec);
            return self.add_assertion_step(description, success);
        }

        // Check each element
        let result = self.value.iter().zip(expected_vec.iter()).all(|(actual, expected)| actual == expected);

        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} equal collection {:?}", expected_vec);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }
}

// Implementation for Vec
impl<T: Debug + Clone + PartialEq> CollectionMatchers<T> for Expectation<Vec<T>> {
    fn to_be_empty(self) -> Self {
        let result = self.value.is_empty();
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} empty");

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_have_length(self, expected: usize) -> Self {
        let actual_length = self.value.len();
        let result = actual_length == expected;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} have length {}", expected);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) -> Self {
        let result = self.value.iter().any(|item| expected == *item);
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} contain {:?}", expected);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) -> Self {
        let result = expected.iter().all(|expected_item| self.value.iter().any(|actual_item| *expected_item == *actual_item));
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} contain all of {:?}", expected);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }

    fn to_equal_collection<U: IntoIterator<Item = T>>(self, expected: U) -> Self
    where
        T: PartialEq,
    {
        // Convert expected into Vec for easier comparison
        let expected_vec: Vec<T> = expected.into_iter().collect();

        // Check if lengths match first
        if self.value.len() != expected_vec.len() {
            let success = self.negated;
            let not = if self.negated { " not" } else { "" };

            let description = format!("does{not} equal collection {:?} (different lengths)", expected_vec);
            return self.add_assertion_step(description, success);
        }

        // Check each element
        let result = self.value.iter().zip(expected_vec.iter()).all(|(actual, expected)| actual == expected);

        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} equal collection {:?}", expected_vec);

        // Add to the assertion chain
        self.add_assertion_step(description, success)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    // Test empty collection
    #[test]
    fn test_empty_collection() {
        crate::Reporter::disable_deduplication();

        let empty_vec: Vec<i32> = vec![];
        let non_empty_vec = vec![1, 2, 3];

        // Positive test cases
        expect!(empty_vec.as_slice()).to_be_empty();
        expect!(non_empty_vec.as_slice()).not().to_be_empty();
    }

    #[test]
    #[should_panic]
    fn test_non_empty_to_be_empty_fails() {
        let non_empty_vec = vec![1, 2, 3];
        expect!(non_empty_vec.as_slice()).to_be_empty();
    }

    #[test]
    #[should_panic]
    fn test_empty_not_to_be_empty_fails() {
        let empty_vec: Vec<i32> = vec![];
        expect!(empty_vec.as_slice()).not().to_be_empty();
    }

    // Test collection length
    #[test]
    fn test_collection_length() {
        crate::Reporter::disable_deduplication();

        let vec = vec![1, 2, 3, 4, 5];

        // Positive test cases
        expect!(vec.as_slice()).to_have_length(5);
        expect!(vec.as_slice()).not().to_have_length(3);
    }

    #[test]
    #[should_panic]
    fn test_wrong_length_fails() {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).to_have_length(3);
    }

    #[test]
    #[should_panic]
    fn test_right_length_not_fails() {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).not().to_have_length(5);
    }

    // Test collection contains
    #[test]
    fn test_collection_contains() {
        crate::Reporter::disable_deduplication();

        let vec = vec![1, 2, 3, 4, 5];

        // Positive test cases
        expect!(vec.as_slice()).to_contain(3);
        expect!(vec.as_slice()).not().to_contain(10);
    }

    #[test]
    #[should_panic]
    fn test_missing_value_fails() {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).to_contain(10);
    }

    #[test]
    #[should_panic]
    fn test_present_value_not_fails() {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).not().to_contain(3);
    }

    // Test collection contains all
    #[test]
    fn test_collection_contains_all() {
        crate::Reporter::disable_deduplication();

        let vec = vec![1, 2, 3, 4, 5];

        // Positive test cases
        expect!(vec.as_slice()).to_contain_all_of(&[1, 3, 5]);
        expect!(vec.as_slice()).not().to_contain_all_of(&[1, 6, 7]);
    }

    #[test]
    #[should_panic]
    fn test_missing_values_fails() {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).to_contain_all_of(&[1, 3, 10]);
    }

    #[test]
    #[should_panic]
    fn test_present_values_not_fails() {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).not().to_contain_all_of(&[1, 3, 5]);
    }

    // Test collection equality
    #[test]
    fn test_collection_equality() {
        crate::Reporter::disable_deduplication();

        let vec = vec![1, 2, 3, 4, 5];

        // Positive test cases
        expect!(vec.as_slice()).to_equal_collection(vec![1, 2, 3, 4, 5]);
        expect!(vec.as_slice()).not().to_equal_collection(vec![5, 4, 3, 2, 1]);
        expect!(vec.as_slice()).not().to_equal_collection(vec![1, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn test_different_collection_fails() {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).to_equal_collection(vec![1, 2, 3, 4, 6]);
    }

    #[test]
    #[should_panic]
    fn test_shorter_collection_fails() {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).to_equal_collection(vec![1, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn test_same_collection_not_fails() {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).not().to_equal_collection(vec![1, 2, 3, 4, 5]);
    }
}
