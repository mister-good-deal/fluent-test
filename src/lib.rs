//! FluentTest: A fluent, Jest-like testing library for Rust
//!
//! This crate provides a more expressive way to write tests in Rust,
//! inspired by JavaScript testing frameworks like Jest.
//!
//! By default, assertions behave like standard Rust assertions.
//! Enable enhanced output with:
//!
//! ```
//! // In your test code:
//! use fluent_test::prelude::*;
//!
//! fn my_test() {
//!     // Enable enhanced output for this test
//!     fluent_test::config().enhanced_output(true).apply();
//!     
//!     expect!(2 + 2).to_equal(4);
//! }
//! ```
//!
//! Or set the FLUENT_TEST_ENHANCED_OUTPUT=true environment variable.

// Allow explicit return statements as part of the coding style
#![allow(clippy::needless_return)]

// Initialization constants and utilities

// Import Once for initialization
use std::sync::Once;

// Initialization for tests
static TEST_INIT: Once = Once::new();

pub mod backend;
pub mod config;
pub mod events;
pub mod frontend;
mod reporter;

// Auto-initialize for tests if enhanced output is enabled
pub fn auto_initialize_for_tests() {
    TEST_INIT.call_once(|| {
        // Check environment variable to enable enhanced output
        let config = config::Config::new();
        if config.enhanced_output {
            // Apply the config which will initialize the event system
            config.apply();
        }
    });
}

// Re-export the initialize function
pub use config::initialize;

// Export attribute macros for fixtures
pub use fluent_test_macros::{after_all, before_all, setup, tear_down, with_fixtures, with_fixtures_module};

// Global exit handler for after_all fixtures
#[ctor::dtor]
fn run_after_all_fixtures() {
    backend::fixtures::run_after_all_fixtures();
}

/// Matcher traits module for bringing the traits into scope
pub mod matchers {
    pub use crate::backend::matchers::boolean::BooleanMatchers;
    pub use crate::backend::matchers::collection::{CollectionExtensions, CollectionMatchers};
    pub use crate::backend::matchers::equality::EqualityMatchers;
    pub use crate::backend::matchers::hashmap::HashMapMatchers;
    pub use crate::backend::matchers::numeric::NumericMatchers;
    pub use crate::backend::matchers::option::OptionMatchers;
    pub use crate::backend::matchers::result::ResultMatchers;
    pub use crate::backend::matchers::string::StringMatchers;

    // Async matchers - only available when async feature is enabled
    #[cfg(feature = "async")]
    pub use crate::backend::matchers::async_matchers::TimeConstraint;
    #[cfg(feature = "async")]
    pub use crate::backend::matchers::async_matchers::{AsyncFailAssertion, AsyncMatchers, AsyncResolveAssertion, AsyncResultMatchers};
}

/// Main prelude module containing everything needed for fluent testing
pub mod prelude {
    pub use crate::backend::Assertion;
    pub use crate::expect;
    pub use crate::expect_not;

    // Fixture attribute macros
    pub use crate::{after_all, before_all, setup, tear_down, with_fixtures, with_fixtures_module};

    // Import all matcher traits
    pub use crate::matchers::*;

    // Import modifiers
    pub use crate::backend::modifiers::*;

    // Import configuration and initialization
    pub use crate::config;
    pub use crate::initialize;

    #[cfg(test)]
    pub use crate::auto_initialize_for_tests;
}

// Re-exports
pub use crate::config::Config;
pub use crate::reporter::Reporter;

/// Creates a new test configuration
pub fn config() -> Config {
    Config::new()
}

/// Main entry point for fluent assertions
///
/// This macro creates an assertion for synchronous values.
/// For asynchronous values, use the `expect!(..).await` pattern.
///
/// # Usage
/// ```
/// use fluent_test::prelude::*;
///
/// // Regular synchronous assertion
/// expect!(2 + 2).to_equal(4);
///
/// // Asynchronous assertion (in an async context)
/// # async fn example() {
/// async fn delayed_value() -> i32 {
///     // Some async operation
///     42
/// }
///
/// // For async values, the future is awaited inside the macro
/// let result = delayed_value();
/// expect!(result).to_resolve_with(42);
/// ```
#[macro_export]
macro_rules! expect {
    ($expr:expr) => {{
        // When the async feature is enabled, check if the type is a Future
        #[cfg(feature = "async")]
        async move {
            // Always auto-initialize
            $crate::auto_initialize_for_tests();

            // Get a reference to avoid moving the value
            let expr_ref = &$expr;

            // Track the start time and create an async block
            use tokio::time::Instant;
            let start = Instant::now();

            // Await the future and measure elapsed time, even non-async code implements `into_future` trait
            let resolved = $expr.into_future().await;
            let elapsed = start.elapsed();

            // Create an assertion with the elapsed time
            $crate::backend::Assertion::with_elapsed(resolved, stringify!($expr), elapsed);
        };
        #[cfg(not(feature = "async"))]
        {
            // Always auto-initialize
            $crate::auto_initialize_for_tests();
            // If async feature is not enabled, just create a normal assertion
            $crate::backend::Assertion::new($expr, stringify!($expr));
        };
    }};
}

/// Shorthand for creating a negated expectation
/// This provides a more natural way to write assertions with not
#[macro_export]
macro_rules! expect_not {
    ($expr:expr) => {{
        // Always auto-initialize
        $crate::auto_initialize_for_tests();

        use $crate::backend::modifiers::NotModifier;
        $crate::backend::Assertion::new($expr, stringify!($expr)).not()
    }};
}

/// Run all FluentTest tests in a module
///
/// This can be used as a test harness to handle initialization
/// and reporting.
#[macro_export]
macro_rules! fluent_test {
    () => {
        #[test]
        fn _fluent_test_runner() {
            // Auto-initialize if enhanced output is enabled
            $crate::auto_initialize_for_tests();

            // Check if enhanced output is enabled
            let enhanced_output = $crate::config::is_enhanced_output_enabled();

            // Setup reporting guard only if enhanced output is enabled
            if enhanced_output {
                // Use a drop guard to ensure reporter runs at the end
                struct ReportOnDrop;
                impl Drop for ReportOnDrop {
                    fn drop(&mut self) {
                        $crate::Reporter::summarize();
                    }
                }

                let _guard = ReportOnDrop;
            }

            // The test itself runs normally
        }
    };
}

// Special module for test support
#[cfg(test)]
pub mod test_utils {
    // Just re-export all the traits for easy importing in tests
    pub use crate::backend::matchers::boolean::BooleanMatchers;
    pub use crate::backend::matchers::collection::{CollectionExtensions, CollectionMatchers};
    pub use crate::backend::matchers::equality::EqualityMatchers;
    pub use crate::backend::matchers::hashmap::HashMapMatchers;
    pub use crate::backend::matchers::numeric::NumericMatchers;
    pub use crate::backend::matchers::option::OptionMatchers;
    pub use crate::backend::matchers::result::ResultMatchers;
    pub use crate::backend::matchers::string::StringMatchers;

    #[cfg(feature = "async")]
    pub use crate::backend::matchers::async_matchers::{AsyncMatchers, AsyncResultMatchers};

    // Helper function to set up testing
    pub fn setup_tests() {
        // Force enhanced output for internal tests
        crate::config().enhanced_output(true).apply();
    }

    // When tests run, make sure we have enhanced output enabled
    // by default for internal library tests
    #[test]
    #[ignore]
    fn _setup_for_tests() {
        setup_tests();
    }
}
