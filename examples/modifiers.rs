use fluent_test::prelude::*;

fn main() {
    // Using AND modifier to chain multiple assertions
    let number = 42;

    // Using AND modifier to check multiple conditions on the same value
    expect!(number).to_be_greater_than(30).and().to_be_less_than(50);

    expect!(number).to_be_even().and().to_be_divisible_by(7).and().to_be_positive();

    // Using OR modifier (will pass if any condition is true)
    expect!(number).to_be_greater_than(50).or().to_be_less_than(30).or().to_equal(42);

    // Combining AND and OR with NOT
    expect!(number).not().to_be_less_than(30).and().not().to_be_greater_than(50);

    // String example
    let name = "Arthur";

    expect!(name).to_contain("th").and().to_have_length(6).and().not().to_be_empty();

    // Report test results
    fluent_test::Reporter::summarize();
}
