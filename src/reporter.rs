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
        Self {
            passed: 0,
            failed: 0,
            failures: Vec::new(),
        }
    }
}

pub(crate) static GLOBAL_CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| RwLock::new(Config::new()));

thread_local! {
    static REPORT: RefCell<TestReport> = RefCell::new(TestReport::new());
}

pub fn report_success(message: &str) {
    REPORT.with(|r| {
        let mut report = r.borrow_mut();
        report.passed += 1;
    });

    let config = GLOBAL_CONFIG.read().unwrap();
    if config.show_success_details {
        let prefix = if config.use_unicode_symbols {
            "✓ "
        } else {
            "+ "
        };
        let message = if config.use_colors {
            format!("{}{}", prefix.green(), message.green())
        } else {
            format!("{}{}", prefix, message)
        };
        println!("{}", message);
    }
}

pub fn report_failure(expected: &str, received: &str) {
    REPORT.with(|r| {
        let mut report = r.borrow_mut();
        report.failed += 1;
        report
            .failures
            .push((expected.to_string(), received.to_string()));
    });

    let config = GLOBAL_CONFIG.read().unwrap();
    let prefix = if config.use_unicode_symbols {
        "✗ "
    } else {
        "- "
    };

    let expected_msg = if config.use_colors {
        expected.red().bold()
    } else {
        expected.normal()
    };

    let received_msg = if config.use_colors {
        received.red()
    } else {
        received.normal()
    };

    println!("{}{}", prefix, expected_msg);
    println!("  {}", received_msg);
}

pub struct Reporter;

impl Reporter {
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
                    if report.passed > 0 {
                        passed_msg.green()
                    } else {
                        passed_msg.normal()
                    },
                    if report.failed > 0 {
                        failed_msg.red().bold()
                    } else {
                        failed_msg.normal()
                    }
                );
            } else {
                println!("  {} / {}", passed_msg, failed_msg);
            }

            if report.failed > 0 {
                println!("\nFailure Details:");
                for (i, (expected, received)) in report.failures.iter().enumerate() {
                    println!("  {}. {}", i + 1, expected);
                    println!("     {}", received);
                }
            }
        });
    }
}
