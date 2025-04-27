# HashMap Matchers

## to_be_empty

Checks if a HashMap is empty.

```rust
fn test_empty_hashmap() {
    use std::collections::HashMap;
    
    let empty_map: HashMap<&str, i32> = HashMap::new();
    let mut non_empty_map = HashMap::new();
    non_empty_map.insert("Alice", 100);
    
    expect!(&empty_map).to_be_empty();             // Passes
    expect!(&non_empty_map).not().to_be_empty();   // Passes
}
```

## to_have_length

Checks if a HashMap has a specific number of entries.

```rust
fn test_hashmap_length() {
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    scores.insert("Alice", 100);
    scores.insert("Bob", 85);
    
    expect!(&scores).to_have_length(2);         // Passes
    expect!(&scores).not().to_have_length(1);   // Passes
}
```

## to_contain_key

Checks if a HashMap contains a specific key.

```rust
fn test_hashmap_contains_key() {
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    scores.insert("Alice", 100);
    scores.insert("Bob", 85);
    
    expect!(&scores).to_contain_key("Alice");          // Passes
    expect!(&scores).not().to_contain_key("Charlie");  // Passes
}
```

## to_contain_entry

Checks if a HashMap contains a specific key-value pair.

```rust
fn test_hashmap_contains_entry() {
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    scores.insert("Alice", 100);
    scores.insert("Bob", 85);
    
    expect!(&scores).to_contain_entry("Alice", &100);         // Passes
    expect!(&scores).not().to_contain_entry("Alice", &50);    // Passes
}
```
