use fluent_test::prelude::*;
use std::collections::HashMap;

#[test]
fn test_hashmap_to_be_empty() {
    // Disable deduplication for tests
    fluent_test::Reporter::disable_deduplication();

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
    fluent_test::Reporter::disable_deduplication();

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
    fluent_test::Reporter::disable_deduplication();

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
    fluent_test::Reporter::disable_deduplication();

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
