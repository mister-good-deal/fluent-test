# FluentTest Development Guide

## Build Commands

- Build project: `cargo build`
- Run tests: `cargo test`
- Run single test: `cargo test test_name`
- Run examples:

  ```bash
  cargo run --example basic && \
  cargo run --example combined_matchers.rs && \
  cargo run --example logical_chain.rs && \
  cargo run --example modifiers.rs && \
  cargo run --example new_matchers.rs && \
  cargo run --example not_modifier.rs 
  ```

- Check formatting: `cargo fmt --check`
- Apply formatting: `cargo fmt`
- Run linter: `cargo clippy -- -D warnings --fix`
- Check errors with `cargo clippy -- -D warnings` after editing code
- Format Markdown: Use markdownlint-cli2 to format Markdown files with `npx markdownlint-cli2 --fix README.md`

## Code Style Guidelines

- **Formatting**: 4-space indentation, trailing commas in multi-line declarations
- **Naming**: snake_case for variables/functions, CamelCase for types/traits, test functions prefixed with "test_"
- **Documentation**: Use //! for modules, /// for items, include examples
- **Organization**: Separate concerns in modules (matchers, expectation, reporter, config)
- **Error Handling**: Descriptive messages with context through Reporter module
- **API Design**: Trait-based, fluent interface with method chaining
- **Testing**: Unit tests alongside implementation, integration tests in tests/ directory
- **Markdown**: Use plain Markdown in README.md, no HTML tags, follow markdownlint.json formatting
- As a coding style use `return` statement in all source code, do not omit it

## Implementation Patterns

- Extend library by implementing new matcher traits
- Follow fluent API pattern for consistency
- Use thread-local storage for test reporting
- Public API exposed through prelude

## Releases and Publishing

- **Version Bump**: Update `version` in Cargo.toml when making significant changes
- **Automated Publishing**: Changes merged to master with a version bump will automatically:
  1. Compare version with latest git tag (not with published crates.io version)
  2. Create a git tag (vX.Y.Z) for the new version
  3. Publish to crates.io
  4. Generate a GitHub release using the CHANGELOG.md entry for the version
  5. If no CHANGELOG entry exists, fall back to auto-generated notes from commits
- **CI Checks**: Publishing only happens if all tests, examples, and linting checks pass
