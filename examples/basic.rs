use fluent_test::prelude::*;

fn main() {
    // Just to demonstrate the API
    let x = 42;
    let name = "Arthur";

    expect!(x).to_be_greater_than(30);
    expect!(x).to_be_less_than(50);
    expect!(x).to_be_even();

    expect!(name).to_contain("th");
    expect!(name).to_have_length(6);

    // Report test results
    fluent_test::Reporter::summarize();
}
