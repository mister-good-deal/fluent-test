# Option Matchers

## to_be_some

Checks if an Option contains a value.

```rust
fn test_option_some() {
    let maybe_value: Option<i32> = Some(42);
    let empty_option: Option<i32> = None;
    
    expect!(&maybe_value).to_be_some();         // Passes
    expect!(&empty_option).not().to_be_some();  // Passes
}
```

## to_be_none

Checks if an Option is None.

```rust
fn test_option_none() {
    let maybe_value: Option<i32> = Some(42);
    let empty_option: Option<i32> = None;
    
    expect!(&empty_option).to_be_none();        // Passes
    expect!(&maybe_value).not().to_be_none();   // Passes
}
```

## to_contain_value

Checks if an Option contains a specific value.

```rust
fn test_option_contains_value() {
    let maybe_value: Option<i32> = Some(42);
    let other_value: Option<i32> = Some(100);
    
    expect!(&maybe_value).to_contain_value(42);        // Passes
    expect!(&other_value).not().to_contain_value(42);  // Passes
}
```