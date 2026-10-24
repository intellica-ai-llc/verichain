use serde_json::json;
use vc_core::{canonical::*, chain::*};

#[test]
fn rfc8785_example() {
    // "@u" stands for a backslash-u escape, substituted at runtime
    let fix = |s: &str| s.replace("@u", "\\u");
    let input = fix(
        r####"{"numbers":[333333333.33333329,1E30,4.50,2e-3,0.000000000000000000000000001],"string":"@u20ac$@u000f@u000aA'@u0042@u0022@u005c\\\"\/","literals":[null,true,false]}"####,
    );
    let want = fix(
        r####"{"literals":[null,true,false],"numbers":[333333333.3333333,1e+30,4.5,0.002,1e-27],"string":"€$@u000f\nA'B\"\\\\\"/"}"####,
    );
    let out = String::from_utf8(canonicalize_str(&input).unwrap()).unwrap();
    assert_eq!(out, want);
}

#[test]
fn key_order_is_canonical() {
    assert_eq!(
        canonicalize_str(r#"{"b":1,"a":2}"#).unwrap(),
        br#"{"a":2,"b":1}"#
    );
}

fn st(
    agent: &str,
    kind: &str,
    seq: u64,
    prev: &str,
    extra: serde_json::Value,
) -> serde_json::Value {
    let mut v = json!({"v":2,"kind":kind,"agent":{"id":agent},"seq":seq,"prev":prev,"ts":"2026-10-23T00:00:00Z"});
    if let (Some(o), Some(e)) = (v.as_object_mut(), extra.as_object()) {
        for (k, x) in e {
            o.insert(k.clone(), x.clone());
        }
    }
    v
}
const Z: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[test]
fn good_chain_and_orphan() {
    let a0 = st("a", "intent", 0, Z, json!({"n":1}));
    let a1 = st(
        "a",
        "intent",
        1,
        &statement_id(&a0).unwrap(),
        json!({"n":2}),
    );
    let id0 = statement_id(&a0).unwrap();
    let a2 = st(
        "a",
        "outcome",
        2,
        &statement_id(&a1).unwrap(),
        json!({"pair":{"intent_id":id0}}),
    );
    let r = validate(&[a0, a1.clone(), a2]).unwrap();
    assert_eq!(r.open_intents, vec![statement_id(&a1).unwrap()]); // a1 is an orphan
}

#[test]
fn negatives() {
    let a0 = st("a", "intent", 0, Z, json!({}));
    let id0 = statement_id(&a0).unwrap();
    let gap = st("a", "intent", 2, &id0, json!({}));
    assert!(matches!(
        validate(&[a0.clone(), gap]),
        Err(ChainError::SeqGap(1, 2, 1))
    ));
    let badprev = st("a", "intent", 1, Z, json!({}));
    assert_eq!(
        validate(&[a0.clone(), badprev]),
        Err(ChainError::BadPrev(1))
    );
    let unk = st(
        "a",
        "outcome",
        1,
        &id0,
        json!({"pair":{"intent_id":"ff00000000000000000000000000000000000000000000000000000000000000"}}),
    );
    assert_eq!(
        validate(&[a0.clone(), unk]),
        Err(ChainError::UnknownIntent(1))
    );
    let o1 = st("a", "outcome", 1, &id0, json!({"pair":{"intent_id":id0}}));
    let o2 = st(
        "a",
        "outcome",
        2,
        &statement_id(&o1).unwrap(),
        json!({"pair":{"intent_id":id0}}),
    );
    assert_eq!(
        validate(&[a0.clone(), o1, o2]),
        Err(ChainError::DuplicateOutcome(2))
    );
    let b0 = st("b", "outcome", 0, Z, json!({"pair":{"intent_id":id0}}));
    assert_eq!(validate(&[a0, b0]), Err(ChainError::AgentMismatch(1)));
}
