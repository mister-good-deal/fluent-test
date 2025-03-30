use crate::expectation::Expectation;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub trait HashMapMatchers<K, V> {
    fn to_be_empty(self) -> Self;
    fn to_have_length(self, expected: usize) -> Self;
    fn to_contain_key<Q>(self, key: &Q) -> Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized;
    fn to_contain_entry<Q, R>(self, key: &Q, value: &R) -> Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
        V: Borrow<R>,
        R: PartialEq + Debug + ?Sized;
}

impl<K, V> HashMapMatchers<K, V> for Expectation<&HashMap<K, V>>
where
    K: Eq + Hash + Debug + Clone,
    V: Debug + Clone,
{
    fn to_be_empty(self) -> Self {
        let result = self.value.is_empty();
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("is{not} empty");

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }

    fn to_have_length(self, expected: usize) -> Self {
        let actual_length = self.value.len();
        let result = actual_length == expected;
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} have length {}", expected);

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }

    fn to_contain_key<Q>(self, key: &Q) -> Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
    {
        let result = self.value.contains_key(key);
        let success = if self.negated { !result } else { result };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} contain key {:?}", key);

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }

    fn to_contain_entry<Q, R>(self, key: &Q, value: &R) -> Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
        V: Borrow<R>,
        R: PartialEq + Debug + ?Sized,
    {
        let entry_exists = self.value.get(key).map(|v| v.borrow() == value).unwrap_or(false);

        let success = if self.negated { !entry_exists } else { entry_exists };
        let not = if self.negated { " not" } else { "" };

        let description = format!("does{not} contain entry ({:?}, {:?})", key, value);

        // Add this assertion to the chain instead of immediately evaluating
        self.add_assertion_step(description, success)
    }
}
