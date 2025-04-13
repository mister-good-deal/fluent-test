//! Backend module for test evaluation and result generation

pub mod assertions;
pub mod fixtures;
pub mod matchers;
pub mod modifiers;

pub use assertions::sentence::AssertionSentence;
pub use assertions::{Assertion, AssertionStep, LogicalOp, TestSessionResult};
pub use fixtures::{is_in_fixture_test, register_setup, register_teardown, run_test_with_fixtures};
