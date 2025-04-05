use fluent_test::prelude::*;
use std::collections::HashMap;
mod helpers;

fn main() {
    // Boolean matchers
    let is_enabled = true;
    expect!(is_enabled).to_be_true();
    expect!(!is_enabled).to_be_false();

    // Option matchers
    let maybe_value: Option<i32> = Some(42);
    expect!(&maybe_value).to_be_some();
    expect!(&maybe_value).to_contain(&42);

    let empty_option: Option<i32> = None;
    expect!(&empty_option).to_be_none();
    expect!(&empty_option).not().to_be_some();

    // Result matchers
    let success_result: Result<i32, &str> = Ok(42);
    expect!(&success_result).to_be_ok();
    expect!(&success_result).to_contain_ok(&42);

    let error_result: Result<i32, &str> = Err("failed");
    expect!(&error_result).to_be_err();
    expect!(&error_result).to_contain_err(&"failed");

    // Collection matchers
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers_slice = numbers.as_slice();
    expect!(numbers_slice).to_have_length(5);
    expect!(numbers_slice).to_contain(3);
    expect!(numbers_slice).to_contain_all_of(&[1, 5]);
    expect!(numbers_slice).to_equal_collection(&[1, 2, 3, 4, 5]);

    let empty_vec: Vec<i32> = vec![];
    let empty_slice = empty_vec.as_slice();
    expect!(empty_slice).to_be_empty();

    // HashMap matchers
    let mut user_scores = HashMap::new();
    user_scores.insert("Alice", 100);
    user_scores.insert("Bob", 85);

    expect!(&user_scores).to_have_length(2);
    expect!(&user_scores).to_contain_key("Alice");
    expect!(&user_scores).to_contain_entry("Bob", &85);
    expect!(&user_scores).not().to_contain_key("Charlie");

    // Report test results
    fluent_test::Reporter::summarize();
}
