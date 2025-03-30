//! FluentTest: A fluent, Jest-like testing library for Rust
//!
//! This crate provides a more expressive way to write tests in Rust,
//! inspired by JavaScript testing frameworks like Jest.

mod config;
pub mod expectation;
mod matchers;
mod reporter;

pub mod prelude {
    pub use crate::expect;
    pub use crate::expectation::Expectation;
    pub use crate::matchers::*;
    pub use crate::not;
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
        $crate::expectation::Expectation::new($expr, stringify!($expr))
    };
}

/// Shorthand for creating a negated expectation
/// This provides a more natural way to write assertions with not
#[macro_export]
macro_rules! not {
    ($expr:expr) => {
        $crate::expectation::Expectation::new($expr, stringify!($expr)).not_method()
    };
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
