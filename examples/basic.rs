use fluent_test::prelude::*;

fn main() {
    // Just to demonstrate the API
    let x = 42;
    let name = "Arthur";

    // Integer tests - direct and by reference
    expect!(x).to_be_greater_than(30);
    expect!(x).to_be_less_than(50);
    expect!(x).to_be_even();

    // Create a reference to demonstrate the & removal in output
    let x_ref = &x;
    expect!(x_ref).to_equal(&42);

    // String tests - direct and by reference
    expect!(name).to_contain("th");
    expect!(name).to_have_length(6);

    // Create a reference to demonstrate the & removal in output
    let name_ref = &name;
    let arthur = "Arthur";
    expect!(name_ref).to_equal(&arthur);

    // Report test results
    fluent_test::Reporter::summarize();
}
