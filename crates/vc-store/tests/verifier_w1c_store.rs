//! W1c adversarial verifier tests for vc-store (commit-record TileLog).
//!
//! Conventions: `bug_*` assert the CORRECT behaviour and FAIL against current code (see
//! docs/reports/verifier_w1c_store_seq.md); `holds_*` pass today and are regression tests.
#[path = "common/mod.rs"]
mod common;

use common::{entry, Shared};
use std::collections::BTreeMap;
use std::io;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use vc_core::hash::{leaf_hash, node_hash, sha256, Hash};
use vc_store::tile::{entry_tile_path, hash_tile_path};
use vc_store::{FsStore, StoreError, TileLog, TileStore};

// ----- independent RFC 9162 root -----
fn mth(h: &[Hash]) -> Hash {
    match h.len() {
        0 => sha256(b""),
        1 => h[0],
        n => {
            let mut k = 1usize;
            while k * 2 < n {
                k *= 2;
            }
            node_hash(&mth(&h[..k]), &mth(&h[k..]))
        }
    }
}

fn ent(tag: &str, i: usize) -> Vec<u8> {
    format!("{tag}{i}").into_bytes()
}

fn root_of(es: &[Vec<u8>]) -> Hash {
    mth(&es.iter().map(|e| leaf_hash(e)).collect::<Vec<_>>())
}

fn append_all<S: TileStore>(
    log: &mut TileLog<S>,
    es: &[Vec<u8>],
) -> Result<(u64, u64), StoreError> {
    let r: Vec<&[u8]> = es.iter().map(|e| e.as_slice()).collect();
    log.append_batch(&r)
}

/// Store wrapper whose Nth put (counted from `arm`) is applied but still reports an error (indeterminate).
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

fn check_log(sh: &Shared, expect: &[Vec<u8>], ctx: &str) {
    let log = TileLog::open(sh.clone()).unwrap_or_else(|e| panic!("{ctx}: reopen failed: {e}"));
    assert_eq!(log.size(), expect.len() as u64, "{ctx}: size");
    assert_eq!(log.root().unwrap(), root_of(expect), "{ctx}: root");
    log.verify_integrity()
        .unwrap_or_else(|e| panic!("{ctx}: integrity: {e}"));
}

// ----- 1. crash injection at every put, two stages, across tile boundaries -----

fn crash_matrix(base: usize, a_len: usize, b_len: usize) {
    let base_es: Vec<Vec<u8>> = (0..base).map(|i| ent("b", i)).collect();
    let a: Vec<Vec<u8>> = (0..a_len).map(|i| ent("A", i)).collect();
    let b: Vec<Vec<u8>> = (0..b_len).map(|i| ent("B", i)).collect();
    let sh0 = Shared::new();
    {
        let mut l = TileLog::open(sh0.clone()).unwrap();
        for ch in base_es.chunks(300) {
            append_all(&mut l, ch).unwrap();
        }
    }
    let snap0 = sh0.snapshot();
    // count puts of A and B
    let total = |es: &[Vec<u8>]| {
        let sh = Shared::new();
        sh.restore(&snap0);
        let mut l = TileLog::open(sh.clone()).unwrap();
        let p0 = sh.puts.load(Ordering::SeqCst);
        append_all(&mut l, es).unwrap();
        (sh.puts.load(Ordering::SeqCst) - p0) as i64
    };
    let (ta, tb) = (total(&a), total(&b));
    for k in 0..=ta {
        let sh = Shared::new();
        sh.restore(&snap0);
        let mut l = TileLog::open(sh.clone()).unwrap();
        sh.fail_after(k);
        let ra = append_all(&mut l, &a);
        sh.unlimited();
        let a_ok = ra.is_ok();
        assert_eq!(a_ok, k >= ta, "base={base} k={k}: result");
        let mut durable = base_es.clone();
        if a_ok {
            durable.extend(a.iter().cloned());
        }
        // crash here: reopen from the raw store state
        check_log(&sh, &durable, &format!("base={base} A k={k} reopen"));
        // stage 2: retry a different batch on the same instance with a failure at every put
        let snap1 = sh.snapshot();
        for j in 0..=tb {
            sh.restore(&snap1);
            let mut l2 = TileLog::open(sh.clone()).unwrap();
            // also try the stale instance `l` once (same state as l2 as far as the committed data is concerned)
            let inst: &mut TileLog<Shared> = if j % 2 == 0 { &mut l2 } else { &mut l };
            if j % 2 == 1 && a_ok {
                continue; // `l` already advanced; only reuse it when A failed
            }
            sh.fail_after(j);
            let rb = append_all(inst, &b);
            sh.unlimited();
            let mut d2 = durable.clone();
            if rb.is_ok() {
                d2.extend(b.iter().cloned());
            }
            // append more until past the next boundary, then verify again
            check_log(&sh, &d2, &format!("base={base} A k={k} B j={j} reopen"));
            let mut l3 = TileLog::open(sh.clone()).unwrap();
            let tail: Vec<Vec<u8>> = (0..300).map(|i| ent("T", i)).collect();
            append_all(&mut l3, &tail).unwrap();
            d2.extend(tail);
            check_log(&sh, &d2, &format!("base={base} A k={k} B j={j} +tail"));
        }
        sh.restore(&snap1);
    }
}

#[test]
fn holds_crash_matrix_leaf_boundary() {
    crash_matrix(250, 12, 3); // A crosses 256, B stays below
    crash_matrix(250, 3, 12); // A stays below, B crosses
    crash_matrix(256, 5, 1); // exactly on a boundary
    crash_matrix(255, 1, 1);
}

#[test]
#[ignore = "slow (~200 s debug): full verify_integrity of 65k entries per injection point"]
fn holds_crash_matrix_level1_boundary() {
    crash_matrix(65534, 4, 1); // A crosses 65536 (level-2 tile appears)
    crash_matrix(65535, 1, 2);
}

// ----- 2. stale writer clobbers committed data -----

/// Two instances opened at the same state. A appends (commit). B (stale) appends a DIFFERENT entry: it fails with
/// Conflict at the commit CAS, but only after it has overwritten A's committed partial bundle and hash tile
/// (same path because same resulting width). The store is then unopenable.
#[test]
fn bug_stale_writer_clobbers_committed_partial_bundle() {
    let sh = Shared::new();
    let base: Vec<Vec<u8>> = (0..4).map(|i| ent("b", i)).collect();
    {
        let mut l = TileLog::open(sh.clone()).unwrap();
        append_all(&mut l, &base).unwrap();
    }
    let mut a = TileLog::open(sh.clone()).unwrap();
    let mut b = TileLog::open(sh.clone()).unwrap();
    let ea = vec![b"from-A".to_vec()];
    let eb = vec![b"from-B".to_vec()];
    append_all(&mut a, &ea).unwrap();
    let rb = append_all(&mut b, &eb);
    assert!(rb.is_err(), "stale writer must be rejected");
    let mut expect = base.clone();
    expect.extend(ea);
    let log = TileLog::open(sh.clone()).unwrap_or_else(|e| {
        panic!("A's committed state destroyed by rejected stale writer B: {e}")
    });
    assert_eq!(log.root().unwrap(), root_of(&expect));
    assert_eq!(log.entry(4).unwrap(), b"from-A");
}

/// Same through FsStore: the flock does not help when the first writer has been dropped (process exit) while the
/// second, stale, instance was opened earlier.
#[test]
fn bug_stale_writer_after_first_writer_exits_bricks_fsstore() {
    let dir = std::env::temp_dir().join(format!("vc-w1c-{}-{}", std::process::id(), 1));
    let _ = std::fs::remove_dir_all(&dir);
    {
        let mut l = TileLog::open(FsStore::new(&dir).unwrap()).unwrap();
        for i in 0..4 {
            l.append(&entry(i)).unwrap();
        }
    }
    let mut stale = TileLog::open(FsStore::new(&dir).unwrap()).unwrap();
    {
        let mut a = TileLog::open(FsStore::new(&dir).unwrap()).unwrap();
        a.append(b"from-A").unwrap();
    } // A exits, releases lock
    let r = stale.append(b"from-B");
    assert!(r.is_err(), "stale writer must be rejected");
    let log = TileLog::open(FsStore::new(&dir).unwrap());
    let _ = std::fs::remove_dir_all(&dir);
    let log = log.unwrap_or_else(|e| panic!("store bricked by rejected stale writer: {e}"));
    assert_eq!(log.size(), 5);
    assert_eq!(log.entry(4).unwrap(), b"from-A");
}

// ----- 3. verified cache pins the verdict, not the bytes -----

/// After a full hash tile and entry bundle were read once (marked verified), replacing both on the store with a
/// self-consistent forged pair is accepted: `entry()` returns the forged entry as if verified.
#[test]
fn bug_verified_cache_accepts_tampering_after_first_read() {
    let sh = Shared::new();
    let es: Vec<Vec<u8>> = (0..600).map(|i| ent("e", i)).collect();
    let mut log = TileLog::open(sh.clone()).unwrap();
    append_all(&mut log, &es).unwrap();
    let log = TileLog::open(sh.clone()).unwrap();
    assert_eq!(log.entry(7).unwrap(), es[7]); // tile (0,0) now "verified"
                                              // forge entry 7 in bundle 0 and the matching leaf hash in tile 0
    let bpath = entry_tile_path(0, 256);
    let tpath = hash_tile_path(0, 0, 256);
    let mut forged = Vec::new();
    let mut hashes = Vec::new();
    for (i, e) in es.iter().take(256).enumerate() {
        let e = if i == 7 {
            b"FORGED".to_vec()
        } else {
            e.clone()
        };
        forged.extend((e.len() as u16).to_be_bytes());
        forged.extend(&e);
        hashes.extend(leaf_hash(&e));
    }
    sh.raw_put(&bpath, &forged);
    sh.raw_put(&tpath, &hashes);
    match log.entry(7) {
        Err(_) => {}
        Ok(e) => panic!(
            "forged entry {:?} returned as verified (tile content re-read without re-verification)",
            String::from_utf8_lossy(&e)
        ),
    }
}

// ----- 4. legacy (no commit record) inference makes an unacknowledged batch durable -----

#[test]
fn bug_legacy_store_failed_batch_becomes_durable_after_reopen() {
    let sh = Shared::new();
    let base: Vec<Vec<u8>> = (0..100).map(|i| ent("b", i)).collect();
    {
        let mut l = TileLog::open(sh.clone()).unwrap();
        append_all(&mut l, &base).unwrap();
    }
    sh.raw_remove("meta/commit"); // pre-commit-record store
    let mut l = TileLog::open(sh.clone()).unwrap();
    assert_eq!(l.size(), 100);
    let extra: Vec<Vec<u8>> = (0..50).map(|i| ent("x", i)).collect();
    sh.fail_after(1); // entry bundle lands, then failure before the commit record
    assert!(append_all(&mut l, &extra).is_err());
    sh.unlimited();
    drop(l);
    let l2 = TileLog::open(sh.clone()).unwrap();
    assert_eq!(
        l2.size(),
        100,
        "unacknowledged (failed) batch of 50 became durable on reopen via size inference"
    );
}

// ----- 5. indeterminate commit (put applied but Err) -----

#[test]
fn holds_indeterminate_commit_then_append_is_refused_and_reopen_is_consistent() {
    for base in [3usize, 255, 256] {
        let inner = Shared::new();
        let st = ApplyErr {
            inner: inner.clone(),
            countdown: Arc::new(AtomicI64::new(-1)),
        };
        let be: Vec<Vec<u8>> = (0..base).map(|i| ent("b", i)).collect();
        let mut l = TileLog::open(st.clone()).unwrap();
        append_all(&mut l, &be).unwrap();
        let a: Vec<Vec<u8>> = (0..3).map(|i| ent("A", i)).collect();
        // find the commit put: it is the last put of the batch
        let probe = Shared::new();
        probe.restore(&inner.snapshot());
        let mut lp = TileLog::open(probe.clone()).unwrap();
        let p0 = probe.puts.load(Ordering::SeqCst);
        append_all(&mut lp, &a).unwrap();
        let n = (probe.puts.load(Ordering::SeqCst) - p0) as i64;
        st.countdown.store(n - 1, Ordering::SeqCst);
        assert!(append_all(&mut l, &a).is_err());
        // instance view is still the old state and self-consistent
        assert_eq!(l.size(), base as u64);
        assert_eq!(l.root().unwrap(), root_of(&be));
        // next append must not silently fork: refused
        let b: Vec<Vec<u8>> = vec![b"B".to_vec()];
        assert!(
            append_all(&mut l, &b).is_err(),
            "base={base}: append after indeterminate commit"
        );
        let mut all = be.clone();
        all.extend(a);
        check_log(&inner, &all, &format!("base={base} indeterminate"));
    }
}

// ----- 6. commit record tampering -----

fn committed_store(n: usize) -> (Shared, Vec<Vec<u8>>) {
    let sh = Shared::new();
    let es: Vec<Vec<u8>> = (0..n).map(|i| ent("e", i)).collect();
    let mut l = TileLog::open(sh.clone()).unwrap();
    for ch in es.chunks(500) {
        append_all(&mut l, ch).unwrap();
    }
    (sh, es)
}

fn hex(h: &Hash) -> String {
    h.iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn holds_commit_record_tampering_never_yields_wrong_data() {
    let (sh, es) = committed_store(600);
    let good = sh.raw_get("meta/commit").unwrap();
    let root = root_of(&es);
    let rh = hex(&root);
    let m = "vc-tilelog-commit-v1";
    let mut cases: Vec<(String, Vec<u8>)> = vec![
        ("empty".into(), vec![]),
        ("garbage".into(), vec![0xff, 0xfe, 0, 1]),
        (
            "no trailing nl".into(),
            format!("{m}\n600\n{rh}").into_bytes(),
        ),
        (
            "crlf".into(),
            format!("{m}\r\n600\r\n{rh}\r\n").into_bytes(),
        ),
        ("size+".into(), format!("{m}\n+600\n{rh}\n").into_bytes()),
        (
            "size lead zero".into(),
            format!("{m}\n0600\n{rh}\n").into_bytes(),
        ),
        (
            "upper hex".into(),
            format!("{m}\n600\n{}\n", rh.to_uppercase()).into_bytes(),
        ),
        (
            "extra line".into(),
            format!("{m}\n600\n{rh}\nx\n").into_bytes(),
        ),
        (
            "short root".into(),
            format!("{m}\n600\n{}\n", &rh[..62]).into_bytes(),
        ),
        (
            "multibyte hex".into(),
            format!("{m}\n600\n{}\n", "é".repeat(32)).into_bytes(),
        ),
        (
            "size u64 max".into(),
            format!("{m}\n{}\n{rh}\n", u64::MAX).into_bytes(),
        ),
        (
            "size 2^64".into(),
            format!("{m}\n18446744073709551616\n{rh}\n").into_bytes(),
        ),
        (
            "size 2^63".into(),
            format!("{m}\n{}\n{rh}\n", 1u64 << 63).into_bytes(),
        ),
        (
            "size 2^64-256".into(),
            format!("{m}\n{}\n{rh}\n", u64::MAX - 255).into_bytes(),
        ),
        (
            "size 2^56".into(),
            format!("{m}\n{}\n{rh}\n", 1u64 << 56).into_bytes(),
        ),
        ("size 601".into(), format!("{m}\n601\n{rh}\n").into_bytes()),
        ("size 599".into(), format!("{m}\n599\n{rh}\n").into_bytes()),
        ("size 512".into(), format!("{m}\n512\n{rh}\n").into_bytes()),
        ("size 256".into(), format!("{m}\n256\n{rh}\n").into_bytes()),
        ("size 0".into(), format!("{m}\n0\n{rh}\n").into_bytes()),
        (
            "root zero".into(),
            format!("{m}\n600\n{}\n", "0".repeat(64)).into_bytes(),
        ),
        (
            "root of empty, size 600".into(),
            format!("{m}\n600\n{}\n", hex(&sha256(b""))).into_bytes(),
        ),
    ];
    for s in [0usize, 1, 255, 256, 257, 511, 512, 599, 600] {
        // prefix-consistent commits (size + correct prefix root): must either be rejected or be exactly that prefix
        cases.push((
            format!("prefix {s}"),
            format!("{m}\n{s}\n{}\n", hex(&root_of(&es[..s]))).into_bytes(),
        ));
    }
    for (name, bytes) in cases {
        let sh2 = Shared::new();
        sh2.restore(&sh.snapshot());
        sh2.raw_put("meta/commit", &bytes);
        let r =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| TileLog::open(sh2.clone())));
        let r = r.unwrap_or_else(|_| panic!("{name}: open panicked"));
        if let Ok(l) = r {
            let n = l.size() as usize;
            assert!(n <= es.len(), "{name}: size {n} exceeds stored entries");
            assert_eq!(
                l.root().unwrap(),
                root_of(&es[..n]),
                "{name}: accepted with wrong root"
            );
            if name.starts_with("prefix") || name == "size 0" {
                l.verify_integrity().unwrap();
            } else {
                panic!("{name}: lying commit record accepted (size {n})");
            }
        }
    }
    let _ = good;
}

// ----- 7. edge tile / bundle tampering -----

#[test]
fn holds_tampering_is_detected_never_silently_served() {
    let (sh, es) = committed_store(600); // 2 full bundles + 88
    let paths = sh.paths();
    for p in paths.iter().filter(|p| p.starts_with("tile/")) {
        let orig = sh.raw_get(p).unwrap();
        for mode in 0..3 {
            let sh2 = Shared::new();
            sh2.restore(&sh.snapshot());
            let mut t = orig.clone();
            match mode {
                0 => {
                    if t.is_empty() {
                        continue;
                    }
                    let n = t.len() / 2;
                    t[n] ^= 1
                }
                1 => {
                    if t.len() < 2 {
                        continue;
                    }
                    t.truncate(t.len() - 1)
                }
                _ => {
                    sh2.raw_remove(p);
                }
            }
            if mode != 2 {
                sh2.raw_put(p, &t);
            }
            // either open fails, or every subsequent read (entries, scan, integrity, root, proofs) is correct-or-error
            match TileLog::open(sh2.clone()) {
                Err(_) => {}
                Ok(l) => {
                    let ok_root = l.root().map(|r| r == root_of(&es)).unwrap_or(true);
                    assert!(ok_root, "{p} mode {mode}: wrong root served");
                    for i in (0..600u64).step_by(37) {
                        if let Ok(e) = l.entry(i) {
                            assert_eq!(
                                e, es[i as usize],
                                "{p} mode {mode}: wrong entry {i} served"
                            );
                        }
                        if let Ok(h) = l.leaf_hash_at(i) {
                            assert_eq!(
                                h,
                                leaf_hash(&es[i as usize]),
                                "{p} mode {mode}: wrong leaf hash {i}"
                            );
                        }
                    }
                    let _ = BTreeMap::<u8, u8>::new();
                    let tampered_entry_or_tile = p.contains("entries") || p.starts_with("tile/0");
                    // (a deleted derived hash tile may legitimately be self-healed by open(); only audit the rest)
                    if tampered_entry_or_tile && (mode != 2 || p.contains("entries")) {
                        // a deep audit must notice (unless the object is unreferenced garbage)
                        let referenced = !p.contains(".p/"); // partial derived tiles are self-healed by open()
                        if referenced && l.verify_integrity().is_ok() {
                            panic!("{p} mode {mode}: verify_integrity passed on a tampered/missing store");
                        }
                    }
                }
            }
        }
    }
}
