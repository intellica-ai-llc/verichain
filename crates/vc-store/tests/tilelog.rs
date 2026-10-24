//! vc-store tests. Requirement trace: FR-L0-07 (storage part), FR-L0-02 (proof generation).
//! Differential: the tile log must reproduce the independently authored RFC 9162 vectors.
use serde_json::Value;
use vc_core::hash::{from_hex, to_hex, Hash};
use vc_log::{verify_consistency, verify_inclusion};
use vc_store::tile::{entry_tile_path, hash_tile_path, index_path};
use vc_store::*;

fn entry(i: usize) -> Vec<u8> {
    format!("entry-{i}").into_bytes()
}

fn build(n: usize) -> TileLog<MemStore> {
    let mut log = TileLog::open(MemStore::new()).unwrap();
    let es: Vec<Vec<u8>> = (0..n).map(entry).collect();
    let refs: Vec<&[u8]> = es.iter().map(|e| e.as_slice()).collect();
    log.append_batch(&refs).unwrap();
    log
}

fn vectors() -> Value {
    let p = format!(
        "{}/../../spec/vectors/merkle_v1.json",
        env!("CARGO_MANIFEST_DIR")
    );
    serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap()
}

#[test]
fn index_path_encoding() {
    assert_eq!(index_path(0), "000");
    assert_eq!(index_path(67), "067");
    assert_eq!(index_path(999), "999");
    assert_eq!(index_path(1000), "x001/000");
    assert_eq!(index_path(1234067), "x001/x234/067"); // example from the Go tlog reference
    assert_eq!(hash_tile_path(2, 5, 256), "tile/2/005");
    assert_eq!(hash_tile_path(0, 5, 17), "tile/0/005.p/17");
    assert_eq!(
        entry_tile_path(1234067, 1),
        "tile/entries/x001/x234/067.p/1"
    );
}

#[test]
fn roots_match_independent_vectors() {
    let d = vectors();
    let max = d["roots"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["size"].as_u64().unwrap())
        .max()
        .unwrap() as usize;
    let log = build(max);
    for r in d["roots"].as_array().unwrap() {
        let size = r["size"].as_u64().unwrap();
        assert_eq!(
            to_hex(&log.root_at(size).unwrap()),
            r["root"].as_str().unwrap(),
            "size {size}"
        );
    }
}

#[test]
fn proofs_match_independent_vectors() {
    let d = vectors();
    let log = build(1025);
    let hs = |v: &Value| -> Vec<Hash> {
        v.as_array()
            .unwrap()
            .iter()
            .map(|x| from_hex(x.as_str().unwrap()).unwrap())
            .collect()
    };
    for v in d["inclusion"].as_array().unwrap() {
        let (i, n) = (
            v["leaf_index"].as_u64().unwrap(),
            v["tree_size"].as_u64().unwrap(),
        );
        assert_eq!(
            log.inclusion_proof(i, n).unwrap(),
            hs(&v["proof"]),
            "{}",
            v["id"]
        );
    }
    for v in d["consistency"].as_array().unwrap() {
        let (m, n) = (
            v["first_size"].as_u64().unwrap(),
            v["second_size"].as_u64().unwrap(),
        );
        assert_eq!(
            log.consistency_proof(m, n).unwrap(),
            hs(&v["proof"]),
            "{}",
            v["id"]
        );
    }
}

#[test]
fn proofs_verify_across_tile_boundaries() {
    // sizes around 256 and 512 and one beyond 65,536 would be slow; use boundary sizes up to 1100
    let log = build(1100);
    for &n in &[
        1u64, 2, 255, 256, 257, 511, 512, 513, 1023, 1024, 1025, 1100,
    ] {
        let root = log.root_at(n).unwrap();
        for &i in &[0, 1, n / 2, n.saturating_sub(1)] {
            if i < n {
                let p = log.inclusion_proof(i, n).unwrap();
                verify_inclusion(
                    &log.leaf_hash_at(i).unwrap(),
                    i as usize,
                    n as usize,
                    &p,
                    &root,
                )
                .unwrap();
            }
        }
        for &m in &[1u64, n / 2, n.saturating_sub(1), n] {
            if m >= 1 && m <= n {
                let p = log.consistency_proof(m, n).unwrap();
                verify_consistency(m as usize, n as usize, &log.root_at(m).unwrap(), &root, &p)
                    .unwrap();
            }
        }
    }
}

#[test]
fn batch_and_single_appends_agree() {
    let a = build(700);
    let mut b = TileLog::open(MemStore::new()).unwrap();
    for i in 0..700 {
        assert_eq!(b.append(&entry(i)).unwrap(), i as u64);
    }
    assert_eq!(a.root().unwrap(), b.root().unwrap());
    assert_eq!(a.size(), 700);
}

#[test]
fn entries_roundtrip_and_range_errors() {
    let log = build(300);
    for i in [0usize, 1, 255, 256, 299] {
        assert_eq!(log.entry(i as u64).unwrap(), entry(i));
    }
    assert!(matches!(log.entry(300), Err(StoreError::Range)));
    assert!(matches!(log.inclusion_proof(5, 4), Err(StoreError::Range)));
    assert!(matches!(
        log.consistency_proof(0, 4),
        Err(StoreError::Range)
    ));
    assert!(matches!(
        log.consistency_proof(5, 4),
        Err(StoreError::Range)
    ));
    assert!(matches!(log.root_at(301), Err(StoreError::Range)));
    let mut l2 = build(1);
    assert!(matches!(
        l2.append(&vec![0u8; 65536]),
        Err(StoreError::TooLarge)
    ));
}

fn tmpdir(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("vc-store-{}-{}", name, std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    d
}

#[test]
fn persists_and_reopens_on_filesystem() {
    let dir = tmpdir("reopen");
    let root;
    {
        let mut log = TileLog::open(FsStore::new(&dir).unwrap()).unwrap();
        for i in 0..600 {
            log.append(&entry(i)).unwrap();
        }
        root = log.root().unwrap();
    }
    let mut log = TileLog::open(FsStore::new(&dir).unwrap()).unwrap();
    assert_eq!(log.size(), 600);
    assert_eq!(log.root().unwrap(), root);
    log.append(&entry(600)).unwrap();
    assert_eq!(log.root_at(600).unwrap(), root);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn recovers_when_hash_tiles_are_lost() {
    // Simulate a crash after the entry bundle was durable but before hash tiles were written.
    let store = MemStore::new();
    let mut log = TileLog::open(store).unwrap();
    for i in 0..700 {
        log.append(&entry(i)).unwrap();
    }
    let want = log.root().unwrap();
    let src = log;
    let broken = MemStore::new();
    // rebuild bundle bytes by hand from entries
    let mut bundle = Vec::new();
    for i in 0..700usize {
        let e = src.entry(i as u64).unwrap();
        bundle.extend_from_slice(&(e.len() as u16).to_be_bytes());
        bundle.extend_from_slice(&e);
        if (i + 1) % 256 == 0 {
            broken
                .put(&entry_tile_path((i / 256) as u64, 256), &bundle)
                .unwrap();
            bundle.clear();
        }
    }
    broken.put(&entry_tile_path(2, 700 % 256), &bundle).unwrap();
    assert!(broken.paths().iter().all(|p| p.starts_with("tile/entries")));
    let recovered = TileLog::open(broken).unwrap();
    assert_eq!(recovered.size(), 700);
    assert_eq!(recovered.root().unwrap(), want);
}

#[test]
fn corrupt_bundle_is_reported() {
    let store = MemStore::new();
    use vc_store::TileStore;
    store.put(&entry_tile_path(0, 3), &[0, 5, 1, 2]).unwrap(); // declares length 5, has 2 bytes
    assert!(TileLog::open(store).is_err());
}
