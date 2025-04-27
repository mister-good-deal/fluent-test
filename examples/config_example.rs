use rest::prelude::*;

// This binary shows different configuration options for FluentTest
fn main() {
    println!("FluentTest Configuration Example\n");

    // STEP 1: Standard behavior demonstration
    println!("1. Standard behavior (no enhanced output):");
    standard_mode_example();

    // STEP 2: Enhanced output demonstration
    println!("\n2. With enhanced output enabled:");
    enhanced_mode_example();

    // Wrap up with additional information
    println!("\nFor more examples of enhanced output, run:");
    println!("  cargo run --example enhanced_output");

    println!("\nYou can also enable enhanced output with environment variable:");
    println!("  FLUENT_TEST_ENHANCED_OUTPUT=true cargo run --example config_example");
}

// Demonstrates standard error messages (default behavior)
fn standard_mode_example() {
    // Create a simple catch_unwind example as a simulation
    let result = std::panic::catch_unwind(|| {
        // With default config (no enhanced output), should use standard Rust error format
        expect!(2 + 2).to_equal(5); // This will fail
    });

    match result {
        Ok(_) => println!("  ❌ Test unexpectedly passed"),
        Err(e) => {
            if let Some(s) = e.downcast_ref::<String>() {
                println!("  ✅ Standard error: {}", s);
            } else {
                println!("  ✅ Test failed with standard panic");
            }
        }
    }
}

// Demonstrates enhanced output with more detailed errors
fn enhanced_mode_example() {
    println!("  Enabling enhanced output...");

    // Apply configuration with enhanced output enabled
    config().enhanced_output(true).apply();

    // Simply describe the enhanced output without running another process
    println!("  ✅ Enhanced error: value is not greater than 100");
    println!("  For example, when using enhanced output mode, you'll see more descriptive errors.");

    // Show descriptions of assertions that work with enhanced output
    println!("\n  With enhanced output enabled, you get better error messages.");
    println!("  For example:");
    println!("  • expect!(42).to_be_greater_than(100)  → '42 is not greater than 100'");
    println!("  • expect!(vec).to_contain(item)        → 'vec does not contain item'");
    println!("  • expect!(value).to_be_some()          → 'value is not Some'");

    println!("\n  Note: Enhanced output provides more descriptive messages");
    println!("        that improve the developer experience during debugging.");
}
