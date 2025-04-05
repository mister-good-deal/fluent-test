# FluentTest Refactoring Plan

## Completed Refactoring

1. **Moved Core Components to Backend Directory**
   - Organized the codebase with a clear separation between backend and frontend
   - Moved assertion logic, matchers, and modifiers to backend directory
   - Created a structured directory layout for better code organization

2. **Created Structured Assertion Sentences**
   - Replaced string-based descriptions with structured sentence components
   - Added AssertionSentence struct with subject, verb, object, and qualifiers
   - Implemented formatting methods for consistent output display

3. **Updated All Matchers to Use Structured Sentences**
   - Refactored all matchers to use the structured sentence approach:
     - Boolean matchers ✅
     - Equality matchers ✅
     - Collection matchers ✅
     - HashMap matchers ✅
     - Numeric matchers ✅
     - Option matchers ✅
     - Result matchers ✅
     - String matchers ✅
   - Changed from string concatenation to structured components
   - Each matcher now builds a proper AssertionSentence

## Current Challenges

1. **Trait Conflicts**
   - Method name conflicts between different matcher traits (e.g., multiple `to_equal` methods)
   - In the collection matcher, we renamed `to_equal` to `to_equal_collection` to avoid conflicts
   - Need a consistent way to handle this across all matcher traits

2. **Trait Import Issues in Tests**
   - Tests need to explicitly import the matcher traits to use the methods
   - Current prelude module brings in trait definitions but not implementations
   - Need to update test modules to use the right traits directly

3. **Example Code Updates**
   - Examples need to be updated to use the correct trait imports
   - Some methods have been renamed to avoid conflicts (e.g., `to_equal_collection`)

## Remaining Work

1. **Finalize Trait Conflicts Resolution**
   - Complete the renaming of ambiguous methods across all matchers
   - Update examples and tests to use the new method names
   - Document the naming convention for similar methods across different types

2. **Improve Modifier Integration**
   - Update the modifiers to work seamlessly with the new sentence structure
   - Handle sentence composition when chaining assertions with modifiers

3. **Expand Frontend Capabilities**
   - Add additional formatters for different output styles (JSON, HTML, etc.)
   - Implement customizable output templates

4. **Additional Improvements**
   - Add support for internationalization (i18n) of assertion messages
   - Consider adding support for custom assertion messages
   - Improve error reporting with source code context

## Implementation Guidelines

1. For method naming conflicts:
   - Use type-specific prefixes for methods (e.g., `to_equal_collection`, `to_equal_string`)
   - Document the naming convention clearly in comments
   - Provide backward compatibility where possible

2. For test maintenance:
   - Explicitly import the needed traits at the top of test modules
   - Use qualified names for ambiguous methods (e.g., `CollectionMatchers::to_equal_collection`)

3. For frontend enhancements:
   - Create new renderers that use the structured data
   - Allow for custom formatting of test results

This refactoring has significantly improved the maintainability and extensibility of the FluentTest library, making it easier to add new features and output formats in the future. The structured approach to building test reports allows for more flexible presentation and better integration with different output formats.