mod common;
use common::guard_test;
use fluent_test::prelude::*;
use std::collections::HashMap;

fn main() {
    // Enable enhanced output for this example
    config().enhanced_output(true).apply();

    println!("FluentTest Example: Combined Matchers with Logical Chaining");

    // Numeric matchers with AND and OR
    let number = 42;
    println!("\n--- Numeric assertions ---");

    let success = guard_test(|| {
        expect!(number).to_be_greater_than(30).and().to_be_less_than(50).and().to_be_even();
    });

    if success {
        println!("✅ Verified number is between 30 and 50 and is even.");
    }

    // String matchers with AND
    let greeting = "Hello, World!";
    println!("\n--- String assertions ---");

    let success = guard_test(|| {
        expect!(greeting).to_contain("Hello").and().to_start_with("Hello").and().to_have_length(13);
    });

    if success {
        println!("✅ Verified greeting contains 'Hello', starts with 'Hello', and has length 13.");
    }

    // Collection matchers with AND, OR, and NOT
    let numbers = vec![1, 2, 3, 4, 5];
    println!("\n--- Collection assertions ---");

    let success = guard_test(|| {
        expect!(&numbers).to_have_length(5).and().to_contain(3).and().not().to_be_empty();
    });

    if success {
        println!("✅ Verified collection has 5 items including 3 and is not empty.");
    }

    // Option type with chaining
    let maybe_value: Option<i32> = Some(42);
    println!("\n--- Option assertions ---");

    let success = guard_test(|| {
        expect!(&maybe_value).to_be_some().and().to_contain(&42);
    });

    if success {
        println!("✅ Verified Option is Some and contains 42.");
    }

    // Result type with chaining
    let result: Result<i32, &str> = Ok(42);
    println!("\n--- Result assertions ---");

    let success = guard_test(|| {
        expect!(&result).to_be_ok().and().to_contain_ok(&42);
    });

    if success {
        println!("✅ Verified Result is Ok and contains 42.");
    }

    // HashMap with modifiers
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    println!("\n--- HashMap assertions ---");

    let success = guard_test(|| {
        expect!(&map).to_have_length(2).and().to_contain_key("key1").and().to_contain_entry("key2", "value2");
    });

    if success {
        println!("✅ Verified HashMap has 2 entries with the expected key-value pairs.");
    }

    // Combined complex assertion with AND, OR, and NOT
    println!("\n--- Complex combined assertion ---");

    let success = guard_test(|| {
        expect!(number).to_be_greater_than(40).and().to_be_less_than(50).or().to_be_even();
    });

    let success2 = guard_test(|| {
        expect!(&numbers).not().to_be_empty().and().to_contain(3).or().to_have_length(10);
    });

    if success && success2 {
        println!("✅ Verified complex combined assertions with AND, OR, and NOT.");
    }

    println!("\nAll examples completed successfully!");
}
