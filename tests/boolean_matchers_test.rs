use fluent_test::prelude::*;

#[test]
fn test_boolean_true() {
    // Disable deduplication for tests
    fluent_test::Reporter::disable_deduplication();

    // This should pass
    expect!(true).to_be_true();
    expect!(false).not().to_be_true();
}

#[test]
#[should_panic]
fn test_not_true_fails() {
    expect!(true).not().to_be_true();
}

#[test]
#[should_panic]
fn test_false_to_be_true_fails() {
    expect!(false).to_be_true();
}

#[test]
fn test_boolean_false() {
    // Disable deduplication for tests
    fluent_test::Reporter::disable_deduplication();

    // This should pass
    expect!(false).to_be_false();
    expect!(true).not().to_be_false();
}

#[test]
#[should_panic]
fn test_not_false_fails() {
    expect!(false).not().to_be_false();
}

#[test]
#[should_panic]
fn test_true_to_be_false_fails() {
    expect!(true).to_be_false();
}
