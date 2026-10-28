//! W1b adversarial verifier tests for vc-store (numeric extremes, DoS/resource, read amplification).
//!
//! Conventions: `bug_*` assert the CORRECT behaviour and FAIL against current code (see
//! docs/reports/verifier_w1b_store_seq.md); `holds_*` pass today and are regression tests; `observe_*` document
//! behaviour and always pass. Run also with `--release` (wraparound semantics differ from debug).
#[path = "common/mod.rs"]
mod common;

use common::Shared;
use std::collections::HashMap;
use std::io;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use vc_core::hash::{leaf_hash, node_hash, Hash};
use vc_log::{verify_consistency, verify_inclusion};
use vc_store::tile::{entry_tile_path, hash_tile_path, index_path, TILE_WIDTH};
use vc_store::{FsStore, MemStore, StoreError, TileLog, TileStore};

// ------------------------------------------------------------------------------------------------------------
// helpers
// ------------------------------------------------------------------------------------------------------------

/// Store wrapper that counts the bytes handed back by `get`.
#[derive(Clone)]
struct Counting {
    inner: Shared,
    bytes: Arc<AtomicU64>,
    gets: Arc<AtomicU64>,
}

impl Counting {
    fn new(inner: Shared) -> Self {
        Counting {
            inner,
            bytes: Arc::new(AtomicU64::new(0)),
            gets: Arc::new(AtomicU64::new(0)),
        }
    }
    fn reset(&self) {
        self.bytes.store(0, Ordering::SeqCst);
        self.gets.store(0, Ordering::SeqCst);
    }
}

impl TileStore for Counting {
    fn get(&self, path: &str) -> io::Result<Option<Vec<u8>>> {
        let r = self.inner.get(path)?;
        self.gets.fetch_add(1, Ordering::SeqCst);
        if let Some(b) = &r {
            self.bytes.fetch_add(b.len() as u64, Ordering::SeqCst);
        }
        Ok(r)
    }
    fn put(&self, path: &str, data: &[u8]) -> io::Result<()> {
        self.inner.put(path, data)
    }
}

fn pad_entry(i: usize, len: usize) -> Vec<u8> {
    let mut v = vec![(i % 251) as u8; len];
    v[..8].copy_from_slice(&(i as u64).to_be_bytes());
    v
}

// ------------------------------------------------------------------------------------------------------------
// ST-9: read amplification (entry() / recover / replay read the whole bundle per entry)
// ------------------------------------------------------------------------------------------------------------

/// FINDING ST-9. `TileLog::open` -> `recover()` calls `entry(i)` for every i; each call fetches and parses the
/// whole 256-entry bundle just to return one entry, so recovery reads ~128x (average 128 of 256 per bundle,
/// worst 256x) the stored bytes. With 64 KB statements the amplification is 16 MB of I/O per recovered entry.
#[test]
fn bug_recover_reads_each_entry_bundle_once() {
    const N: usize = 1100;
    const LEN: usize = 2000;
    let sh = Shared::new();
    {
        let mut log = TileLog::open(sh.clone()).unwrap();
        let es: Vec<Vec<u8>> = (0..N).map(|i| pad_entry(i, LEN)).collect();
        for c in es.chunks(256) {
            let r: Vec<&[u8]> = c.iter().map(|e| e.as_slice()).collect();
            log.append_batch(&r).unwrap();
        }
    }
    // lose every hash tile: entries are the source of truth, so open must rebuild them
    for p in sh.paths() {
        let is_hash = p
            .strip_prefix("tile/")
            .is_some_and(|r| r.as_bytes()[0].is_ascii_digit());
        if is_hash {
            sh.raw_remove(&p);
        }
    }
    let stored: u64 = sh
        .paths()
        .iter()
        .filter(|p| p.starts_with("tile/entries/") && !p.contains(".p/"))
        .map(|p| sh.raw_get(p).unwrap().len() as u64)
        .sum();
    let c = Counting::new(sh.clone());
    let t = Instant::now();
    let log = TileLog::open(c.clone()).unwrap();
    let el = t.elapsed();
    assert_eq!(log.size() as usize, N);
    let read = c.bytes.load(Ordering::SeqCst);
    eprintln!(
        "ST-9: recover read {read} B for {stored} B of full bundles ({:.0}x), {el:?}",
        read as f64 / stored as f64
    );
    assert!(
        read <= 4 * stored.max(1),
        "recovery read {read} bytes to rebuild from {stored} bytes of entry bundles ({:.0}x)",
        read as f64 / stored as f64
    );
}

/// FINDING ST-10. `entry()` for an index inside the current partial bundle clones the ENTIRE in-memory
/// `entry_edge` (up to 255 x 65535 B = 16 MB) on every call, then parses it. A lookup of a 60 KB entry costs
/// milliseconds instead of microseconds, and replay/duplicate checks inherit it.
#[test]
fn bug_entry_lookup_in_partial_bundle_does_not_clone_whole_edge() {
    let mut log = TileLog::open(MemStore::new()).unwrap();
    let es: Vec<Vec<u8>> = (0..255).map(|i| pad_entry(i, 60_000)).collect();
    let r: Vec<&[u8]> = es.iter().map(|e| e.as_slice()).collect();
    log.append_batch(&r).unwrap();
    let mut d: Vec<Duration> = Vec::new();
    for _ in 0..60 {
        let t = Instant::now();
        let e = log.entry(100).unwrap();
        d.push(t.elapsed());
        assert_eq!(e.len(), 60_000);
    }
    d.sort();
    let med = d[d.len() / 2];
    eprintln!("ST-9(b): median entry() in a 15 MB partial bundle = {med:?}");
    assert!(
        med < Duration::from_micros(200),
        "median lookup {med:?} (copies the whole 15 MB edge per call)"
    );
}

#[test]
fn observe_full_bundle_lookup_cost() {
    let sh = Shared::new();
    let mut log = TileLog::open(sh.clone()).unwrap();
    let es: Vec<Vec<u8>> = (0..512).map(|i| pad_entry(i, 1000)).collect();
    let r: Vec<&[u8]> = es.iter().map(|e| e.as_slice()).collect();
    log.append_batch(&r).unwrap();
    let c = Counting::new(sh);
    let log = TileLog::open(c.clone()).unwrap();
    c.reset();
    log.entry(3).unwrap();
    eprintln!(
        "observe: entry(3) of a 1000 B entry fetched {} B from the store",
        c.bytes.load(Ordering::SeqCst)
    );
}

// ------------------------------------------------------------------------------------------------------------
// Numeric extremes: synthetic store that serves a *consistent* huge log of empty entries without storing it
// ------------------------------------------------------------------------------------------------------------

/// A store that synthesises a log of `bundles * 256` empty entries (every leaf hash is `leaf_hash(b"")`) and the
/// matching, internally consistent hash tiles for every level.
struct Synth {
    bundles: u64,
    size: u64,
    p: Vec<Hash>, // p[l] = hash of a perfect subtree of 256^l identical leaves
    bundle: Vec<u8>,
}

fn parse_index(rest: &str) -> Option<u64> {
    let mut n: u64 = 0;
    for g in rest.split('/') {
        let g = g.strip_prefix('x').unwrap_or(g);
        if g.len() != 3 {
            return None;
        }
        n = n.checked_mul(1000)?.checked_add(g.parse::<u64>().ok()?)?;
    }
    Some(n)
}

fn mth_pow2_same(l: Hash, log2: u32) -> Hash {
    let mut h = l;
    for _ in 0..log2 {
        h = node_hash(&h, &h);
    }
    h
}

/// MTH of n identical leaves (independent of the code under test).
fn mth_same(leaf: Hash, n: u64) -> Hash {
    assert!(n >= 1);
    if n == 1 {
        return leaf;
    }
    let k = 1u64 << (63 - (n - 1).leading_zeros());
    let left = mth_pow2_same(leaf, k.trailing_zeros());
    if n == k {
        return left;
    }
    node_hash(&left, &mth_same(leaf, n - k))
}

impl Synth {
    fn new(bundles: u64) -> Self {
        let l0 = leaf_hash(b"");
        let mut p = vec![l0];
        for _ in 0..7 {
            let last = *p.last().unwrap();
            p.push(mth_pow2_same(last, 8));
        }
        Synth {
            bundles,
            size: bundles * TILE_WIDTH,
            p,
            bundle: [0u8, 0u8].repeat(256),
        }
    }
}

impl TileStore for Synth {
    fn get(&self, path: &str) -> io::Result<Option<Vec<u8>>> {
        let Some(rest) = path.strip_prefix("tile/") else {
            return Ok(None); // metadata (commit record) is not synthesised: exercises the pre-commit open path
        };
        let (kind, rest) = rest.split_once('/').unwrap();
        let (idx_s, width) = match rest.split_once(".p/") {
            Some((i, w)) => (i, Some(w.parse::<u64>().unwrap())),
            None => (rest, None),
        };
        let n = parse_index(idx_s).unwrap();
        if kind == "entries" {
            return Ok((width.is_none() && n < self.bundles).then(|| self.bundle.clone()));
        }
        let l: usize = kind.parse().unwrap();
        let count = self.size >> (8 * l);
        let (full, w) = (count / 256, count % 256);
        let hit = match width {
            None => n < full,
            Some(x) => n == full && x == w && w > 0,
        };
        Ok(hit.then(|| {
            let k = width.unwrap_or(256) as usize;
            self.p[l]
                .iter()
                .copied()
                .cycle()
                .take(32 * k)
                .collect::<Vec<u8>>()
        }))
    }
    fn put(&self, _path: &str, _data: &[u8]) -> io::Result<()> {
        panic!("synthetic store is read-only")
    }
}

fn check_big(bundles: u64) {
    let l0 = leaf_hash(b"");
    let log = TileLog::open(Synth::new(bundles)).unwrap();
    let size = bundles * 256;
    assert_eq!(log.size(), size);
    assert_eq!(log.root().unwrap(), mth_same(l0, size), "root of {size}");
    let sizes = [
        1,
        255,
        256,
        257,
        65_535,
        65_536,
        65_537,
        size / 2 + 3,
        size - 1,
        size,
    ];
    for &sz in &sizes {
        if sz > size || sz == 0 {
            continue;
        }
        let root = log.root_at(sz).unwrap();
        assert_eq!(root, mth_same(l0, sz), "root_at({sz})");
        for &i in &[0, 1, sz / 2, sz.saturating_sub(2), sz - 1] {
            if i >= sz {
                continue;
            }
            let p = log.inclusion_proof(i, sz).unwrap();
            assert!(p.len() <= 64);
            verify_inclusion(&l0, i as usize, sz as usize, &p, &root)
                .unwrap_or_else(|e| panic!("inclusion {i}/{sz}: {e:?}"));
            assert_eq!(log.leaf_hash_at(i).unwrap(), l0);
        }
        for &m in &[1, sz / 3 + 1, sz / 2, sz - 1, sz] {
            if m == 0 || m > sz {
                continue;
            }
            let p = log.consistency_proof(m, sz).unwrap();
            let fr = log.root_at(m).unwrap();
            verify_consistency(m as usize, sz as usize, &fr, &root, &p)
                .unwrap_or_else(|e| panic!("consistency {m}->{sz}: {e:?}"));
        }
    }
    // extreme / hostile arguments on a huge log: always Err, never panic or wrap
    for &(i, n) in &[
        (u64::MAX, u64::MAX),
        (0, u64::MAX),
        (size, size),
        (size - 1, size + 1),
        (u64::MAX - 1, size),
        (1u64 << 63, 1u64 << 63),
        (0, 1u64 << 63),
    ] {
        assert!(
            matches!(log.inclusion_proof(i, n), Err(StoreError::Range)),
            "inclusion_proof({i},{n})"
        );
    }
    for &(m, n) in &[
        (0, size),
        (1, u64::MAX),
        (u64::MAX, u64::MAX),
        (size + 1, size + 1),
        (size, size + 1),
        (1u64 << 63, 1u64 << 63),
        (u64::MAX, 1),
    ] {
        assert!(
            matches!(log.consistency_proof(m, n), Err(StoreError::Range)),
            "consistency_proof({m},{n})"
        );
    }
    for &s in &[
        size + 1,
        u64::MAX,
        1u64 << 63,
        (1u64 << 63) + 1,
        u64::MAX - 1,
    ] {
        assert!(matches!(log.root_at(s), Err(StoreError::Range)));
    }
    for &i in &[size, u64::MAX, 1u64 << 63] {
        assert!(matches!(log.entry(i), Err(StoreError::Range)));
        assert!(matches!(log.leaf_hash_at(i), Err(StoreError::Range)));
    }
    assert_eq!(log.entry(size - 1).unwrap(), Vec::<u8>::new());
}

/// 2^24 leaves (level 3 exists, partial tile at level 3): roots, proofs and hostile arguments.
#[test]
fn holds_synthetic_16m_log_roots_proofs_and_extreme_args() {
    check_big(65_536);
}

/// An awkward size: 2^24 + 2^16 + 256 leaves (partial tiles at several levels).
#[test]
fn holds_synthetic_irregular_huge_log() {
    check_big(65_536 + 256 + 1);
}

/// > 2^32 leaves (checks usize/u32 truncation). Needs 16.7M store probes at open.
#[test]
#[ignore = "slow in debug (16.7M synthetic get calls, ~3 min); run with --release --include-ignored"]
fn holds_synthetic_2pow32_log() {
    check_big(1u64 << 24);
}

// ------------------------------------------------------------------------------------------------------------
// Tile path addressing for extreme indices
// ------------------------------------------------------------------------------------------------------------

fn decode_index_path(p: &str) -> u64 {
    parse_index(p).unwrap_or_else(|| panic!("undecodable {p}"))
}

#[test]
fn holds_index_path_roundtrip_and_shape() {
    let mut xs: Vec<u64> = vec![
        0,
        1,
        9,
        10,
        99,
        100,
        999,
        1000,
        1001,
        999_999,
        1_000_000,
        1_000_001,
        u32::MAX as u64,
        1 << 53,
        (1 << 53) - 1,
        (1 << 53) + 1,
        u64::MAX - 1,
        u64::MAX,
        1 << 63,
    ];
    let mut k = 1u64;
    while k < u64::MAX / 10 {
        xs.extend([k - 1, k, k + 1]);
        k *= 10;
    }
    let mut seen = HashMap::new();
    for &n in &xs {
        let p = index_path(n);
        assert_eq!(decode_index_path(&p), n, "roundtrip {n} via {p}");
        let parts: Vec<&str> = p.split('/').collect();
        assert!(parts.iter().all(|g| g.trim_start_matches('x').len() == 3));
        assert!(
            parts[..parts.len() - 1].iter().all(|g| g.starts_with('x'))
                && !parts.last().unwrap().starts_with('x')
        );
        assert!(p.len() < 255, "component limits");
        if let Some(prev) = seen.insert(p.clone(), n) {
            assert_eq!(prev, n, "collision {p}");
        }
    }
}

/// No tile path may be a directory prefix of another (file/dir clash on a real filesystem), and distinct
/// (kind, index, width) triples map to distinct paths, including partial widths and extreme indices.
#[test]
fn holds_tile_paths_injective_and_prefix_free() {
    let idxs = [0u64, 1, 999, 1000, 1_000_000, 1 << 53, u64::MAX];
    let mut all: Vec<String> = Vec::new();
    for &n in &idxs {
        for w in [1u64, 2, 127, 255, 256] {
            for l in [0u32, 1, 7, 8, 63, u32::MAX] {
                all.push(hash_tile_path(l, n, w));
            }
            all.push(entry_tile_path(n, w));
        }
    }
    let mut s = all.clone();
    s.sort();
    s.dedup();
    assert_eq!(s.len(), all.len(), "path collision");
    for a in &s {
        for b in &s {
            assert!(
                !b.starts_with(&format!("{a}/")),
                "{a} is a file and a directory of {b}"
            );
        }
    }
}

#[test]
fn holds_fsstore_roundtrip_at_extreme_tile_paths() {
    let dir = std::env::temp_dir().join(format!("vc-w1b-{}-{}", std::process::id(), line!()));
    let _ = std::fs::remove_dir_all(&dir);
    let fs = FsStore::new(&dir).unwrap();
    let paths = [
        entry_tile_path(u64::MAX, 256),
        entry_tile_path(u64::MAX, 7),
        hash_tile_path(7, u64::MAX, 256),
        hash_tile_path(7, u64::MAX, 255),
        hash_tile_path(u32::MAX, 1 << 53, 1),
    ];
    for (i, p) in paths.iter().enumerate() {
        fs.put(p, &[i as u8; 5]).unwrap();
    }
    for (i, p) in paths.iter().enumerate() {
        assert_eq!(fs.get(p).unwrap().unwrap(), vec![i as u8; 5], "{p}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// MAX_LEVELS (8 levels of 256 = 2^64 leaves) cannot be exhausted through the public API: the `size` counter would
/// overflow first, and the level-7 tile would need 2^64 leaves. Documented rather than tested.
#[test]
fn observe_max_levels_unreachable() {
    let s = Synth::new(1 << 12);
    let log = TileLog::open(s).unwrap();
    assert_eq!(log.size(), 1 << 20);
}

// ------------------------------------------------------------------------------------------------------------
// DoS / resource
// ------------------------------------------------------------------------------------------------------------

/// A single batch of 100k empty entries: linear time, correct root, entries recoverable.
#[test]
fn holds_huge_batch_of_tiny_entries() {
    let es: Vec<Vec<u8>> = (0..100_000usize)
        .map(|i| (i as u32).to_be_bytes()[..(i % 5).min(4)].to_vec())
        .collect();
    let r: Vec<&[u8]> = es.iter().map(|e| e.as_slice()).collect();
    let sh = Shared::new();
    let mut log = TileLog::open(sh.clone()).unwrap();
    let t = Instant::now();
    let (first, size) = log.append_batch(&r).unwrap();
    let el = t.elapsed();
    assert_eq!((first, size), (0, 100_000));
    assert!(
        el < Duration::from_secs(30),
        "100k tiny entries took {el:?}"
    );
    let leaves: Vec<Hash> = es.iter().map(|e| leaf_hash(e)).collect();
    assert_eq!(log.root().unwrap(), vc_log::root(&leaves));
    let log2 = TileLog::open(sh).unwrap();
    assert_eq!(log2.root().unwrap(), log.root().unwrap());
    assert_eq!(log2.entry(99_999).unwrap(), es[99_999]);
}

/// Repeated opens on a log must not grow the store (no per-open writes when tiles are intact).
#[test]
fn holds_open_is_read_only_when_intact() {
    let sh = Shared::new();
    {
        let mut log = TileLog::open(sh.clone()).unwrap();
        let es: Vec<Vec<u8>> = (0..700).map(|i| pad_entry(i, 10)).collect();
        let r: Vec<&[u8]> = es.iter().map(|e| e.as_slice()).collect();
        log.append_batch(&r).unwrap();
    }
    let before = sh.puts.load(Ordering::SeqCst);
    for _ in 0..3 {
        TileLog::open(sh.clone()).unwrap();
    }
    assert_eq!(sh.puts.load(Ordering::SeqCst), before);
}

/// Appending one entry at a time in a partially filled bundle rewrites the whole partial bundle and every partial
/// hash tile each time (O(width) per append, O(256^2) bytes per bundle). Reported, not asserted.
#[test]
fn observe_single_append_write_amplification() {
    let sh = Shared::new();
    let mut log = TileLog::open(sh.clone()).unwrap();
    let e = pad_entry(0, 1000);
    for _ in 0..255 {
        log.append(&e).unwrap();
    }
    let total: u64 = sh.puts.load(Ordering::SeqCst);
    eprintln!(
        "observe: 255 single appends of 1000 B = {total} puts; stale partial bundles left: {}",
        sh.paths()
            .iter()
            .filter(|p| p.contains("entries") && p.contains(".p/"))
            .count()
    );
}
