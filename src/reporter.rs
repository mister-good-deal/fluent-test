use crate::config::Config;
use colored::*;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::sync::RwLock;

struct TestReport {
    passed: usize,
    failed: usize,
    failures: Vec<(String, String)>,
}

impl TestReport {
    fn new() -> Self {
        Self { passed: 0, failed: 0, failures: Vec::new() }
    }
}

pub(crate) static GLOBAL_CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| RwLock::new(Config::new()));

thread_local! {
    static REPORT: RefCell<TestReport> = RefCell::new(TestReport::new());
}

use std::collections::HashSet;
thread_local! {
    // Track already reported messages to avoid duplicates
    static REPORTED_MESSAGES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
    // Flag to enable/disable deduplication
    static DEDUPLICATE_ENABLED: RefCell<bool> = RefCell::new(true);
    // Flag to enable silent mode for intermediate steps in a chain
    static SILENT_MODE: RefCell<bool> = RefCell::new(false);
}

pub fn report_success(message: &str) {
    REPORT.with(|r| {
        let mut report = r.borrow_mut();
        report.passed += 1;
    });

    // Check if silent mode is enabled
    let silent = SILENT_MODE.with(|silent| *silent.borrow());
    if silent {
        return;
    }

    // Check if we should deduplicate
    let should_report = DEDUPLICATE_ENABLED.with(|enabled| {
        if !*enabled.borrow() {
            // Deduplication disabled, always report
            return true;
        }

        // Only report each unique success message once
        REPORTED_MESSAGES.with(|msgs| {
            let mut messages = msgs.borrow_mut();
            if !messages.contains(message) {
                messages.insert(message.to_string());
                true
            } else {
                false
            }
        })
    });

    if should_report {
        let config = GLOBAL_CONFIG.read().unwrap();
        if config.show_success_details {
            let prefix = if config.use_unicode_symbols { "✓ " } else { "+ " };
            let formatted =
                if config.use_colors { format!("{}{}", prefix.green(), message.green()) } else { format!("{}{}", prefix, message) };
            println!("{}", formatted);
        }
    }
}

pub fn report_failure(expected: &str, received: &str) {
    REPORT.with(|r| {
        let mut report = r.borrow_mut();
        report.failed += 1;
        report.failures.push((expected.to_string(), received.to_string()));
    });

    // Check if silent mode is enabled
    let silent = SILENT_MODE.with(|silent| *silent.borrow());
    if silent {
        return;
    }

    // Check if we should deduplicate
    let should_report = DEDUPLICATE_ENABLED.with(|enabled| {
        if !*enabled.borrow() {
            // Deduplication disabled, always report
            return true;
        }

        // Only report each unique failure message once
        let key = format!("{}:{}", expected, received);
        REPORTED_MESSAGES.with(|msgs| {
            let mut messages = msgs.borrow_mut();
            if !messages.contains(&key) {
                messages.insert(key);
                true
            } else {
                false
            }
        })
    });

    if should_report {
        let config = GLOBAL_CONFIG.read().unwrap();
        let prefix = if config.use_unicode_symbols { "✗ " } else { "- " };

        let expected_msg = if config.use_colors { expected.red().bold() } else { expected.normal() };

        // Print the main error message
        println!("{}{}", prefix, expected_msg);

        // The details will have been properly formatted in the assertion.rs
        if config.use_colors {
            // Print the details with colors
            for line in received.lines() {
                if line.contains("✓") {
                    println!("{}", line.green());
                } else if line.contains("✗") {
                    println!("{}", line.red());
                } else {
                    println!("{}", line);
                }
            }
        } else {
            // Print without colors
            println!("{}", received);
        }
    }
}

pub struct Reporter;

impl Reporter {
    /// Clear the message cache to allow duplicated messages in different test scopes
    pub fn reset_message_cache() {
        REPORTED_MESSAGES.with(|msgs| {
            msgs.borrow_mut().clear();
        });
    }

    /// Enable deduplication of messages
    pub fn enable_deduplication() {
        DEDUPLICATE_ENABLED.with(|enabled| {
            *enabled.borrow_mut() = true;
        });
    }

    /// Disable deduplication of messages (for tests)
    pub fn disable_deduplication() {
        DEDUPLICATE_ENABLED.with(|enabled| {
            *enabled.borrow_mut() = false;
        });
    }

    /// Enable silent mode to suppress intermediate output in chains
    pub fn enable_silent_mode() {
        SILENT_MODE.with(|silent| {
            *silent.borrow_mut() = true;
        });
    }

    /// Disable silent mode to show all output
    pub fn disable_silent_mode() {
        SILENT_MODE.with(|silent| {
            *silent.borrow_mut() = false;
        });
    }

    pub fn summarize() {
        REPORT.with(|r| {
            let report = r.borrow();
            let config = GLOBAL_CONFIG.read().unwrap();

            println!("\nTest Results:");

            let passed_msg = format!("{} passed", report.passed);
            let failed_msg = format!("{} failed", report.failed);

            if config.use_colors {
                println!(
                    "  {} / {}",
                    if report.passed > 0 { passed_msg.green() } else { passed_msg.normal() },
                    if report.failed > 0 { failed_msg.red().bold() } else { failed_msg.normal() }
                );
            } else {
                println!("  {} / {}", passed_msg, failed_msg);
            }

            if report.failed > 0 {
                println!("\nFailure Details:");
                for (i, (expected, received)) in report.failures.iter().enumerate() {
                    println!("  {}. {}", i + 1, expected);

                    // Process each line of the details and ensure proper indentation
                    for line in received.lines() {
                        println!("     {}", line);
                    }
                }
            }
        });

        // Clear reported messages
        Self::reset_message_cache();

        // Reset deduplication to default (enabled)
        Self::enable_deduplication();
    }
}
