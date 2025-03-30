use crate::expectation::Expectation;
use std::fmt::Debug;

pub trait CollectionMatchers<T> {
    fn to_be_empty(self);
    fn to_have_length(self, expected: usize);
    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U);
    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]);
    fn to_equal_collection<U: IntoIterator<Item = T>>(self, expected: U)
    where
        T: PartialEq;
}

// Implementation for slices which cover arrays
impl<T: Debug + Clone + PartialEq> CollectionMatchers<T> for Expectation<&[T]> {
    fn to_be_empty(self) {
        let result = self.value.is_empty();
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("is{not} empty"));
        } else {
            let expected_msg = format!("Expected {}{not} to be empty", self.expr_str);
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }

    fn to_have_length(self, expected: usize) {
        let actual_length = self.value.len();
        let result = actual_length == expected;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("does{not} have length {}", expected));
        } else {
            let expected_msg = format!("Expected {}{not} to have length {}", self.expr_str, expected);
            self.report_failure(&expected_msg, &format!("Actual length: {}", actual_length));
        }
    }

    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) {
        let result = self.value.iter().any(|item| expected == *item);
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("does{not} contain {:?}", expected));
        } else {
            let expected_msg = format!("Expected {}{not} to contain {:?}", self.expr_str, expected);
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }

    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) {
        let result = expected.iter().all(|expected_item| self.value.iter().any(|actual_item| *expected_item == *actual_item));
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("does{not} contain all of {:?}", expected));
        } else {
            let expected_msg = format!("Expected {}{not} to contain all of {:?}", self.expr_str, expected);
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }

    fn to_equal_collection<U: IntoIterator<Item = T>>(self, expected: U)
    where
        T: PartialEq,
    {
        // Convert expected into Vec for easier comparison
        let expected_vec: Vec<T> = expected.into_iter().collect();

        // Check if lengths match first
        if self.value.len() != expected_vec.len() {
            let success = if self.negated { true } else { false };
            let not = if self.negated { " not" } else { "" };

            if success {
                self.report_success(&format!("does{not} equal collection {:?}", expected_vec));
            } else {
                let expected_msg = format!("Expected {}{not} to equal collection {:?}", self.expr_str, expected_vec);
                self.report_failure(
                    &expected_msg,
                    &format!("Collections have different lengths: {} vs {}", self.value.len(), expected_vec.len()),
                );
            }
            return;
        }

        // Check each element
        let result = self.value.iter().zip(expected_vec.iter()).all(|(actual, expected)| actual == expected);

        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("does{not} equal collection {:?}", expected_vec));
        } else {
            let expected_msg = format!("Expected {}{not} to equal collection {:?}", self.expr_str, expected_vec);
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }
}

// Implementation for Vec
impl<T: Debug + Clone + PartialEq> CollectionMatchers<T> for Expectation<Vec<T>> {
    fn to_be_empty(self) {
        let result = self.value.is_empty();
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("is{not} empty"));
        } else {
            let expected_msg = format!("Expected {}{not} to be empty", self.expr_str);
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }

    fn to_have_length(self, expected: usize) {
        let actual_length = self.value.len();
        let result = actual_length == expected;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("does{not} have length {}", expected));
        } else {
            let expected_msg = format!("Expected {}{not} to have length {}", self.expr_str, expected);
            self.report_failure(&expected_msg, &format!("Actual length: {}", actual_length));
        }
    }

    fn to_contain<U: PartialEq<T> + Debug>(self, expected: U) {
        let result = self.value.iter().any(|item| expected == *item);
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("does{not} contain {:?}", expected));
        } else {
            let expected_msg = format!("Expected {}{not} to contain {:?}", self.expr_str, expected);
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }

    fn to_contain_all_of<U: PartialEq<T> + Debug>(self, expected: &[U]) {
        let result = expected.iter().all(|expected_item| self.value.iter().any(|actual_item| *expected_item == *actual_item));
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("does{not} contain all of {:?}", expected));
        } else {
            let expected_msg = format!("Expected {}{not} to contain all of {:?}", self.expr_str, expected);
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }

    fn to_equal_collection<U: IntoIterator<Item = T>>(self, expected: U)
    where
        T: PartialEq,
    {
        // Convert expected into Vec for easier comparison
        let expected_vec: Vec<T> = expected.into_iter().collect();

        // Check if lengths match first
        if self.value.len() != expected_vec.len() {
            let success = if self.negated { true } else { false };
            let not = if self.negated { " not" } else { "" };

            if success {
                self.report_success(&format!("does{not} equal collection {:?}", expected_vec));
            } else {
                let expected_msg = format!("Expected {}{not} to equal collection {:?}", self.expr_str, expected_vec);
                self.report_failure(
                    &expected_msg,
                    &format!("Collections have different lengths: {} vs {}", self.value.len(), expected_vec.len()),
                );
            }
            return;
        }

        // Check each element
        let result = self.value.iter().zip(expected_vec.iter()).all(|(actual, expected)| actual == expected);

        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("does{not} equal collection {:?}", expected_vec));
        } else {
            let expected_msg = format!("Expected {}{not} to equal collection {:?}", self.expr_str, expected_vec);
            self.report_failure(&expected_msg, &format!("Received: {:?}", self.value));
        }
    }
}
