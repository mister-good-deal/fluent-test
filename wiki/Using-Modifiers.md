# Using Modifiers

FluentTest provides several modifiers that allow you to create more complex assertions.

## Not Modifier

FluentTest supports two ways to negate expectations:

```rust
#[test]
fn test_not_modifiers() {
    let value = 42;
    let name = "Arthur";
    
    // Two ways to use negated expectations
    
    // 1. Using the .not() method (fluent API)
    expect!(value).not().to_equal(100);
    expect!(name).not().to_contain("Bob");
    
    // 2. Using the expect_not! macro
    expect_not!(value).to_equal(100);
}
```

## Logical Chain Modifiers (AND/OR)

FluentTest allows chaining multiple assertions with logical operators:

```rust
#[test]
fn test_chain_modifiers() {
    let number = 42;
    
    // Using AND to require all conditions to pass
    expect!(number).to_be_greater_than(30)
                 .and().to_be_less_than(50)
                 .and().to_be_even();
    
    // Using OR where at least one condition must pass
    expect!(number).to_be_greater_than(100)  // This fails
                 .or().to_be_less_than(30)   // This fails
                 .or().to_equal(42);         // This passes, so overall expression passes
    
    // Combining NOT with logical modifiers
    expect!(number).not().to_be_less_than(30)
                 .and().not().to_be_greater_than(50);
}
```

The output from logical chains is clean and concise, showing just the final result with properly indented details for failing chains.
