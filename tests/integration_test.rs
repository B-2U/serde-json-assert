use serde::Serialize;
use serde_json::json;
use serde_json_assert::{
    assert_json_contains, assert_json_eq, assert_json_include, assert_json_matches,
    assert_json_matches_no_panic, CompareMode, Config, FloatCompareMode, NumericMode,
};

#[test]
fn can_pass() {
    assert_json_include!(
        actual: json!({ "a": { "b": true }, "c": [true, null, 1] }),
        expected: json!({ "a": { "b": true }, "c": [true, null, 1] })
    );

    assert_json_include!(
        actual: json!({ "a": { "b": true } }),
        expected: json!({ "a": {} })
    );

    assert_json_include!(
        actual: json!({ "a": { "b": true } }),
        expected: json!({ "a": {} }),
    );

    assert_json_include!(
        expected: json!({ "a": {} }),
        actual: json!({ "a": { "b": true } }),
    );
}

#[test]
#[should_panic]
fn can_fail() {
    assert_json_include!(
        actual: json!({ "a": { "b": true }, "c": [true, null, 1] }),
        expected: json!({ "a": { "b": false }, "c": [false, null, {}] })
    );
}

#[test]
#[should_panic]
fn different_numeric_types_include_should_fail() {
    assert_json_include!(
        actual: json!({ "a": { "b": true }, "c": 1 }),
        expected: json!({ "a": { "b": true }, "c": 1.0 })
    );
}

#[test]
#[should_panic]
fn different_numeric_types_eq_should_fail() {
    assert_json_eq!(
        json!({ "a": { "b": true }, "c": 1 }),
        json!({ "a": { "b": true }, "c": 1.0 })
    );
}

#[test]
fn different_numeric_types_assume_float() {
    let actual = json!({ "a": { "b": true }, "c": [true, null, 1] });
    let expected = json!({ "a": { "b": true }, "c": [true, null, 1.0] });
    let config = Config::new(CompareMode::Inclusive).numeric_mode(NumericMode::AssumeFloat);
    assert_json_matches!(&actual, &expected, &config);

    let config = config.compare_mode(CompareMode::Strict);
    assert_json_matches!(actual, expected, &config);
}

#[test]
fn can_pass_with_exact_match() {
    assert_json_eq!(json!({ "a": { "b": true } }), json!({ "a": { "b": true } }));
    assert_json_eq!(json!({ "a": { "b": true } }), json!({ "a": { "b": true } }),);
}

#[test]
fn can_pass_with_contains_match() {
    // null contains null
    assert_json_contains!(container: json!(null), contained: json!(null));
    // numeric value contains numeric value
    assert_json_contains!(container: json!(1), contained: json!(1));
    // string contains string
    assert_json_contains!(container: json!("a"), contained: json!("a"));
    // object 1 contains identical object 2
    assert_json_contains!(
        container: json!({ "a": { "b": true } }),
        contained: json!({ "a": { "b": true } })
    );
    // object 1 has more keys than object 2, but the keys on object 2 match the keys on object 1
    assert_json_contains!(
        container: json!({ "a": { "b": true }, "c": 1}),
        contained: json!({ "a": { "b": true } })
    );
    // array 1 contains identical array 2
    assert_json_contains!(container: json!([1, 2, 3]), contained: json!([1, 2, 3]));
    // array 1 contains all items on array 2, even itens on array 2 being in different order than
    // they are on array 1
    assert_json_contains!(container: json!([1, 2, 3]), contained: json!([2, 3, 1]));
    // array 1 contains more items than array 2, but items on array 2 match items on array 1 in the
    // same order
    assert_json_contains!(container: json!([1, 2, 3, 4]), contained: json!([1, 2, 3]));
    // array 1 contains more items than array 2, but items on array 2 match items on array 1 in
    // diferent order
    assert_json_contains!(container: json!([1, 2, 3, 4]), contained: json!([2, 3, 1]));
    // array 1 contains all items on array2 with the same amount of repeated items on both, in the
    // same order
    assert_json_contains!(
        container: json!([1, 2, 3, 1, 4]),
        contained: json!([1, 2, 3, 1, 4])
    );
    // array 1 contains all items on array2 with the same amount of repeated items on both, in
    // different order
    assert_json_contains!(
        container: json!([1, 2, 3, 1, 4]),
        contained: json!([3, 1, 2, 1, 4])
    );
    // array 1 contains more items than array 2, but items on aray 2 match items on array 1 with
    // repeated items on both in the same order
    assert_json_contains!(
        container: json!([1, 2, 3, 1, 4]),
        contained: json!([1, 2, 3, 1])
    );
    // array 1 contains more items than array 2, but items on array 2 match items on array 1 with
    // repeated items on both in different order
    assert_json_contains!(
        container: json!([1, 2, 3, 1, 4]),
        contained: json!([2, 1, 3, 1])
    );
}

#[test]
#[should_panic]
fn can_fail_with_exact_match() {
    assert_json_eq!(json!({ "a": { "b": true } }), json!({ "a": {} }));
}

#[test]
fn inclusive_match_without_panicking() {
    let config = Config::new(CompareMode::Inclusive).numeric_mode(NumericMode::Strict);
    assert!(
        assert_json_matches_no_panic(&json!({ "a": 1, "b": 2 }), &json!({ "b": 2}), &config)
            .is_ok()
    );

    assert!(
        assert_json_matches_no_panic(&json!({ "a": 1, "b": 2 }), &json!("foo"), &config,).is_err()
    );
}

#[test]
fn exact_match_without_panicking() {
    let config = Config::new(CompareMode::Strict).numeric_mode(NumericMode::Strict);
    assert!(assert_json_matches_no_panic(&json!([1, 2, 3]), &json!([1, 2, 3]), &config,).is_ok());

    assert!(assert_json_matches_no_panic(&json!([1, 2, 3]), &json!("foo"), &config).is_err());
}

#[derive(Serialize)]
struct User {
    id: i32,
    username: String,
}

#[test]
fn include_with_serializable() {
    let user = User {
        id: 1,
        username: "bob".to_string(),
    };

    assert_json_include!(
        actual: json!({
            "id": 1,
            "username": "bob",
            "email": "bob@example.com"
        }),
        expected: user,
    );
}

#[test]
fn include_with_serializable_ref() {
    let user = User {
        id: 1,
        username: "bob".to_string(),
    };

    assert_json_include!(
        actual: &json!({
             "id": 1,
             "username": "bob",
             "email": "bob@example.com"
         }),
        expected: &user,
    );
}

#[test]
fn eq_with_serializable() {
    let user = User {
        id: 1,
        username: "bob".to_string(),
    };

    assert_json_eq!(
        json!({
            "id": 1,
            "username": "bob"
        }),
        user,
    );
}

#[test]
fn eq_with_serializable_ref() {
    let user = User {
        id: 1,
        username: "bob".to_string(),
    };

    assert_json_eq!(
        &json!({
            "id": 1,
            "username": "bob"
        }),
        &user,
    );
}

#[derive(Serialize)]
struct Person {
    name: String,
    height: f64,
}

#[test]
fn can_pass_with_exact_float_comparison() {
    let person = Person {
        name: "bob".to_string(),
        height: 1.79,
    };

    let config = Config::new(CompareMode::Strict).float_compare_mode(FloatCompareMode::Exact);
    assert_json_matches!(
        &json!({
            "name": "bob",
            "height": 1.79
        }),
        &person,
        &config,
    );
}

#[test]
#[should_panic]
fn can_fail_with_exact_float_comparison() {
    let person = Person {
        name: "bob".to_string(),
        height: 1.79,
    };
    let config = Config::new(CompareMode::Strict).float_compare_mode(FloatCompareMode::Exact);

    assert_json_matches!(
        &json!({
            "name": "bob",
            "height": 1.7900001
        }),
        &person,
        &config
    );
}

#[test]
fn can_pass_with_epsilon_based_float_comparison() {
    let person = Person {
        name: "bob".to_string(),
        height: 1.79,
    };
    let config =
        Config::new(CompareMode::Strict).float_compare_mode(FloatCompareMode::Epsilon(0.00001));

    assert_json_matches!(
        &json!({
            "name": "bob",
            "height": 1.7900001
        }),
        &person,
        &config
    );
}

#[test]
#[should_panic]
fn can_fail_with_epsilon_based_float_comparison() {
    let person = Person {
        name: "bob".to_string(),
        height: 1.79,
    };
    let config =
        Config::new(CompareMode::Strict).float_compare_mode(FloatCompareMode::Epsilon(0.00001));

    assert_json_matches!(
        &json!({
            "name": "bob",
            "height": 1.7901
        }),
        &person,
        &config
    );
}

#[test]
fn ignore_array_sorting_with_inclusive_comparisons() {
    let actual = json!([
        {
            "a": 1,
            "b": true,
            "c": "foo"
        },
        {
            "a": 2,
            "b": false,
            "c": "bar"
        },
        {
            "a": 3,
            "b": false,
            "c": "baz"
        }
    ]);
    let expected = json!([
        {
            "b": false,
            "c": "bar"
        },
        {
            "b": true,
            "c": "foo"
        }
    ]);
    let config = Config::new(CompareMode::Inclusive).consider_array_sorting(false);
    assert_json_matches!(&actual, &expected, &config);
}

#[test]
#[should_panic]
fn can_fail_ignore_array_sorting_with_strict_comparisons() {
    let actual = json!([
        {
            "b": true,
            "c": "foo"
        },
        {
            "b": false,
            "c": "bar"
        },
        {
            "b": false,
            "c": "baz"
        }
    ]);
    let expected = json!([
        {
            "b": false,
            "c": "bar"
        },
        {
            "b": true,
            "c": "foo"
        }
    ]);
    let config = Config::new(CompareMode::Strict).consider_array_sorting(false);
    assert_json_matches!(&actual, &expected, &config);
}

#[test]
fn assert_json_contains_can_fail_with_message() {
    let result = std::panic::catch_unwind(|| {
        assert_json_contains!(
            container: json!({ "a": { "b": true } }),
            contained: json!({ "a": { "b": false } }),
            "The {} assert failed because of {}",
            "'contains'",
            "'reasons'"
        );
    });

    assert!(result.is_err());

    let error = result.unwrap_err();
    let msg = error.downcast_ref::<String>().unwrap();
    assert!(msg.contains("The 'contains' assert failed because of 'reasons'"));
}

#[test]
fn assert_json_include_can_fail_with_message() {
    let result = std::panic::catch_unwind(|| {
        assert_json_include!(
            actual: json!({ "a": { "b": true } }),
            expected: json!({ "a": { "b": false } }),
            "The {} assert failed because of {}",
            "'include'",
            "'reasons'"
        );
    });

    assert!(result.is_err());

    let error = result.unwrap_err();
    let msg = error.downcast_ref::<String>().unwrap();
    assert!(msg.contains("The 'include' assert failed because of 'reasons'"));
}

#[test]
fn assert_json_eq_can_fail_with_message() {
    let result = std::panic::catch_unwind(|| {
        assert_json_eq!(
            json!({ "a": { "b": true } }),
            json!({ "a": { "b": false } }),
            "The {} assert failed because of {}",
            "'eq'",
            "'reasons'"
        );
    });

    assert!(result.is_err());

    let error = result.unwrap_err();
    let msg = error.downcast_ref::<String>().unwrap();
    assert!(msg.contains("The 'eq' assert failed because of 'reasons'"));
}

#[test]
fn assert_json_matches_can_fail_with_message() {
    let config = Config::new(CompareMode::Strict).consider_array_sorting(false);
    let result = std::panic::catch_unwind(|| {
        assert_json_matches!(
            json!({ "a": { "b": true } }),
            json!({ "a": { "b": false } }),
            &config,
            "The {} assert failed because of {}",
            "'matches'",
            "'reasons'"
        );
    });

    assert!(result.is_err());

    let error = result.unwrap_err();
    let msg = error.downcast_ref::<String>().unwrap();
    assert!(msg.contains("The 'matches' assert failed because of 'reasons'"));
}
