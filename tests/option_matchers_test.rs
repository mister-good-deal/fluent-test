use fluent_test::prelude::*;

#[test]
fn test_option_to_be_some() {
    // Disable deduplication for tests
    fluent_test::Reporter::disable_deduplication();

    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    // Positive test cases
    expect!(&some_value).to_be_some();
    expect!(&none_value).not().to_be_some();
}

#[test]
#[should_panic]
fn test_none_to_be_some_fails() {
    let none_value: Option<i32> = None;
    expect!(&none_value).to_be_some();
}

#[test]
#[should_panic]
fn test_some_not_to_be_some_fails() {
    let some_value: Option<i32> = Some(42);
    expect!(&some_value).not().to_be_some();
}

#[test]
fn test_option_to_be_none() {
    // Disable deduplication for tests
    fluent_test::Reporter::disable_deduplication();

    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    // Positive test cases
    expect!(&none_value).to_be_none();
    expect!(&some_value).not().to_be_none();
}

#[test]
#[should_panic]
fn test_some_to_be_none_fails() {
    let some_value: Option<i32> = Some(42);
    expect!(&some_value).to_be_none();
}

#[test]
#[should_panic]
fn test_none_not_to_be_none_fails() {
    let none_value: Option<i32> = None;
    expect!(&none_value).not().to_be_none();
}

#[test]
fn test_option_to_contain_value() {
    // Disable deduplication for tests
    fluent_test::Reporter::disable_deduplication();

    let some_value: Option<i32> = Some(42);
    let other_value: Option<i32> = Some(100);
    let none_value: Option<i32> = None;

    // Positive test cases
    expect!(&some_value).to_contain_value(42);
    expect!(&other_value).not().to_contain_value(42);
    expect!(&none_value).not().to_contain_value(42);
}

#[test]
#[should_panic]
fn test_wrong_value_fails() {
    let some_value: Option<i32> = Some(42);
    expect!(&some_value).to_contain_value(100);
}

#[test]
#[should_panic]
fn test_right_value_not_fails() {
    let some_value: Option<i32> = Some(42);
    expect!(&some_value).not().to_contain_value(42);
}
