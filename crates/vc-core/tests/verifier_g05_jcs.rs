//! Verifier G05: RFC 8785 conformance probes for vc-core::canonical (serde_jcs 0.1.0 + serde_json Value).
use vc_core::canonical::*;

fn c(s: &str) -> String {
    String::from_utf8(canonicalize_str(s).unwrap()).unwrap()
}

// CLAIM (bug): RFC 8785 3.2.3 sorts keys by UTF-16 code units. U+1F600 is D83D DE00 in UTF-16,
// which sorts BEFORE U+E000. serde_jcs sorts by UTF-8 bytes, so U+E000 comes first.
#[test]
fn bug_key_order_utf16_astral_vs_bmp_high() {
    let out = c("{\"\u{e000}\":1,\"\u{1F600}\":2}");
    assert_eq!(
        out, "{\"\u{1F600}\":2,\"\u{e000}\":1}",
        "keys must be ordered by UTF-16 code units"
    );
}

// CLAIM (bug): RFC 8785 3.2.2.3 / ES6: numbers are IEEE-754 doubles. 2^53+1 must serialize as 9007199254740992.
// serde_json keeps it as u64 and serde_jcs prints exact digits.
#[test]
fn bug_integer_above_2_53_not_reduced_to_double() {
    assert_eq!(c("{\"n\":9007199254740993}"), "{\"n\":9007199254740992}");
}

// CLAIM (bug): same for u64::MAX -> ES6 prints 18446744073709552000
#[test]
fn bug_u64_max_es6_form() {
    assert_eq!(c("[18446744073709551615]"), "[18446744073709552000]");
}

// CLAIM (bug): duplicate object keys must be rejected (RFC 8785 sec. 3.1 / I-JSON, RFC 7493 sec. 2.3).
// serde_json Value silently keeps the last one.
#[test]
fn bug_duplicate_keys_rejected() {
    assert!(
        canonicalize_str("{\"a\":1,\"a\":2}").is_err(),
        "duplicate keys silently accepted (last wins)"
    );
}

// regression: number edge cases per ES6 Number::toString
#[test]
fn regression_number_forms() {
    assert_eq!(c("[1e21]"), "[1e+21]");
    assert_eq!(c("[1e-7]"), "[1e-7]");
    assert_eq!(c("[0.000001]"), "[0.000001]");
    assert_eq!(c("[1.0]"), "[1]");
    assert_eq!(c("[-0]"), "[0]");
    assert_eq!(c("[-0.0]"), "[0]");
    assert_eq!(c("[123456789012345680000]"), "[123456789012345680000]");
    assert_eq!(c("[1E30]"), "[1e+30]");
    assert_eq!(c("[5e-324]"), "[5e-324]");
    assert_eq!(c("[1.7976931348623157e308]"), "[1.7976931348623157e+308]");
}

// regression: non-finite / out-of-range numbers are rejected not panicking
#[test]
fn regression_out_of_range_number_rejected() {
    assert!(canonicalize_str("[1e400]").is_err());
    assert!(canonicalize_str("[NaN]").is_err());
}

// regression: lone surrogate escapes are rejected by the parser (no panic, no U+FFFD substitution)
#[test]
fn regression_lone_surrogates_rejected() {
    assert!(canonicalize_str("[\"\\ud800\"]").is_err());
    assert!(canonicalize_str("[\"\\udc00\"]").is_err());
    assert_eq!(c("[\"\\ud83d\\ude00\"]"), "[\"\u{1F600}\"]");
}

// regression: nesting depth bounded by serde_json recursion limit (128) -> error, not stack overflow
#[test]
fn regression_deep_nesting_errors() {
    let deep = |n: usize| format!("{}{}", "[".repeat(n), "]".repeat(n));
    assert!(canonicalize_str(&deep(100)).is_ok());
    assert!(canonicalize_str(&deep(100_000)).is_err());
}

// regression: control chars and escapes
#[test]
fn regression_string_escapes() {
    assert_eq!(
        c("[\"\\u0000\\u001f\\u007f\\u2028\"]"),
        "[\"\\u0000\\u001f\u{7f}\u{2028}\"]"
    );
    assert_eq!(c("[\"\\b\\f\\n\\r\\t\"]"), "[\"\\b\\f\\n\\r\\t\"]");
}

// regression: field order of the input never changes canonical bytes
#[test]
fn regression_order_independent() {
    assert_eq!(
        canonicalize_str("{\"a\":1,\"b\":{\"y\":1,\"x\":2}}").unwrap(),
        canonicalize_str("{\"b\":{\"x\":2,\"y\":1},\"a\":1}").unwrap()
    );
}
