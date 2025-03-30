use fluent_test::prelude::*;

#[test]
fn test_result_to_be_ok() {
    // Disable deduplication for tests
    fluent_test::Reporter::disable_deduplication();

    let ok_result: Result<i32, &str> = Ok(42);
    let err_result: Result<i32, &str> = Err("error");

    // Positive test cases
    expect!(&ok_result).to_be_ok();
    expect!(&err_result).not().to_be_ok();
}

#[test]
#[should_panic]
fn test_err_to_be_ok_fails() {
    let err_result: Result<i32, &str> = Err("error");
    expect!(&err_result).to_be_ok();
}

#[test]
#[should_panic]
fn test_ok_not_to_be_ok_fails() {
    let ok_result: Result<i32, &str> = Ok(42);
    expect!(&ok_result).not().to_be_ok();
}

#[test]
fn test_result_to_be_err() {
    // Disable deduplication for tests
    fluent_test::Reporter::disable_deduplication();

    let ok_result: Result<i32, &str> = Ok(42);
    let err_result: Result<i32, &str> = Err("error");

    // Positive test cases
    expect!(&err_result).to_be_err();
    expect!(&ok_result).not().to_be_err();
}

#[test]
#[should_panic]
fn test_ok_to_be_err_fails() {
    let ok_result: Result<i32, &str> = Ok(42);
    expect!(&ok_result).to_be_err();
}

#[test]
#[should_panic]
fn test_err_not_to_be_err_fails() {
    let err_result: Result<i32, &str> = Err("error");
    expect!(&err_result).not().to_be_err();
}

#[test]
fn test_result_to_contain_ok() {
    // Disable deduplication for tests
    fluent_test::Reporter::disable_deduplication();

    let ok_result: Result<i32, &str> = Ok(42);
    let different_ok: Result<i32, &str> = Ok(100);
    let err_result: Result<i32, &str> = Err("error");

    // Positive test cases
    expect!(&ok_result).to_contain_ok(42);
    expect!(&different_ok).not().to_contain_ok(42);
    expect!(&err_result).not().to_contain_ok(42);
}

#[test]
#[should_panic]
fn test_wrong_ok_value_fails() {
    let ok_result: Result<i32, &str> = Ok(42);
    expect!(&ok_result).to_contain_ok(100);
}

#[test]
#[should_panic]
fn test_right_ok_value_not_fails() {
    let ok_result: Result<i32, &str> = Ok(42);
    expect!(&ok_result).not().to_contain_ok(42);
}

#[test]
fn test_result_to_contain_err() {
    // Disable deduplication for tests
    fluent_test::Reporter::disable_deduplication();

    let err_result: Result<i32, &str> = Err("network error");
    let different_err: Result<i32, &str> = Err("authentication error");
    let ok_result: Result<i32, &str> = Ok(42);

    // Positive test cases
    expect!(&err_result).to_contain_err("network error");
    expect!(&different_err).not().to_contain_err("network error");
    expect!(&ok_result).not().to_contain_err("network error");
}

#[test]
#[should_panic]
fn test_wrong_err_value_fails() {
    let err_result: Result<i32, &str> = Err("network error");
    expect!(&err_result).to_contain_err("authentication error");
}

#[test]
#[should_panic]
fn test_right_err_value_not_fails() {
    let err_result: Result<i32, &str> = Err("network error");
    expect!(&err_result).not().to_contain_err("network error");
}
