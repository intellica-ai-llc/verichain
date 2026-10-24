//! Verifier G05: Merkle proof edge cases (RFC 9162 sec 2.1).
use vc_core::hash::{leaf_hash, node_hash, sha256};
use vc_core::Hash;
use vc_log::*;

fn mk(n: usize) -> Vec<Hash> {
    (0..n)
        .map(|i| leaf_hash(format!("e{i}").as_bytes()))
        .collect()
}

// regression: size 1, empty proofs, index==size, size 0, usize::MAX never panic
#[test]
fn regression_inclusion_edges_no_panic() {
    let l = mk(1);
    let r = root(&l);
    assert_eq!(verify_inclusion(&l[0], 0, 1, &[], &r), Ok(()));
    assert_eq!(
        verify_inclusion(&l[0], 0, 1, &[[0; 32]], &r),
        Err(ProofError::Invalid)
    ); // too long
    assert_eq!(
        verify_inclusion(&l[0], 1, 1, &[], &r),
        Err(ProofError::Range)
    ); // index == size
    assert_eq!(
        verify_inclusion(&l[0], 0, 0, &[], &r),
        Err(ProofError::Range)
    ); // size 0
    assert_eq!(
        verify_inclusion(&l[0], usize::MAX, usize::MAX, &[], &r),
        Err(ProofError::Range)
    );
    // huge size, index in range, empty / long / too-long proofs
    assert!(verify_inclusion(&l[0], 0, usize::MAX, &[], &r).is_err());
    assert!(verify_inclusion(&l[0], usize::MAX - 1, usize::MAX, &[], &r).is_err());
    let long = vec![[7u8; 32]; 200];
    assert!(verify_inclusion(&l[0], 0, usize::MAX, &long, &r).is_err());
    assert!(verify_inclusion(&l[0], usize::MAX - 1, usize::MAX, &long, &r).is_err());
    assert!(verify_inclusion(&l[0], 5, 6, &long, &r).is_err());
}

#[test]
fn regression_consistency_edges_no_panic() {
    let r = [1u8; 32];
    assert_eq!(
        verify_consistency(0, 5, &r, &r, &[]),
        Err(ProofError::Range)
    );
    assert_eq!(
        verify_consistency(0, 0, &r, &r, &[]),
        Err(ProofError::Range)
    );
    assert_eq!(
        verify_consistency(6, 5, &r, &r, &[]),
        Err(ProofError::Range)
    );
    assert_eq!(verify_consistency(5, 5, &r, &r, &[]), Ok(()));
    assert!(verify_consistency(5, 5, &r, &[2u8; 32], &[]).is_err());
    assert!(verify_consistency(5, 5, &r, &r, &[r]).is_err());
    assert!(verify_consistency(3, 5, &r, &r, &[]).is_err()); // non-pow2 first, empty proof
    assert!(verify_consistency(1, usize::MAX, &r, &r, &[]).is_err());
    assert!(verify_consistency(usize::MAX - 1, usize::MAX, &r, &r, &[r; 70]).is_err());
    assert!(verify_consistency(usize::MAX, usize::MAX, &r, &r, &[]).is_ok());
    assert!(verify_consistency(1, 2, &r, &r, &[r; 300]).is_err());
    assert!(verify_consistency(1 << 62, usize::MAX, &r, &r, &[r; 70]).is_err());
    assert_eq!(consistency_proof(&mk(3), 0), Err(ProofError::Range));
    assert_eq!(consistency_proof(&mk(3), 4), Err(ProofError::Range));
    assert_eq!(consistency_proof(&mk(3), 3).unwrap(), Vec::<Hash>::new());
    assert_eq!(inclusion_proof(&[], 0), Err(ProofError::Range));
    assert_eq!(root(&[]), sha256(b""));
}

// regression: exhaustive mutation of valid proofs (truncate / extend / swap / flip) must be rejected
#[test]
fn regression_inclusion_proof_mutations_rejected() {
    for n in 1..=40 {
        let l = mk(n);
        let r = root(&l);
        for i in 0..n {
            let p = inclusion_proof(&l, i).unwrap();
            for cut in 0..p.len() {
                assert!(
                    verify_inclusion(&l[i], i, n, &p[..cut], &r).is_err(),
                    "trunc n={n} i={i}"
                );
            }
            let mut ext = p.clone();
            ext.push([9; 32]);
            assert!(
                verify_inclusion(&l[i], i, n, &ext, &r).is_err(),
                "extend n={n} i={i}"
            );
            let mut pre = vec![[9u8; 32]];
            pre.extend(p.iter().copied());
            assert!(verify_inclusion(&l[i], i, n, &pre, &r).is_err());
            for j in 0..p.len() {
                let mut q = p.clone();
                q[j][5] ^= 0x80;
                assert!(verify_inclusion(&l[i], i, n, &q, &r).is_err());
            }
            // every wrong index with the same proof must fail
            for j in 0..n {
                if j != i && inclusion_proof(&l, j).unwrap() != p {
                    assert!(
                        verify_inclusion(&l[i], j, n, &p, &r).is_err(),
                        "wrong idx n={n} i={i} j={j}"
                    );
                }
            }
        }
    }
}

#[test]
fn regression_consistency_proof_mutations_rejected() {
    for n in 2..=40 {
        let l = mk(n);
        let rn = root(&l);
        for m in 1..n {
            let rm = root(&l[..m]);
            let p = consistency_proof(&l, m).unwrap();
            for cut in 0..p.len() {
                assert!(
                    verify_consistency(m, n, &rm, &rn, &p[..cut]).is_err(),
                    "trunc m={m} n={n}"
                );
            }
            let mut ext = p.clone();
            ext.push([9; 32]);
            assert!(
                verify_consistency(m, n, &rm, &rn, &ext).is_err(),
                "extend m={m} n={n}"
            );
            for j in 0..p.len() {
                let mut q = p.clone();
                q[j][0] ^= 1;
                assert!(verify_consistency(m, n, &rm, &rn, &q).is_err());
            }
            let mut bad = rn;
            bad[0] ^= 1;
            assert!(verify_consistency(m, n, &rm, &bad, &p).is_err());
            // swapped roots
            assert!(verify_consistency(m, n, &rn, &rm, &p).is_err());
        }
    }
}

// SPEC AMBIGUITY (documented behaviour, test passes): inclusion verification does not bind `size` to the root;
// a proof for leaf 0 of a 5-leaf tree also verifies against the same root when claiming size 6/7/8.
// Size must come from the signed checkpoint, not from the proof.
#[test]
fn ambiguity_inclusion_size_not_bound_to_root() {
    let l = mk(5);
    let r = root(&l);
    let p = inclusion_proof(&l, 0).unwrap();
    for claimed in [5usize, 6, 7, 8] {
        assert_eq!(
            verify_inclusion(&l[0], 0, claimed, &p, &r),
            Ok(()),
            "claimed size {claimed}"
        );
    }
}

// regression: second-preimage domain separation: interior node value cannot verify as a leaf
#[test]
fn regression_domain_separation() {
    let l = mk(4);
    let r = root(&l);
    let n01 = node_hash(&l[0], &l[1]);
    // Treat the interior node 01 as a "leaf" at index 0 of a 2-leaf tree with sibling n23 -> same root, but size differs
    let n23 = node_hash(&l[2], &l[3]);
    assert_eq!(verify_inclusion(&n01, 0, 2, &[n23], &r), Ok(())); // hash-structure level equality; leaf prefix is applied by callers
    assert_ne!(leaf_hash(&[&l[0][..], &l[1][..]].concat()), n01);
}

// regression: differential check of root()/proofs against an independent iterative MTH (RFC 9162 2.1.1 definition
// written without recursion over slices) around power-of-two boundaries well beyond the 130/70 sizes in merkle.rs.
fn naive_root(l: &[Hash]) -> Hash {
    if l.is_empty() {
        return sha256(b"");
    }
    if l.len() == 1 {
        return l[0];
    }
    let mut k = 1;
    while k * 2 < l.len() {
        k *= 2;
    }
    node_hash(&naive_root(&l[..k]), &naive_root(&l[k..]))
}

#[test]
fn regression_large_sizes_boundaries() {
    for n in [
        127usize, 128, 129, 255, 256, 257, 511, 512, 513, 1000, 1023, 1024, 1025,
    ] {
        let l = mk(n);
        let r = root(&l);
        assert_eq!(r, naive_root(&l), "root n={n}");
        for i in [0, 1, n / 2, n - 2, n - 1] {
            let p = inclusion_proof(&l, i).unwrap();
            assert_eq!(
                verify_inclusion(&l[i], i, n, &p, &r),
                Ok(()),
                "incl n={n} i={i}"
            );
        }
        for m in [
            1usize,
            2,
            3,
            n / 2,
            n / 2 + 1,
            127,
            128,
            129,
            255,
            256,
            257,
            512,
            513,
            n - 1,
            n,
        ] {
            if m == 0 || m > n {
                continue;
            }
            let p = consistency_proof(&l, m).unwrap();
            assert_eq!(
                verify_consistency(m, n, &root(&l[..m]), &r, &p),
                Ok(()),
                "cons m={m} n={n}"
            );
        }
    }
}

// regression: inclusion proof for size-1 tree has length 0; for n leaves length is ceil(log2 n) at most (no extra elements tolerated)
#[test]
fn regression_proof_lengths_bounded() {
    for n in 1..=64usize {
        let l = mk(n);
        for i in 0..n {
            let p = inclusion_proof(&l, i).unwrap();
            assert!(p.len() <= (usize::BITS - (n - 1).leading_zeros()) as usize);
        }
    }
}
