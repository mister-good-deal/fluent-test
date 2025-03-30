use fluent_test::prelude::*;

// Test empty collection
#[test]
fn test_empty_collection() {
    fluent_test::Reporter::disable_deduplication();

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
    fluent_test::Reporter::disable_deduplication();

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
    fluent_test::Reporter::disable_deduplication();

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
    fluent_test::Reporter::disable_deduplication();

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
    fluent_test::Reporter::disable_deduplication();

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
