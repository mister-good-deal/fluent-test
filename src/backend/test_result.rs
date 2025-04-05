use crate::backend::assertions::LogicalOp;
use crate::backend::assertions::sentence::AssertionSentence;

/// Represents a step in an assertion evaluation
#[derive(Debug, Clone)]
pub struct AssertionStepResult {
    /// The assertion sentence components
    pub sentence: AssertionSentence,
    /// Whether this step passed
    pub passed: bool,
    /// The logical operation connecting this step to the next one
    pub logical_op: Option<LogicalOp>,
}

/// Represents the complete result of an assertion chain
#[derive(Debug, Clone)]
pub struct AssertionResult {
    /// The expression being evaluated
    pub expr_str: String,
    /// All steps in the assertion evaluation
    pub steps: Vec<AssertionStepResult>,
    /// Whether the overall assertion passed or failed
    pub passed: bool,
}

/// Represents the complete result of a test session
#[derive(Debug, Default)]
pub struct TestSessionResult {
    /// Number of passed tests
    pub passed_count: usize,
    /// Number of failed tests
    pub failed_count: usize,
    /// Detailed results of failed tests
    pub failures: Vec<AssertionResult>,
}
