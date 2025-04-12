//! Backend module for test evaluation and result generation

pub mod assertions;
pub mod matchers;
pub mod modifiers;

pub use assertions::sentence::AssertionSentence;
pub use assertions::{Assertion, AssertionStep, LogicalOp, TestSessionResult};
