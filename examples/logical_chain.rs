use fluent_test::prelude::*;
mod helpers;
use helpers::run_test;

fn main() {
    // Example of using the AND logical operator
    let value = 42;

    println!("\n----- Testing AND chains -----");
    println!("For value = {}", value);

    // A passing chain - all conditions are true
    println!("\nExpecting value to be greater than 30 AND less than 50 AND even:");
    expect!(value).to_be_greater_than(30).and().to_be_less_than(50).and().to_be_even();

    // A failing chain - one condition is false
    println!("\nExpecting value to be greater than 30 AND less than 40 AND odd (will fail):");
    let success = run_test(|| {
        expect!(value).to_be_greater_than(30).and().to_be_less_than(40).and().to_be_odd();
    });

    if !success {
        println!("Test failed as expected (AND assertion)");
    }

    // Example of using the OR logical operator
    println!("\n----- Testing OR chains -----");

    // A passing chain - at least one condition is true
    println!("\nExpecting value to be less than 30 OR greater than 40 (will pass):");
    let success = run_test(|| {
        expect!(value).to_be_less_than(30).or().to_be_greater_than(40);
    });

    if success {
        println!("Test passed as expected (OR assertion with one true condition)");
    }

    // A failing chain - all conditions are false
    println!("\nExpecting value to be less than 30 OR negative (will fail):");
    let success = run_test(|| {
        expect!(value).to_be_less_than(30).or().to_be_negative();
    });

    if !success {
        println!("Test failed as expected (OR assertion with all false conditions)");
    }

    // Mixed AND and OR chains
    println!("\n----- Testing mixed AND/OR chains -----");

    // A complex passing chain
    println!("\nComplex passing chain with mixed AND/OR:");
    let success = run_test(|| {
        expect!(value).to_be_greater_than(30).and().to_be_less_than(50).or().to_be_even();
    });

    if success {
        println!("Mixed chain test passed as expected");
    }

    // Using NOT with chaining
    println!("\n----- Testing with NOT modifier -----");

    println!("\nExpecting value to NOT be greater than 50 AND to NOT be less than 30:");
    let success = run_test(|| {
        expect!(value).not().to_be_greater_than(50).and().not().to_be_less_than(30);
    });

    if success {
        println!("Test with NOT modifiers passed as expected");
    }

    // Using the fluent API with different types
    println!("\n----- Testing with different types -----");

    let text = "Hello, world!";
    println!("\nString tests: text = \"{}\"", text);
    let success = run_test(|| {
        expect!(text).to_contain("Hello").and().to_end_with("!").and().not().to_be_empty();
    });

    if success {
        println!("String test passed as expected");
    }

    let numbers = vec![1, 2, 3, 4, 5];
    println!("\nVec tests: numbers = {:?}", numbers);
    let success = run_test(|| {
        expect!(&numbers).to_contain(3).and().to_have_length(5).and().not().to_be_empty();
    });

    if success {
        println!("Vector test passed as expected");
    }

    println!("\nAll examples completed!");
}
