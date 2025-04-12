use fluent_test::prelude::*;

fn main() {
    println!("FluentTest Enhanced Output Example\n");

    // Enable enhanced output for this example
    config().enhanced_output(true).apply();

    // Use various expectations to show the enhanced formatting
    println!("Running assertions with enhanced output enabled:");

    // Number assertions
    let number = 42;
    print_result("expect!(number).to_be_even()", || {
        // Call evaluate explicitly to get the boolean result
        expect!(number).to_be_even().evaluate()
    });

    print_result("expect!(number).to_be_greater_than(30).and().to_be_less_than(50)", || {
        expect!(number).to_be_greater_than(30).and().to_be_less_than(50).evaluate()
    });

    // Failing assertion - use catch_unwind for this since it will panic
    let result = std::panic::catch_unwind(|| {
        expect!(number).to_be_greater_than(100);
    });
    match result {
        Ok(_) => println!("\n🔍 expect!(number).to_be_greater_than(100)\n  ❌ Failed but did not panic"),
        Err(e) => {
            if let Some(s) = e.downcast_ref::<String>() {
                println!("\n🔍 expect!(number).to_be_greater_than(100)\n  ❌ Failed with: {}", s);
            } else {
                println!("\n🔍 expect!(number).to_be_greater_than(100)\n  ❌ Failed with panic");
            }
        }
    }

    // String assertions
    let text = "Hello, world!";
    print_result("expect!(text).to_contain(\"world\")", || expect!(text).to_contain("world").evaluate());

    // Collection assertions
    let items = vec![1, 2, 3, 4, 5];
    print_result("expect!(items.as_slice()).to_have_length(5)", || expect!(items.as_slice()).to_have_length(5).evaluate());

    println!("\nNOTE: Without enhanced output enabled (the default), these assertions would");
    println!("      produce standard Rust assertion messages instead of the detailed ones shown above.");
    println!("\nUse config().enhanced_output(true).apply() to enable enhanced output in your code,");
    println!("or set the FLUENT_TEST_ENHANCED_OUTPUT=true environment variable.");
}

// Helper function to run an assertion and print the result
fn print_result<F>(desc: &str, f: F)
where
    F: FnOnce() -> bool,
{
    println!("\n🔍 {}", desc);
    match f() {
        true => println!("  ✅ Passed"),
        false => println!("  ❌ Failed (but did not panic)"),
    }
}
