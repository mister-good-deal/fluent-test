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

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::collections::HashMap;

    #[test]
    fn test_hashmap_to_be_empty() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let empty_map: HashMap<&str, i32> = HashMap::new();
        let mut non_empty_map = HashMap::new();
        non_empty_map.insert("Alice", 100);

        // Positive test cases
        expect!(&empty_map).to_be_empty();
        expect!(&non_empty_map).not().to_be_empty();
    }

    #[test]
    #[should_panic]
    fn test_non_empty_to_be_empty_fails() {
        let mut map = HashMap::new();
        map.insert("Alice", 100);
        expect!(&map).to_be_empty();
    }

    #[test]
    #[should_panic]
    fn test_empty_not_to_be_empty_fails() {
        let map: HashMap<&str, i32> = HashMap::new();
        expect!(&map).not().to_be_empty();
    }

    #[test]
    fn test_hashmap_to_have_length() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);

        // Positive test cases
        expect!(&map).to_have_length(2);
        expect!(&map).not().to_have_length(3);
    }

    #[test]
    #[should_panic]
    fn test_wrong_length_fails() {
        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);
        expect!(&map).to_have_length(3);
    }

    #[test]
    #[should_panic]
    fn test_right_length_not_fails() {
        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);
        expect!(&map).not().to_have_length(2);
    }

    #[test]
    fn test_hashmap_to_contain_key() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);

        // Positive test cases
        expect!(&map).to_contain_key("Alice");
        expect!(&map).not().to_contain_key("Charlie");
    }

    #[test]
    #[should_panic]
    fn test_missing_key_fails() {
        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);
        expect!(&map).to_contain_key("Charlie");
    }

    #[test]
    #[should_panic]
    fn test_present_key_not_fails() {
        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);
        expect!(&map).not().to_contain_key("Alice");
    }

    #[test]
    fn test_hashmap_to_contain_entry() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);

        // Positive test cases
        expect!(&map).to_contain_entry("Alice", &100);
        expect!(&map).not().to_contain_entry("Alice", &50);
        expect!(&map).not().to_contain_entry("Charlie", &100);
    }

    #[test]
    #[should_panic]
    fn test_wrong_value_fails() {
        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);
        expect!(&map).to_contain_entry("Alice", &50);
    }

    #[test]
    #[should_panic]
    fn test_missing_key_entry_fails() {
        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);
        expect!(&map).to_contain_entry("Charlie", &100);
    }

    #[test]
    #[should_panic]
    fn test_right_entry_not_fails() {
        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);
        expect!(&map).not().to_contain_entry("Alice", &100);
    }
}
