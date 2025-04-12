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

    /// Format the sentence into a readable string (raw format, without subject)
    pub fn format(&self) -> String {
        let mut result = if self.negated { format!("not {} {}", self.verb, self.object) } else { format!("{} {}", self.verb, self.object) };

        if !self.qualifiers.is_empty() {
            result.push(' ');
            result.push_str(&self.qualifiers.join(" "));
        }

        return result;
    }

    /// Format the sentence with the correct verb conjugation based on the subject
    pub fn format_with_conjugation(&self, subject: &str) -> String {
        // Determine if the subject is plural
        let is_plural = Self::is_plural_subject(subject);

        // Convert the infinitive verb to the correct form based on plurality
        let conjugated_verb = self.conjugate_verb(is_plural);

        let mut result =
            if self.negated { format!("not {} {}", conjugated_verb, self.object) } else { format!("{} {}", conjugated_verb, self.object) };

        if !self.qualifiers.is_empty() {
            result.push(' ');
            result.push_str(&self.qualifiers.join(" "));
        }

        return result;
    }

    /// Determine if a subject name is likely plural
    fn is_plural_subject(subject: &str) -> bool {
        // Extract the base variable name from expressions like "var.method()" or "&var"
        let base_name = Self::extract_base_name(subject);

        // Common plural endings in English
        let plural_endings = ["s", "es", "ies"];

        // Check for common plural variable naming patterns
        let is_plural_ending = plural_endings.iter().any(|ending| base_name.ends_with(ending));

        // Also check for commonly used pluralized variable names in programming
        let common_plurals = [
            "items",
            "elements",
            "values",
            "arrays",
            "lists",
            "maps",
            "sets",
            "objects",
            "attributes",
            "properties",
            "entries",
            "keys",
            "numbers",
            "strings",
            "data",
            "results",
            "options",
            "errors",
            "children",
        ];

        let is_common_plural = common_plurals.contains(&base_name.to_lowercase().as_str());

        // Return true if either condition is met
        return is_plural_ending || is_common_plural;
    }

    /// Extract the base variable name from expressions
    fn extract_base_name(expr: &str) -> String {
        // Remove reference symbols
        let without_ref = expr.trim_start_matches('&');

        // Handle method calls like "var.method()" - extract "var"
        if let Some(dot_pos) = without_ref.find('.') {
            return without_ref[0..dot_pos].to_string();
        }

        // Handle array/slice indexing like "var[0]" - extract "var"
        if let Some(bracket_pos) = without_ref.find('[') {
            return without_ref[0..bracket_pos].to_string();
        }

        // No special case, return as is
        return without_ref.to_string();
    }

    /// Conjugate the verb based on plurality
    fn conjugate_verb(&self, is_plural: bool) -> String {
        // Special case handling for common verbs
        match self.verb.as_str() {
            "be" => {
                if is_plural {
                    "are".to_string()
                } else {
                    "is".to_string()
                }
            }
            "have" => {
                if is_plural {
                    "have".to_string()
                } else {
                    "has".to_string()
                }
            }
            "contain" => {
                if is_plural {
                    "contain".to_string()
                } else {
                    "contains".to_string()
                }
            }
            // For other verbs, add 's' in singular form
            verb => {
                if is_plural {
                    verb.to_string()
                } else {
                    // Handle special cases for verbs ending in certain characters
                    if verb.ends_with('s') || verb.ends_with('x') || verb.ends_with('z') || verb.ends_with("sh") || verb.ends_with("ch") {
                        format!("{}es", verb)
                    } else if verb.ends_with('y')
                        && !verb.ends_with("ay")
                        && !verb.ends_with("ey")
                        && !verb.ends_with("oy")
                        && !verb.ends_with("uy")
                    {
                        format!("{}ies", &verb[0..verb.len() - 1])
                    } else {
                        format!("{}s", verb)
                    }
                }
            }
        }
    }
}

impl Display for AssertionSentence {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.format());
    }
}
