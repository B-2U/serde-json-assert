use serde_json::json;
use serde_json_assert::assert_json_matches_no_panic;
use serde_json_assert::Config;
use std::time::Instant;

#[test]
fn test_diff_json_values_speed() {
    let value1 = json!({
        "name": "Alice",
        "age": 30,
        "emails": ["alice@example.com", "alice@work.com"],
        "address": {
            "city": "Wonderland",
            "zip": "12345"
        }
    });

    let value2 = json!({
        "name": "Bob",
        "age": 25,
        "emails": ["bob@example.com"],
        "address": {
            "city": "Builderland",
            "zip": "54321"
        }
    });

    let iterations = 100_000;
    let start = Instant::now();
    let config = Config::new(serde_json_assert::CompareMode::Strict);

    for _ in 0..iterations {
        let _ = assert_json_matches_no_panic(&value1, &value2, &config);
    }

    let duration = start.elapsed();
    println!(
        "Comparing two serde_json::Value {} times took: {:?}",
        iterations, duration
    );
}
