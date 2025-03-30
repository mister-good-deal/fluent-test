use fluent_test::prelude::*;

#[test]
fn test_and_modifier() {
    // Disable deduplication for tests
    fluent_test::Reporter::disable_deduplication();

    let value = 42;

    // Both assertions pass - this should work and return true
    let chain = expect!(value).to_be_greater_than(30).and().to_be_less_than(50);
    let result = chain.evaluate();
    assert!(result, "AND chain with all passing conditions should evaluate to true");
}

#[test]
fn test_and_modifier_failing() {
    // Disable deduplication for tests
    fluent_test::Reporter::disable_deduplication();

    let value = 42;
    // First passes, second fails - should return false, not panic
    let chain = expect!(value).to_be_greater_than(30).and().to_be_less_than(40);
    let result = chain.evaluate();
    assert!(!result, "AND chain with a failing condition should evaluate to false");
}

#[test]
fn test_or_modifier() {
    // Disable deduplication for tests
    fluent_test::Reporter::disable_deduplication();

    let value = 42;

    // Create a variable to hold the chain so it doesn't get dropped immediately
    let chain = expect!(value)
        .to_be_greater_than(100) // This fails
        .or()
        .to_be_less_than(100); // This passes

    // Manually evaluate the chain - should not panic because OR with a passing condition
    // It should return true since one of the conditions passes
    let result = chain.evaluate();
    assert!(result, "OR chain with one passing condition should evaluate to true");
}

#[test]
fn test_or_modifier_failing() {
    let value = 42;
    // Both fail - should return false, not panic
    let chain = expect!(value).to_be_greater_than(100).or().to_be_less_than(10);
    // Manually evaluate the chain - should return false because both conditions fail
    let result = chain.evaluate();
    assert!(!result, "OR chain with all failing conditions should evaluate to false");
}

#[test]
fn test_not_with_and_or() {
    // Disable deduplication for tests
    fluent_test::Reporter::disable_deduplication();

    let value = 42;

    // Both conditions pass
    let chain = expect!(value)
        .not()
        .to_be_less_than(30) // "not less than 30" is true for 42
        .and()
        .not()
        .to_be_greater_than(50); // "not greater than 50" is true for 42

    let result = chain.evaluate();
    assert!(result, "NOT chain with AND should evaluate to true when both conditions are true");
}
