# Numeric Matchers

## to_be_greater_than

Checks if a number is greater than another number.

```rust
fn test_greater_than() {
    let value = 42;
    
    expect!(value).to_be_greater_than(30);       // Passes
    expect!(value).not().to_be_greater_than(50); // Passes
}
```

## to_be_greater_than_or_equal

Checks if a number is greater than or equal to another number.

```rust
fn test_greater_than_or_equal() {
    let value = 42;
    
    expect!(value).to_be_greater_than_or_equal(42);       // Passes
    expect!(value).to_be_greater_than_or_equal(40);       // Passes
    expect!(value).not().to_be_greater_than_or_equal(50); // Passes
}
```

## to_be_less_than

Checks if a number is less than another number.

```rust
fn test_less_than() {
    let value = 42;
    
    expect!(value).to_be_less_than(50);       // Passes
    expect!(value).not().to_be_less_than(30); // Passes
}
```

## to_be_less_than_or_equal

Checks if a number is less than or equal to another number.

```rust
fn test_less_than_or_equal() {
    let value = 42;
    
    expect!(value).to_be_less_than_or_equal(42);       // Passes
    expect!(value).to_be_less_than_or_equal(50);       // Passes
    expect!(value).not().to_be_less_than_or_equal(30); // Passes
}
```

## to_be_even

Checks if a number is even.

```rust
fn test_even() {
    let even = 42;
    let odd = 43;
    
    expect!(even).to_be_even();       // Passes
    expect!(odd).not().to_be_even();  // Passes
}
```

## to_be_odd

Checks if a number is odd.

```rust
fn test_odd() {
    let even = 42;
    let odd = 43;
    
    expect!(odd).to_be_odd();        // Passes
    expect!(even).not().to_be_odd(); // Passes
}
```

## to_be_zero

Checks if a number is zero.

```rust
fn test_zero() {
    let zero = 0;
    let non_zero = 42;
    
    expect!(zero).to_be_zero();        // Passes
    expect!(non_zero).not().to_be_zero(); // Passes
}
```

## to_be_positive

Checks if a number is positive (greater than zero).

```rust
fn test_positive() {
    let positive = 42;
    let negative = -42;
    
    expect!(positive).to_be_positive();       // Passes
    expect!(negative).not().to_be_positive(); // Passes
}
```

## to_be_negative

Checks if a number is negative (less than zero).

```rust
fn test_negative() {
    let positive = 42;
    let negative = -42;
    
    expect!(negative).to_be_negative();       // Passes
    expect!(positive).not().to_be_negative(); // Passes
}
```

## to_be_in_range

Checks if a number is within a specified range.

```rust
fn test_in_range() {
    let value = 42;
    
    expect!(value).to_be_in_range(40..=45);  // Inclusive range 40 to 45
    expect!(value).not().to_be_in_range(50..60);  // Not in range 50 to 60
    
    // Different range types
    expect!(value).to_be_in_range(40..46);  // Half-open range
    expect!(value).to_be_in_range(30..);  // Range from 30 upwards
    expect!(value).to_be_in_range(..50);  // Range up to but not including 50
}
```