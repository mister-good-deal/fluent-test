use std::fmt::{self, Display, Formatter};

/// Represents a complete sentence structure for an assertion
#[derive(Debug, Clone)]
pub struct AssertionSentence {
    /// The subject of the assertion (usually the variable name)
    pub subject: String,
    /// The verb of the assertion (e.g., "be", "have", "contain")
    pub verb: String,
    /// The object of the assertion (e.g., "greater than 42", "of length 5", "'test'")
    pub object: String,
    /// Optional qualifiers for the assertion (e.g., "within tolerance", "when rounded")
    pub qualifiers: Vec<String>,
    /// Whether the assertion is negated (e.g., "not be", "does not have")
    pub negated: bool,
}

impl AssertionSentence {
    /// Create a new assertion sentence
    pub fn new(verb: impl Into<String>, object: impl Into<String>) -> Self {
        return Self { subject: "".to_string(), verb: verb.into(), object: object.into(), qualifiers: Vec::new(), negated: false };
    }

    /// Set whether the assertion is negated
    pub fn with_negation(mut self, negated: bool) -> Self {
        self.negated = negated;
        return self;
    }

    /// Add a qualifier to the assertion
    pub fn with_qualifier(mut self, qualifier: impl Into<String>) -> Self {
        self.qualifiers.push(qualifier.into());
        return self;
    }

    /// Format the sentence into a readable string
    pub fn format(&self) -> String {
        let mut result = if self.negated { format!("not {} {}", self.verb, self.object) } else { format!("{} {}", self.verb, self.object) };

        if !self.qualifiers.is_empty() {
            result.push(' ');
            result.push_str(&self.qualifiers.join(" "));
        }

        return result;
    }
}

impl Display for AssertionSentence {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.format());
    }
}
