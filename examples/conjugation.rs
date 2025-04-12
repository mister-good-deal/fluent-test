use fluent_test::prelude::*;

fn main() {
    // Initialize the event system
    initialize_event_system();

    println!("Testing verb conjugation in assertions\n");

    println!("=== Testing singular vs plural variable names ===");

    // Singular variable names
    let number = 42;
    let value = 42;
    let item = "hello";
    let element = [1, 2, 3];

    expect!(number).to_be_even(); // Should be "number is even"
    expect!(value).to_be_positive(); // Should be "value is positive"
    expect!(item).to_have_length(5); // Should be "item has length 5"
    expect!(&element).to_have_length(3); // Should be "element has length 3"

    // Plural variable names
    let numbers = vec![1, 2, 3, 4, 5];
    let values = vec![10, 20, 30];
    let items = ["a", "b", "c"];
    let elements = [5, 6, 7];

    expect!(numbers.as_slice()).to_have_length(5); // Should be "numbers have length 5"
    expect!(values.as_slice()).to_have_length(3); // Should be "values have length 3"
    expect!(&items).to_contain("b"); // Should be "items contain 'b'"
    expect!(&elements).to_have_length(3); // Should be "elements have length 3"

    println!("\n=== Testing logical chains ===");

    // Singular with chains
    expect!(number).to_be_greater_than(30).and().to_be_less_than(50); // Should be "number is greater than 30 AND be less than 50"

    // Plural with chains
    expect!(numbers.as_slice()).to_have_length(5).and().to_contain(3); // Should be "numbers have length 5 AND contain 3"

    // Mixed cases
    let user = "John";
    let users = ["John", "Alice", "Bob"];

    expect!(user).to_equal("John"); // Should be "user is equal to 'John'"
    expect!(&users).to_contain("Alice"); // Should be "users contain 'Alice'"

    fluent_test::Reporter::summarize();
}
