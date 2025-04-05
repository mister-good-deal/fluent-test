use fluent_test::prelude::*;
mod helpers;

fn main() {
    println!("Running not modifier examples");

    // --- Two ways to use negated expectations ---

    // 1. Using the .not() method (fluent API)
    expect!(5).not().to_equal(10);

    // 2. Using the expect_not! macro
    expect_not!(5).to_equal(10);

    // --- Testing different matchers with not() ---

    // String matchers
    let name = "John Doe";
    expect!(name).not().to_be_empty();
    expect!(name).not().to_have_length(5);
    expect!(name).not().to_contain("Smith");
    expect!(name).not().to_start_with("Bob");
    expect!(name).not().to_end_with("Smith");
    expect!(name).not().to_match("^Smith");

    // Numeric matchers
    let age = 30;
    expect!(age).not().to_be_greater_than(50); // true (30 is not > 50)
    expect!(age).not().to_be_less_than(20); // true (30 is not < 20)
    expect!(age).not().to_be_odd(); // true (30 is even, not odd)
    expect!(age).not().to_be_negative(); // true (30 is positive)
    expect!(age).not().to_be_in_range(40..50); // true (30 is not in 40..50)

    // Demonstrating both approaches
    let value = 42;
    expect!(value).not().to_equal(100); // Using .not() method
    expect_not!(value).to_equal(100); // Using expect_not! macro

    println!("All examples passed!");
}
