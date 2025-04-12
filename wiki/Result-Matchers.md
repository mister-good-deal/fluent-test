# Result Matchers

## to_be_ok

Checks if a Result is Ok.

```rust
fn test_result_ok() {
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("failed");
    
    expect!(&success).to_be_ok();         // Passes
    expect!(&failure).not().to_be_ok();  // Passes
}
```

## to_be_err

Checks if a Result is Err.

```rust
fn test_result_err() {
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("failed");
    
    expect!(&failure).to_be_err();        // Passes
    expect!(&success).not().to_be_err();  // Passes
}
```

## to_contain_ok

Checks if a Result contains a specific Ok value.

```rust
fn test_result_contains_ok() {
    let success: Result<i32, &str> = Ok(42);
    let other_success: Result<i32, &str> = Ok(100);
    
    expect!(&success).to_contain_ok(42);           // Passes
    expect!(&other_success).not().to_contain_ok(42); // Passes
}
```

## to_contain_err

Checks if a Result contains a specific error value.

```rust
fn test_result_contains_err() {
    let network_err: Result<i32, &str> = Err("network error");
    let auth_err: Result<i32, &str> = Err("authentication error");
    
    expect!(&network_err).to_contain_err("network error");            // Passes
    expect!(&auth_err).not().to_contain_err("network error");         // Passes
}
```