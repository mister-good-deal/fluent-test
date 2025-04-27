# String Matchers

## to_be_empty

Checks if a string is empty.

```rust
fn test_empty_string() {
    let empty = "";
    let not_empty = "hello";
    
    expect!(empty).to_be_empty();        // Passes
    expect!(not_empty).not().to_be_empty(); // Passes
}
```

## to_contain

Checks if a string contains a specified substring.

```rust
fn test_string_contains() {
    let greeting = "Hello, world!";
    
    expect!(greeting).to_contain("world");       // Passes
    expect!(greeting).not().to_contain("moon");  // Passes
}
```

## to_start_with

Checks if a string starts with a specified prefix.

```rust
fn test_string_starts_with() {
    let greeting = "Hello, world!";
    
    expect!(greeting).to_start_with("Hello");       // Passes
    expect!(greeting).not().to_start_with("Goodbye"); // Passes
}
```

## to_end_with

Checks if a string ends with a specified suffix.

```rust
fn test_string_ends_with() {
    let greeting = "Hello, world!";
    
    expect!(greeting).to_end_with("world!");        // Passes
    expect!(greeting).not().to_end_with("universe"); // Passes
}
```

## to_match

Checks if a string matches a pattern.

```rust
fn test_string_matches() {
    let greeting = "Hello, world!";
    
    expect!(greeting).to_match("world");        // Passes
    expect!(greeting).not().to_match("goodbye"); // Passes
}
```

## to_have_length

Checks if a string has a specific length.

```rust
fn test_string_length() {
    let greeting = "Hello, world!";
    
    expect!(greeting).to_have_length(13);        // Passes
    expect!(greeting).not().to_have_length(10);  // Passes
}
```
