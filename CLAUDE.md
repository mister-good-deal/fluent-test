# FluentTest Development Guide

## Build Commands
- Build project: `cargo build`
- Run tests: `cargo test`
- Run single test: `cargo test test_name`
- Run examples: `cargo run --example basic`
- Check formatting: `cargo fmt --check`
- Apply formatting: `cargo fmt`
- Run linter: `cargo clippy`

## Code Style Guidelines
- **Formatting**: 4-space indentation, trailing commas in multi-line declarations
- **Naming**: snake_case for variables/functions, CamelCase for types/traits, test functions prefixed with "test_"
- **Documentation**: Use //! for modules, /// for items, include examples
- **Organization**: Separate concerns in modules (matchers, expectation, reporter, config)
- **Error Handling**: Descriptive messages with context through Reporter module
- **API Design**: Trait-based, fluent interface with method chaining
- **Testing**: Unit tests alongside implementation, integration tests in tests/ directory

## Implementation Patterns
- Extend library by implementing new matcher traits
- Follow fluent API pattern for consistency
- Use thread-local storage for test reporting
- Public API exposed through prelude