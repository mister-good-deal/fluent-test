# Architecture

FluentTest is built around a few core components:

1. The `expect!` macro which captures both values and their textual representation
2. The `expect_not!` macro which creates negated expectations
3. The `Assertion<T>` struct which holds the value and provides the fluent API
4. Trait implementations for different types of assertions
5. An event-based system that decouples assertion evaluation from reporting
6. A custom test reporter that enhances the standard output

## System Components

FluentTest uses a modular, event-driven architecture:

### 1. Backend Layer - Core assertion evaluation logic

- `Assertion<T>` - The main struct that holds values and builds assertions
- Matchers - Trait implementations for different types of assertions
- Modifiers - Support for logical operations (AND, OR, NOT)

### 2. Config System - Controls the library's behavior

- `Config` - Configuration options including enabling enhanced output
- Environment variables for controlling behavior without code changes
- Automatic initialization when enhanced output is enabled

### 3. Event System - Decouples assertion execution from reporting

- `AssertionEvent` - Events emitted when assertions succeed or fail
- `EventEmitter` - Responsible for delivering events to registered handlers
- Thread-local handlers for managing assertions across test contexts

### 4. Frontend Layer - Reporting and user interface

- `Reporter` - Listens to events and manages test sessions
- `ConsoleRenderer` - Formats and displays test results

## Benefits of the Architecture

This separation of concerns makes the library more maintainable and extensible, allowing for:

- Multiple reporter implementations (console, JSON, HTML, etc.)
- Clean extension points for custom matchers
- Proper isolation between test evaluation and reporting logic
- Flexibility in choosing between standard or enhanced output

## Example Flow

When you use `expect!(value).to_equal(5)`:

1. The `expect!` macro captures both the value and its text representation
2. An `Assertion<T>` object is created with this information
3. The `to_equal` method checks if the values match
4. An `AssertionStep` is created with the result and a sentence
5. If the assertion fails, an event is emitted to the `EventEmitter`
6. The `Reporter` receives the event and formats the output
7. The `ConsoleRenderer` displays the message using the configured format

This event-based architecture allows assertions to be evaluated and reported independently, making the system more flexible and extensible.