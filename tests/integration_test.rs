use fluent_test::prelude::*;
mod common;

#[test]
fn test_basic_assertions() {
    let value = 42;

    expect!(value).to_equal(42);
    expect!(value).not().to_equal(13);
    expect!(value).to_be_greater_than(30);
    expect!(value).to_be_less_than(50);
}

#[test]
fn test_with_standard_assertions() {
    // Can mix with standard assertions
    let value = 42;
    assert_eq!(value, 42);

    expect!(value).to_be_even();
}

// This test is designed to fail to demonstrate error reporting
#[test]
#[should_panic]
fn test_failure_reporting() {
    let my_special_value = 13;
    expect!(my_special_value).to_be_even();
}
