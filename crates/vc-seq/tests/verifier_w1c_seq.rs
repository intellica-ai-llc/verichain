//! W1c adversarial verifier tests for vc-seq (transactional append, poison/self-heal, ceilings).
//!
//! Conventions: `bug_*` assert the CORRECT behaviour and FAIL against current code (see
//! docs/reports/verifier_w1c_store_seq.md); `holds_*` pass today and are regression tests.
#[path = "../../vc-store/tests/common/mod.rs"]
mod common;

use common::Shared;
use ed25519_dalek::SigningKey;
use serde_json::{json, Value};
use std::io;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use vc_core::canonical::canonicalize;
use vc_core::chain::statement_id;
use vc_seq::*;
use vc_store::TileStore;

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
    fn next(&mut self, kind: &str, extra: Value) -> Vec<u8> {
        let mut v = json!({"v":2,"kind":kind,"agent":{"id":self.id},"seq":self.seq,"prev":self.prev,"ts":TS});
        for (k, x) in extra.as_object().unwrap() {
            v[k] = x.clone();
        }
        let id = statement_id(&v).unwrap();
        let b = vc_cose::sign1(
            &self.key,
            self.id.as_bytes(),
            "application/vc-statement+json",
            &canonicalize(&v).unwrap(),
        );
        self.prev = id;
        self.seq += 1;
        b
    }
}

fn registry(a: &Ag, mc: u8, ma: u8) -> Registry {
    let mut r = Registry::new();
    r.register(
        a.id.as_bytes(),
        Issuer {
            key: a.key.verifying_key(),
            agent_id: a.id.clone(),
            max_completeness: mc,
            max_assurance: ma,
        },
    );
    r
}

fn open<S: TileStore>(s: S, a: &Ag) -> Sequencer<S> {
    Sequencer::open(
        s,
        registry(a, 3, 3),
        SigningKey::from_bytes(&[200; 32]),
        "op",
        ORIGIN,
    )
    .unwrap()
}

/// Store: `put` failures from `Shared`, plus a switch to make an applied put report an error.
#[derive(Clone)]
struct ApplyErr {
    inner: Shared,
    countdown: Arc<AtomicI64>,
}
impl TileStore for ApplyErr {
    fn get(&self, p: &str) -> io::Result<Option<Vec<u8>>> {
        self.inner.get(p)
    }
    fn put(&self, p: &str, d: &[u8]) -> io::Result<()> {
        self.inner.put(p, d)?;
        let c = self.countdown.load(Ordering::SeqCst);
        if c == 0 {
            self.countdown.store(-1, Ordering::SeqCst);
            return Err(io::Error::other("applied but reported failure"));
        }
        if c > 0 {
            self.countdown.store(c - 1, Ordering::SeqCst);
        }
        Ok(())
    }
}

/// While poisoned (rollback replay impossible) `open_intents` must not report an intent that never became durable.
#[test]
fn bug_open_intents_reports_nondurable_intent_while_poisoned() {
    // now a non-durable intent, with the store unable to take puts AND unable to serve reads (rollback replay fails)
    let sh = Shared::new();
    let mut a = Ag::new(1, "agent-a");
    let mut s = open(sh.clone(), &a);
    for _ in 0..300 {
        s.submit(&a.next("policy", json!({}))).unwrap();
    }
    let intent = a.next("intent", json!({}));
    sh.fail_after(0);
    sh.fail_get.store(true, Ordering::SeqCst);
    let r = s.submit(&intent);
    assert!(r.is_err());
    let live = s.open_intents();
    sh.unlimited();
    sh.fail_get.store(false, Ordering::SeqCst);
    let durable = open(sh.clone(), &Ag::new(1, "agent-a"));
    assert!(durable.open_intents().is_empty());
    assert!(
        live.is_empty(),
        "poisoned sequencer reports {} open intent(s) that were never made durable",
        live.len()
    );
}

/// Crash/failure at every put of a multi-statement batch spanning a bundle boundary: after reopening, the sequencer
/// holds exactly the durable prefix (all-or-nothing), a retry of every statement is acknowledged consistently, and no
/// statement acknowledged to a client is missing.
#[test]
fn holds_batch_failure_matrix_all_or_nothing() {
    for base in [0usize, 253, 255, 256] {
        let mut probe_puts = 0;
        for k in 0..12i64 {
            let sh = Shared::new();
            let mut a = Ag::new(1, "agent-a");
            let mut s = open(sh.clone(), &a);
            for _ in 0..base {
                s.submit(&a.next("policy", json!({}))).unwrap();
            }
            let batch_owned: Vec<Vec<u8>> = (0..5).map(|_| a.next("policy", json!({}))).collect();
            let batch: Vec<&[u8]> = batch_owned.iter().map(|b| b.as_slice()).collect();
            let p0 = sh.puts.load(Ordering::SeqCst);
            sh.fail_after(k);
            let rs = s.submit_batch(&batch);
            sh.unlimited();
            let n_ok = rs.iter().filter(|r| r.is_ok()).count();
            assert!(
                n_ok == 0 || n_ok == 5,
                "base={base} k={k}: partial ack {n_ok}"
            );
            if n_ok == 5 {
                probe_puts = (sh.puts.load(Ordering::SeqCst) - p0) as i64;
            }
            assert_eq!(s.size(), (base + n_ok) as u64);
            // live state equals durable state
            let ag0 = Ag::new(1, "agent-a");
            let durable = open(sh.clone(), &ag0);
            assert_eq!(durable.size(), (base + n_ok) as u64, "base={base} k={k}");
            assert_eq!(
                durable.checkpoint().unwrap().0.root,
                s.checkpoint().unwrap().0.root
            );
            // retry everything: acknowledged, duplicates only where durable
            let rs2 = s.submit_batch(&batch);
            for (i, r) in rs2.iter().enumerate() {
                let r = r
                    .as_ref()
                    .unwrap_or_else(|e| panic!("base={base} k={k}: retry {i}: {e}"));
                assert_eq!(r.duplicate, n_ok == 5, "base={base} k={k} i={i}");
                assert_eq!(r.index, (base + i) as u64);
            }
            let again = open(sh.clone(), &ag0);
            assert_eq!(again.size(), (base + 5) as u64);
            if k >= probe_puts && probe_puts > 0 {
                break;
            }
        }
    }
}

/// Indeterminate commit (put applied but returned Err): the client is told "failed"; the sequencer must not
/// acknowledge the retry as a duplicate with an index from non-durable memory and must not fork; after reopen the
/// statement is durable and the retry is a duplicate at its real index.
#[test]
fn holds_indeterminate_commit_no_false_ack_no_fork() {
    for base in [0usize, 255] {
        let inner = Shared::new();
        let st = ApplyErr {
            inner: inner.clone(),
            countdown: Arc::new(AtomicI64::new(-1)),
        };
        let mut a = Ag::new(1, "agent-a");
        let mut s = open(st.clone(), &a);
        for _ in 0..base {
            s.submit(&a.next("policy", json!({}))).unwrap();
        }
        let x = a.next("policy", json!({}));
        let p0 = inner.puts.load(Ordering::SeqCst);
        // find number of puts of a single append by a dry run on a clone
        let dry = Shared::new();
        dry.restore(&inner.snapshot());
        let mut sd = open(dry.clone(), &Ag::new(1, "agent-a"));
        let d0 = dry.puts.load(Ordering::SeqCst);
        sd.submit(&x).unwrap();
        let n = (dry.puts.load(Ordering::SeqCst) - d0) as i64;
        let _ = p0;
        st.countdown.store(n - 1, Ordering::SeqCst);
        assert!(s.submit(&x).is_err());
        assert_eq!(s.size(), base as u64);
        if let Ok(r) = s.submit(&x) {
            assert!(!r.duplicate || r.index == base as u64, "false ack {r:?}");
        }
        // next statement of the chain must never be accepted at a forked index while the commit is unresolved
        let y = a.next("policy", json!({}));
        let _ = s.submit(&y);
        let re = open(inner.clone(), &Ag::new(1, "agent-a")); // must replay cleanly
        assert!(re.size() > base as u64);
        let mut re = re;
        let r = re.submit(&x).unwrap();
        assert!(r.duplicate);
        assert_eq!(r.index, base as u64);
    }
}

/// Ceilings: completeness / assurance rules, incl. odd values.
#[test]
fn holds_ceilings_schema_strict() {
    let a = Ag::new(1, "agent-a");
    let mk = |mc: u8, ma: u8| open_ceiling(&a, mc, ma);
    fn open_ceiling(a: &Ag, mc: u8, ma: u8) -> Sequencer<vc_store::MemStore> {
        Sequencer::open(
            vc_store::MemStore::new(),
            registry(a, mc, ma),
            SigningKey::from_bytes(&[200; 32]),
            "op",
            ORIGIN,
        )
        .unwrap()
    }
    let cases: Vec<(&str, Value, u8, u8, bool)> = vec![
        ("C0 at max0", json!({"completeness":"C0"}), 0, 0, true),
        ("C1 at max0", json!({"completeness":"C1"}), 0, 0, false),
        ("c1 lowercase", json!({"completeness":"c1"}), 3, 3, false),
        ("C4", json!({"completeness":"C4"}), 3, 3, false),
        ("C03", json!({"completeness":"C03"}), 3, 3, false),
        (
            "null completeness",
            json!({"completeness":null}),
            3,
            3,
            false,
        ),
        (
            "number completeness",
            json!({"completeness":1}),
            3,
            3,
            false,
        ),
        (
            "array completeness",
            json!({"completeness":["C1"]}),
            3,
            3,
            false,
        ),
        ("assurance []", json!({"assurance":[]}), 0, 0, true),
        ("assurance A0 at 0", json!({"assurance":["A0"]}), 0, 0, true),
        (
            "assurance A1 at 0",
            json!({"assurance":["A1"]}),
            0,
            0,
            false,
        ),
        (
            "assurance mixed over",
            json!({"assurance":["A0","A3"]}),
            0,
            2,
            false,
        ),
        ("assurance string", json!({"assurance":"A1"}), 3, 3, false),
        ("assurance null", json!({"assurance":null}), 3, 3, false),
        ("assurance [null]", json!({"assurance":[null]}), 3, 3, false),
        ("assurance [A9]", json!({"assurance":["A9"]}), 3, 3, false),
        (
            "assurance obj",
            json!({"assurance":{"A1":true}}),
            3,
            3,
            false,
        ),
        (
            "ceiling 255 + C3",
            json!({"completeness":"C3","assurance":["A3"]}),
            255,
            255,
            true,
        ),
        (
            "ceiling 255 + C4",
            json!({"completeness":"C4"}),
            255,
            255,
            false,
        ),
    ];
    for (name, extra, mc, ma, ok) in cases {
        let mut s = mk(mc, ma);
        let mut ag = a.clone();
        let st = ag.next("policy", extra);
        let r = s.submit(&st);
        assert_eq!(r.is_ok(), ok, "{name}: {r:?}");
        assert_eq!(s.size(), ok as u64, "{name}");
    }
}

/// A failed statement in the middle of a batch, duplicates and in-batch twins: only valid, new statements get
/// appended, results line up, and a failed append rolls everything back consistently.
#[test]
fn holds_mixed_batch_dups_invalid_and_append_failure() {
    let sh = Shared::new();
    let mut a = Ag::new(1, "agent-a");
    let mut s = open(sh.clone(), &a);
    let s0 = a.next("policy", json!({}));
    s.submit(&s0).unwrap();
    let s1 = a.next("policy", json!({}));
    let bad = {
        let mut b = a.clone();
        b.seq = 9;
        b.next("policy", json!({}))
    };
    let s2 = a.next("policy", json!({}));
    let batch: Vec<&[u8]> = vec![&s0, &s1, &bad, &s1, &s2, &s0];
    sh.fail_after(0);
    let rs = s.submit_batch(&batch);
    assert!(rs[0].as_ref().unwrap().duplicate); // durable already
    assert!(rs[1].is_err() && rs[3].is_err() && rs[4].is_err());
    assert!(rs[2].is_err());
    assert!(rs[5].as_ref().unwrap().duplicate);
    sh.unlimited();
    assert_eq!(s.size(), 1);
    let rs = s.submit_batch(&batch);
    assert_eq!(rs[1].as_ref().unwrap().index, 1);
    assert!(!rs[1].as_ref().unwrap().duplicate);
    assert!(rs[3].as_ref().unwrap().duplicate);
    assert_eq!(rs[4].as_ref().unwrap().index, 2);
    assert!(rs[2].is_err());
    assert_eq!(open(sh.clone(), &Ag::new(1, "agent-a")).size(), 3);
}

/// Poison then self-heal: reads fail during the rollback, later succeed; no acknowledgement while poisoned.
#[test]
fn holds_poison_then_self_heal() {
    let sh = Shared::new();
    let mut a = Ag::new(1, "agent-a");
    let mut s = open(sh.clone(), &a);
    for _ in 0..300 {
        s.submit(&a.next("policy", json!({}))).unwrap();
    }
    let x = a.next("policy", json!({}));
    sh.fail_after(0);
    sh.fail_get.store(true, Ordering::SeqCst);
    assert!(s.submit(&x).is_err());
    assert!(matches!(s.submit(&x), Err(SeqError::Poisoned(_))));
    assert!(matches!(s.checkpoint(), Err(SeqError::Poisoned(_))));
    sh.unlimited();
    sh.fail_get.store(false, Ordering::SeqCst);
    let r = s.submit(&x).unwrap();
    assert_eq!((r.index, r.duplicate), (300, false));
    assert_eq!(open(sh.clone(), &Ag::new(1, "agent-a")).size(), 301);
}
