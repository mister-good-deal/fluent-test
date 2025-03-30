use fluent_test::prelude::*;

#[test]
fn test_collection_to_be_empty() {
    let empty_vec: Vec<i32> = vec![];
    let non_empty_vec = vec![1, 2, 3];

    // Positive test cases
    expect!(empty_vec.as_slice()).to_be_empty();
    expect!(non_empty_vec.as_slice()).not().to_be_empty();

    // Negative test cases - should panic
    let test_non_empty_to_be_empty_fails = || {
        let non_empty_vec = vec![1, 2, 3];
        expect!(non_empty_vec.as_slice()).to_be_empty();
    };

    let test_empty_not_to_be_empty_fails = || {
        let empty_vec: Vec<i32> = vec![];
        expect!(empty_vec.as_slice()).not().to_be_empty();
    };

    assert!(std::panic::catch_unwind(test_non_empty_to_be_empty_fails).is_err());
    assert!(std::panic::catch_unwind(test_empty_not_to_be_empty_fails).is_err());
}

#[test]
fn test_collection_to_have_length() {
    let vec = vec![1, 2, 3, 4, 5];

    // Positive test cases
    expect!(vec.as_slice()).to_have_length(5);
    expect!(vec.as_slice()).not().to_have_length(3);

    // Negative test cases - should panic
    let test_wrong_length_fails = || {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).to_have_length(3);
    };

    let test_right_length_not_fails = || {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).not().to_have_length(5);
    };

    assert!(std::panic::catch_unwind(test_wrong_length_fails).is_err());
    assert!(std::panic::catch_unwind(test_right_length_not_fails).is_err());
}

#[test]
fn test_collection_to_contain() {
    let vec = vec![1, 2, 3, 4, 5];

    // Positive test cases
    expect!(vec.as_slice()).to_contain(3);
    expect!(vec.as_slice()).not().to_contain(10);

    // Negative test cases - should panic
    let test_missing_value_fails = || {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).to_contain(10);
    };

    let test_present_value_not_fails = || {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).not().to_contain(3);
    };

    assert!(std::panic::catch_unwind(test_missing_value_fails).is_err());
    assert!(std::panic::catch_unwind(test_present_value_not_fails).is_err());
}

#[test]
fn test_collection_to_contain_all_of() {
    let vec = vec![1, 2, 3, 4, 5];

    // Positive test cases
    expect!(vec.as_slice()).to_contain_all_of(&[1, 3, 5]);
    expect!(vec.as_slice()).not().to_contain_all_of(&[1, 6, 7]);

    // Negative test cases - should panic
    let test_missing_values_fails = || {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).to_contain_all_of(&[1, 3, 10]);
    };

    let test_present_values_not_fails = || {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).not().to_contain_all_of(&[1, 3, 5]);
    };

    assert!(std::panic::catch_unwind(test_missing_values_fails).is_err());
    assert!(std::panic::catch_unwind(test_present_values_not_fails).is_err());
}

#[test]
fn test_collection_to_equal_collection() {
    let vec = vec![1, 2, 3, 4, 5];

    // Positive test cases
    expect!(vec.as_slice()).to_equal_collection(vec![1, 2, 3, 4, 5]);
    expect!(vec.as_slice()).not().to_equal_collection(vec![5, 4, 3, 2, 1]);
    expect!(vec.as_slice()).not().to_equal_collection(vec![1, 2, 3]);

    // Negative test cases - should panic
    let test_different_collection_fails = || {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).to_equal_collection(vec![1, 2, 3, 4, 6]);
    };

    let test_shorter_collection_fails = || {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).to_equal_collection(vec![1, 2, 3]);
    };

    let test_same_collection_not_fails = || {
        let vec = vec![1, 2, 3, 4, 5];
        expect!(vec.as_slice()).not().to_equal_collection(vec![1, 2, 3, 4, 5]);
    };

    assert!(std::panic::catch_unwind(test_different_collection_fails).is_err());
    assert!(std::panic::catch_unwind(test_shorter_collection_fails).is_err());
    assert!(std::panic::catch_unwind(test_same_collection_not_fails).is_err());
}
