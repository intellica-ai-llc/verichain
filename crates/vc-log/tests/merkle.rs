use vc_core::hash::{from_hex, to_hex};
use vc_log::*;

/// Entries of the widely used 8-leaf Certificate Transparency reference tree.
fn ct_entries() -> Vec<Vec<u8>> {
    [
        "",
        "00",
        "10",
        "2021",
        "3031",
        "40414243",
        "5051525354555657",
        "606162636465666768696a6b6c6d6e6f",
    ]
    .iter()
    .map(|h| hex::decode(h).unwrap())
    .collect()
}

#[test]
fn ct_reference_roots() {
    let e = ct_entries();
    let refs: Vec<&[u8]> = e.iter().map(|v| v.as_slice()).collect();
    let l = leaves_from_entries(&refs);
    let want = [
        "6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d",
        "fac54203e7cc696cf0dfcb42c92a1d9dbaf70ad9e621f4bd8d98662f00e3c125",
        "aeb6bcfe274b70a14fb067a5e5578264db0fa9b51af5e0ba159158f329e06e77",
        "d37ee418976dd95753c1c73862b9398fa2a2cf9b4ff0fdfe8b30cd95209614b7",
        "4e3bbb1f7b478dcfe71fb631631519a3bca12c9aefca1612bfce4c13a86264d4",
        "76e67dadbcdf1e10e1b74ddc608abd2f98dfb16fbce75277b5232a127f2087ef",
        "ddb89be403809e325750d3d263cd78929c2942b7942a34b77e122c9594a74c8c",
        "5dc9da79a70659a9ad559cb701ded9a2ab9d823aad2f4960cfe370eff4604328",
    ];
    for (i, w) in want.iter().enumerate() {
        assert_eq!(&to_hex(&root(&l[..=i])), w, "size {}", i + 1);
    }
}

fn mk(n: usize) -> Vec<vc_core::Hash> {
    (0..n)
        .map(|i| vc_core::hash::leaf_hash(format!("entry-{i}").as_bytes()))
        .collect()
}

#[test]
fn inclusion_all_sizes() {
    for n in 1..=130 {
        let l = mk(n);
        let r = root(&l);
        for i in 0..n {
            let p = inclusion_proof(&l, i).unwrap();
            verify_inclusion(&l[i], i, n, &p, &r).unwrap();
            // negative: wrong index / tampered leaf
            let mut bad = l[i];
            bad[0] ^= 1;
            assert!(verify_inclusion(&bad, i, n, &p, &r).is_err());
            if n > 1 {
                assert!(
                    verify_inclusion(&l[i], (i + 1) % n, n, &p, &r).is_err()
                        || n == 1
                        || (i + 1) % n == i
                );
            }
        }
    }
}

#[test]
fn consistency_all_pairs() {
    for n in 1..=70 {
        let l = mk(n);
        let rn = root(&l);
        for m in 1..=n {
            let rm = root(&l[..m]);
            let p = consistency_proof(&l, m).unwrap();
            verify_consistency(m, n, &rm, &rn, &p).unwrap();
            if m < n {
                let mut bad = rm;
                bad[0] ^= 1;
                assert!(
                    verify_consistency(m, n, &bad, &rn, &p).is_err(),
                    "split-view m={m} n={n}"
                );
            }
        }
    }
}

#[test]
fn empty_root_is_sha256_empty() {
    assert_eq!(
        to_hex(&root(&[])),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert!(from_hex("zz").is_none());
}
