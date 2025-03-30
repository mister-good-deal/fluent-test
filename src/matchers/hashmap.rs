use crate::expectation::Expectation;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub trait HashMapMatchers<K, V> {
    fn to_be_empty(self);
    fn to_have_length(self, expected: usize);
    fn to_contain_key<Q>(self, key: &Q)
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized;
    fn to_contain_entry<Q, R>(self, key: &Q, value: &R)
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
        V: Borrow<R>,
        R: PartialEq + Debug + ?Sized;
}

impl<K, V> HashMapMatchers<K, V> for Expectation<&HashMap<K, V>>
where
    K: Eq + Hash + Debug,
    V: Debug,
{
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

    fn to_contain_key<Q>(self, key: &Q)
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
    {
        let result = self.value.contains_key(key);
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("does{not} contain key {:?}", key));
        } else {
            let expected_msg = format!("Expected {}{not} to contain key {:?}", self.expr_str, key);
            let keys: Vec<&K> = self.value.keys().collect();
            self.report_failure(&expected_msg, &format!("Available keys: {:?}", keys));
        }
    }

    fn to_contain_entry<Q, R>(self, key: &Q, value: &R)
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
        V: Borrow<R>,
        R: PartialEq + Debug + ?Sized,
    {
        let entry_exists = self.value.get(key).map(|v| v.borrow() == value).unwrap_or(false);

        let success = if self.negated { !entry_exists } else { entry_exists };
        let not = if self.negated { " not" } else { "" };

        if success {
            self.report_success(&format!("does{not} contain entry ({:?}, {:?})", key, value));
        } else {
            let expected_msg = format!("Expected {}{not} to contain entry ({:?}, {:?})", self.expr_str, key, value);

            // Check if the key exists to provide a more helpful error message
            let key_exists = self.value.contains_key(key);
            let details = if key_exists {
                // Get the actual value for this key
                let actual_value = self.value.get(key).unwrap();
                format!("Found key {:?} but with value: {:?}", key, actual_value)
            } else {
                format!("Key {:?} not found in map", key)
            };

            self.report_failure(&expected_msg, &details);
        }
    }
}
