//! FluentTest: A fluent, Jest-like testing library for Rust
//!
//! This crate provides a more expressive way to write tests in Rust,
//! inspired by JavaScript testing frameworks like Jest.

// Allow explicit return statements as part of the coding style
#![allow(clippy::needless_return)]

pub mod backend;
mod config;
pub mod frontend;
mod reporter;

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
}

/// Main prelude module containing everything needed for fluent testing
pub mod prelude {
    pub use crate::backend::Expectation;
    pub use crate::expect;
    pub use crate::expect_not;

    // Import all matcher traits
    pub use crate::matchers::*;

    // Import modifiers
    pub use crate::backend::modifiers::*;
}

// Re-exports
pub use crate::config::Config;
pub use crate::reporter::Reporter;

/// Creates a new test configuration
pub fn config() -> Config {
    Config::new()
}

/// Main entry point for fluent assertions
#[macro_export]
macro_rules! expect {
    ($expr:expr) => {
        $crate::backend::Expectation::new($expr, stringify!($expr))
    };
}

/// Shorthand for creating a negated expectation
/// This provides a more natural way to write assertions with not
#[macro_export]
macro_rules! expect_not {
    ($expr:expr) => {{
        use $crate::backend::modifiers::NotModifier;
        $crate::backend::Expectation::new($expr, stringify!($expr)).not()
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
            // Setup code here

            // Use a drop guard to ensure reporter runs at the end
            struct ReportOnDrop;
            impl Drop for ReportOnDrop {
                fn drop(&mut self) {
                    $crate::Reporter::summarize();
                }
            }

            let _guard = ReportOnDrop;

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
}
