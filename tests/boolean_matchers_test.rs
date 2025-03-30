use fluent_test::prelude::*;

#[test]
fn test_boolean_to_be_true() {
    // Positive test cases
    expect!(true).to_be_true();
    expect!(false).not().to_be_true();

    // Negative test cases - should panic
    let test_not_true_fails = || {
        expect!(true).not().to_be_true();
    };

    let test_false_to_be_true_fails = || {
        expect!(false).to_be_true();
    };

    assert!(std::panic::catch_unwind(|| test_not_true_fails()).is_err());
    assert!(std::panic::catch_unwind(|| test_false_to_be_true_fails()).is_err());
}

#[test]
fn test_boolean_to_be_false() {
    // Positive test cases
    expect!(false).to_be_false();
    expect!(true).not().to_be_false();

    // Negative test cases - should panic
    let test_not_false_fails = || {
        expect!(false).not().to_be_false();
    };

    let test_true_to_be_false_fails = || {
        expect!(true).to_be_false();
    };

    assert!(std::panic::catch_unwind(|| test_not_false_fails()).is_err());
    assert!(std::panic::catch_unwind(|| test_true_to_be_false_fails()).is_err());
}
