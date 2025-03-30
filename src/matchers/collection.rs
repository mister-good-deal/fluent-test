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
