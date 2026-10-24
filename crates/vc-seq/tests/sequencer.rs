//! Sequencer tests. Requirement trace: FR-L0-07 (accept only valid in-policy statements; durable before return).
use ed25519_dalek::SigningKey;
use serde_json::{json, Value};
use vc_core::canonical::canonicalize;
use vc_core::chain::statement_id;
use vc_core::hash::sha256;
use vc_log::checkpoint::{verify_note, KeyInfo};
use vc_log::verify_inclusion;
use vc_seq::*;
use vc_store::{FsStore, MemStore};

const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";

struct Agent {
    key: SigningKey,
    id: String,
    seq: u64,
    prev: String,
}

impl Agent {
    fn new(n: u8, id: &str) -> Self {
        Agent {
            key: SigningKey::from_bytes(&[n; 32]),
            id: id.into(),
            seq: 0,
            prev: ZERO.into(),
        }
    }
    fn stmt(&self, kind: &str, extra: Value) -> Value {
        let mut v = json!({"v":2,"kind":kind,"agent":{"id":self.id},"seq":self.seq,"prev":self.prev,"ts":"2026-10-24T00:00:00Z"});
        for (k, x) in extra.as_object().unwrap() {
            v[k] = x.clone();
        }
        v
    }
    /// Sign the current statement and advance the chain locally.
    fn sign(&mut self, v: &Value) -> Vec<u8> {
        let bytes = vc_cose::sign1(
            &self.key,
            self.id.as_bytes(),
            "application/vc-statement+json",
            &canonicalize(v).unwrap(),
        );
        self.prev = statement_id(v).unwrap();
        self.seq += 1;
        bytes
    }
}

fn registry(agents: &[&Agent], max: u8) -> Registry {
    let mut r = Registry::new();
    for a in agents {
        r.register(
            a.id.as_bytes(),
            Issuer {
                key: a.key.verifying_key(),
                agent_id: a.id.clone(),
                max_completeness: max,
            },
        );
    }
    r
}

fn seq_mem(agents: &[&Agent], max: u8) -> Sequencer<MemStore> {
    Sequencer::open(
        MemStore::new(),
        registry(agents, max),
        SigningKey::from_bytes(&[200; 32]),
        "op",
        "vc.example/log",
    )
    .unwrap()
}

#[test]
fn accepts_chain_and_issues_verifiable_checkpoint() {
    let mut a = Agent::new(1, "agent-a");
    let mut s = seq_mem(&[&a], 3);
    let i0 = a.stmt("intent", json!({}));
    let id0 = statement_id(&i0).unwrap();
    let r0 = s.submit(&a.sign(&i0)).unwrap();
    let o0 = a.stmt("outcome", json!({"pair":{"intent_id": id0}}));
    let r1 = s.submit(&a.sign(&o0)).unwrap();
    assert_eq!((r0.index, r1.index, s.size()), (0, 1, 2));
    assert!(s.open_intents().is_empty());

    let (cp, note) = s.checkpoint().unwrap();
    let op = KeyInfo {
        name: "op".into(),
        key: SigningKey::from_bytes(&[200; 32]).verifying_key(),
    };
    assert_eq!(verify_note(&note, &op, &[], 0).unwrap(), cp);
    // inclusion proof for statement 1 against the checkpoint root
    let p = s.log().inclusion_proof(1, cp.size).unwrap();
    verify_inclusion(&r1.leaf_hash, 1, cp.size as usize, &p, &cp.root).unwrap();
}

#[test]
fn rejects_invalid_submissions() {
    let mut a = Agent::new(1, "agent-a");
    let other = Agent::new(2, "agent-b");
    let mut s = seq_mem(&[&a], 1);

    // unknown issuer key
    let mut b = Agent::new(2, "agent-b");
    let st = b.stmt("intent", json!({}));
    assert!(matches!(s.submit(&b.sign(&st)), Err(SeqError::Cose(_))));

    // signature tampering
    let st = a.stmt("intent", json!({}));
    let good = a.sign(&st);
    let mut bad = good.clone();
    let n = bad.len();
    bad[n - 3] ^= 1;
    assert!(s.submit(&bad).is_err());

    // issuer speaking for another agent id
    let mut imposter = Agent::new(1, "agent-b"); // same key as agent-a but claims agent-b
    imposter.key = a.key.clone();
    let st2 = imposter.stmt("intent", json!({}));
    let bytes = vc_cose::sign1(&imposter.key, b"agent-a", "t", &canonicalize(&st2).unwrap());
    assert!(matches!(s.submit(&bytes), Err(SeqError::AgentBinding)));

    // completeness above the issuer's ceiling (C1)
    let mut c = Agent::new(1, "agent-a");
    let st3 = c.stmt("intent", json!({"completeness":"C3"}));
    assert!(matches!(
        s.submit(&c.sign(&st3)),
        Err(SeqError::Completeness { .. })
    ));

    // non-canonical payload (pretty-printed JSON, even though correctly signed)
    let pretty = serde_json::to_vec_pretty(&a.stmt("intent", json!({}))).unwrap();
    let bytes = vc_cose::sign1(&a.key, b"agent-a", "t", &pretty);
    assert!(matches!(s.submit(&bytes), Err(SeqError::NotCanonical)));

    // oversized
    let huge = vec![0u8; MAX_STATEMENT_BYTES + 1];
    assert!(matches!(s.submit(&huge), Err(SeqError::TooLarge)));

    // nothing was appended by any rejection
    assert_eq!(s.size(), 0);
    // and the original good statement still registers afterwards
    assert_eq!(s.submit(&good).unwrap().index, 0);
    let _ = other;
}

#[test]
fn enforces_chain_rules() {
    let mut a = Agent::new(1, "agent-a");
    let mut s = seq_mem(&[&a], 3);
    // seq gap: skip statement 0
    let first = a.stmt("intent", json!({}));
    let _skipped = a.sign(&first);
    let second = a.stmt("intent", json!({}));
    let r = s.submit(&a.sign(&second));
    assert!(matches!(
        r,
        Err(SeqError::Chain(vc_core::chain::ChainError::SeqGap(..)))
    ));
    // outcome for an unknown intent
    let mut b = Agent::new(1, "agent-a");
    let st = b.stmt("outcome", json!({"pair":{"intent_id": "ab".repeat(32)}}));
    assert!(matches!(
        s.submit(&b.sign(&st)),
        Err(SeqError::Chain(vc_core::chain::ChainError::UnknownIntent(
            ..
        )))
    ));
    assert_eq!(s.size(), 0);
}

#[test]
fn idempotent_resubmission() {
    let mut a = Agent::new(1, "agent-a");
    let mut s = seq_mem(&[&a], 3);
    let st = a.stmt("intent", json!({}));
    let bytes = a.sign(&st);
    let r1 = s.submit(&bytes).unwrap();
    let r2 = s.submit(&bytes).unwrap();
    assert!(!r1.duplicate && r2.duplicate);
    assert_eq!((r1.index, r1.leaf_hash), (r2.index, r2.leaf_hash));
    assert_eq!(s.size(), 1);
}

#[test]
fn batch_with_dependencies_and_failures() {
    let mut a = Agent::new(1, "agent-a");
    let mut s = seq_mem(&[&a], 3);
    let i0 = a.stmt("intent", json!({}));
    let id0 = statement_id(&i0).unwrap();
    let b0 = a.sign(&i0);
    let o0 = a.stmt("outcome", json!({"pair":{"intent_id": id0}}));
    let b1 = a.sign(&o0);
    let garbage = vec![1u8, 2, 3];
    let res = s.submit_batch(&[&b0, &garbage, &b1]);
    assert!(res[0].is_ok() && res[1].is_err() && res[2].is_ok());
    assert_eq!(
        (
            res[0].as_ref().unwrap().index,
            res[2].as_ref().unwrap().index
        ),
        (0, 1)
    );
    assert_eq!(s.size(), 2);
}

#[test]
fn survives_restart_and_continues_chain() {
    let dir = std::env::temp_dir().join(format!("vc-seq-reopen-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let mut a = Agent::new(1, "agent-a");
    let (b0, b1, root);
    {
        let mut s = Sequencer::open(
            FsStore::new(&dir).unwrap(),
            registry(&[&a], 3),
            SigningKey::from_bytes(&[200; 32]),
            "op",
            "vc.example/log",
        )
        .unwrap();
        let i0 = a.stmt("intent", json!({}));
        b0 = a.sign(&i0);
        s.submit(&b0).unwrap();
        let i1 = a.stmt("intent", json!({}));
        b1 = a.sign(&i1);
        s.submit(&b1).unwrap();
        root = s.checkpoint().unwrap().0.root;
    }
    let mut s = Sequencer::open(
        FsStore::new(&dir).unwrap(),
        registry(&[&a], 3),
        SigningKey::from_bytes(&[200; 32]),
        "op",
        "vc.example/log",
    )
    .unwrap();
    assert_eq!(s.size(), 2);
    assert_eq!(s.checkpoint().unwrap().0.root, root);
    assert_eq!(s.open_intents().len(), 2); // both intents still orphans
    assert!(s.submit(&b0).unwrap().duplicate);
    let i2 = a.stmt("intent", json!({}));
    assert_eq!(s.submit(&a.sign(&i2)).unwrap().index, 2);
    assert_eq!(sha256(b"x").len(), 32);
    let _ = std::fs::remove_dir_all(&dir);
    let _ = b1;
}
