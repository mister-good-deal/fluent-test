use fluent_test::prelude::*;

#[test]
fn test_option_to_be_some() {
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    // Positive test cases
    expect!(&some_value).to_be_some();
    expect!(&none_value).not().to_be_some();

    // Negative test cases - should panic
    let test_none_to_be_some_fails = || {
        let none_value: Option<i32> = None;
        expect!(&none_value).to_be_some();
    };

    let test_some_not_to_be_some_fails = || {
        let some_value: Option<i32> = Some(42);
        expect!(&some_value).not().to_be_some();
    };

    assert!(std::panic::catch_unwind(test_none_to_be_some_fails).is_err());
    assert!(std::panic::catch_unwind(test_some_not_to_be_some_fails).is_err());
}

#[test]
fn test_option_to_be_none() {
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    // Positive test cases
    expect!(&none_value).to_be_none();
    expect!(&some_value).not().to_be_none();

    // Negative test cases - should panic
    let test_some_to_be_none_fails = || {
        let some_value: Option<i32> = Some(42);
        expect!(&some_value).to_be_none();
    };

    let test_none_not_to_be_none_fails = || {
        let none_value: Option<i32> = None;
        expect!(&none_value).not().to_be_none();
    };

    assert!(std::panic::catch_unwind(test_some_to_be_none_fails).is_err());
    assert!(std::panic::catch_unwind(test_none_not_to_be_none_fails).is_err());
}

#[test]
fn test_option_to_contain_value() {
    let some_value: Option<i32> = Some(42);
    let other_value: Option<i32> = Some(100);
    let none_value: Option<i32> = None;

    // Positive test cases
    expect!(&some_value).to_contain_value(42);
    expect!(&other_value).not().to_contain_value(42);
    expect!(&none_value).not().to_contain_value(42);

    // Negative test cases - should panic
    let test_wrong_value_fails = || {
        let some_value: Option<i32> = Some(42);
        expect!(&some_value).to_contain_value(100);
    };

    let test_right_value_not_fails = || {
        let some_value: Option<i32> = Some(42);
        expect!(&some_value).not().to_contain_value(42);
    };

    assert!(std::panic::catch_unwind(test_wrong_value_fails).is_err());
    assert!(std::panic::catch_unwind(test_right_value_not_fails).is_err());
}
