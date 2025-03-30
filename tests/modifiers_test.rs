use fluent_test::prelude::*;

#[test]
fn test_and_modifier() {
    let value = 42;

    // Both assertions pass - this should work
    expect!(value).to_be_greater_than(30).and().to_be_less_than(50);

    // First passes, second fails - should panic overall
    let failing_test = || {
        expect!(value).to_be_greater_than(30).and().to_be_less_than(40);
    };

    assert!(std::panic::catch_unwind(failing_test).is_err());
}

#[test]
fn test_or_modifier() {
    let value = 42;

    // One of them passes (the second one), so the whole thing should pass
    expect!(value)
        .to_be_greater_than(100) // This fails
        .or()
        .to_be_less_than(100); // This passes

    // Both fail - should panic
    let failing_test = || {
        expect!(value).to_be_greater_than(100).or().to_be_less_than(10);
    };

    assert!(std::panic::catch_unwind(failing_test).is_err());
}

#[test]
fn test_not_with_and_or() {
    let value = 42;

    // Both conditions pass
    expect!(value)
        .not()
        .to_be_less_than(30) // "not less than 30" is true for 42
        .and()
        .not()
        .to_be_greater_than(50); // "not greater than 50" is true for 42
}
