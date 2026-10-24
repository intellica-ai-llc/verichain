//! Differential tests against INDEPENDENTLY authored vectors (spec/vectors/*.json, produced by
//! tools/vectors/gen_merkle.py from RFC 9162 alone). Requirement trace: FR-L0-02, FR-L0-03.
use serde_json::Value;
use vc_core::hash::{from_hex, leaf_hash, to_hex, Hash};
use vc_log::*;

fn load(name: &str) -> Value {
    let p = format!("{}/../../spec/vectors/{}", env!("CARGO_MANIFEST_DIR"), name);
    serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap()
}
fn h(v: &Value) -> Hash {
    from_hex(v.as_str().unwrap()).unwrap()
}
fn ho(v: &Value) -> Option<Hash> {
    from_hex(v.as_str()?)
}
fn hso(v: &Value) -> Option<Vec<Hash>> {
    v.as_array()?.iter().map(ho).collect()
}
fn hs(v: &Value) -> Vec<Hash> {
    v.as_array().unwrap().iter().map(h).collect()
}
fn u(v: &Value) -> usize {
    v.as_u64().unwrap() as usize
}
fn leaves(n: usize) -> Vec<Hash> {
    (0..n)
        .map(|i| leaf_hash(format!("entry-{i}").as_bytes()))
        .collect()
}

#[test]
fn roots_match_independent_implementation() {
    let d = load("merkle_v1.json");
    for r in d["roots"].as_array().unwrap() {
        assert_eq!(
            root(&leaves(u(&r["size"]))),
            h(&r["root"]),
            "size {}",
            r["size"]
        );
    }
}

#[test]
fn ct_reference_roots() {
    let d = load("ct_reference_v1.json");
    let entries: Vec<Vec<u8>> = d["entries_hex"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| hex::decode(e.as_str().unwrap()).unwrap())
        .collect();
    let l: Vec<Hash> = entries.iter().map(|e| leaf_hash(e)).collect();
    for r in d["roots"].as_array().unwrap() {
        assert_eq!(
            to_hex(&root(&l[..u(&r["size"])])),
            r["root"].as_str().unwrap()
        );
        assert_eq!(r["matches_recalled"], true);
    }
}

#[test]
fn inclusion_vectors() {
    let d = load("merkle_v1.json");
    for v in d["inclusion"].as_array().unwrap() {
        let (i, n) = (u(&v["leaf_index"]), u(&v["tree_size"]));
        let proof = hs(&v["proof"]);
        verify_inclusion(&h(&v["leaf_hash"]), i, n, &proof, &h(&v["root"]))
            .unwrap_or_else(|e| panic!("{}: {e}", v["id"]));
        // our generator agrees with the independent one
        assert_eq!(
            inclusion_proof(&leaves(n), i).unwrap(),
            proof,
            "{}",
            v["id"]
        );
    }
    let mut accepted_negatives = vec![];
    for v in d["inclusion_negative"].as_array().unwrap() {
        // a hash that is not 32 bytes of hex is rejected at parse time (counts as rejection)
        let (Some(leaf), Some(proof), Some(rt)) =
            (ho(&v["leaf_hash"]), hso(&v["proof"]), ho(&v["root"]))
        else {
            continue;
        };
        let r = verify_inclusion(&leaf, u(&v["leaf_index"]), u(&v["tree_size"]), &proof, &rt);
        if r.is_ok() {
            accepted_negatives.push(v["id"].as_str().unwrap().to_string());
        }
    }
    assert!(
        accepted_negatives.is_empty(),
        "negative inclusion vectors accepted: {accepted_negatives:?}"
    );
}

#[test]
fn consistency_vectors() {
    let d = load("merkle_v1.json");
    for v in d["consistency"].as_array().unwrap() {
        let (m, n) = (u(&v["first_size"]), u(&v["second_size"]));
        let proof = hs(&v["proof"]);
        verify_consistency(m, n, &h(&v["first_root"]), &h(&v["second_root"]), &proof)
            .unwrap_or_else(|e| panic!("{}: {e}", v["id"]));
        assert_eq!(
            consistency_proof(&leaves(n), m).unwrap(),
            proof,
            "{}",
            v["id"]
        );
    }
    let mut accepted = vec![];
    for v in d["consistency_negative"].as_array().unwrap() {
        let (Some(fr), Some(sr), Some(proof)) = (
            ho(&v["first_root"]),
            ho(&v["second_root"]),
            hso(&v["proof"]),
        ) else {
            continue;
        };
        let r = verify_consistency(u(&v["first_size"]), u(&v["second_size"]), &fr, &sr, &proof);
        if r.is_ok() {
            accepted.push(v["id"].as_str().unwrap().to_string());
        }
    }
    assert!(
        accepted.is_empty(),
        "negative consistency vectors accepted: {accepted:?}"
    );
}
