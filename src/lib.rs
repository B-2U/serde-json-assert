//! This crate includes macros for comparing two serializable values by diffing their JSON
//! representations. It is designed to give much more helpful error messages than the standard
//! [`assert_eq!`]. It basically does a diff of the two objects and tells you the exact
//! differences. This is useful when asserting that two large JSON objects are the same.
//!
//! It uses the [serde] and [serde_json] to perform the serialization.
//!
//! [serde]: https://crates.io/crates/serde
//! [serde_json]: https://crates.io/crates/serde_json
//! [`assert_eq!`]: https://doc.rust-lang.org/std/macro.assert_eq.html
//!
//! ## Partial matching
//!
//! If you want to assert that one JSON value is "included" in another use
//! [`assert_json_include`](macro.assert_json_include.html):
//!
//! ```should_panic
//! use serde_json_assert::assert_json_include;
//! use serde_json::json;
//!
//! let a = json!({
//!     "data": {
//!         "users": [
//!             {
//!                 "id": 1,
//!                 "country": {
//!                     "name": "Denmark"
//!                 }
//!             },
//!             {
//!                 "id": 24,
//!                 "country": {
//!                     "name": "Denmark"
//!                 }
//!             }
//!         ]
//!     }
//! });
//!
//! let b = json!({
//!     "data": {
//!         "users": [
//!             {
//!                 "id": 1,
//!                 "country": {
//!                     "name": "Sweden"
//!                 }
//!             },
//!             {
//!                 "id": 2,
//!                 "country": {
//!                     "name": "Denmark"
//!                 }
//!             }
//!         ]
//!     }
//! });
//!
//! assert_json_include!(actual: a, expected: b)
//! ```
//!
//! This will panic with the error message:
//!
//! ```text
//! json atoms at path ".data.users[0].country.name" are not equal:
//!     expected:
//!         "Sweden"
//!     actual:
//!         "Denmark"
//!
//! json atoms at path ".data.users[1].id" are not equal:
//!     expected:
//!         2
//!     actual:
//!         24
//! ```
//!
//! [`assert_json_include`](macro.assert_json_include.html) allows extra data in `actual` but not in
//! `expected`. That is so you can verify just a part of the JSON without having to specify the
//! whole thing. For example this test passes:
//!
//! ```
//! use serde_json_assert::assert_json_include;
//! use serde_json::json;
//!
//! assert_json_include!(
//!     actual: json!({
//!         "a": { "b": 1 },
//!     }),
//!     expected: json!({
//!         "a": {},
//!     })
//! )
//! ```
//!
//! However `expected` cannot contain additional data so this test fails:
//!
//! ```should_panic
//! use serde_json_assert::assert_json_include;
//! use serde_json::json;
//!
//! assert_json_include!(
//!     actual: json!({
//!         "a": {},
//!     }),
//!     expected: json!({
//!         "a": { "b": 1 },
//!     })
//! )
//! ```
//!
//! That will print
//!
//! ```text
//! json atom at path ".a.b" is missing from actual
//! ```
//!
//! ## Exact matching
//!
//! If you want to ensure two JSON values are *exactly* the same, use
//! [`assert_json_eq`](macro.assert_json_eq.html).
//!
//! ```rust,should_panic
//! use serde_json_assert::assert_json_eq;
//! use serde_json::json;
//!
//! assert_json_eq!(
//!     json!({ "a": { "b": 1 } }),
//!     json!({ "a": {} })
//! )
//! ```
//!
//! This will panic with the error message:
//!
//! ```text
//! json atom at path ".a.b" is missing from lhs
//! ```
//! ## Custom message
//!
//! Like [`assert!`], the `assert_json` macros all have second forms, where a
//! custom panic message can be provided. These messages are printed in
//! addition to the default messages.
//!
//! ```rust
//! use serde_json_assert::assert_json_eq;
//! use serde_json::json;
//!
//! let lhs = json!({ "a": 1 });
//! let rhs = json!({ "a": 1 });
//!
//! assert_json_eq!(
//!     lhs,
//!     rhs,
//!     "We are testing {} and {} for equality",
//!     "A",
//!     "B"
//! )
//! ```
//!
//! ## Further customization
//!
//! You can use [`assert_json_matches`] to further customize the comparison.

#![deny(
    missing_docs,
    unused_imports,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unknown_lints
)]

use diff::diff;
use serde::Serialize;

pub use crate::diff::{Difference, Key, Path};

mod core_ext;
mod diff;

/// Assert that a JSON value contains other JSON value
///
/// See [crate documentation](index.html) for examples.
#[macro_export]
macro_rules! assert_json_contains {
    (container: $container:expr, contained: $contained:expr $(,)?) => {{
        let config =
            $crate::Config::new($crate::CompareMode::Inclusive).consider_array_sorting(false);
        $crate::assert_json_matches!($container, $contained, &config)
    }};
    (container: $container:expr, contained: $contained:expr, $($arg:tt)+) => {{
        let config =
            $crate::Config::new($crate::CompareMode::Inclusive).consider_array_sorting(false);
        $crate::assert_json_matches!($container, $contained, &config, $($arg)+)
    }};
}

/// Compare two JSON values for an inclusive match.
///
/// It allows `actual` to contain additional data. If you want an exact match use
/// [`assert_json_eq`](macro.assert_json_eq.html) instead.
///
/// See [crate documentation](index.html) for examples.
#[macro_export]
macro_rules! assert_json_include {
    (actual: $actual:expr, expected: $expected:expr $(,)?) => {{
        let config = $crate::Config::new($crate::CompareMode::Inclusive);
        $crate::assert_json_matches!($actual, $expected, &config)
    }};
    (expected: $expected:expr, actual: $actual:expr $(,)?) => {{
        $crate::assert_json_include!(actual: $actual, expected: $expected)
    }};
    (actual: $actual:expr, expected: $expected:expr, $($arg:tt)+) => {{
        let config = $crate::Config::new($crate::CompareMode::Inclusive);
        $crate::assert_json_matches!($actual, $expected, &config, $($arg)+)
    }};
    (expected: $expected:expr, actual: $actual:expr, $($arg:tt)+) => {{
        $crate::assert_json_include!(actual: $actual, expected: $expected, $($arg)+)
    }};
}

/// Compare two JSON values for an exact match.
///
/// If you want an inclusive match use [`assert_json_include`](macro.assert_json_include.html)
/// instead.
///
/// See [crate documentation](index.html) for examples.
#[macro_export]
macro_rules! assert_json_eq {
    ($lhs:expr, $rhs:expr $(,)?) => {{
        let config = $crate::Config::new($crate::CompareMode::Strict);
        $crate::assert_json_matches!($lhs, $rhs, &config)
    }};
    ($lhs:expr, $rhs:expr, $($arg:tt)+) => {{
        let config = $crate::Config::new($crate::CompareMode::Strict);
        $crate::assert_json_matches!($lhs, $rhs, &config, $($arg)+)
    }};
}

/// Compare two JSON values according to a configuration.
///
/// ```
/// use serde_json_assert::{
///     CompareMode,
///     Config,
///     NumericMode,
///     assert_json_matches,
/// };
/// use serde_json::json;
///
/// let config = Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat);
///
/// assert_json_matches!(
///     json!({
///         "a": { "b": [1, 2, 3.0] },
///     }),
///     json!({
///         "a": { "b": [1, 2.0, 3] },
///     }),
///     &config,
/// );
///
/// assert_json_matches!(
///     json!({
///         "a": { "b": [1, 2, 3.0] },
///     }),
///     json!({
///         "a": { "b": [1, 2.0, 3] },
///     }),
///     &config,
///     "Failed to assert equality between {} and {}",
///     "lhs",
///     "rhs"
/// );
/// ```
///
/// When using `CompareMode::Inclusive` the first argument is `actual` and the second argument is
/// `expected`. Example:
///
/// ```
/// # use serde_json_assert::{
/// #     CompareMode,
/// #     Config,
/// #     NumericMode,
/// #     assert_json_matches,
/// #     assert_json_include,
/// # };
/// # use serde_json::json;
/// #
/// // This
/// let config = Config::new(CompareMode::Inclusive);
/// assert_json_matches!(
///     json!({
///         "a": { "b": 1 },
///     }),
///     json!({
///         "a": {},
///     }),
///     &config,
/// );
///
/// // Is the same as this
/// assert_json_include!(
///     actual: json!({
///         "a": { "b": 1 },
///     }),
///     expected: json!({
///         "a": {},
///     }),
/// );
/// ```
#[macro_export]
macro_rules! assert_json_matches {
    ($lhs:expr, $rhs:expr, $config:expr $(,)?) => {{
        if let Err(error) = $crate::assert_json_matches_no_panic(&$lhs, &$rhs, $config) {
            panic!("\n{}", error);
        }
    }};
    ($lhs:expr, $rhs:expr, $config:expr, $($arg:tt)+) => {{
        if let Err(error) = $crate::assert_json_matches_no_panic(&$lhs, &$rhs, $config) {
            panic!("\n{}\n\n{}", format_args!($($arg)+), error);
        }
    }};
}

/// Compares two JSON values without panicking.
///
/// Instead it returns a `Result` where the error is the message that would be passed to `panic!`.
/// This is might be useful if you want to control how failures are reported and don't want to deal
/// with panics.
pub fn assert_json_matches_no_panic<Lhs, Rhs>(
    lhs: &Lhs,
    rhs: &Rhs,
    config: &Config,
) -> Result<(), String>
where
    Lhs: Serialize,
    Rhs: Serialize,
{
    let lhs = serde_json::to_value(lhs).unwrap_or_else(|err| {
        panic!(
            "Couldn't convert left hand side value to JSON. Serde error: {}",
            err
        )
    });
    let rhs = serde_json::to_value(rhs).unwrap_or_else(|err| {
        panic!(
            "Couldn't convert right hand side value to JSON. Serde error: {}",
            err
        )
    });

    let diffs = diff(&lhs, &rhs, config);

    if diffs.is_empty() {
        Ok(())
    } else {
        let msg = diffs
            .into_iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join("\n\n");
        Err(msg)
    }
}

/// Compares two JSON values without panicking.
///
/// Returns a `Result` containing either `Ok(())` if the values match,
/// or an `Err` with a [`Vec<Difference>`](Difference) describing the differences.
///
/// # Note:
///
/// This function performs some cloning and may be less efficient.
///
/// If you only need a string error message, use [`assert_json_matches_no_panic`] or the assertion
/// macros.
///
/// # Examples
///
/// ```
/// use serde_json_assert::{try_assert_json_matches, Config, CompareMode};
/// use serde_json::json;
///
/// let lhs = json!({ "a": 1, "b": 2 });
/// let rhs = json!({ "a": 1 });
/// let config = Config::new(CompareMode::Inclusive);
///
/// let result = try_assert_json_matches(&lhs, &rhs, &config);
/// assert!(result.is_ok());
///
/// let lhs = json!({ "a": 1 });
/// let rhs = json!({ "a": 2 });
/// let config = Config::new(CompareMode::Strict);
///
/// let result = try_assert_json_matches(&lhs, &rhs, &config);
/// assert!(result.is_err());
/// ```
pub fn try_assert_json_matches<Lhs, Rhs>(
    lhs: &Lhs,
    rhs: &Rhs,
    config: &Config,
) -> Result<(), Vec<Difference>>
where
    Lhs: Serialize,
    Rhs: Serialize,
{
    let lhs = serde_json::to_value(lhs).unwrap_or_else(|err| {
        panic!(
            "Couldn't convert left hand side value to JSON. Serde error: {}",
            err
        )
    });
    let rhs = serde_json::to_value(rhs).unwrap_or_else(|err| {
        panic!(
            "Couldn't convert right hand side value to JSON. Serde error: {}",
            err
        )
    });

    let diffs = diff(&lhs, &rhs, config);
    let diffs_buf: Vec<Difference> = diffs.into_iter().map(|d| d.into()).collect();

    if diffs_buf.is_empty() {
        Ok(())
    } else {
        Err(diffs_buf)
    }
}

/// Configuration for how JSON values should be compared.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_copy_implementations)]
pub struct Config {
    /// Should array sorting be taken in consideration.
    pub array_sorting_mode: ArraySortingMode,
    /// How should JSON values be compared.
    pub compare_mode: CompareMode,
    /// How should numbers be compared.
    pub numeric_mode: NumericMode,
    /// How should floating point numbers be compared.
    pub float_compare_mode: FloatCompareMode,
}

impl Config {
    /// Create a new [`Config`] using the given [`CompareMode`].
    ///
    /// The default `numeric_mode` is be [`NumericMode::Strict`].
    pub fn new(compare_mode: CompareMode) -> Self {
        Self {
            array_sorting_mode: ArraySortingMode::Consider,
            compare_mode,
            numeric_mode: NumericMode::Strict,
            float_compare_mode: FloatCompareMode::Exact,
        }
    }

    /// Change the config's numeric mode.
    ///
    /// The default `numeric_mode` is be [`NumericMode::Strict`].
    pub fn numeric_mode(mut self, numeric_mode: NumericMode) -> Self {
        self.numeric_mode = numeric_mode;
        self
    }

    /// Change the config's compare mode.
    pub fn compare_mode(mut self, compare_mode: CompareMode) -> Self {
        self.compare_mode = compare_mode;
        self
    }

    /// Change the config's float compare mode.
    ///
    /// The default `float_compare_mode` is [`FloatCompareMode::Exact`].
    pub fn float_compare_mode(mut self, float_compare_mode: FloatCompareMode) -> Self {
        self.float_compare_mode = float_compare_mode;
        self
    }

    /// configure array sorting mode
    pub fn consider_array_sorting(mut self, consider: bool) -> Self {
        if consider {
            if self.compare_mode == CompareMode::Strict {
                panic!("strict comparison does not allow array ordering to be ignored");
            }
            self.array_sorting_mode = ArraySortingMode::Consider;
        } else {
            self.array_sorting_mode = ArraySortingMode::Ignore;
        }
        self
    }
}

/// Mode for how JSON values should be compared.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CompareMode {
    /// The two JSON values don't have to be exactly equal. The "expected" value is only required
    /// to be "contained" inside "actual". See [crate documentation](index.html) for examples.
    ///
    /// The mode used with [`assert_json_include`].
    Inclusive,
    /// The two JSON values must be exactly equal.
    ///
    /// The mode used with [`assert_json_eq`].
    Strict,
}

/// Should array sorting be taken in consideration
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ArraySortingMode {
    ///consider
    Consider,
    /// ignore
    Ignore,
}

/// How should numbers be compared.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NumericMode {
    /// Different numeric types aren't considered equal.
    Strict,
    /// All numeric types are converted to float before comparison.
    AssumeFloat,
}

/// How should floating point numbers be compared.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FloatCompareMode {
    /// Different floats are never considered equal.
    Exact,
    /// Floats are considered equal if they differ by at most this epsilon value.
    Epsilon(f64),
}

impl Eq for FloatCompareMode {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    use std::fmt::Write;

    #[test]
    fn boolean_root() {
        let result = test_partial_match(json!(true), json!(true));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!(false), json!(false));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!(false), json!(true));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    expected:
        true
    actual:
        false"#),
        );

        let result = test_partial_match(json!(true), json!(false));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    expected:
        false
    actual:
        true"#),
        );
    }

    #[test]
    fn string_root() {
        let result = test_partial_match(json!("true"), json!("true"));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!("false"), json!("false"));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!("false"), json!("true"));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    expected:
        "true"
    actual:
        "false""#),
        );

        let result = test_partial_match(json!("true"), json!("false"));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    expected:
        "false"
    actual:
        "true""#),
        );
    }

    #[test]
    fn number_root() {
        let result = test_partial_match(json!(1), json!(1));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!(0), json!(0));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!(0), json!(1));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    expected:
        1
    actual:
        0"#),
        );

        let result = test_partial_match(json!(1), json!(0));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    expected:
        0
    actual:
        1"#),
        );
    }

    #[test]
    fn null_root() {
        let result = test_partial_match(json!(null), json!(null));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!(null), json!(1));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    expected:
        1
    actual:
        null"#),
        );

        let result = test_partial_match(json!(1), json!(null));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    expected:
        null
    actual:
        1"#),
        );
    }

    #[test]
    fn into_object() {
        let result = test_partial_match(json!({ "a": true }), json!({ "a": true }));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!({ "a": false }), json!({ "a": true }));
        assert_output_eq(
            result,
            Err(r#"json atoms at path ".a" are not equal:
    expected:
        true
    actual:
        false"#),
        );

        let result =
            test_partial_match(json!({ "a": { "b": true } }), json!({ "a": { "b": true } }));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!({ "a": true }), json!({ "a": { "b": true } }));
        assert_output_eq(
            result,
            Err(r#"json atoms at path ".a" are not equal:
    expected:
        {
          "b": true
        }
    actual:
        true"#),
        );

        let result = test_partial_match(json!({}), json!({ "a": true }));
        assert_output_eq(
            result,
            Err(r#"json atom at path ".a" is missing from actual"#),
        );

        let result = test_partial_match(json!({ "a": { "b": true } }), json!({ "a": true }));
        assert_output_eq(
            result,
            Err(r#"json atoms at path ".a" are not equal:
    expected:
        true
    actual:
        {
          "b": true
        }"#),
        );
    }

    #[test]
    fn into_array() {
        let result = test_partial_match(json!([1]), json!([1]));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!([2]), json!([1]));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "[0]" are not equal:
    expected:
        1
    actual:
        2"#),
        );

        let result = test_partial_match(json!([1, 2, 4]), json!([1, 2, 3]));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "[2]" are not equal:
    expected:
        3
    actual:
        4"#),
        );

        let result = test_partial_match(json!({ "a": [1, 2, 3]}), json!({ "a": [1, 2, 4]}));
        assert_output_eq(
            result,
            Err(r#"json atoms at path ".a[2]" are not equal:
    expected:
        4
    actual:
        3"#),
        );

        let result = test_partial_match(json!({ "a": [1, 2, 3]}), json!({ "a": [1, 2]}));
        assert_output_eq(result, Ok(()));

        let result = test_partial_match(json!({ "a": [1, 2]}), json!({ "a": [1, 2, 3]}));
        assert_output_eq(
            result,
            Err(r#"json atom at path ".a[2]" is missing from actual"#),
        );
    }

    #[test]
    fn exact_matching() {
        let result = test_exact_match(json!(true), json!(true));
        assert_output_eq(result, Ok(()));

        let result = test_exact_match(json!("s"), json!("s"));
        assert_output_eq(result, Ok(()));

        let result = test_exact_match(json!("a"), json!("b"));
        assert_output_eq(
            result,
            Err(r#"json atoms at path "(root)" are not equal:
    lhs:
        "a"
    rhs:
        "b""#),
        );

        let result = test_exact_match(
            json!({ "a": [1, { "b": 2 }] }),
            json!({ "a": [1, { "b": 3 }] }),
        );
        assert_output_eq(
            result,
            Err(r#"json atoms at path ".a[1].b" are not equal:
    lhs:
        2
    rhs:
        3"#),
        );
    }

    #[test]
    fn exact_match_output_message() {
        let result = test_exact_match(json!({ "a": { "b": 1 } }), json!({ "a": {} }));
        assert_output_eq(
            result,
            Err(r#"json atom at path ".a.b" is missing from rhs"#),
        );

        let result = test_exact_match(json!({ "a": {} }), json!({ "a": { "b": 1 } }));
        assert_output_eq(
            result,
            Err(r#"json atom at path ".a.b" is missing from lhs"#),
        );
    }

    fn assert_output_eq(actual: Result<(), String>, expected: Result<(), &str>) {
        match (actual, expected) {
            (Ok(()), Ok(())) => {}

            (Err(actual_error), Ok(())) => {
                let mut f = String::new();
                writeln!(f, "Did not expect error, but got").unwrap();
                writeln!(f, "{}", actual_error).unwrap();
                panic!("{}", f);
            }

            (Ok(()), Err(expected_error)) => {
                let expected_error = expected_error.to_string();
                let mut f = String::new();
                writeln!(f, "Expected error, but did not get one. Expected error:").unwrap();
                writeln!(f, "{}", expected_error).unwrap();
                panic!("{}", f);
            }

            (Err(actual_error), Err(expected_error)) => {
                let expected_error = expected_error.to_string();
                if actual_error != expected_error {
                    let mut f = String::new();
                    writeln!(f, "Errors didn't match").unwrap();
                    writeln!(f, "Expected:").unwrap();
                    writeln!(f, "{}", expected_error).unwrap();
                    writeln!(f, "Got:").unwrap();
                    writeln!(f, "{}", actual_error).unwrap();
                    panic!("{}", f);
                }
            }
        }
    }

    fn test_partial_match(lhs: Value, rhs: Value) -> Result<(), String> {
        assert_json_matches_no_panic(&lhs, &rhs, &Config::new(CompareMode::Inclusive))
    }

    fn test_exact_match(lhs: Value, rhs: Value) -> Result<(), String> {
        assert_json_matches_no_panic(&lhs, &rhs, &Config::new(CompareMode::Strict))
    }
}
