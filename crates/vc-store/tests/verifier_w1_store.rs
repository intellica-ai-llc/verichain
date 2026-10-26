//! W1 adversarial verifier tests for vc-store (independent verifier, not the builder).
//!
//! Tests named `bug_*` assert the CORRECT behaviour and are expected to FAIL against the current code (each
//! maps to a finding in docs/reports/verifier_w1_store_seq.md). Tests named `holds_*` pass today and are kept as
//! regression tests for properties the verifier tried to break and could not.
mod common;

use common::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;
use vc_core::hash::{leaf_hash, node_hash, sha256, Hash};
use vc_store::tile::{entry_tile_path, hash_tile_path};
use vc_store::*;

// ---------------------------------------------------------------------------------------------------------
// Independent RFC 9162 reference (own implementation, straight from the RFC text; memoised).
// ---------------------------------------------------------------------------------------------------------
struct Reference {
    leaves: Vec<Hash>,
    cache: RefCell<HashMap<(usize, usize), Hash>>,
}

fn pow2_below(n: usize) -> usize {
    let mut k = 1;
    while k * 2 < n {
        k *= 2;
    }
    k
}

impl Reference {
    fn new(leaves: Vec<Hash>) -> Self {
        Reference {
            leaves,
            cache: RefCell::new(HashMap::new()),
        }
    }
    fn mth(&self, a: usize, b: usize) -> Hash {
        match b - a {
            0 => sha256(b""),
            1 => self.leaves[a],
            n => {
                if let Some(h) = self.cache.borrow().get(&(a, b)) {
                    return *h;
                }
                let k = pow2_below(n);
                let h = node_hash(&self.mth(a, a + k), &self.mth(a + k, b));
                self.cache.borrow_mut().insert((a, b), h);
                h
            }
        }
    }
    fn root(&self, size: usize) -> Hash {
        self.mth(0, size)
    }
    /// RFC 9162 2.1.3.1 PATH(m, D[a:b]).
    fn path(&self, m: usize, a: usize, b: usize) -> Vec<Hash> {
        let n = b - a;
        if n == 1 {
            return vec![];
        }
        let k = pow2_below(n);
        if m < k {
            let mut p = self.path(m, a, a + k);
            p.push(self.mth(a + k, b));
            p
        } else {
            let mut p = self.path(m - k, a + k, b);
            p.push(self.mth(a, a + k));
            p
        }
    }
    fn inclusion(&self, m: usize, size: usize) -> Vec<Hash> {
        self.path(m, 0, size)
    }
    /// RFC 9162 2.1.4.1 SUBPROOF.
    fn subproof(&self, m: usize, a: usize, b: usize, complete: bool) -> Vec<Hash> {
        let n = b - a;
        if m == n {
            return if complete { vec![] } else { vec![self.mth(a, b)] };
        }
        let k = pow2_below(n);
        if m <= k {
            let mut p = self.subproof(m, a, a + k, complete);
            p.push(self.mth(a + k, b));
            p
        } else {
            let mut p = self.subproof(m - k, a + k, b, false);
            p.push(self.mth(a, a + k));
            p
        }
    }
    fn consistency(&self, m: usize, n: usize) -> Vec<Hash> {
        self.subproof(m, 0, n, true)
    }
}

fn build_in_chunks<S: TileStore>(log: &mut TileLog<S>, from: usize, to: usize, chunk: usize) {
    let mut i = from;
    while i < to {
        let j = (i + chunk).min(to);
        let es: Vec<Vec<u8>> = (i..j).map(entry).collect();
        log.append_batch(&refs(&es)).unwrap();
        i = j;
    }
}

// ---------------------------------------------------------------------------------------------------------
// 1. Crash consistency
// ---------------------------------------------------------------------------------------------------------

/// (base size, batch size) scenarios chosen around tile boundaries.
const SCENARIOS: &[(usize, usize)] = &[
    (0, 1),
    (0, 3),
    (0, 300),
    (100, 1),
    (100, 156),
    (100, 200),
    (255, 1),
    (255, 2),
    (256, 1),
    (300, 300),
    (511, 2),
    (65535, 2),
];

struct Prepared {
    shared: Shared,
    snap: std::collections::BTreeMap<String, Vec<u8>>,
    puts_needed: u64,
    reference: Reference,
}

fn prepare(base: usize, batch: usize) -> Prepared {
    let shared = Shared::new();
    let mut log = TileLog::open(shared.clone()).unwrap();
    build_in_chunks(&mut log, 0, base, 1000);
    let snap = shared.snapshot();
    // count the puts one full batch needs
    let before = shared.puts.load(std::sync::atomic::Ordering::SeqCst);
    let es: Vec<Vec<u8>> = (base..base + batch).map(entry).collect();
    log.append_batch(&refs(&es)).unwrap();
    let puts_needed = shared.puts.load(std::sync::atomic::Ordering::SeqCst) - before;
    shared.restore(&snap);
    Prepared {
        shared,
        snap,
        puts_needed,
        reference: Reference::new(leaves(base + batch)),
    }
}

/// After a crash at ANY point of a batch append, reopening must give a consistent prefix whose root equals the
/// independent RFC 9162 root of exactly those entries, and all entries/proofs must be intact.
#[test]
fn holds_crash_at_every_put_reopens_to_consistent_prefix() {
    let mut violations = Vec::new();
    for &(base, batch) in SCENARIOS {
        let p = prepare(base, batch);
        for k in 0..=p.puts_needed {
            p.shared.restore(&p.snap);
            p.shared.unlimited();
            let mut log = TileLog::open(p.shared.clone()).unwrap();
            p.shared.fail_after(k as i64);
            let es: Vec<Vec<u8>> = (base..base + batch).map(entry).collect();
            let r = catch_unwind(AssertUnwindSafe(|| log.append_batch(&refs(&es))));
            p.shared.unlimited();
            drop(log); // "crash": instance state is lost, only the store survives
            let ctx = format!("base={base} batch={batch} fail_after={k}");
            match TileLog::open(p.shared.clone()) {
                Err(e) => violations.push(format!("{ctx}: reopen failed: {e}")),
                Ok(re) => {
                    let sz = re.size() as usize;
                    if sz < base || sz > base + batch {
                        violations.push(format!("{ctx}: reopened size {sz} outside [{base},{}]", base + batch));
                        continue;
                    }
                    if matches!(r, Ok(Ok(_))) && sz != base + batch {
                        violations.push(format!("{ctx}: append acked but reopened size {sz}"));
                    }
                    if re.root().ok() != Some(p.reference.root(sz)) {
                        violations.push(format!("{ctx}: root at size {sz} != independent reference"));
                    }
                    for i in [0usize, sz / 2, sz.saturating_sub(1)] {
                        if i < sz {
                            if re.entry(i as u64).ok() != Some(entry(i)) {
                                violations.push(format!("{ctx}: entry {i} wrong"));
                            }
                            if re.inclusion_proof(i as u64, sz as u64).ok()
                                != Some(p.reference.inclusion(i, sz))
                            {
                                violations.push(format!("{ctx}: inclusion({i},{sz}) wrong"));
                            }
                        }
                    }
                    if sz > 1
                        && re.consistency_proof((sz / 2).max(1) as u64, sz as u64).ok()
                            != Some(p.reference.consistency((sz / 2).max(1), sz))
                    {
                        violations.push(format!("{ctx}: consistency wrong"));
                    }
                }
            }
        }
    }
    assert!(violations.is_empty(), "{} violations, first: {:#?}", violations.len(), &violations[..violations.len().min(8)]);
}

/// FINDING ST-1. A failed `append_batch` must leave the live instance consistent with the store (size == durable
/// size, root == reference root). The implementation mutates `size`/`entry_edge` before the writes succeed and
/// `mem::take`s `edge` and never restores it on error, so the instance diverges (and `root()` can panic).
#[test]
fn bug_failed_append_leaves_instance_consistent_with_store() {
    let mut violations = Vec::new();
    for &(base, batch) in SCENARIOS {
        let p = prepare(base, batch);
        for k in 0..p.puts_needed {
            p.shared.restore(&p.snap);
            p.shared.unlimited();
            let mut log = TileLog::open(p.shared.clone()).unwrap();
            p.shared.fail_after(k as i64);
            let es: Vec<Vec<u8>> = (base..base + batch).map(entry).collect();
            let r = log.append_batch(&refs(&es));
            p.shared.unlimited();
            if r.is_ok() {
                continue;
            }
            let durable = TileLog::open(p.shared.clone()).unwrap().size();
            let ctx = format!("base={base} batch={batch} fail_after={k}");
            if log.size() != durable {
                violations.push(format!("{ctx}: live size {} != durable size {durable}", log.size()));
                continue;
            }
            let got = catch_unwind(AssertUnwindSafe(|| log.root()));
            match got {
                Err(_) => violations.push(format!("{ctx}: root() PANICS after failed append")),
                Ok(Err(e)) => violations.push(format!("{ctx}: root() errors after failed append: {e}")),
                Ok(Ok(root)) if root != p.reference.root(durable as usize) => {
                    violations.push(format!("{ctx}: live root != reference at durable size"))
                }
                _ => {}
            }
        }
    }
    assert!(violations.is_empty(), "{} violations, first: {:#?}", violations.len(), &violations[..violations.len().min(8)]);
}

/// FINDING ST-1 (consequence). After a failed batch the caller was told "error"; a later successful append on the
/// same instance must not durably commit the phantom entries of the failed batch, and the live instance must agree
/// with what a fresh reopen sees.
#[test]
fn bug_retry_after_failed_append_agrees_with_reopen() {
    let mut violations = Vec::new();
    for &(base, batch) in &[(0usize, 3usize), (100, 156), (255, 2), (300, 300)] {
        let p = prepare(base, batch);
        for k in 0..p.puts_needed {
            p.shared.restore(&p.snap);
            p.shared.unlimited();
            let mut log = TileLog::open(p.shared.clone()).unwrap();
            p.shared.fail_after(k as i64);
            let es: Vec<Vec<u8>> = (base..base + batch).map(entry).collect();
            if log.append_batch(&refs(&es)).is_ok() {
                continue;
            }
            p.shared.unlimited();
            let ctx = format!("base={base} batch={batch} fail_after={k}");
            let w = b"after-failure".to_vec();
            let res = catch_unwind(AssertUnwindSafe(|| log.append(&w)));
            let Ok(Ok(idx)) = res else {
                violations.push(format!("{ctx}: append after failure did not succeed cleanly"));
                continue;
            };
            let live_size = log.size();
            let live_root = catch_unwind(AssertUnwindSafe(|| log.root())).ok().and_then(|r| r.ok());
            let fresh = TileLog::open(p.shared.clone()).unwrap();
            if fresh.size() != live_size || Some(fresh.root().unwrap()) != live_root {
                violations.push(format!("{ctx}: live (size {live_size}) != reopen (size {})", fresh.size()));
            }
            // whatever the store holds must be internally consistent with the entries it contains
            let ents: Vec<Vec<u8>> = (0..fresh.size()).map(|i| fresh.entry(i).unwrap()).collect();
            let lv: Vec<Hash> = ents.iter().map(|e| leaf_hash(e)).collect();
            if fresh.root().unwrap() != Reference::new(lv).root(ents.len()) {
                violations.push(format!("{ctx}: reopened root != root of its own entries"));
            }
            if fresh.entry(idx).ok() != Some(w) {
                violations.push(format!("{ctx}: acked append idx {idx} not found after reopen"));
            }
        }
    }
    assert!(violations.is_empty(), "{} violations, first: {:#?}", violations.len(), &violations[..violations.len().min(8)]);
}

/// FINDING ST-8 (Low). A hash tile that is torn (length not a multiple of 32) on the open path makes `open` fail
/// permanently instead of falling back to recovery from entries (which is what a short/missing tile does).
#[test]
fn bug_torn_edge_hash_tile_should_be_recovered_not_fatal() {
    let shared = Shared::new();
    let mut log = TileLog::open(shared.clone()).unwrap();
    build_in_chunks(&mut log, 0, 300, 100);
    let want = log.root().unwrap();
    drop(log);
    let p = hash_tile_path(0, 1, 44);
    let mut b = shared.raw_get(&p).expect("edge tile exists");
    b.pop();
    shared.raw_put(&p, &b);
    let re = TileLog::open(shared.clone()).expect("torn derived tile must be recoverable from the entries");
    assert_eq!(re.root().unwrap(), want);
}

// ---------------------------------------------------------------------------------------------------------
// 2. Hash tiles / bundles not verified against entries on load
// ---------------------------------------------------------------------------------------------------------

fn flip(shared: &Shared, path: &str, at: usize) {
    let mut b = shared.raw_get(path).unwrap_or_else(|| panic!("no {path}; have {:?}", shared.paths()));
    b[at] ^= 0x01;
    shared.raw_put(path, &b);
}

/// A silent-proof check: for every leaf, either the call errors or the proof verifies against the reference root.
fn proofs_never_silently_bogus(log: &TileLog<Shared>, rf: &Reference, size: usize) -> Vec<String> {
    let mut bad = Vec::new();
    for i in 0..size {
        match log.inclusion_proof(i as u64, size as u64) {
            Err(_) => {}
            Ok(p) => {
                if p != rf.inclusion(i, size) {
                    bad.push(format!("inclusion({i},{size}) returned a wrong proof silently"));
                }
            }
        }
        match log.leaf_hash_at(i as u64) {
            Err(_) => {}
            Ok(h) => {
                if h != rf.leaves[i] {
                    bad.push(format!("leaf_hash_at({i}) wrong silently"));
                }
            }
        }
    }
    bad
}

/// FINDING ST-2. A full level-0 hash tile corrupted on disk is not detected by open or by any read: wrong leaf
/// hashes and wrong inclusion proofs are served with `Ok`.
#[test]
fn bug_tampered_full_hash_tile_is_detected() {
    let shared = Shared::new();
    let mut log = TileLog::open(shared.clone()).unwrap();
    build_in_chunks(&mut log, 0, 600, 100);
    drop(log);
    flip(&shared, &hash_tile_path(0, 0, 256), 5 * 32 + 3);
    let rf = Reference::new(leaves(600));
    match TileLog::open(shared.clone()) {
        Err(_) => {}
        Ok(re) => {
            let bad = proofs_never_silently_bogus(&re, &rf, 600);
            assert!(bad.is_empty(), "{} silent bogus answers, e.g. {:?}", bad.len(), &bad[..bad.len().min(3)]);
        }
    }
}

/// FINDING ST-2. A corrupted level-1 edge tile changes `root()` itself: the log reports (and a sequencer would
/// sign) a root that is not the Merkle root of its entries.
#[test]
fn bug_tampered_level1_edge_tile_changes_root_silently() {
    let shared = Shared::new();
    let mut log = TileLog::open(shared.clone()).unwrap();
    build_in_chunks(&mut log, 0, 1100, 100); // level-1 count = 4 -> tile/1/000.p/4
    drop(log);
    flip(&shared, &hash_tile_path(1, 0, 4), 40);
    let rf = Reference::new(leaves(1100));
    if let Ok(re) = TileLog::open(shared.clone()) {
        let r = re.root();
        assert!(
            r.is_err() || r.unwrap() == rf.root(1100),
            "root() silently differs from the Merkle root of the stored entries"
        );
    }
}

/// FINDING ST-2. An entry bundle altered on disk (same length) is served by `entry()` although its leaf hash no
/// longer matches the hash tile: nothing cross-checks entries against the tree.
#[test]
fn bug_tampered_entry_bundle_is_detected() {
    let shared = Shared::new();
    let mut log = TileLog::open(shared.clone()).unwrap();
    build_in_chunks(&mut log, 0, 600, 100);
    drop(log);
    // entry i is "e{i}": flip a payload byte of the first entry of bundle 0 (offset 2 = after the u16 length)
    flip(&shared, &entry_tile_path(0, 256), 3);
    match TileLog::open(shared.clone()) {
        Err(_) => {}
        Ok(re) => {
            let e = re.entry(0);
            let h = re.leaf_hash_at(0);
            let consistent = match (e, h) {
                (Err(_), _) | (_, Err(_)) => true,
                (Ok(e), Ok(h)) => leaf_hash(&e) == h,
            };
            assert!(consistent, "entry 0 content disagrees with its leaf hash in the tile and no error is raised");
        }
    }
}

/// FINDING ST-2 (variant). A full entry bundle that was truncated at an entry boundary (255 of 256 entries) is
/// accepted at open: `size` is still reported as 256 per full bundle and the last entry is unreadable.
#[test]
fn bug_short_full_entry_bundle_rejected_at_open() {
    let shared = Shared::new();
    let mut log = TileLog::open(shared.clone()).unwrap();
    build_in_chunks(&mut log, 0, 600, 100);
    drop(log);
    let p = entry_tile_path(0, 256);
    let mut b = shared.raw_get(&p).unwrap();
    // drop the last entry (entries are "e<i>" with a 2-byte length prefix)
    let last = format!("e{}", 255).len() + 2;
    b.truncate(b.len() - last);
    shared.raw_put(&p, &b);
    match TileLog::open(shared.clone()) {
        Err(_) => {}
        Ok(re) => {
            let all_readable = (0..re.size()).all(|i| re.entry(i).is_ok());
            assert!(all_readable, "open succeeded with size {} but some entries are unreadable", re.size());
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// 3. Boundary sizes and proof sweeps against the independent reference
// ---------------------------------------------------------------------------------------------------------

const BOUNDARY: &[usize] = &[0, 1, 2, 255, 256, 257, 511, 512, 513, 65535, 65536, 65537, 70000];

/// Grow ONE log through every boundary size with awkward batch sizes; at each stop compare root, reopen, and
/// sample inclusion proofs against the independent reference.
#[test]
fn holds_boundary_sizes_roots_reopen_and_inclusion() {
    let shared = Shared::new();
    let mut log = TileLog::open(shared.clone()).unwrap();
    let rf = Reference::new(leaves(70000));
    let mut cur = 0usize;
    for &n in BOUNDARY {
        // odd chunking so batches straddle tile edges differently each time
        build_in_chunks(&mut log, cur, n, 777);
        cur = n;
        assert_eq!(log.size() as usize, n);
        assert_eq!(log.root().unwrap(), rf.root(n), "root at {n}");
        // fresh instance over the same store (as after restart)
        let re = TileLog::open(shared.clone()).unwrap();
        assert_eq!(re.size() as usize, n, "reopen size at {n}");
        assert_eq!(re.root().unwrap(), rf.root(n), "reopen root at {n}");
        if n == 0 {
            assert!(log.inclusion_proof(0, 0).is_err());
            continue;
        }
        let mut idx = vec![0usize, 1, n / 2, n - 1];
        for t in [255usize, 256, 257, 65535, 65536] {
            if t < n {
                idx.push(t);
            }
        }
        for i in idx.into_iter().filter(|&i| i < n) {
            assert_eq!(
                re.inclusion_proof(i as u64, n as u64).unwrap(),
                rf.inclusion(i, n),
                "inclusion {i}/{n}"
            );
            assert_eq!(re.entry(i as u64).unwrap(), entry(i));
            assert_eq!(re.leaf_hash_at(i as u64).unwrap(), rf.leaves[i]);
        }
    }
}

/// Inclusion at every prefix size (not just current size) and consistency between all boundary-size pairs.
#[test]
fn holds_boundary_consistency_all_pairs_and_historic_inclusion() {
    let mut log = TileLog::open(MemStore::new()).unwrap();
    build_in_chunks(&mut log, 0, 70000, 5000);
    let rf = Reference::new(leaves(70000));
    for &n in BOUNDARY.iter().filter(|&&n| n > 0) {
        assert_eq!(log.root_at(n as u64).unwrap(), rf.root(n), "root_at {n}");
        for &m in BOUNDARY.iter().filter(|&&m| m > 0 && m <= n) {
            let got = log.consistency_proof(m as u64, n as u64).unwrap();
            assert_eq!(got, rf.consistency(m, n), "consistency {m}->{n}");
            vc_log::verify_consistency(m as usize, n as usize, &rf.root(m), &rf.root(n), &got)
                .unwrap_or_else(|e| panic!("vc-log rejects store proof {m}->{n}: {e:?}"));
        }
        for i in [0usize, n / 3, n / 2, n - 1] {
            let got = log.inclusion_proof(i as u64, n as u64).unwrap();
            assert_eq!(got, rf.inclusion(i, n), "inclusion {i}/{n}");
            vc_log::verify_inclusion(&rf.leaves[i], i, n, &got, &rf.root(n))
                .unwrap_or_else(|e| panic!("vc-log rejects store inclusion {i}/{n}: {e:?}"));
        }
    }
}

/// Dense exhaustive sweep around the first two tile boundaries.
#[test]
#[ignore = "slow (~60s debug); runs in nightly CI with --include-ignored"]
fn holds_dense_sweep_around_tile_edges() {
    let mut log = TileLog::open(MemStore::new()).unwrap();
    build_in_chunks(&mut log, 0, 530, 53);
    let rf = Reference::new(leaves(530));
    let mut sizes: Vec<usize> = (1..=40).collect();
    sizes.extend(250..=262);
    sizes.extend(508..=530);
    for &n in &sizes {
        assert_eq!(log.root_at(n as u64).unwrap(), rf.root(n), "root {n}");
        for i in 0..n {
            assert_eq!(log.inclusion_proof(i as u64, n as u64).unwrap(), rf.inclusion(i, n), "incl {i}/{n}");
        }
        for m in 1..=n {
            assert_eq!(log.consistency_proof(m as u64, n as u64).unwrap(), rf.consistency(m, n), "cons {m}->{n}");
        }
    }
}

/// Edge-case API ranges: nothing panics for out-of-range or extreme arguments.
#[test]
fn holds_range_arguments_never_panic() {
    let mut log = TileLog::open(MemStore::new()).unwrap();
    build_in_chunks(&mut log, 0, 300, 300);
    let r = catch_unwind(AssertUnwindSafe(|| {
        for &(i, n) in &[(0u64, 0u64), (u64::MAX, u64::MAX), (299, 301), (300, 300), (0, u64::MAX)] {
            assert!(log.inclusion_proof(i, n).is_err());
        }
        for &(m, n) in &[(0u64, 0u64), (1, 0), (u64::MAX, u64::MAX), (1, 301), (301, 300)] {
            assert!(log.consistency_proof(m, n).is_err());
        }
        assert!(log.root_at(u64::MAX).is_err());
        assert!(log.entry(u64::MAX).is_err());
        assert!(log.leaf_hash_at(u64::MAX).is_err());
        let empty = TileLog::open(MemStore::new()).unwrap();
        assert_eq!(empty.root().unwrap(), sha256(b""));
        assert_eq!(log.consistency_proof(300, 300).unwrap(), Vec::<Hash>::new());
    }));
    assert!(r.is_ok(), "panic on out-of-range argument");
}

/// Entry-size edge cases: 0-byte and 65535-byte entries round-trip, 65536 is refused atomically (no state change).
#[test]
fn holds_entry_size_limits_and_atomic_refusal() {
    let mut log = TileLog::open(MemStore::new()).unwrap();
    log.append(b"").unwrap();
    let big = vec![7u8; 65535];
    log.append(&big).unwrap();
    assert_eq!(log.entry(0).unwrap(), b"");
    assert_eq!(log.entry(1).unwrap(), big);
    let root = log.root().unwrap();
    let too = vec![0u8; 65536];
    assert!(matches!(log.append_batch(&[b"ok".as_slice(), too.as_slice()]), Err(StoreError::TooLarge)));
    assert_eq!(log.size(), 2);
    assert_eq!(log.root().unwrap(), root);
}

// ---------------------------------------------------------------------------------------------------------
// 4. FsStore
// ---------------------------------------------------------------------------------------------------------

fn tmpdir(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("vc-w1-{}-{}", name, std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    d
}

/// FINDING ST-3. FsStore's temp name is `.tmp-<pid>-<filename>`: two writers (threads, or two FsStore instances)
/// in the same process targeting the same path share one temp file. Writers truncate each other, rename fails with
/// ENOENT, and readers can observe a mix of two payloads.
#[test]
fn bug_fsstore_concurrent_put_same_path_collides() {
    let dir = tmpdir("tmpcoll");
    let st = Arc::new(FsStore::new(&dir).unwrap());
    let payloads: Vec<Vec<u8>> = (0..8u8).map(|t| vec![t; 300_000]).collect();
    let errors = Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
    let hs: Vec<_> = (0..8usize)
        .map(|t| {
            let st = st.clone();
            let pl = payloads[t].clone();
            let errors = errors.clone();
            std::thread::spawn(move || {
                for _ in 0..60 {
                    if let Err(e) = st.put("tile/0/000", &pl) {
                        errors.lock().unwrap().push(format!("thread {t}: {e}"));
                    }
                }
            })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    let errs = errors.lock().unwrap().clone();
    let fin = st.get("tile/0/000").unwrap().unwrap();
    let whole = payloads.iter().any(|p| *p == fin);
    let _ = std::fs::remove_dir_all(&dir);
    assert!(errs.is_empty(), "{} put errors, e.g. {:?}", errs.len(), &errs[..errs.len().min(2)]);
    assert!(whole, "final file is not any writer's complete payload (len {})", fin.len());
}

/// Readers must never see a partial file while a single writer alternates payloads (atomic rename).
#[test]
fn holds_fsstore_reader_never_sees_partial_file() {
    let dir = tmpdir("partial");
    let st = Arc::new(FsStore::new(&dir).unwrap());
    let (a, b) = (vec![0xAAu8; 500_000], vec![0xBBu8; 123_457]);
    st.put("tile/0/000", &a).unwrap();
    let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let w = {
        let (st, stop, a, b) = (st.clone(), stop.clone(), a.clone(), b.clone());
        std::thread::spawn(move || {
            for i in 0..300 {
                st.put("tile/0/000", if i % 2 == 0 { &b } else { &a }).unwrap();
            }
            stop.store(true, std::sync::atomic::Ordering::SeqCst);
        })
    };
    let mut reads = 0;
    while !stop.load(std::sync::atomic::Ordering::SeqCst) {
        let got = st.get("tile/0/000").unwrap().unwrap();
        assert!(got == a || got == b, "partial read of len {}", got.len());
        reads += 1;
    }
    w.join().unwrap();
    assert!(reads > 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn holds_fsstore_rejects_path_traversal_keys() {
    let dir = tmpdir("trav");
    let st = FsStore::new(dir.join("root")).unwrap();
    std::fs::write(dir.join("secret"), b"s").unwrap();
    let keys = [
        "../secret",
        "a/../../secret",
        "/etc/passwd",
        "",
        "a//b",
        "a/./b",
        "./a",
        "a/",
        "tile/../../secret",
        "a/..",
        "a\0b",
    ];
    for k in keys {
        assert!(st.get(k).is_err(), "get({k:?}) should be refused");
        assert!(st.put(k, b"x").is_err(), "put({k:?}) should be refused");
    }
    assert_eq!(std::fs::read(dir.join("secret")).unwrap(), b"s");
    // nothing escaped the root
    let escaped: Vec<_> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().file_name()).collect();
    assert_eq!(escaped.len(), 2, "{escaped:?}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// FINDING ST-4. Two TileLog instances opened on the same directory (misconfiguration or failover without
/// fencing): there is no writer exclusion, and an append that returned Ok can be silently lost.
#[test]
fn bug_two_writers_on_one_dir_lose_acknowledged_appends() {
    let dir = tmpdir("twowriters");
    let mut a = TileLog::open(FsStore::new(&dir).unwrap()).unwrap();
    let second = TileLog::open(FsStore::new(&dir).unwrap());
    // Either the second open is refused (lock/fencing) ...
    let Ok(mut b) = second else {
        let _ = std::fs::remove_dir_all(&dir);
        return;
    };
    let ea: Vec<Vec<u8>> = (0..3).map(|i| format!("A{i}").into_bytes()).collect();
    let eb: Vec<Vec<u8>> = (0..2).map(|i| format!("B{i}").into_bytes()).collect();
    a.append_batch(&refs(&ea)).unwrap();
    let rb = b.append_batch(&refs(&eb)); // B has no idea A wrote
    // ... or B's append must fail; it must never be acknowledged and then vanish.
    if rb.is_ok() {
        let re = TileLog::open(FsStore::new(&dir).unwrap()).unwrap();
        let all: Vec<Vec<u8>> = (0..re.size()).map(|i| re.entry(i).unwrap()).collect();
        let _ = std::fs::remove_dir_all(&dir);
        for e in ea.iter().chain(eb.iter()) {
            assert!(all.contains(e), "acknowledged entry {:?} lost after two-writer interleave", String::from_utf8_lossy(e));
        }
    } else {
        let _ = std::fs::remove_dir_all(&dir);
    }
}

/// Observation (not a failing test): leftover `.tmp-*` and stale partial tiles accumulate; characterised so the
/// numbers are in the test log. Passes by construction.
#[test]
fn observe_stale_partial_tiles_accumulate() {
    let shared = Shared::new();
    let mut log = TileLog::open(shared.clone()).unwrap();
    for i in 0..600 {
        log.append(&entry(i)).unwrap();
    }
    let paths = shared.paths();
    let stale: Vec<_> = paths
        .iter()
        .filter(|p| p.contains(".p/") && p.starts_with("tile/entries/000"))
        .collect();
    eprintln!(
        "OBSERVATION: 600 single appends -> {} store objects, {} stale partial bundles for completed tile 0",
        paths.len(),
        stale.len()
    );
    assert!(!paths.is_empty());
}

// ---------------------------------------------------------------------------------------------------------
// 6. Performance / DoS characterisation
// ---------------------------------------------------------------------------------------------------------

/// FINDING ST-5. open() must not cost O(total stored bytes) of I/O just to learn the size, and recovery from lost
/// hash tiles must not be quadratic. Budgets are generous (debug build).
#[test]
#[ignore = "perf; nightly CI with --include-ignored"]
fn perf_open_and_recover_70k_entries() {
    let shared = Shared::new();
    let mut log = TileLog::open(shared.clone()).unwrap();
    let es: Vec<Vec<u8>> = (0..70_000).map(|i| vec![(i % 251) as u8; 600]).collect();
    for c in es.chunks(5000) {
        log.append_batch(&refs(c)).unwrap();
    }
    let want = log.root().unwrap();
    drop(log);
    let t = std::time::Instant::now();
    let re = TileLog::open(shared.clone()).unwrap();
    let t_open = t.elapsed();
    assert_eq!(re.root().unwrap(), want);
    // lose every hash tile
    for p in shared.paths() {
        if !p.starts_with("tile/entries") {
            shared.raw_remove(&p);
        }
    }
    let t = std::time::Instant::now();
    let rec = TileLog::open(shared.clone()).unwrap();
    let t_rec = t.elapsed();
    assert_eq!(rec.root().unwrap(), want);
    eprintln!("PERF: open(70k x 600B) = {t_open:?}; recover-from-entries = {t_rec:?}");
    assert!(t_rec.as_secs() < 60, "recovery of 70k entries took {t_rec:?}");
}

#[test]
fn tile_paths_extreme_indices() {
    use vc_store::tile::index_path;
    assert_eq!(index_path(999_999), "x999/999");
    assert_eq!(index_path(1_000_000), "x001/x000/000");
    assert_eq!(index_path(u64::MAX), "x018/x446/x744/x073/x709/x551/615");
}
