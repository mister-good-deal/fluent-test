use crate::backend::Expectation;
use crate::backend::assertions::sentence::AssertionSentence;
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
        V: Borrow<R>,
        Q: Hash + Eq + Debug + ?Sized,
        R: PartialEq + Debug + ?Sized;
}

use std::borrow::Borrow;

impl<K, V> HashMapMatchers<K, V> for Expectation<&HashMap<K, V>>
where
    K: Hash + Eq + Debug + Clone,
    V: Debug + Clone,
{
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

    fn to_contain_key<Q>(self, key: &Q) -> Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
    {
        let result = self.value.contains_key(key);
        let sentence = AssertionSentence::new("contain", format!("key {:?}", key));

        return self.add_assertion_step(sentence, result);
    }

    fn to_contain_entry<Q, R>(self, key: &Q, value: &R) -> Self
    where
        K: Borrow<Q>,
        V: Borrow<R>,
        Q: Hash + Eq + Debug + ?Sized,
        R: PartialEq + Debug + ?Sized,
    {
        let result = self.value.get(key).is_some_and(|v| v.borrow() == value);
        let sentence = AssertionSentence::new("contain", format!("entry ({:?}, {:?})", key, value));

        return self.add_assertion_step(sentence, result);
    }
}

// Implementation for owned HashMap
impl<K, V> HashMapMatchers<K, V> for Expectation<HashMap<K, V>>
where
    K: Hash + Eq + Debug + Clone,
    V: Debug + Clone,
{
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

    fn to_contain_key<Q>(self, key: &Q) -> Self
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Debug + ?Sized,
    {
        let result = self.value.contains_key(key);
        let sentence = AssertionSentence::new("contain", format!("key {:?}", key));

        return self.add_assertion_step(sentence, result);
    }

    fn to_contain_entry<Q, R>(self, key: &Q, value: &R) -> Self
    where
        K: Borrow<Q>,
        V: Borrow<R>,
        Q: Hash + Eq + Debug + ?Sized,
        R: PartialEq + Debug + ?Sized,
    {
        let result = self.value.get(key).is_some_and(|v| v.borrow() == value);
        let sentence = AssertionSentence::new("contain", format!("entry ({:?}, {:?})", key, value));

        return self.add_assertion_step(sentence, result);
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

        let empty: HashMap<i32, i32> = HashMap::new();
        let not_empty: HashMap<i32, i32> = [(1, 2)].iter().cloned().collect();

        // These should pass
        expect!(&empty).to_be_empty();
        expect!(&not_empty).not().to_be_empty();
    }

    #[test]
    #[should_panic(expected = "be empty")]
    fn test_non_empty_to_be_empty_fails() {
        let map: HashMap<i32, i32> = [(1, 2)].iter().cloned().collect();
        let _assertion = expect!(&map).to_be_empty();
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "not be empty")]
    fn test_empty_not_to_be_empty_fails() {
        let map: HashMap<i32, i32> = HashMap::new();
        let _assertion = expect!(&map).not().to_be_empty();
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_hashmap_to_have_length() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let map: HashMap<i32, i32> = [(1, 2), (3, 4)].iter().cloned().collect();

        // These should pass
        expect!(&map).to_have_length(2);
        expect!(&map).not().to_have_length(3);
    }

    #[test]
    #[should_panic(expected = "have length")]
    fn test_wrong_length_fails() {
        let map: HashMap<i32, i32> = [(1, 2)].iter().cloned().collect();
        let _assertion = expect!(&map).to_have_length(2);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "not have length")]
    fn test_right_length_not_fails() {
        let map: HashMap<i32, i32> = [(1, 2)].iter().cloned().collect();
        let _assertion = expect!(&map).not().to_have_length(1);
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_hashmap_to_contain_key() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let map: HashMap<i32, i32> = [(1, 2), (3, 4)].iter().cloned().collect();

        // These should pass
        expect!(&map).to_contain_key(&1);
        expect!(&map).not().to_contain_key(&2);
    }

    #[test]
    #[should_panic(expected = "not contain key")]
    fn test_present_key_not_fails() {
        let map: HashMap<i32, i32> = [(1, 2)].iter().cloned().collect();
        let _assertion = expect!(&map).not().to_contain_key(&1);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "contain key")]
    fn test_missing_key_fails() {
        let map: HashMap<i32, i32> = [(1, 2)].iter().cloned().collect();
        let _assertion = expect!(&map).to_contain_key(&2);
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_hashmap_to_contain_entry() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let map: HashMap<i32, i32> = [(1, 2), (3, 4)].iter().cloned().collect();

        // These should pass
        expect!(&map).to_contain_entry(&1, &2);
        expect!(&map).not().to_contain_entry(&1, &3);
        expect!(&map).not().to_contain_entry(&2, &3);
    }

    #[test]
    #[should_panic(expected = "not contain entry")]
    fn test_right_entry_not_fails() {
        let map: HashMap<i32, i32> = [(1, 2)].iter().cloned().collect();
        let _assertion = expect!(&map).not().to_contain_entry(&1, &2);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "contain entry")]
    fn test_missing_key_entry_fails() {
        let map: HashMap<i32, i32> = [(1, 2)].iter().cloned().collect();
        let _assertion = expect!(&map).to_contain_entry(&2, &3);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "contain entry")]
    fn test_wrong_value_fails() {
        let map: HashMap<i32, i32> = [(1, 2)].iter().cloned().collect();
        let _assertion = expect!(&map).to_contain_entry(&1, &3);
        std::hint::black_box(_assertion);
    }
}
