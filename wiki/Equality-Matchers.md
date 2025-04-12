# Equality Matchers

## to_equal

Checks if a value equals another value.

```rust
fn test_equality() {
    let value = 42;
    let name = "Arthur";
    
    expect!(value).to_equal(42);         // Passes
    expect!(value).not().to_equal(13);   // Passes
    expect!(name).to_equal("Arthur");    // Passes
}
```