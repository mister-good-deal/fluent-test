# Output Formatting

FluentTest enhances the standard test output with colors, symbols, and improved formatting:

- **Color Coding**: Green for passing tests, red for failing tests
- **Unicode Symbols**: Check (✓) marks for passing conditions, cross (✗) for failing ones
- **Clean Variable Names**: Reference symbols (`&`) are automatically removed from output
- **Consistent Indentation**: Multi-line output is properly indented for readability
- **Logical Assertions**: AND/OR chains produce concise, readable output

## Customizing Output

You can customize the output for CI environments or other special cases:

```rust
// In your test module or test helper file
#[test]
fn setup() {
    // Configure the output formatting
    fluent_test::config()
        .enhanced_output(true)    // Enable enhanced output
        .use_colors(true)         // Use colored output
        .use_unicode_symbols(true) // Use ✓ and ✗ symbols
        .show_success_details(false) // Don't show details for passing tests
        .apply();
}
```

## Environment Variables

You can also control formatting through environment variables:

```bash
# Enable enhanced output mode
FLUENT_TEST_ENHANCED_OUTPUT=true cargo test

# Run specific test with enhanced output
FLUENT_TEST_ENHANCED_OUTPUT=true cargo test test_name
```

## Sample Output

When using enhanced output, a failing test produces something like:

```
✗ age is greater than 50 OR is less than 30 OR is greater than 100
  ✗ is greater than 50
  ✗ is less than 30
  ✗ is greater than 100
```

A passing AND chain looks like:

```
✓ number is greater than 30 AND is less than 50 AND is even
```

## Output in CI Environments

In CI environments, you might want to disable colors and Unicode symbols:

```rust
fluent_test::config()
    .enhanced_output(true)
    .use_colors(false)
    .use_unicode_symbols(false)
    .apply();
```

This will use plain ASCII characters instead of Unicode symbols and standard terminal colors, making the output more compatible with various CI systems.