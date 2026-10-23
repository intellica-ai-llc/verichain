//! vc-log: RFC 9162-style Merkle tree hashing, inclusion and consistency proofs.
pub mod checkpoint;
use vc_core::hash::{leaf_hash, node_hash, sha256, Hash};

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum ProofError {
    #[error("index or size out of range")]
    Range,
    #[error("proof does not verify")]
    Invalid,
}

/// Largest power of two strictly less than n (n >= 2).
fn split(n: usize) -> usize {
    1usize << (usize::BITS - 1 - (n - 1).leading_zeros())
}

/// Merkle tree hash over already-hashed leaves (RFC 9162 §2.1.1).
pub fn root(leaves: &[Hash]) -> Hash {
    match leaves.len() {
        0 => sha256(b""),
        1 => leaves[0],
        n => {
            let k = split(n);
            node_hash(&root(&leaves[..k]), &root(&leaves[k..]))
        }
    }
}

pub fn leaves_from_entries(entries: &[&[u8]]) -> Vec<Hash> {
    entries.iter().map(|e| leaf_hash(e)).collect()
}

/// Inclusion path for leaf `m` in a tree of `leaves` (RFC 9162 §2.1.3.1).
pub fn inclusion_proof(leaves: &[Hash], m: usize) -> Result<Vec<Hash>, ProofError> {
    if m >= leaves.len() {
        return Err(ProofError::Range);
    }
    fn path(m: usize, d: &[Hash]) -> Vec<Hash> {
        if d.len() == 1 {
            return vec![];
        }
        let k = split(d.len());
        if m < k {
            let mut p = path(m, &d[..k]);
            p.push(root(&d[k..]));
            p
        } else {
            let mut p = path(m - k, &d[k..]);
            p.push(root(&d[..k]));
            p
        }
    }
    Ok(path(m, leaves))
}

/// Verify inclusion (RFC 9162 §2.1.3.2).
pub fn verify_inclusion(leaf: &Hash, index: usize, size: usize, path: &[Hash], expected_root: &Hash) -> Result<(), ProofError> {
    if index >= size {
        return Err(ProofError::Range);
    }
    let (mut fnum, mut snum) = (index, size - 1);
    let mut r = *leaf;
    for p in path {
        if snum == 0 {
            return Err(ProofError::Invalid);
        }
        if fnum & 1 == 1 || fnum == snum {
            r = node_hash(p, &r);
            if fnum & 1 == 0 {
                while fnum & 1 == 0 && fnum != 0 {
                    fnum >>= 1;
                    snum >>= 1;
                }
            }
        } else {
            r = node_hash(&r, p);
        }
        fnum >>= 1;
        snum >>= 1;
    }
    if snum == 0 && r == *expected_root { Ok(()) } else { Err(ProofError::Invalid) }
}

/// Consistency proof between the first `m` leaves and all `leaves` (RFC 9162 §2.1.4.1).
pub fn consistency_proof(leaves: &[Hash], m: usize) -> Result<Vec<Hash>, ProofError> {
    let n = leaves.len();
    if m == 0 || m > n {
        return Err(ProofError::Range);
    }
    fn sub(m: usize, d: &[Hash], b: bool) -> Vec<Hash> {
        let n = d.len();
        if m == n {
            return if b { vec![] } else { vec![root(d)] };
        }
        let k = split(n);
        if m <= k {
            let mut p = sub(m, &d[..k], b);
            p.push(root(&d[k..]));
            p
        } else {
            let mut p = sub(m - k, &d[k..], false);
            p.push(root(&d[..k]));
            p
        }
    }
    Ok(sub(m, leaves, true))
}

/// Verify consistency (RFC 9162 §2.1.4.2).
pub fn verify_consistency(first: usize, second: usize, first_root: &Hash, second_root: &Hash, proof: &[Hash]) -> Result<(), ProofError> {
    if first == 0 || first > second {
        return Err(ProofError::Range);
    }
    if first == second {
        return if proof.is_empty() && first_root == second_root { Ok(()) } else { Err(ProofError::Invalid) };
    }
    let mut path: Vec<Hash> = Vec::with_capacity(proof.len() + 1);
    if first.is_power_of_two() {
        path.push(*first_root);
    }
    path.extend_from_slice(proof);
    if path.is_empty() {
        return Err(ProofError::Invalid);
    }
    let (mut fnum, mut snum) = (first - 1, second - 1);
    while fnum & 1 == 1 {
        fnum >>= 1;
        snum >>= 1;
    }
    let (mut fr, mut sr) = (path[0], path[0]);
    for c in &path[1..] {
        if snum == 0 {
            return Err(ProofError::Invalid);
        }
        if fnum & 1 == 1 || fnum == snum {
            fr = node_hash(c, &fr);
            sr = node_hash(c, &sr);
            if fnum & 1 == 0 {
                while fnum != 0 && fnum & 1 == 0 {
                    fnum >>= 1;
                    snum >>= 1;
                }
            }
        } else {
            sr = node_hash(&sr, c);
        }
        fnum >>= 1;
        snum >>= 1;
    }
    if fr == *first_root && sr == *second_root && snum == 0 { Ok(()) } else { Err(ProofError::Invalid) }
}
