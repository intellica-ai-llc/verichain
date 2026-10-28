//! W1b adversarial verifier tests for vc-seq (duplicate_of, numeric extremes, DoS/resource, registry).
//!
//! Conventions: `bug_*` assert the CORRECT behaviour and FAIL against current code (see
//! docs/reports/verifier_w1b_store_seq.md); `holds_*` pass today and are regression tests; `observe_*` document
//! behaviour and always pass. Run also with `--release`.
#[path = "../../vc-store/tests/common/mod.rs"]
mod common;

use common::Shared;
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde_json::{json, Value};
use std::io;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use vc_core::canonical::canonicalize;
use vc_core::chain::statement_id;
use vc_core::hash::{leaf_hash, Hash};
use vc_seq::*;
use vc_store::{MemStore, TileStore};

const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const ORIGIN: &str = "vc.example/log";
const TS: &str = "2026-10-24T00:00:00Z";

#[derive(Clone)]
struct Ag {
    key: SigningKey,
    id: String,
    seq: u64,
    prev: String,
}

impl Ag {
    fn new(n: u8, id: &str) -> Self {
        Ag {
            key: SigningKey::from_bytes(&[n; 32]),
            id: id.into(),
            seq: 0,
            prev: ZERO.into(),
        }
    }
    fn numbered(n: u64) -> Self {
        let mut k = [7u8; 32];
        k[..8].copy_from_slice(&n.to_le_bytes());
        Ag {
            key: SigningKey::from_bytes(&k),
            id: format!("agent-{n}"),
            seq: 0,
            prev: ZERO.into(),
        }
    }
    fn stmt(&self, kind: &str, extra: Value) -> Value {
        let mut v = json!({"v":2,"kind":kind,"agent":{"id":self.id},"seq":self.seq,"prev":self.prev,"ts":TS});
        for (k, x) in extra.as_object().unwrap() {
            v[k] = x.clone();
        }
        v
    }
    fn raw(&self, payload: &[u8]) -> Vec<u8> {
        vc_cose::sign1(
            &self.key,
            self.id.as_bytes(),
            "application/vc-statement+json",
            payload,
        )
    }
    fn next(&mut self, kind: &str, extra: Value) -> (Vec<u8>, String) {
        let v = self.stmt(kind, extra);
        let id = statement_id(&v).unwrap();
        let b = self.raw(&canonicalize(&v).unwrap());
        self.prev = id.clone();
        self.seq += 1;
        (b, id)
    }
    /// Hand-written canonical payload (keys in JCS order) with raw JSON text for `seq`/`ext`.
    fn text(&self, seq_lit: &str, ext: Option<&str>) -> String {
        let ext = ext.map(|e| format!(r#""ext":{e},"#)).unwrap_or_default();
        format!(
            r#"{{"agent":{{"id":"{}"}},{ext}"kind":"intent","prev":"{}","seq":{seq_lit},"ts":"{TS}","v":2}}"#,
            self.id, self.prev
        )
    }
}

fn issuer(a: &Ag, max: u8) -> Issuer {
    Issuer {
        key: a.key.verifying_key(),
        agent_id: a.id.clone(),
        max_completeness: max,
        max_assurance: max,
    }
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

fn open<S: TileStore>(store: S, agents: &[&Ag], max: u8) -> Sequencer<S> {
    Sequencer::open(store, registry(agents, max), op_key(), "op", ORIGIN).unwrap()
}

fn on_big_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    std::thread::Builder::new()
        .stack_size(2 * 1024 * 1024)
        .spawn(f)
        .unwrap()
        .join()
        .expect("thread panicked")
}

/// Run `f` on a thread; fail (instead of hanging) if it exceeds `limit`.
fn with_timeout<T: Send + 'static>(limit: Duration, f: impl FnOnce() -> T + Send + 'static) -> T {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(f());
    });
    rx.recv_timeout(limit)
        .unwrap_or_else(|_| panic!("timed out after {limit:?}"))
}

#[derive(Clone)]
struct Counting {
    inner: Shared,
    bytes: Arc<AtomicU64>,
}

impl TileStore for Counting {
    fn get(&self, path: &str) -> io::Result<Option<Vec<u8>>> {
        let r = self.inner.get(path)?;
        if let Some(b) = &r {
            self.bytes.fetch_add(b.len() as u64, Ordering::SeqCst);
        }
        Ok(r)
    }
    fn put(&self, path: &str, data: &[u8]) -> io::Result<()> {
        self.inner.put(path, data)
    }
}

// ------------------------------------------------------------------------------------------------------------
// 1. duplicate_of semantics
// ------------------------------------------------------------------------------------------------------------

/// Same statement id (same canonical payload), different COSE bytes: must never be reported as a duplicate, and
/// must never be appended twice (chain forbids it).
#[test]
fn holds_same_payload_different_envelope_is_not_a_duplicate() {
    let a = Ag::new(1, "agent-a");
    let a2 = Ag::new(2, "agent-a"); // second key for the same agent id
    let mut reg = registry(&[&a], 3);
    reg.register(b"agent-a#2", issuer(&a2, 3));
    let mut s = Sequencer::open(MemStore::new(), reg, op_key(), "op", ORIGIN).unwrap();
    let ag = a.clone();
    let v = ag.stmt("intent", json!({}));
    let payload = canonicalize(&v).unwrap();
    let orig = a.raw(&payload);
    assert!(!s.submit(&orig).unwrap().duplicate);
    let twins: Vec<(&str, Vec<u8>)> = vec![
        ("other content type", vc_cose::sign1(&a.key, b"agent-a", "application/other", &payload)),
        ("other key + kid, same agent", vc_cose::sign1(&a2.key, b"agent-a#2", "application/vc-statement+json", &payload)),
        ("non-canonical twin (whitespace)", a.raw(format!(" {}", String::from_utf8(payload.clone()).unwrap()).as_bytes())),
        ("non-canonical twin (key order)", a.raw(br#"{"v":2,"ts":"2026-10-24T00:00:00Z","seq":0,"prev":"0000000000000000000000000000000000000000000000000000000000000000","kind":"intent","agent":{"id":"agent-a"}}"#)),
    ];
    for (name, t) in &twins {
        let r = s.submit(t);
        assert!(r.is_err(), "{name}: accepted as {r:?}");
    }
    assert_eq!(s.size(), 1);
    // and the original is still a duplicate afterwards
    assert!(s.submit(&orig).unwrap().duplicate);
}

/// The duplicate path never consults the registry, ceilings or binding: after reopening with a changed registry
/// (issuer removed / key rotated / ceiling lowered) a retry of a stored statement is still answered `Ok(dup)`.
/// Documented behaviour (idempotent retry of an already-durable fact), not asserted as a defect.
#[test]
fn observe_duplicate_ignores_current_registry() {
    let a = Ag::new(1, "agent-a");
    let sh = Shared::new();
    let mut ag = a.clone();
    let (s0, _) = ag.next("intent", json!({"completeness":"C3"}));
    {
        let mut s = open(sh.clone(), &[&a], 3);
        s.submit(&s0).unwrap();
    }
    let rotated = Ag::new(77, "agent-a");
    let strict = open(sh.clone(), &[&a], 0);
    let gone = open(sh.clone(), &[], 3);
    let rot = open(sh.clone(), &[&rotated], 3);
    for (name, mut s) in [
        ("ceiling lowered to C0", strict),
        ("issuer removed", gone),
        ("key rotated", rot),
    ] {
        let r = s.submit(&s0);
        eprintln!("observe: {name}: retry of stored statement -> {r:?}");
        let (n, _) = ag.clone().next("intent", json!({}));
        eprintln!(
            "observe: {name}: fresh statement from that issuer -> {:?}",
            s.submit(&n).map(|r| r.index).map_err(|e| e.to_string())
        );
    }
}

#[test]
fn holds_statement_ids_do_not_collide_across_agents() {
    let a = Ag::new(1, "agent-a");
    let b = Ag::new(2, "agent-b");
    let mut s = open(MemStore::new(), &[&a, &b], 3);
    let (mut ag, mut bg) = (a.clone(), b.clone());
    let (ca, ia) = ag.next("intent", json!({}));
    let (cb, ib) = bg.next("intent", json!({}));
    assert_ne!(ia, ib);
    let (ra, rb) = (s.submit(&ca).unwrap(), s.submit(&cb).unwrap());
    assert!(!ra.duplicate && !rb.duplicate);
    assert_eq!((ra.index, rb.index), (0, 1));
    // B (or anyone) replaying A's bytes only gets A's own registration back
    let r = s.submit(&ca).unwrap();
    assert!(r.duplicate && r.index == 0 && r.statement_id == ia);
    // B re-signing A's exact payload under B's kid is rejected (agent binding), not treated as a duplicate of A
    let evil = vc_cose::sign1(
        &b.key,
        b"agent-b",
        "application/vc-statement+json",
        &canonicalize(&a.stmt("intent", json!({}))).unwrap(),
    );
    assert!(s.submit(&evil).is_err());
}

/// FINDING SQ-8. After a failed durable append the sequencer answers a retry with `Ok(duplicate: true)` and the
/// original index although nothing was made durable: the client is told "already registered" for a statement that
/// vanishes on restart (violates "never acknowledges before durable write", ARC42 s5.1).
#[test]
fn bug_duplicate_ack_after_failed_append_is_not_durable() {
    let a = Ag::new(1, "agent-a");
    let mut ag = a.clone();
    let (s0, _) = ag.next("intent", json!({}));
    let mut violations = Vec::new();
    for k in 0..6i64 {
        let sh = Shared::new();
        let mut s = open(sh.clone(), &[&a], 3);
        sh.fail_after(k);
        let first = s.submit(&s0);
        sh.unlimited();
        if first.is_ok() {
            break; // k puts were enough: no failure to examine
        }
        let retry = s.submit(&s0);
        let durable = open(sh.clone(), &[&a], 3).size();
        if let Ok(r) = &retry {
            if durable < r.index + 1 {
                violations.push(format!("fail_after={k}: retry acknowledged index {} (duplicate={}) but a reopened log has size {durable}", r.index, r.duplicate));
            }
        }
    }
    assert!(violations.is_empty(), "{violations:#?}");
}

/// FINDING SQ-9. `submit_batch` discards the result of the rollback `replay()`. If the replay itself fails
/// (transient read error), chain state and `by_id` stay truncated to the point of failure. The sequencer then
/// forgets agents it already holds: it re-accepts an already-registered statement as NEW (second seq 0 for the
/// agent) and refuses the real next one; the log is corrupted so that a later `open` fails.
#[test]
fn bug_failed_rollback_replay_lets_log_fork_the_chain() {
    let a = Ag::new(1, "agent-a");
    let mut ag = a.clone();
    let sh = Shared::new();
    let all: Vec<Vec<u8>> = (0..300).map(|_| ag.next("intent", json!({})).0).collect();
    let (extra, _) = ag.next("intent", json!({}));
    let mut s = open(sh.clone(), &[&a], 3);
    for c in all.chunks(100) {
        let r: Vec<&[u8]> = c.iter().map(|x| x.as_slice()).collect();
        assert!(s.submit_batch(&r).iter().all(|x| x.is_ok()));
    }
    // the durable append fails AND the store cannot be read while the sequencer rolls back
    sh.fail_after(0);
    sh.fail_get.store(true, Ordering::SeqCst);
    assert!(s.submit(&extra).is_err());
    sh.unlimited();
    sh.fail_get.store(false, Ordering::SeqCst);
    // store is healthy again; retrying the same statement must be fine, and replaying an old one a duplicate
    let dup = s.submit(&all[0]);
    let again = s.submit(&extra);
    let reopened = Sequencer::open(sh.clone(), registry(&[&a], 3), op_key(), "op", ORIGIN);
    assert!(
        matches!(&dup, Ok(r) if r.duplicate) && again.is_ok() && reopened.is_ok(),
        "old statement -> {dup:?}; next statement -> {:?}; reopen -> {:?}",
        again.map(|r| r.index).map_err(|e| e.to_string()),
        reopened.map(|s| s.size()).map_err(|e| e.to_string())
    );
}

/// FINDING SQ-10. `checkpoint()` after a failed durable append must either refuse or describe the durable log;
/// it must not panic (the signer would crash). Same root cause as ST-1/SQ-5 (TileLog::edge is `mem::take`n and
/// never restored), listed here as the sequencer-level manifestation on the signing path.
#[test]
fn bug_checkpoint_after_failed_append_must_not_panic_or_sign_non_durable_data() {
    let a = Ag::new(1, "agent-a");
    for base in [0usize, 5, 255] {
        let mut ag = a.clone();
        let sh = Shared::new();
        let mut s = open(sh.clone(), &[&a], 3);
        for _ in 0..base {
            let (c, _) = ag.next("intent", json!({}));
            s.submit(&c).unwrap();
        }
        let (n1, _) = ag.next("intent", json!({}));
        let (n2, _) = ag.next("intent", json!({}));
        for k in 0..8i64 {
            let snap = sh.snapshot();
            let mut s2 = open(sh.clone(), &[&a], 3);
            sh.fail_after(k);
            let r = s2.submit_batch(&[&n1, &n2]);
            sh.unlimited();
            if r.iter().all(|x| x.is_ok()) {
                sh.restore(&snap);
                break;
            }
            let cp = catch_unwind(AssertUnwindSafe(|| s2.checkpoint())).unwrap_or_else(|_| {
                panic!("base={base} k={k}: checkpoint() PANICKED after a failed append")
            });
            if let Ok((cp, _)) = cp {
                let d = open(sh.clone(), &[&a], 3);
                let dcp = d.checkpoint().unwrap().0;
                assert!(
                    cp.size <= dcp.size,
                    "base={base} k={k}: signed size {} > durable {}",
                    cp.size,
                    dcp.size
                );
                if cp.size == dcp.size {
                    assert_eq!(
                        cp.root, dcp.root,
                        "base={base} k={k}: root differs from durable log"
                    );
                }
            }
            sh.restore(&snap);
        }
    }
}

#[test]
fn holds_empty_inputs() {
    let a = Ag::new(1, "agent-a");
    let sh = Shared::new();
    let mut s = open(sh.clone(), &[&a], 3);
    let before = sh.puts.load(Ordering::SeqCst);
    assert!(s.submit_batch(&[]).is_empty());
    assert!(s.submit(&[]).is_err());
    assert!(s.submit(&[0xd2]).is_err());
    assert_eq!(sh.puts.load(Ordering::SeqCst), before);
    assert_eq!(s.size(), 0);
}

// ------------------------------------------------------------------------------------------------------------
// 2. Numeric extremes in statements
// ------------------------------------------------------------------------------------------------------------

fn seq_cases(first: bool) -> Vec<&'static str> {
    let mut v = vec![
        "-0",
        "0.0",
        "0e0",
        "0E0",
        "00",
        "+0",
        "0.",
        "-1",
        "2",
        "9007199254740991",
        "9007199254740992",
        "9007199254740993",
        "18446744073709551615",
        "18446744073709551616",
        "-9223372036854775808",
        "1e999",
        "1e19",
        "1.8446744073709552e19",
        "0.5",
        "null",
        "\"0\"",
        "true",
        "[0]",
        "{}",
        "1e0",
        "1.0",
        "+1",
        "01",
        "0.1e1",
        "10e-1",
        "1.00000000000000000001",
        "4294967296",
        "4294967295",
        "2147483648",
        "9223372036854775807",
        "9223372036854775808",
    ];
    v.push(if first { "1" } else { "0" }); // wrong-by-one for this position
    v
}

/// seq in every non-canonical / out-of-range numeric spelling must be refused (never coerced to the expected value),
/// without panics, and without changing the log.
#[test]
fn holds_seq_numeric_extremes_rejected_without_panic() {
    let a = Ag::new(1, "agent-a");
    let mut s = open(MemStore::new(), &[&a], 3);
    let mut ag = a.clone();
    for (round, lit) in ["0", "1"].iter().enumerate() {
        for bad in seq_cases(round == 0) {
            let c = ag.raw(ag.text(bad, None).as_bytes());
            let r = catch_unwind(AssertUnwindSafe(|| s.submit(&c)));
            let r = r.unwrap_or_else(|_| panic!("PANIC on seq literal {bad}"));
            assert!(
                r.is_err(),
                "seq literal `{bad}` accepted at position {round}: {r:?}"
            );
        }
        assert_eq!(s.size() as usize, round);
        let good = ag.raw(ag.text(lit, None).as_bytes());
        let reg = s.submit(&good).unwrap();
        assert_eq!(reg.index as usize, round);
        ag.seq += 1;
        ag.prev = reg.statement_id;
    }
    assert_eq!(s.size(), 2);
}

/// `v` and other numeric schema constants in alternative spellings.
#[test]
fn holds_version_numeric_spellings_rejected() {
    let a = Ag::new(1, "agent-a");
    let mut s = open(MemStore::new(), &[&a], 3);
    let ag = a.clone();
    for lit in [
        "2.0",
        "2e0",
        "02",
        "+2",
        "2.00",
        "0.2e1",
        "3",
        "1",
        "0",
        "-2",
        "18446744073709551618",
        "1e999",
    ] {
        let t = ag
            .text("0", None)
            .replace(r#""v":2"#, &format!(r#""v":{lit}"#));
        assert!(s.submit(&ag.raw(t.as_bytes())).is_err(), "v={lit} accepted");
    }
    assert_eq!(s.size(), 0);
}

/// Huge numbers in free-form fields: any accepted statement must survive a reopen (replay parity), and numbers
/// above 2^53 or in exponent form must be refused rather than silently rewritten.
#[test]
fn holds_numeric_extremes_in_ext_roundtrip_or_refuse() {
    let a = Ag::new(1, "agent-a");
    let sh = Shared::new();
    let mut s = open(sh.clone(), &[&a], 3);
    let mut ag = a.clone();
    let mut accepted = 0usize;
    for lit in [
        "0",
        "-0",
        "1e21",
        "1e-7",
        "0.000001",
        "5e-324",
        "1.7976931348623157e308",
        "9007199254740991",
        "9007199254740992",
        "9007199254740993",
        "-9007199254740993",
        "18446744073709551615",
        "18446744073709551616",
        "1E5",
        "1.5e300",
        "123456789012345678901234567890",
        "0.1",
        "100",
        "1e5",
    ] {
        let c = ag.raw(
            ag.text(&ag.seq.to_string(), Some(&format!("[{lit}]")))
                .as_bytes(),
        );
        let r = s.submit(&c);
        if let Ok(reg) = r {
            accepted += 1;
            // only a literal that is already the canonical ES6 form may be accepted
            let canon = String::from_utf8(
                canonicalize(&serde_json::from_str::<Value>(&format!("[{lit}]")).unwrap()).unwrap(),
            )
            .unwrap();
            assert_eq!(
                canon,
                format!("[{lit}]"),
                "non-canonical number `{lit}` accepted"
            );
            ag.seq += 1;
            ag.prev = reg.statement_id;
        }
    }
    assert!(accepted >= 3);
    let size = s.size();
    let again = Sequencer::open(sh.clone(), registry(&[&a], 3), op_key(), "op", ORIGIN)
        .expect("replay of accepted numeric statements");
    assert_eq!(again.size(), size);
}

// ------------------------------------------------------------------------------------------------------------
// 3. DoS / resource
// ------------------------------------------------------------------------------------------------------------

/// FINDING SQ-11. `duplicate_of` runs BEFORE the size check and before any signature check: an unauthenticated
/// caller can make the sequencer CBOR-decode, UTF-8-validate, JCS-canonicalise (and hash) an arbitrarily large payload
/// before it gets `TooLarge`. An 8 MB payload from an unregistered key should be refused by length alone.
#[test]
fn bug_oversize_unauthenticated_statement_is_refused_before_being_parsed() {
    let stranger = Ag::new(9, "agent-x");
    let mut body = String::with_capacity(8 << 20);
    body.push('[');
    for _ in 0..(4 << 20) {
        body.push_str("1,");
    }
    body.push_str("1]");
    let payload = format!(
        r#"{{"agent":{{"id":"agent-x"}},"ext":{body},"kind":"intent","prev":"{ZERO}","seq":0,"ts":"{TS}","v":2}}"#
    );
    let cose = stranger.raw(payload.as_bytes());
    drop(payload);
    drop(body);
    assert!(cose.len() > 8 << 20);
    let mut s = open(MemStore::new(), &[], 3);
    let t = Instant::now();
    let r = s.submit(&cose);
    let el = t.elapsed();
    eprintln!(
        "SQ-11: {} MB unauthenticated submit -> {r:?} in {el:?}",
        cose.len() >> 20
    );
    assert!(matches!(r, Err(SeqError::TooLarge)));
    assert!(
        el < Duration::from_millis(15),
        "refusal of an oversize statement took {el:?}: it was parsed/canonicalised first"
    );
}

/// Deeply nested JSON inside the size limit (from registered and unregistered signers, as arrays and objects)
/// must neither overflow the stack (2 MB thread) nor panic nor be accepted.
#[test]
fn holds_deep_json_nesting_is_refused_without_stack_overflow() {
    on_big_stack(|| {
        let reg = Ag::new(1, "agent-a");
        let stranger = Ag::new(9, "agent-a"); // same agent id, key not registered
        let mut s = open(MemStore::new(), &[&reg], 3);
        let arr = |d: usize| format!("{}{}", "[".repeat(d), "]".repeat(d));
        let obj = |d: usize| format!("{}1{}", r#"{"a":"#.repeat(d), "}".repeat(d));
        for d in [200usize, 1_000, 10_000, 30_000, 32_000] {
            for (nm, ext) in [("array", arr(d)), ("object", obj(d.min(9_000)))] {
                for who in [&reg, &stranger] {
                    let c = who.raw(reg.text("0", Some(&ext)).as_bytes());
                    if c.len() > 65_536 {
                        continue;
                    }
                    let r = s.submit(&c);
                    assert!(r.is_err(), "depth {d} {nm} accepted");
                }
            }
        }
        assert_eq!(s.size(), 0);
    });
}

/// Whatever nesting depth the sequencer accepts must replay on reopen (validate and replay must agree).
#[test]
fn holds_accepted_nesting_depth_survives_reopen() {
    on_big_stack(|| {
        let a = Ag::new(1, "agent-a");
        let sh = Shared::new();
        let mut s = open(sh.clone(), &[&a], 3);
        let mut ag = a.clone();
        let (mut accepted, mut max_ok, mut first_bad) = (0, 0, None);
        for d in 1..=140usize {
            let ext = format!("{}{}", "[".repeat(d), "]".repeat(d));
            let c = ag.raw(ag.text(&ag.seq.to_string(), Some(&ext)).as_bytes());
            match s.submit(&c) {
                Ok(r) => {
                    assert!(
                        first_bad.is_none(),
                        "depth {d} accepted after depth {first_bad:?} was refused"
                    );
                    accepted += 1;
                    max_ok = d;
                    ag.seq += 1;
                    ag.prev = r.statement_id;
                }
                Err(_) => first_bad = first_bad.or(Some(d)),
            }
        }
        eprintln!("observe: max accepted nesting depth = {max_ok} (+1 for ext itself); first refused = {first_bad:?}");
        assert!(accepted > 10);
        let r = Sequencer::open(sh.clone(), registry(&[&a], 3), op_key(), "op", ORIGIN)
            .expect("replay");
        assert_eq!(r.size() as usize, accepted);
    });
}

/// Pathological-but-valid statements just under the limit: huge kid, huge agent id, many keys, long strings.
#[test]
fn holds_pathological_sizes_under_the_limit() {
    let mut big = Ag::new(1, &"a".repeat(20_000));
    let kid = vec![0x41u8; 40_000];
    let mut reg = Registry::new();
    reg.register(&kid, issuer(&big, 3));
    let sh = Shared::new();
    let mut s = Sequencer::open(sh.clone(), reg.clone(), op_key(), "op", ORIGIN).unwrap();
    let t = Instant::now();
    let payload = canonicalize(&big.stmt("intent", json!({}))).unwrap();
    let c = vc_cose::sign1(&big.key, &kid, "application/vc-statement+json", &payload);
    assert!(c.len() < 65_536, "{}", c.len());
    let r = s.submit(&c).unwrap();
    big.seq += 1;
    big.prev = r.statement_id;
    // many object members
    let members: String = (0..3_000)
        .map(|i| format!(r#""k{i:05}":{i}"#))
        .collect::<Vec<_>>()
        .join(",");
    let ext = format!("{{{members}}}");
    let mut small = Ag::new(2, "agent-b");
    let mut reg2 = reg.clone();
    reg2.register(b"agent-b", issuer(&small, 3));
    let mut s2 = Sequencer::open(MemStore::new(), reg2, op_key(), "op", ORIGIN).unwrap();
    let c2 = small.raw(small.text("0", Some(&ext)).as_bytes());
    assert!(c2.len() < 65_536);
    let r2 = s2.submit(&c2).unwrap();
    small.seq = 1;
    small.prev = r2.statement_id;
    // long string with escapes that must not be re-escaped
    let (c3, _) = small.next(
        "intent",
        json!({"ext":{"s":"\u{00e9}".repeat(20_000) + &"\\\"".repeat(1000)}}),
    );
    s2.submit(&c3).unwrap();
    let el = t.elapsed();
    assert!(
        el < Duration::from_secs(20),
        "pathological statements took {el:?}"
    );
    let again = Sequencer::open(sh, reg, op_key(), "op", ORIGIN).unwrap();
    assert_eq!(again.size(), 1);
}

/// 2000 valid tiny statements from 200 agents in ONE batch, then a restart.
#[test]
#[ignore = "~20s in debug (signing 2000 statements); run with --release --include-ignored"]
fn holds_many_tiny_statements_in_one_batch() {
    let mut ags: Vec<Ag> = (0..200).map(Ag::numbered).collect();
    let mut reg = Registry::new();
    for a in &ags {
        reg.register(a.id.as_bytes(), issuer(a, 3));
    }
    let mut stmts: Vec<Vec<u8>> = Vec::new();
    for _ in 0..10 {
        for a in ags.iter_mut() {
            stmts.push(a.next("intent", json!({})).0);
        }
    }
    let sh = Shared::new();
    let mut s = Sequencer::open(sh.clone(), reg.clone(), op_key(), "op", ORIGIN).unwrap();
    let refs: Vec<&[u8]> = stmts.iter().map(|c| c.as_slice()).collect();
    let t = Instant::now();
    let res = s.submit_batch(&refs);
    assert!(res.iter().all(|r| r.is_ok()));
    assert!(t.elapsed() < Duration::from_secs(60), "{:?}", t.elapsed());
    let want = vc_log::root(&stmts.iter().map(|c| leaf_hash(c)).collect::<Vec<Hash>>());
    assert_eq!(s.checkpoint().unwrap().0.root, want);
    let s2 = Sequencer::open(sh, reg, op_key(), "op", ORIGIN).unwrap();
    assert_eq!(s2.checkpoint().unwrap().0.root, want);
    assert_eq!(s2.open_intents().len(), 2000);
}

/// A batch made only of duplicates of stored statements (and one big batch of the same statement repeated) is
/// answered idempotently, quickly, and writes nothing.
#[test]
fn holds_huge_batch_of_duplicates_writes_nothing() {
    let a = Ag::new(1, "agent-a");
    let mut ag = a.clone();
    let sh = Shared::new();
    let mut s = open(sh.clone(), &[&a], 3);
    let stmts: Vec<Vec<u8>> = (0..200).map(|_| ag.next("intent", json!({})).0).collect();
    let refs: Vec<&[u8]> = stmts.iter().map(|c| c.as_slice()).collect();
    assert!(s.submit_batch(&refs).iter().all(|r| r.is_ok()));
    let puts = sh.puts.load(Ordering::SeqCst);
    let mut dup: Vec<Vec<u8>> = Vec::new();
    for _ in 0..25 {
        dup.extend(stmts.iter().cloned());
    }
    let t = Instant::now();
    let r = with_timeout(Duration::from_secs(120), move || {
        let d: Vec<&[u8]> = dup.iter().map(|c| c.as_slice()).collect();
        let r = s.submit_batch(&d);
        (s, r)
    });
    let (s, r) = r;
    assert!(
        r.iter().all(|x| matches!(x, Ok(g) if g.duplicate)),
        "5000 duplicates"
    );
    assert_eq!(s.size(), 200);
    assert_eq!(
        sh.puts.load(Ordering::SeqCst),
        puts,
        "duplicates must not write"
    );
    assert!(t.elapsed() < Duration::from_secs(60), "{:?}", t.elapsed());
}

fn replay_bytes(n: usize, pad: usize) -> (u64, u64) {
    let a = Ag::new(1, "agent-a");
    let mut ag = a.clone();
    let sh = Shared::new();
    {
        let mut s = open(sh.clone(), &[&a], 3);
        let stmts: Vec<Vec<u8>> = (0..n)
            .map(|_| ag.next("intent", json!({"ext":{"pad":"x".repeat(pad)}})).0)
            .collect();
        for c in stmts.chunks(128) {
            let r: Vec<&[u8]> = c.iter().map(|x| x.as_slice()).collect();
            assert!(s.submit_batch(&r).iter().all(|x| x.is_ok()));
        }
    }
    let stored: u64 = sh
        .paths()
        .iter()
        .filter(|p| p.starts_with("tile/entries/") && !p.contains(".p/"))
        .map(|p| sh.raw_get(p).unwrap().len() as u64)
        .sum::<u64>()
        + sh.paths()
            .iter()
            .filter(|p| p.starts_with("tile/entries/") && p.contains(".p/"))
            .map(|p| sh.raw_get(p).unwrap().len() as u64)
            .max()
            .unwrap_or(0);
    let c = Counting {
        inner: sh,
        bytes: Arc::new(AtomicU64::new(0)),
    };
    let t = Instant::now();
    let s = Sequencer::open(c.clone(), registry(&[&a], 3), op_key(), "op", ORIGIN).unwrap();
    eprintln!("replay n={n} pad={pad}: {:?}", t.elapsed());
    assert_eq!(s.size() as usize, n);
    (c.bytes.load(Ordering::SeqCst), stored)
}

/// FINDING SQ-12. `Sequencer::open` replays with `log.entry(i)` per index; every call fetches and parses the whole
/// bundle (256 entries), so startup reads ~100-250x the log size. With 64 KB statements a 1 GB log is read ~160 GB.
#[test]
fn bug_open_replay_reads_each_bundle_once() {
    let (read, stored) = replay_bytes(300, 8000);
    eprintln!(
        "SQ-12: replay read {read} B for {stored} B stored ({:.0}x)",
        read as f64 / stored as f64
    );
    assert!(
        read <= 4 * stored,
        "replay read {read} B of a {stored} B log ({:.0}x)",
        read as f64 / stored as f64
    );
}

/// Replay I/O is linear (not quadratic) in the number of entries: doubling n doubles the bytes read (x2..x2.6).
#[test]
#[ignore = "~25s in debug (signing 2300 statements); run with --release --include-ignored"]
fn holds_replay_io_is_linear_in_log_size() {
    let (r1, _) = replay_bytes(768, 200);
    let (r2, _) = replay_bytes(1536, 200);
    let ratio = r2 as f64 / r1 as f64;
    eprintln!("observe: replay bytes n=768 -> {r1}, n=1536 -> {r2} (x{ratio:.2})");
    assert!(ratio < 3.2, "super-linear replay I/O: x{ratio:.2}");
}

// ------------------------------------------------------------------------------------------------------------
// 4. Registry semantics
// ------------------------------------------------------------------------------------------------------------

/// `Registry::register` silently replaces an existing kid (including rebinding it to another agent id / key).
/// Returns nothing, so callers cannot detect an accidental or malicious overwrite. Documented.
#[test]
fn observe_register_overwrites_silently() {
    let a = Ag::new(1, "agent-a");
    let b = Ag::new(2, "agent-b");
    let mut r = Registry::new();
    r.register(b"k", issuer(&a, 3));
    r.register(b"k", issuer(&b, 0)); // same kid, different agent, key and ceiling: no error, no signal
    let mut s = Sequencer::open(MemStore::new(), r, op_key(), "op", ORIGIN).unwrap();
    let c = vc_cose::sign1(
        &a.key,
        b"k",
        "t",
        &canonicalize(&a.stmt("intent", json!({}))).unwrap(),
    );
    eprintln!(
        "observe: original owner of kid after overwrite -> {:?}",
        s.submit(&c).map_err(|e| e.to_string())
    );
    let c = vc_cose::sign1(
        &b.key,
        b"k",
        "t",
        &canonicalize(&b.stmt("intent", json!({}))).unwrap(),
    );
    eprintln!(
        "observe: new owner -> {:?}",
        s.submit(&c).map(|r| r.index).map_err(|e| e.to_string())
    );
}

/// Several kids / keys may speak for one agent id and continue the same chain (rotation); also the same public key
/// registered under two agents works independently (binding is per kid).
#[test]
fn holds_multiple_kids_per_agent_share_one_chain_and_cannot_cross() {
    let a1 = Ag::new(1, "agent-a");
    let a2 = Ag::new(2, "agent-a");
    let b = Ag::new(1, "agent-b"); // same key bytes as a1
    let mut r = Registry::new();
    r.register(b"a-old", issuer(&a1, 3));
    r.register(b"a-new", issuer(&a2, 3));
    r.register(b"b", issuer(&b, 3));
    let mut s = Sequencer::open(MemStore::new(), r, op_key(), "op", ORIGIN).unwrap();
    let mut ag = a1.clone();
    let st0 = ag.stmt("intent", json!({}));
    let c0 = vc_cose::sign1(&a1.key, b"a-old", "t", &canonicalize(&st0).unwrap());
    assert_eq!(s.submit(&c0).unwrap().index, 0);
    ag.seq = 1;
    ag.prev = statement_id(&st0).unwrap();
    let st1 = ag.stmt("intent", json!({}));
    let c1 = vc_cose::sign1(&a2.key, b"a-new", "t", &canonicalize(&st1).unwrap());
    assert_eq!(s.submit(&c1).unwrap().index, 1);
    // kid "b" may not speak for agent-a even though its key equals a-old's key
    ag.seq = 2;
    ag.prev = statement_id(&st1).unwrap();
    let st2 = ag.stmt("intent", json!({}));
    let c2 = vc_cose::sign1(&b.key, b"b", "t", &canonicalize(&st2).unwrap());
    assert!(matches!(s.submit(&c2), Err(SeqError::AgentBinding)));
    // old key under the new kid fails the signature
    let c3 = vc_cose::sign1(&a1.key, b"a-new", "t", &canonicalize(&st2).unwrap());
    assert!(s.submit(&c3).is_err());
    assert_eq!(s.size(), 2);
}

/// A registered weak (identity-point) key must not make every signature forgeable.
#[test]
fn holds_registered_weak_key_cannot_forge() {
    let mut ident = [0u8; 32];
    ident[0] = 1; // compressed Edwards identity
    let weak = VerifyingKey::from_bytes(&ident).unwrap();
    let a = Ag::new(1, "agent-a");
    let mut r = Registry::new();
    r.register(
        b"agent-a",
        Issuer {
            key: weak,
            agent_id: "agent-a".into(),
            max_completeness: 3,
            max_assurance: 3,
        },
    );
    let mut s = Sequencer::open(MemStore::new(), r, op_key(), "op", ORIGIN).unwrap();
    let mut c = a.raw(&canonicalize(&a.stmt("intent", json!({}))).unwrap());
    let n = c.len();
    c[n - 64..n - 32].copy_from_slice(&ident); // R = identity
    c[n - 32..].fill(0); // S = 0
    let r = s.submit(&c);
    assert!(
        r.is_err(),
        "forged signature under a registered identity key accepted: {r:?}"
    );
    assert_eq!(s.size(), 0);
}
