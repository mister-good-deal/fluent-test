# Boolean Matchers

## to_be_true

Checks if a boolean is true.

```rust
fn test_boolean_true() {
    let is_enabled = true;
    let is_disabled = false;
    
    expect!(is_enabled).to_be_true();     // Passes
    expect!(is_disabled).not().to_be_true(); // Passes
}
```

## to_be_false

Checks if a boolean is false.

```rust
fn test_boolean_false() {
    let is_enabled = true;
    let is_disabled = false;
    
    expect!(is_disabled).to_be_false();     // Passes
    expect!(is_enabled).not().to_be_false(); // Passes
}
```