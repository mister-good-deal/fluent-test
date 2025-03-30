use fluent_test::prelude::*;
use std::collections::HashMap;

#[test]
fn test_hashmap_to_be_empty() {
    let empty_map: HashMap<&str, i32> = HashMap::new();
    let mut non_empty_map = HashMap::new();
    non_empty_map.insert("Alice", 100);

    // Positive test cases
    expect!(&empty_map).to_be_empty();
    expect!(&non_empty_map).not().to_be_empty();

    // Negative test cases - should panic
    let test_non_empty_to_be_empty_fails = || {
        let mut map = HashMap::new();
        map.insert("Alice", 100);
        expect!(&map).to_be_empty();
    };

    let test_empty_not_to_be_empty_fails = || {
        let map: HashMap<&str, i32> = HashMap::new();
        expect!(&map).not().to_be_empty();
    };

    assert!(std::panic::catch_unwind(test_non_empty_to_be_empty_fails).is_err());
    assert!(std::panic::catch_unwind(test_empty_not_to_be_empty_fails).is_err());
}

#[test]
fn test_hashmap_to_have_length() {
    let mut map = HashMap::new();
    map.insert("Alice", 100);
    map.insert("Bob", 85);

    // Positive test cases
    expect!(&map).to_have_length(2);
    expect!(&map).not().to_have_length(3);

    // Negative test cases - should panic
    let test_wrong_length_fails = || {
        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);
        expect!(&map).to_have_length(3);
    };

    let test_right_length_not_fails = || {
        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);
        expect!(&map).not().to_have_length(2);
    };

    assert!(std::panic::catch_unwind(test_wrong_length_fails).is_err());
    assert!(std::panic::catch_unwind(test_right_length_not_fails).is_err());
}

#[test]
fn test_hashmap_to_contain_key() {
    let mut map = HashMap::new();
    map.insert("Alice", 100);
    map.insert("Bob", 85);

    // Positive test cases
    expect!(&map).to_contain_key("Alice");
    expect!(&map).not().to_contain_key("Charlie");

    // Negative test cases - should panic
    let test_missing_key_fails = || {
        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);
        expect!(&map).to_contain_key("Charlie");
    };

    let test_present_key_not_fails = || {
        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);
        expect!(&map).not().to_contain_key("Alice");
    };

    assert!(std::panic::catch_unwind(test_missing_key_fails).is_err());
    assert!(std::panic::catch_unwind(test_present_key_not_fails).is_err());
}

#[test]
fn test_hashmap_to_contain_entry() {
    let mut map = HashMap::new();
    map.insert("Alice", 100);
    map.insert("Bob", 85);

    // Positive test cases
    expect!(&map).to_contain_entry("Alice", &100);
    expect!(&map).not().to_contain_entry("Alice", &50);
    expect!(&map).not().to_contain_entry("Charlie", &100);

    // Negative test cases - should panic
    let test_wrong_value_fails = || {
        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);
        expect!(&map).to_contain_entry("Alice", &50);
    };

    let test_missing_key_fails = || {
        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);
        expect!(&map).to_contain_entry("Charlie", &100);
    };

    let test_right_entry_not_fails = || {
        let mut map = HashMap::new();
        map.insert("Alice", 100);
        map.insert("Bob", 85);
        expect!(&map).not().to_contain_entry("Alice", &100);
    };

    assert!(std::panic::catch_unwind(test_wrong_value_fails).is_err());
    assert!(std::panic::catch_unwind(test_missing_key_fails).is_err());
    assert!(std::panic::catch_unwind(test_right_entry_not_fails).is_err());
}
