use fluent_test::prelude::*;

fn main() {
    println!("Running not modifier examples");

    // --- Four different ways to use negated expectations ---

    // 1. Traditional method chaining style
    expect!(5).not_method().to_equal(10);

    // 2. Using the .not_prop() method (alternative approach)
    expect!("hello").not_prop().to_contain("world");

    // 3. Using the Not trait implementation (we provide better approaches)
    // Note: This won't actually work in most cases due to type issues
    // Leaving as an example of what's possible with traits
    let expectation = expect!(true);
    let _negated = !expectation; // This works but isn't practical for direct use

    // 4. Using the not! macro
    not!(5).to_equal(10);

    // 5. Jest-style method call (similar to Jest's property)
    expect!(5).not().to_equal(10);

    // --- Testing different matchers with the Jest-style ---

    // String matchers
    let name = "John Doe";
    expect!(name).not().to_be_empty();
    expect!(name).not().to_have_length(5);
    expect!(name).not().to_contain("Smith");
    expect!(name).not().to_start_with("Bob");
    expect!(name).not().to_end_with("Smith");
    expect!(name).not().to_match_regex("^Smith");

    // Numeric matchers
    let age = 30;
    expect!(age).not().to_be_greater_than(50); // true (30 is not > 50)
    expect!(age).not().to_be_less_than(20); // true (30 is not < 20)
    expect!(age).not().to_be_odd(); // true (30 is even, not odd)
    expect!(age).not().to_be_negative(); // true (30 is positive)
    expect!(age).not().to_be_in_range(40..50); // true (30 is not in 40..50)

    // Demonstrating all approaches for comparison
    let value = 42;
    expect!(value).not_method().to_equal(100); // Traditional
    expect!(value).not_prop().to_equal(100); // Alternative method
    // Using Not trait is not practical directly
    not!(value).to_equal(100); // Using macro
    expect!(value).not().to_equal(100); // Jest-style method

    println!("All examples passed!");
}
