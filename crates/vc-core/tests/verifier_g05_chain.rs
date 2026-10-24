//! Verifier G05: chain validator probes.
use serde_json::{json, Value};
use vc_core::chain::*;

const Z: &str = "0000000000000000000000000000000000000000000000000000000000000000";
fn st(agent: &str, kind: &str, seq: u64, prev: &str, extra: Value) -> Value {
    let mut v = json!({"v":2,"kind":kind,"agent":{"id":agent},"seq":seq,"prev":prev,"ts":"2026-01-01T00:00:00Z"});
    for (k, x) in extra.as_object().unwrap() {
        v.as_object_mut().unwrap().insert(k.clone(), x.clone());
    }
    v
}

// CLAIM (bug, low): validate() does not enforce schema constants: v==2, known kind, non-empty agent.id, ts present.
#[test]
fn bug_schema_constants_not_enforced() {
    let mut s = st("", "bogus-kind", 0, Z, json!({}));
    s["v"] = json!(99);
    s.as_object_mut().unwrap().remove("ts");
    assert!(
        validate(&[s]).is_err(),
        "statement with v=99, unknown kind, empty agent id, no ts accepted"
    );
}

// regression: uppercase-hex prev is rejected (BadPrev), not normalised.
#[test]
fn regression_uppercase_prev_rejected() {
    let a0 = st("a", "intent", 0, Z, json!({}));
    let id0 = statement_id(&a0).unwrap();
    let a1 = st("a", "intent", 1, &id0.to_uppercase(), json!({}));
    assert_eq!(validate(&[a0, a1]), Err(ChainError::BadPrev(1)));
}

#[test]
fn regression_first_seq_nonzero() {
    assert_eq!(
        validate(&[st("a", "intent", 1, Z, json!({}))]),
        Err(ChainError::SeqGap(0, 1, 0))
    );
}

#[test]
fn regression_seq_forms() {
    for bad in [json!(1.0), json!(-1), json!("1"), json!(null), json!(1.5)] {
        let mut s = st("a", "intent", 0, Z, json!({}));
        s["seq"] = bad.clone();
        assert_eq!(
            validate(&[s]),
            Err(ChainError::Field(0, "seq")),
            "seq={bad}"
        );
    }
    // missing fields
    for f in ["seq", "prev", "kind"] {
        let mut s = st("a", "intent", 0, Z, json!({}));
        s.as_object_mut().unwrap().remove(f);
        assert!(matches!(validate(&[s]), Err(ChainError::Field(0, _))));
    }
    let mut s = st("a", "intent", 0, Z, json!({}));
    s["agent"] = json!({});
    assert_eq!(validate(&[s]), Err(ChainError::Field(0, "agent.id")));
}

#[test]
fn regression_outcome_before_intent_and_self_reference() {
    let a0 = st("a", "intent", 0, Z, json!({}));
    let id0 = statement_id(&a0).unwrap();
    // outcome first in log order, pointing at an intent that appears later
    let o = st("a", "outcome", 0, Z, json!({"pair":{"intent_id":id0}}));
    assert_eq!(
        validate(&[o, a0.clone()]),
        Err(ChainError::UnknownIntent(0))
    );
    // outcome pointing at its own (guessed) id / an outcome id is unknown
    let o1 = st("a", "outcome", 1, &id0, json!({"pair":{"intent_id":id0}}));
    let oid = statement_id(&o1).unwrap();
    let o2 = st("a", "outcome", 2, &oid, json!({"pair":{"intent_id":oid}}));
    assert_eq!(validate(&[a0, o1, o2]), Err(ChainError::UnknownIntent(2)));
}

#[test]
fn regression_resubmitted_statement_rejected() {
    let a0 = st("a", "intent", 0, Z, json!({}));
    assert_eq!(
        validate(&[a0.clone(), a0]),
        Err(ChainError::SeqGap(1, 0, 1))
    );
}

#[test]
fn regression_interleaved_agents_and_open_intents() {
    let a0 = st("a", "intent", 0, Z, json!({}));
    let b0 = st("b", "intent", 0, Z, json!({}));
    let r = validate(&[a0.clone(), b0.clone()]).unwrap();
    assert_eq!(
        r.open_intents,
        vec![statement_id(&a0).unwrap(), statement_id(&b0).unwrap()]
    );
}

#[test]
fn regression_statement_id_independent_of_field_order_and_number_form() {
    let a: Value = serde_json::from_str(
        r#"{"v":2,"kind":"intent","agent":{"id":"a"},"seq":0,"prev":"x","n":1.0}"#,
    )
    .unwrap();
    let b: Value = serde_json::from_str(
        r#"{"n":1,"prev":"x","seq":0,"agent":{"id":"a"},"kind":"intent","v":2}"#,
    )
    .unwrap();
    assert_eq!(statement_id(&a).unwrap(), statement_id(&b).unwrap());
}

// regression: performance of 30k-statement log (hash-map based, linear)
#[test]
fn regression_large_log_linear() {
    let mut log = vec![];
    let mut last: Vec<String> = vec![Z.to_string(); 100];
    let mut seqs = vec![0u64; 100];
    for i in 0..30_000usize {
        let a = i % 100;
        let s = st(
            &format!("agent-{a}"),
            "intent",
            seqs[a],
            &last[a],
            json!({"i": i}),
        );
        last[a] = statement_id(&s).unwrap();
        seqs[a] += 1;
        log.push(s);
    }
    let t = std::time::Instant::now();
    let r = validate(&log).unwrap();
    assert_eq!(r.open_intents.len(), 30_000);
    assert!(
        t.elapsed().as_secs() < 30,
        "validate too slow: {:?}",
        t.elapsed()
    );
}
