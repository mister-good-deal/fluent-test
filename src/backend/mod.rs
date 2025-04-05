//! Backend module for test evaluation and result generation

pub mod assertions;
pub mod matchers;
pub mod modifiers;
mod test_result;

pub use assertions::sentence::AssertionSentence;
pub use assertions::{AssertionChain, AssertionStep, Expectation, LogicalOp};
pub use test_result::{AssertionResult, AssertionStepResult, TestSessionResult};
