//! W1 adversarial verifier tests for vc-seq (independent verifier, not the builder).
//!
//! `bug_*` tests assert the CORRECT behaviour and FAIL today (see docs/reports/verifier_w1_store_seq.md);
//! `holds_*` tests pass today and are regression tests for properties the verifier tried to break.
#[path = "../../vc-store/tests/common/mod.rs"]
mod common;

use common::Shared;
use ed25519_dalek::SigningKey;
use serde_json::{json, Value};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::Ordering;
use vc_core::canonical::canonicalize;
use vc_core::chain::{statement_id, ChainError};
use vc_core::hash::{leaf_hash, sha256, Hash};
use vc_log::checkpoint::{verify_note_for_origin, CheckpointError, KeyInfo};
use vc_seq::*;
use vc_store::MemStore;

const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const ORIGIN: &str = "vc.example/log";

#[derive(Clone)]
struct Ag {
    key: SigningKey,
    id: String,
    seq: u64,
    prev: String,
}

impl Ag {
    fn new(n: u8, id: &str) -> Self {
        Ag { key: SigningKey::from_bytes(&[n; 32]), id: id.into(), seq: 0, prev: ZERO.into() }
    }
    fn stmt(&self, kind: &str, extra: Value) -> Value {
        let mut v = json!({"v":2,"kind":kind,"agent":{"id":self.id},"seq":self.seq,"prev":self.prev,"ts":"2026-10-24T00:00:00Z"});
        for (k, x) in extra.as_object().unwrap() {
            v[k] = x.clone();
        }
        v
    }
    fn raw(&self, payload: &[u8]) -> Vec<u8> {
        vc_cose::sign1(&self.key, self.id.as_bytes(), "application/vc-statement+json", payload)
    }
    fn sign(&mut self, v: &Value) -> Vec<u8> {
        let b = self.raw(&canonicalize(v).unwrap());
        self.prev = statement_id(v).unwrap();
        self.seq += 1;
        b
    }
    fn next(&mut self, kind: &str, extra: Value) -> (Vec<u8>, String) {
        let v = self.stmt(kind, extra);
        let id = statement_id(&v).unwrap();
        (self.sign(&v), id)
    }
}

/// The single place that constructs `Issuer`; if the builder adds an assurance ceiling, edit here only.
fn issuer(a: &Ag, max_completeness: u8) -> Issuer {
    Issuer { key: a.key.verifying_key(), agent_id: a.id.clone(), max_completeness }
}

fn registry(agents: &[&Ag], max: u8) -> Registry {
    let mut r = Registry::new();
    for a in agents {
        r.register(a.id.as_bytes(), issuer(a, max));
    }
    r
}

fn op_key() -> SigningKey {
    SigningKey::from_bytes(&[200; 32])
}

fn open<S: vc_store::TileStore>(store: S, agents: &[&Ag], max: u8) -> Sequencer<S> {
    Sequencer::open(store, registry(agents, max), op_key(), "op", ORIGIN).unwrap()
}

fn leaves_of(cose: &[Vec<u8>]) -> Vec<Hash> {
    cose.iter().map(|c| leaf_hash(c)).collect()
}

// ------------------------------------------------------------------------------------------------------------
// 5a. Registration policy
// ------------------------------------------------------------------------------------------------------------

#[test]
fn holds_registration_policy_negatives() {
    let a = Ag::new(1, "agent-a");
    let b = Ag::new(2, "agent-b");
    let mut s = open(MemStore::new(), &[&a, &b], 3);
    let mut bad: Vec<(String, Vec<u8>)> = Vec::new();

    // unregistered key + unregistered kid
    let mut x = Ag::new(9, "agent-x");
    bad.push(("unregistered agent".into(), x.next("intent", json!({})).0));
    // registered kid, but signed by a different key
    let st = canonicalize(&a.stmt("intent", json!({}))).unwrap();
    bad.push(("registered kid, wrong key".into(), vc_cose::sign1(&b.key, b"agent-a", "t", &st)));
    // A's key labelled with B's kid
    bad.push(("kid swap".into(), vc_cose::sign1(&a.key, b"agent-b", "t", &st)));
    // empty kid / kid prefix / kid with trailing byte
    for kid in [&b""[..], b"agent-", b"agent-a\0", b"AGENT-A"] {
        bad.push((format!("kid {kid:?}"), vc_cose::sign1(&a.key, kid, "t", &st)));
    }
    // agent.id variants signed by the registered key of agent-a
    for id in ["agent-a ", "Agent-a", "agent-\u{0430}", " agent-a", "agent-a\u{200b}", ""] {
        let mut v = a.stmt("intent", json!({}));
        v["agent"]["id"] = json!(id);
        bad.push((format!("agent.id {id:?}"), a.raw(&canonicalize(&v).unwrap())));
    }
    // agent.id missing / not a string
    for aid in [json!(null), json!(7), json!({"id":"agent-a"}), json!(["agent-a"])] {
        let mut v = a.stmt("intent", json!({}));
        v["agent"]["id"] = aid.clone();
        bad.push((format!("agent.id = {aid}"), a.raw(&canonicalize(&v).unwrap())));
    }
    // B hijacks A's chain: right agent id, B's key
    let mut a2 = a.clone();
    let (a0, _) = a2.next("intent", json!({}));
    s.submit(&a0).unwrap();
    let hijack = a2.stmt("intent", json!({}));
    bad.push(("B signs A's next statement".into(), vc_cose::sign1(&b.key, b"agent-b", "t", &canonicalize(&hijack).unwrap())));
    bad.push(("B signs A's next statement under kid A".into(), vc_cose::sign1(&b.key, b"agent-a", "t", &canonicalize(&hijack).unwrap())));

    let size = s.size();
    for (what, cose) in &bad {
        let r = s.submit(cose);
        assert!(r.is_err(), "ACCEPTED: {what}");
    }
    assert_eq!(s.size(), size, "a rejected submission changed the log");
    // the legitimate next statement still works afterwards (no state poisoned by rejections)
    let (a1, _) = a2.next("intent", json!({}));
    assert_eq!(s.submit(&a1).unwrap().index, 1);
}

/// FINDING SQ-1. `completeness` is only checked when it is a JSON string; any other JSON type skips the ceiling
/// check and the statement is registered (schema says enum C0..C3).
#[test]
fn bug_completeness_of_non_string_type_bypasses_ceiling() {
    let a = Ag::new(1, "agent-a");
    let variants = [json!(3), json!(99), json!(true), json!(null), json!(["C3"]), json!({"level":"C3"}), json!("C3"), json!("c3"), json!("C9"), json!("")];
    let mut accepted = Vec::new();
    for (i, c) in variants.iter().enumerate() {
        let mut s = open(MemStore::new(), &[&a], 0); // issuer ceiling C0
        let mut ag = a.clone();
        let v = ag.stmt("intent", json!({"completeness": c}));
        if s.submit(&ag.sign(&v)).is_ok() {
            accepted.push(format!("variant {i}: completeness={c}"));
        }
    }
    assert!(accepted.is_empty(), "ceiling C0 bypassed: {accepted:?}");
}

/// FINDING SQ-2. `assurance` (A0..A3) is never checked at all: schema-invalid values are registered.
#[test]
fn bug_assurance_schema_invalid_values_accepted() {
    let a = Ag::new(1, "agent-a");
    let invalid = [json!(["A9"]), json!("A3"), json!([1]), json!(["bogus"]), json!({"x":1}), json!(3), json!([["A3"]])];
    let mut accepted = Vec::new();
    for (i, c) in invalid.iter().enumerate() {
        let mut s = open(MemStore::new(), &[&a], 3);
        let mut ag = a.clone();
        let v = ag.stmt("intent", json!({"assurance": c}));
        if s.submit(&ag.sign(&v)).is_ok() {
            accepted.push(format!("variant {i}: assurance={c}"));
        }
    }
    assert!(accepted.is_empty(), "schema-invalid assurance accepted: {accepted:?}");
}

/// FINDING SQ-2. ARC42 §5.2 rule 6: completeness AND assurance are checked against the registered issuer's allowed
/// maximum. `Issuer` has no assurance ceiling; an issuer with the lowest completeness ceiling can assert A3
/// (zero-knowledge proof of the gate) and the sequencer registers it unchecked.
/// NOTE for the fixer: this test builds `Issuer` via `issuer()` above; add the new ceiling there (lowest = A0).
#[test]
fn bug_assurance_ceiling_is_not_enforced() {
    let a = Ag::new(1, "agent-a");
    let mut s = open(MemStore::new(), &[&a], 0);
    let mut ag = a.clone();
    let v = ag.stmt("intent", json!({"assurance": ["A3"]}));
    assert!(s.submit(&ag.sign(&v)).is_err(), "issuer registered with the lowest ceiling asserted A3 and was accepted");
}

/// FINDING SQ-3 (Low). An issuer registered with `max_completeness` outside 0..=3 turns the ceiling into a no-op for
/// unknown values: with 255, "C9" (not a level) is accepted.
#[test]
fn bug_out_of_range_issuer_ceiling_accepts_unknown_levels() {
    let a = Ag::new(1, "agent-a");
    let mut s = open(MemStore::new(), &[&a], 255);
    let mut ag = a.clone();
    let v = ag.stmt("intent", json!({"completeness": "C9"}));
    assert!(s.submit(&ag.sign(&v)).is_err(), "unknown completeness level C9 registered");
}

#[test]
fn holds_completeness_ceiling_exact() {
    let a = Ag::new(1, "agent-a");
    for (max, ok, no) in [(0u8, "C0", "C1"), (1, "C1", "C2"), (2, "C2", "C3")] {
        let mut s = open(MemStore::new(), &[&a], max);
        let mut ag = a.clone();
        let v = ag.stmt("intent", json!({"completeness": no}));
        assert!(matches!(s.submit(&ag.sign(&v)), Err(SeqError::Completeness { .. })));
        let mut ag = a.clone();
        let v = ag.stmt("intent", json!({"completeness": ok}));
        assert!(s.submit(&ag.sign(&v)).is_ok());
    }
}

// ------------------------------------------------------------------------------------------------------------
// 5b. seq/prev, intent/outcome, duplicates
// ------------------------------------------------------------------------------------------------------------

#[test]
fn holds_seq_prev_and_pairing_rules() {
    let a = Ag::new(1, "agent-a");
    let b = Ag::new(2, "agent-b");
    let mut s = open(MemStore::new(), &[&a, &b], 3);
    let (mut ag, mut bg) = (a.clone(), b.clone());
    let (i0, id0) = ag.next("intent", json!({}));
    s.submit(&i0).unwrap();

    // replay of the first statement with a different envelope (content type) -> not idempotent, must fail
    let replay = vc_cose::sign1(&a.key, b"agent-a", "other/type", &canonicalize(&a.stmt("intent", json!({}))).unwrap());
    assert!(matches!(s.submit(&replay), Err(SeqError::Chain(ChainError::SeqGap(..)))));

    // fork: second statement at seq 1 with wrong prev, and a competing pair at the same seq
    let mut wrong = ag.stmt("intent", json!({}));
    wrong["prev"] = json!("11".repeat(32));
    assert!(matches!(s.submit(&ag.raw(&canonicalize(&wrong).unwrap())), Err(SeqError::Chain(ChainError::BadPrev(_)))));
    let f1 = ag.stmt("intent", json!({"ext":{"fork":1}}));
    let f2 = ag.stmt("intent", json!({"ext":{"fork":2}}));
    let (c1, c2) = (ag.raw(&canonicalize(&f1).unwrap()), ag.raw(&canonicalize(&f2).unwrap()));
    let res = s.submit_batch(&[&c1, &c2]);
    assert!(res[0].is_ok() && matches!(res[1], Err(SeqError::Chain(ChainError::SeqGap(..)))), "fork in one batch: {res:?}");
    assert!(matches!(s.submit(&c2), Err(SeqError::Chain(_))), "fork across calls");
    ag.seq += 1;
    ag.prev = statement_id(&f1).unwrap();

    // B outcome for A's intent -> agent mismatch
    let ob = bg.stmt("outcome", json!({"pair":{"intent_id": id0}}));
    assert!(matches!(s.submit(&bg.raw(&canonicalize(&ob).unwrap())), Err(SeqError::Chain(ChainError::AgentMismatch(_)))));
    // A outcome referencing a non-intent id (id of A's own seq-1 statement is an intent too; use an outcome id later)
    let (o0, oid0) = ag.next("outcome", json!({"pair":{"intent_id": id0}}));
    s.submit(&o0).unwrap();
    // second outcome for the same intent
    let dup = ag.stmt("outcome", json!({"pair":{"intent_id": id0}}));
    assert!(matches!(s.submit(&ag.raw(&canonicalize(&dup).unwrap())), Err(SeqError::Chain(ChainError::DuplicateOutcome(_)))));
    // outcome pointing at an OUTCOME id
    let bad = ag.stmt("outcome", json!({"pair":{"intent_id": oid0}}));
    assert!(matches!(s.submit(&ag.raw(&canonicalize(&bad).unwrap())), Err(SeqError::Chain(ChainError::UnknownIntent(_)))));
    // outcome before its intent inside one batch
    let mut cg = ag.clone();
    let intent = cg.stmt("intent", json!({}));
    let intent_id = statement_id(&intent).unwrap();
    let mut early = cg.clone();
    let intent_b = cg.sign(&intent);
    early.seq += 1; // outcome is seq+1 but is submitted first
    early.prev = intent_id.clone();
    let outc = early.stmt("outcome", json!({"pair":{"intent_id": intent_id}}));
    let outc_b = early.raw(&canonicalize(&outc).unwrap());
    let r = s.submit_batch(&[&outc_b, &intent_b]);
    assert!(r[0].is_err() && r[1].is_err(), "out-of-order batch: {r:?}"); // intent_b has seq now stale? it is seq 3 == expected
}

#[test]
fn bug_duplicate_inside_one_batch_is_not_idempotent() {
    let a = Ag::new(1, "agent-a");
    let mut s = open(MemStore::new(), &[&a], 3);
    let mut ag = a.clone();
    let (b0, _) = ag.next("intent", json!({}));
    let r = s.submit_batch(&[&b0, &b0]);
    assert!(r[0].is_ok());
    // API contract: "POST /entries ... idempotent". The same statement twice in one request must not be an error.
    assert!(r[1].is_ok(), "second copy in the same batch: {:?}", r[1]);
    assert_eq!(s.size(), 1);
}

// ------------------------------------------------------------------------------------------------------------
// 5c. Size limits
// ------------------------------------------------------------------------------------------------------------

fn exact_len(a: &Ag, total: usize) -> Vec<u8> {
    let mut n = total - 300;
    for _ in 0..8 {
        let v = a.stmt("intent", json!({"ext":{"pad":"x".repeat(n)}}));
        let b = a.raw(&canonicalize(&v).unwrap());
        if b.len() == total {
            return b;
        }
        n = (n as isize + total as isize - b.len() as isize) as usize;
    }
    panic!("could not hit {total}")
}

#[test]
fn bug_statement_limit_exceeds_tile_entry_limit() {
    // vc-store refuses entries > 65535 bytes (u16 length prefix) but the sequencer admits up to 65536.
    assert!(MAX_STATEMENT_BYTES <= u16::MAX as usize, "MAX_STATEMENT_BYTES={MAX_STATEMENT_BYTES} > 65535");
}

/// FINDING SQ-4. A validly signed 65,536-byte statement passes the sequencer limit, fails inside the store, and the
/// whole batch (including unrelated agents' valid statements) is reported as failed.
#[test]
fn bug_one_oversize_by_one_statement_poisons_the_whole_batch() {
    let a = Ag::new(1, "agent-a");
    let b = Ag::new(2, "agent-b");
    let c = Ag::new(3, "agent-c");
    let mut s = open(MemStore::new(), &[&a, &b, &c], 3);
    let (mut ag, mut cg) = (a.clone(), c.clone());
    let (ga, _) = ag.next("intent", json!({}));
    let big = exact_len(&b, MAX_STATEMENT_BYTES);
    assert_eq!(big.len(), 65536);
    let (gc, _) = cg.next("intent", json!({}));
    let r = s.submit_batch(&[&ga, &big, &gc]);
    assert!(r[1].is_err(), "oversize accepted?");
    assert!(r[0].is_ok() && r[2].is_ok(), "valid statements of other agents were failed: {:?}", r.iter().map(|x| x.is_ok()).collect::<Vec<_>>());
}

// ------------------------------------------------------------------------------------------------------------
// 5d. Durable-append failure: chain state must equal the log
// ------------------------------------------------------------------------------------------------------------

struct Scenario {
    base: Vec<Vec<u8>>,
    batch: Vec<Vec<u8>>,
}

fn scenario(base_n: usize) -> (Scenario, Ag, Ag) {
    let a = Ag::new(1, "agent-a");
    let b = Ag::new(2, "agent-b");
    let (mut ag, mut bg) = (a.clone(), b.clone());
    let base: Vec<Vec<u8>> = (0..base_n).map(|_| ag.next("intent", json!({})).0).collect();
    // batch: A intent, A outcome for it, B intent, A intent
    let (i, iid) = ag.next("intent", json!({}));
    let (o, _) = ag.next("outcome", json!({"pair":{"intent_id": iid}}));
    let (bi, _) = bg.next("intent", json!({}));
    let (i2, _) = ag.next("intent", json!({}));
    (Scenario { base, batch: vec![i, o, bi, i2] }, a, b)
}

fn puts(sh: &Shared) -> u64 {
    sh.puts.load(Ordering::SeqCst)
}

fn failure_sweep(base_n: usize) -> Vec<String> {
    let (sc, a, b) = scenario(base_n);
    let sh = Shared::new();
    {
        let mut s = open(sh.clone(), &[&a, &b], 3);
        let refs: Vec<&[u8]> = sc.base.iter().map(|x| x.as_slice()).collect();
        for c in refs.chunks(100) {
            assert!(s.submit_batch(c).iter().all(|r| r.is_ok()));
        }
    }
    let snap = sh.snapshot();
    let brefs: Vec<&[u8]> = sc.batch.iter().map(|x| x.as_slice()).collect();
    let before = puts(&sh);
    {
        let mut s = open(sh.clone(), &[&a, &b], 3);
        assert!(s.submit_batch(&brefs).iter().all(|r| r.is_ok()));
    }
    let needed = puts(&sh) - before;
    let all: Vec<Vec<u8>> = sc.base.iter().chain(sc.batch.iter()).cloned().collect();
    let mut v = Vec::new();
    for k in 0..needed {
        sh.restore(&snap);
        sh.unlimited();
        let mut s = open(sh.clone(), &[&a, &b], 3);
        sh.fail_after(k as i64);
        let res = catch_unwind(AssertUnwindSafe(|| s.submit_batch(&brefs)));
        sh.unlimited();
        let ctx = format!("base={base_n} fail_after={k}");
        let Ok(res) = res else {
            v.push(format!("{ctx}: submit_batch PANICKED"));
            continue;
        };
        if res.iter().all(|r| r.is_ok()) {
            continue;
        }
        let durable = open(sh.clone(), &[&a, &b], 3);
        // (a) the live sequencer must describe exactly what the log holds
        if s.size() != durable.size() {
            v.push(format!("{ctx}: live size {} != durable {}", s.size(), durable.size()));
        }
        if s.open_intents() != durable.open_intents() {
            v.push(format!("{ctx}: live open_intents {} != durable {}", s.open_intents().len(), durable.open_intents().len()));
        }
        // (b) retrying the identical batch must now succeed (idempotent) and agree with the store
        let retry = catch_unwind(AssertUnwindSafe(|| s.submit_batch(&brefs)));
        match retry {
            Err(_) => v.push(format!("{ctx}: retry PANICKED")),
            Ok(r) => {
                let ok = r.iter().filter(|x| x.is_ok()).count();
                if ok != brefs.len() {
                    v.push(format!("{ctx}: retry accepted only {ok}/{} (first err: {:?})", brefs.len(), r.iter().find_map(|x| x.as_ref().err().map(|e| e.to_string()))));
                    continue;
                }
                let fresh = open(sh.clone(), &[&a, &b], 3);
                let want = vc_log::root(&leaves_of(&all));
                if s.size() as usize != all.len() || fresh.size() as usize != all.len() {
                    v.push(format!("{ctx}: after retry sizes live={} fresh={} want={}", s.size(), fresh.size(), all.len()));
                } else if fresh.checkpoint().unwrap().0.root != want || s.checkpoint().map(|c| c.0.root).ok() != Some(want) {
                    v.push(format!("{ctx}: after retry root != reference"));
                }
            }
        }
    }
    v
}

/// FINDING ST-1 / SQ-5. When the durable append fails, results are errors and the sequencer must roll back to what
/// the log holds. Because TileLog advances `size` before the writes succeed, `replay()` rebuilds chain state from
/// entries that are not durable (or fails and the error is discarded), so the chain state diverges and the same
/// batch can no longer be retried.
#[test]
fn bug_failed_append_rolls_chain_state_back_to_the_log() {
    let mut all = Vec::new();
    for base in [0usize, 254] {
        all.extend(failure_sweep(base));
    }
    assert!(all.is_empty(), "{} violations, first: {:#?}", all.len(), &all[..all.len().min(8)]);
}

// ------------------------------------------------------------------------------------------------------------
// 5e. Replay on open / storage tampering / rollback
// ------------------------------------------------------------------------------------------------------------

fn find(hay: &[u8], needle: &[u8]) -> usize {
    hay.windows(needle.len()).position(|w| w == needle).expect("needle")
}

/// FINDING ST-2 / SQ-6. `Sequencer::open` replays entries with `peek_payload` (no signature check) and never
/// compares entries against the hash tiles, so a modified stored statement (signature now invalid, leaf hash no
/// longer matching the tile) is accepted at open and the next checkpoint is signed over the unmodified tiles.
#[test]
fn bug_open_accepts_modified_stored_statement() {
    let a = Ag::new(1, "agent-a");
    let sh = Shared::new();
    let mut ag = a.clone();
    {
        let mut s = open(sh.clone(), &[&a], 3);
        for _ in 0..3 {
            let (c, _) = ag.next("intent", json!({}));
            s.submit(&c).unwrap();
        }
    }
    let path = "tile/entries/000.p/3";
    let mut b = sh.raw_get(path).unwrap();
    let at = find(&b, b"2026-10-24"); // first statement's ts: same-length edit keeps CBOR framing valid
    b[at + 3] = b'7';
    sh.raw_put(path, &b);
    match Sequencer::open(sh.clone(), registry(&[&a], 3), op_key(), "op", ORIGIN) {
        Err(_) => {}
        Ok(s) => {
            // acceptable alternative: it refuses to sign
            assert!(s.checkpoint().is_err(), "modified log entry 0 was accepted at open and a checkpoint was signed");
        }
    }
}

/// FINDING SQ-6. The sequencer keeps no durable high-water mark: if the newest bundle/tiles vanish (lost write,
/// storage rollback, tampering) `open` silently reports a smaller log, dropping statements that were acknowledged,
/// and would later sign a different root for an already-issued size.
#[test]
fn bug_acknowledged_statements_silently_dropped_on_reopen() {
    let a = Ag::new(1, "agent-a");
    let sh = Shared::new();
    let mut ag = a.clone();
    {
        let mut s = open(sh.clone(), &[&a], 3);
        for _ in 0..10 {
            let (c, _) = ag.next("intent", json!({}));
            s.submit(&c).unwrap(); // acknowledged one by one
        }
    }
    for p in sh.paths() {
        if p.ends_with(".p/10") {
            sh.raw_remove(&p);
        }
    }
    match Sequencer::open(sh.clone(), registry(&[&a], 3), op_key(), "op", ORIGIN) {
        Err(_) => {}
        Ok(s) => assert_eq!(s.size(), 10, "10 acknowledged statements, reopened with {} and no error", s.size()),
    }
}

#[test]
fn holds_restart_replays_chain_and_pairing() {
    let a = Ag::new(1, "agent-a");
    let sh = Shared::new();
    let mut ag = a.clone();
    let (i0, id0) = ag.next("intent", json!({}));
    let (i1, _) = ag.next("intent", json!({}));
    {
        let mut s = open(sh.clone(), &[&a], 3);
        s.submit(&i0).unwrap();
        s.submit(&i1).unwrap();
    }
    let mut s = open(sh.clone(), &[&a], 3);
    assert_eq!(s.open_intents().len(), 2);
    let (o0, _) = ag.next("outcome", json!({"pair":{"intent_id": id0}}));
    assert_eq!(s.submit(&o0).unwrap().index, 2);
    assert_eq!(s.open_intents().len(), 1);
    // replaying an old statement after restart is idempotent, a gap is refused
    assert!(s.submit(&i0).unwrap().duplicate);
    let mut gap = ag.clone();
    gap.seq += 5;
    let (g, _) = gap.next("intent", json!({}));
    assert!(s.submit(&g).is_err());
}

// ------------------------------------------------------------------------------------------------------------
// 6. Checkpoint
// ------------------------------------------------------------------------------------------------------------

#[test]
fn holds_checkpoint_signature_size_root_origin() {
    let a = Ag::new(1, "agent-a");
    let mut s = open(MemStore::new(), &[&a], 3);
    let opk = KeyInfo { name: "op".into(), key: op_key().verifying_key() };
    // empty log
    let (cp, note) = s.checkpoint().unwrap();
    assert_eq!((cp.size, cp.root), (0, sha256(b"")));
    assert_eq!(verify_note_for_origin(&note, ORIGIN, &opk, &[], 0).unwrap(), cp);
    let mut ag = a.clone();
    let mut all = Vec::new();
    let mut prev: Option<(u64, Hash)> = None;
    for round in 0..4 {
        let cs: Vec<Vec<u8>> = (0..(round * 130 + 1)).map(|_| ag.next("intent", json!({})).0).collect();
        let refs: Vec<&[u8]> = cs.iter().map(|c| c.as_slice()).collect();
        assert!(s.submit_batch(&refs).iter().all(|r| r.is_ok()));
        all.extend(cs);
        let (cp, note) = s.checkpoint().unwrap();
        assert_eq!(cp.size as usize, all.len());
        assert_eq!(cp.root, vc_log::root(&leaves_of(&all)), "root != independent computation over submitted statements");
        assert_eq!(verify_note_for_origin(&note, ORIGIN, &opk, &[], 0).unwrap(), cp);
        assert_eq!(verify_note_for_origin(&note, "other.example/log", &opk, &[], 0), Err(CheckpointError::WrongOrigin));
        let wrong = KeyInfo { name: "op".into(), key: SigningKey::from_bytes(&[7; 32]).verifying_key() };
        assert!(verify_note_for_origin(&note, ORIGIN, &wrong, &[], 0).is_err());
        let wrong_name = KeyInfo { name: "op2".into(), key: op_key().verifying_key() };
        assert!(verify_note_for_origin(&note, ORIGIN, &wrong_name, &[], 0).is_err());
        if let Some((m, mroot)) = prev {
            let p = s.log().consistency_proof(m, cp.size).unwrap();
            vc_log::verify_consistency(m as usize, cp.size as usize, &mroot, &cp.root, &p).unwrap();
        }
        // tampered note text must not verify
        let forged = note.replacen(&format!("\n{}\n", cp.size), &format!("\n{}\n", cp.size + 1), 1);
        assert!(verify_note_for_origin(&forged, ORIGIN, &opk, &[], 0).is_err());
        prev = Some((cp.size, cp.root));
    }
}

/// FINDING SQ-7 (Low). Origin / operator name are not validated at open: an empty origin (or one the note format
/// cannot carry) yields a "checkpoint" that no verifier can parse, silently.
#[test]
fn bug_unusable_origin_or_operator_name_is_not_rejected() {
    let a = Ag::new(1, "agent-a");
    let opk = KeyInfo { name: "op".into(), key: op_key().verifying_key() };
    for origin in ["", "has\nnewline", " "] {
        match Sequencer::open(MemStore::new(), registry(&[&a], 3), op_key(), "op", origin) {
            Err(_) => {}
            Ok(s) => match s.checkpoint() {
                Err(_) => {}
                Ok((_, note)) => assert!(
                    verify_note_for_origin(&note, origin, &opk, &[], 0).is_ok(),
                    "origin {origin:?}: sequencer issued a checkpoint note that does not verify"
                ),
            },
        }
    }
    for name in ["", "two words", "plus+sign"] {
        let opk2 = KeyInfo { name: name.into(), key: op_key().verifying_key() };
        match Sequencer::open(MemStore::new(), registry(&[&a], 3), op_key(), name, ORIGIN) {
            Err(_) => {}
            Ok(s) => match s.checkpoint() {
                Err(_) => {}
                Ok((_, note)) => {
                    // a signed-note key name must not contain whitespace or '+'
                    assert!(vc_log::checkpoint::valid_name(name) && verify_note_for_origin(&note, ORIGIN, &opk2, &[], 0).is_ok(), "operator name {name:?} accepted and note issued");
                }
            },
        }
    }
}

// ------------------------------------------------------------------------------------------------------------
// 5f. Throughput / DoS
// ------------------------------------------------------------------------------------------------------------

/// Time a 100k-statement run. Reports numbers; asserts only against pathological (quadratic) behaviour.
#[test]
fn perf_100k_submit_batch_and_reopen() {
    const AGENTS: usize = 500;
    const PER: usize = 200;
    let mut ags: Vec<Ag> = (0..AGENTS)
        .map(|i| {
            let mut k = [0u8; 32];
            k[..8].copy_from_slice(&(i as u64 + 1000).to_le_bytes());
            Ag { key: SigningKey::from_bytes(&k), id: format!("agent-{i}"), seq: 0, prev: ZERO.into() }
        })
        .collect();
    let mut reg = Registry::new();
    for a in &ags {
        reg.register(a.id.as_bytes(), issuer(a, 3));
    }
    let t = std::time::Instant::now();
    let mut stmts: Vec<Vec<u8>> = Vec::with_capacity(AGENTS * PER);
    for _ in 0..PER {
        for a in ags.iter_mut() {
            stmts.push(a.next("intent", json!({"action":{"type":"t","target":"x"}})).0);
        }
    }
    let t_sign = t.elapsed();
    let sh = Shared::new();
    let mut s = Sequencer::open(sh.clone(), reg.clone(), op_key(), "op", ORIGIN).unwrap();
    let t = std::time::Instant::now();
    for chunk in stmts.chunks(1000) {
        let refs: Vec<&[u8]> = chunk.iter().map(|c| c.as_slice()).collect();
        for r in s.submit_batch(&refs) {
            r.unwrap();
        }
    }
    let t_submit = t.elapsed();
    assert_eq!(s.size() as usize, AGENTS * PER);
    let t = std::time::Instant::now();
    let (cp, _) = s.checkpoint().unwrap();
    let t_cp = t.elapsed();
    assert_eq!(cp.root, vc_log::root(&leaves_of(&stmts)));
    let t = std::time::Instant::now();
    let s2 = Sequencer::open(sh.clone(), reg, op_key(), "op", ORIGIN).unwrap();
    let t_open = t.elapsed();
    assert_eq!(s2.size() as usize, AGENTS * PER);
    eprintln!(
        "PERF 100k: build+sign {t_sign:?}; submit_batch(1000) x100 = {t_submit:?} ({:.0} stmt/s); checkpoint {t_cp:?}; reopen+replay {t_open:?}",
        100_000.0 / t_submit.as_secs_f64()
    );
    assert!(t_submit.as_secs() < 300 && t_open.as_secs() < 300);
}
