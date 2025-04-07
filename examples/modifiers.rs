use fluent_test::helpers::run_test;
use fluent_test::prelude::*;

fn main() {
    println!("\n=== Testing AND modifier (all true) ===");
    let number = 42;
    // Using AND modifier with both conditions true
    expect!(number).to_be_greater_than(30).and().to_be_less_than(50);

    println!("\n=== Testing complex AND chain (passing) ===");
    // Complex chain with multiple AND conditions
    expect!(number).to_be_even().and().to_be_positive();

    println!("\n=== Testing failing OR chain with passing part ===");
    // Using OR modifier (will pass if any condition is true)
    expect!(number).to_be_greater_than(50).or().to_be_less_than(30).or().to_equal(42);

    println!("\n=== Testing OR chain (all false) ===");
    // All conditions fail - should report failure and handle the panic
    let success = run_test(|| {
        expect!(number).to_be_greater_than(50).or().to_be_less_than(30).or().to_be_greater_than(100);
    });

    if !success {
        println!("Test failed as expected (all conditions were false)");
    }

    println!("\n=== Testing combined NOT with AND ===");
    // Combining AND and OR with NOT
    expect!(number).not().to_be_less_than(30).and().not().to_be_greater_than(50);

    println!("\n=== Testing string matchers with AND ===");
    // String example
    let name = "Arthur";
    expect!(name).to_contain("th").and().to_have_length(6).and().not().to_be_empty();

    // Report test results
    println!("\n=== Test Summary ===");
    fluent_test::Reporter::summarize();
}
