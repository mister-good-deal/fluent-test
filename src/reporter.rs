use crate::backend::{Assertion, TestSessionResult};
use crate::config::Config;
use crate::events::{AssertionEvent, EventEmitter, on_failure, on_success};
use crate::frontend::ConsoleRenderer;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::RwLock;

pub(crate) static GLOBAL_CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| RwLock::new(Config::new()));

thread_local! {
    static TEST_SESSION: RefCell<TestSessionResult> = RefCell::new(TestSessionResult::default());
    // Track already reported messages to avoid duplicates
    static REPORTED_MESSAGES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
    // Flag to enable/disable deduplication
    static DEDUPLICATE_ENABLED: RefCell<bool> = const { RefCell::new(true) };
    // Flag to enable silent mode for intermediate steps in a chain
    static SILENT_MODE: RefCell<bool> = const { RefCell::new(false) };
}

pub struct Reporter;

impl Reporter {
    /// Initialize the reporter with event handlers
    pub fn init() {
        // Register success event handler
        on_success(|result| {
            Self::handle_success_event(result);
        });

        // Register failure event handler
        on_failure(|result| {
            Self::handle_failure_event(result);
        });
    }

    /// Handle success events
    fn handle_success_event(result: Assertion<()>) {
        TEST_SESSION.with(|session| {
            let mut session = session.borrow_mut();
            session.passed_count += 1;
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
                let key = format!("{:?}", result);
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
            let renderer = ConsoleRenderer::new(Config {
                use_colors: config.use_colors,
                use_unicode_symbols: config.use_unicode_symbols,
                show_success_details: config.show_success_details,
            });
            renderer.print_success(&result);
        }
    }

    /// Handle failure events
    fn handle_failure_event(result: Assertion<()>) {
        TEST_SESSION.with(|session| {
            let mut session = session.borrow_mut();
            session.failed_count += 1;
            session.failures.push(result.clone());
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
            let key = format!("{:?}", result);
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
            let renderer = ConsoleRenderer::new(Config {
                use_colors: config.use_colors,
                use_unicode_symbols: config.use_unicode_symbols,
                show_success_details: config.show_success_details,
            });
            renderer.print_failure(&result);
        }
    }

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
        TEST_SESSION.with(|session| {
            let session = session.borrow();
            let config = GLOBAL_CONFIG.read().unwrap();
            let renderer = ConsoleRenderer::new(Config {
                use_colors: config.use_colors,
                use_unicode_symbols: config.use_unicode_symbols,
                show_success_details: config.show_success_details,
            });
            renderer.print_session_summary(&session);
        });

        // Emit session completed event
        EventEmitter::emit(AssertionEvent::SessionCompleted);

        // Clear reported messages
        Self::reset_message_cache();

        // Reset deduplication to default (enabled)
        Self::enable_deduplication();
    }
}
