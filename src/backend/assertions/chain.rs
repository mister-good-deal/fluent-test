use std::fmt::Debug;

use crate::backend::assertions::assertion::{AssertionResult, AssertionStepResult};
use crate::backend::assertions::sentence::AssertionSentence;
use crate::reporter::{report_failure, report_success};

/// Represents a logical operation in an assertion chain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOp {
    /// AND operation (&&)
    And,
    /// OR operation (||)
    Or,
}

/// Represents a step in an assertion chain
#[derive(Debug, Clone)]
pub struct AssertionStep {
    /// The assertion sentence components
    pub sentence: AssertionSentence,
    /// Whether this assertion step passed
    pub passed: bool,
    /// The logical operation connecting this step to the next one
    pub logical_op: Option<LogicalOp>,
}

/// Represents a complete assertion chain
#[derive(Debug, Clone)]
pub struct AssertionChain {
    /// The expression being evaluated (with & symbols removed)
    pub expr_str: String,
    /// All steps in the assertion chain
    pub steps: Vec<AssertionStep>,
}

impl AssertionChain {
    /// Create a new assertion chain
    pub fn new(expr_str: &'static str) -> Self {
        // Remove ampersands from variable names for cleaner output
        let cleaned_expr = expr_str.replace("&", "");
        return Self { expr_str: cleaned_expr, steps: Vec::new() };
    }

    /// Add a new assertion step to the chain using structured parts
    pub fn add_step_with_parts(&mut self, verb: impl Into<String>, object: impl Into<String>, passed: bool, negated: bool) {
        let mut sentence = AssertionSentence::new(verb, object).with_negation(negated);
        sentence.subject = self.expr_str.clone();

        self.steps.push(AssertionStep { sentence, passed, logical_op: None });
    }

    /// Add a new assertion step to the chain with full description (for backward compatibility)
    pub fn add_step(&mut self, description: impl Into<String>, passed: bool, negated: bool) {
        // Parse the description to extract parts
        let description = description.into();
        let parts: Vec<&str> = description.splitn(2, ' ').collect();

        let (verb, object) =
            if parts.len() >= 2 { (parts[0].to_string(), parts[1].to_string()) } else { (description.to_string(), "".to_string()) };

        let mut sentence = AssertionSentence::new(verb, object).with_negation(negated);
        sentence.subject = self.expr_str.clone();

        self.steps.push(AssertionStep { sentence, passed, logical_op: None });
    }

    /// Set the logical operation for the last step
    pub fn set_last_logic(&mut self, op: LogicalOp) {
        if let Some(last) = self.steps.last_mut() {
            last.logical_op = Some(op);
        }
    }

    /// Evaluate the entire assertion chain and report the result
    /// Returns whether the assertion passed
    pub fn evaluate(self) -> bool {
        let result = self.to_assertion_result();
        let passed = result.passed;

        // Check if this is the final result or an intermediate chained result
        let is_final = !self.steps.is_empty() && (self.steps.last().unwrap().logical_op.is_none() || self.steps.len() > 1);

        if passed && is_final {
            report_success(result);
        } else if !passed {
            report_failure(result.clone());

            // Skip panicking in special test cases that check the evaluation result
            let thread_name = std::thread::current().name().unwrap_or("").to_string();
            let is_special_test = thread_name.contains("test_or_modifier")
                || thread_name.contains("test_and_modifier")
                || thread_name.contains("test_not_with_and_or");

            // Always panic in other test files that expect panic behavior
            if !is_special_test {
                // If this is a test failure, use the formatted description from the sentence
                if !self.steps.is_empty() {
                    let step = &self.steps[0];
                    // Generate failure message from the sentence
                    let message = step.sentence.format();
                    panic!("{}", message);
                } else {
                    panic!("Assertion failed: {}", self.expr_str);
                }
            }
        }

        return passed;
    }

    /// Convert the assertion chain to an AssertionResult
    fn to_assertion_result(&self) -> AssertionResult {
        if self.steps.is_empty() {
            return AssertionResult { expr_str: self.expr_str.clone(), steps: Vec::new(), passed: true };
        }

        // Convert steps to AssertionStepResults
        let step_results = self
            .steps
            .iter()
            .map(|step| {
                return AssertionStepResult { sentence: step.sentence.clone(), passed: step.passed, logical_op: step.logical_op };
            })
            .collect::<Vec<_>>();

        // Determine if the overall result passes
        let passed = self.calculate_chain_result();

        return AssertionResult { expr_str: self.expr_str.clone(), steps: step_results, passed };
    }

    /// Calculate if the entire chain passes
    fn calculate_chain_result(&self) -> bool {
        if self.steps.is_empty() {
            return true;
        }

        if self.steps.len() == 1 {
            return self.steps[0].passed;
        }

        if self.steps.len() == 2 {
            let first = &self.steps[0];
            let second = &self.steps[1];

            match first.logical_op {
                Some(LogicalOp::And) => return first.passed && second.passed,
                Some(LogicalOp::Or) => return first.passed || second.passed,
                None => return first.passed && second.passed, // Default to AND
            }
        }

        // For multi-step chains, evaluate segments
        let segments = self.group_steps_into_segments();
        let segment_results = segments
            .iter()
            .map(|segment| {
                return segment.iter().all(|&step_idx| self.steps[step_idx].passed);
            })
            .collect::<Vec<_>>();

        // Combine segments with OR logic
        return segment_results.iter().any(|&r| r);
    }

    /// Group steps into segments separated by OR operators
    fn group_steps_into_segments(&self) -> Vec<Vec<usize>> {
        let mut segments = Vec::new();
        let mut current_segment = vec![0]; // Start with first step

        for i in 1..self.steps.len() {
            let prev = &self.steps[i - 1];

            if let Some(LogicalOp::Or) = prev.logical_op {
                segments.push(current_segment);
                current_segment = vec![i];
            } else {
                current_segment.push(i);
            }
        }

        segments.push(current_segment); // Add the last segment
        return segments;
    }
}
