//! Frontend module for rendering test results

mod console;

pub use crate::backend::{AssertionResult, AssertionStepResult, TestSessionResult};
pub use console::ConsoleRenderer;
