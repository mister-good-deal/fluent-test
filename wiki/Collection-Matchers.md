# Collection Matchers

## to_be_empty

Checks if a collection is empty.

```rust
fn test_empty_collection() {
    let empty_vec: Vec<i32> = vec![];
    let non_empty_vec = vec![1, 2, 3];
    
    expect!(empty_vec.as_slice()).to_be_empty();             // Passes
    expect!(non_empty_vec.as_slice()).not().to_be_empty();   // Passes
}
```

## to_have_length

Checks if a collection has a specific length.

```rust
fn test_collection_length() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    expect!(numbers.as_slice()).to_have_length(5);         // Passes
    expect!(numbers.as_slice()).not().to_have_length(3);   // Passes
}
```

## to_contain

Checks if a collection contains a specific element.

```rust
fn test_collection_contains() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    expect!(numbers.as_slice()).to_contain(3);           // Passes
    expect!(numbers.as_slice()).not().to_contain(10);    // Passes
}
```

## to_contain_all_of

Checks if a collection contains all of the specified elements.

```rust
fn test_collection_contains_all() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    expect!(numbers.as_slice()).to_contain_all_of(&[1, 3, 5]);        // Passes
    expect!(numbers.as_slice()).not().to_contain_all_of(&[1, 6, 7]);  // Passes
}
```

## to_equal_collection

Compares two collections for element-wise equality.

```rust
fn test_equal_collection() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    expect!(numbers.as_slice()).to_equal_collection(vec![1, 2, 3, 4, 5]);         // Passes
    expect!(numbers.as_slice()).not().to_equal_collection(vec![5, 4, 3, 2, 1]);   // Passes
}
```