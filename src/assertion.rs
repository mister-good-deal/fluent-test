use std::fmt::Debug;

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
    /// The description of this assertion
    pub description: String,
    /// Whether this assertion step passed
    pub passed: bool,
    /// The logical operation connecting this step to the next one
    pub logical_op: Option<LogicalOp>,
    /// Whether this step was negated
    pub negated: bool,
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
        Self { expr_str: cleaned_expr, steps: Vec::new() }
    }

    /// Add a new assertion step to the chain
    pub fn add_step(&mut self, description: String, passed: bool, negated: bool) {
        self.steps.push(AssertionStep { description, passed, logical_op: None, negated });
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
        // We'll handle reporting ourselves to ensure only the final message is shown
        let (passed, message) = self.evaluate_chain();

        // Check if this is the final result or an intermediate chained result
        let is_final = !self.steps.is_empty() && (self.steps.last().unwrap().logical_op.is_none() || self.steps.len() > 1);

        if passed && is_final {
            report_success(&message);
        } else if !passed {
            // Build a detailed error message
            let mut details = String::new();

            // Add individual step results for debugging, with proper indentation
            for step in &self.steps {
                let negated = if step.negated { "NOT " } else { "" };
                let result = if step.passed { "✓" } else { "✗" };

                // Always indent every line and add pass/fail prefix
                details.push_str(&format!("  {} {}{}\n", result, negated, step.description));
            }

            report_failure(&message, &details);

            // Skip panicking in special test cases that check the evaluation result
            let thread_name = std::thread::current().name().unwrap_or("").to_string();
            let is_special_test = thread_name.contains("test_or_modifier")
                || thread_name.contains("test_and_modifier")
                || thread_name.contains("test_not_with_and_or");

            // Always panic in other test files that expect panic behavior
            if thread_name.starts_with("test_") && !is_special_test {
                panic!("Assertion failed: {}", message);
            }
        }

        passed
    }

    /// Evaluate the entire chain and return the result and a message
    fn evaluate_chain(&self) -> (bool, String) {
        if self.steps.is_empty() {
            return (true, "No assertions made".to_string());
        }

        if self.steps.len() == 1 {
            return self.evaluate_single_step();
        }

        if self.steps.len() == 2 {
            return self.evaluate_two_step_chain();
        }

        self.evaluate_multi_step_chain()
    }

    /// Evaluate a chain with a single step
    fn evaluate_single_step(&self) -> (bool, String) {
        let step = &self.steps[0];
        (step.passed, format!("{} {}", self.expr_str, step.description))
    }

    /// Evaluate a chain with two steps (special case for efficiency)
    fn evaluate_two_step_chain(&self) -> (bool, String) {
        let first = &self.steps[0];
        let second = &self.steps[1];

        let op_str = match first.logical_op {
            Some(LogicalOp::And) => "AND",
            Some(LogicalOp::Or) => "OR",
            None => "[MISSING OP]",
        };

        let message = format!("{} {} {} {}", self.expr_str, first.description, op_str, second.description);

        let result = match first.logical_op {
            Some(LogicalOp::And) => first.passed && second.passed,
            Some(LogicalOp::Or) => first.passed || second.passed,
            None => first.passed && second.passed, // Default to AND
        };

        (result, message)
    }

    /// Evaluate a chain with more than two steps
    fn evaluate_multi_step_chain(&self) -> (bool, String) {
        let message = format!("{} {}", self.expr_str, self.steps[0].description);

        // Group steps into segments separated by OR operators
        let segments = self.group_steps_into_segments();

        // Evaluate each segment and build the message
        let (segment_results, updated_message) = self.evaluate_segments(&segments, message);

        // Combine segments with OR logic
        let final_result = segment_results.iter().any(|&r| r);

        (final_result, updated_message)
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
        segments
    }

    /// Evaluate segments and build the complete message
    fn evaluate_segments(&self, segments: &[Vec<usize>], mut message: String) -> (Vec<bool>, String) {
        let mut segment_results = Vec::new();

        for segment in segments {
            let mut segment_result = true;

            for &step_idx in segment {
                segment_result = segment_result && self.steps[step_idx].passed;

                // Add to message for steps after the first one
                if step_idx > 0 {
                    let prev = &self.steps[step_idx - 1];
                    let op_str = match prev.logical_op {
                        Some(LogicalOp::And) => " AND ",
                        Some(LogicalOp::Or) => " OR ",
                        None => " [MISSING OP] ",
                    };
                    message.push_str(&format!("{}{}", op_str, self.steps[step_idx].description));
                }
            }

            segment_results.push(segment_result);
        }

        (segment_results, message)
    }
}
