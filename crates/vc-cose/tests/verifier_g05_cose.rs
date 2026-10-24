//! Verifier G05: COSE_Sign1 probes. `bug_*` assert CORRECT behaviour and FAIL today; `regression_*` pass.
use ciborium::value::{Integer, Value};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use vc_cose::*;

fn sk(n: u8) -> SigningKey {
    SigningKey::from_bytes(&[n; 32])
}
fn int(i: i64) -> Value {
    Value::Integer(Integer::from(i))
}
fn enc(v: &Value) -> Vec<u8> {
    let mut o = vec![];
    ciborium::ser::into_writer(v, &mut o).unwrap();
    o
}
fn sigstruct(p: &[u8], payload: &[u8]) -> Vec<u8> {
    enc(&Value::Array(vec![
        Value::Text("Signature1".into()),
        Value::Bytes(p.to_vec()),
        Value::Bytes(vec![]),
        Value::Bytes(payload.to_vec()),
    ]))
}
/// Build a tagged COSE_Sign1 from explicit protected map entries and an arbitrary unprotected value, correctly signed.
fn build(k: &SigningKey, prot: Vec<(Value, Value)>, unprot: Value, payload: Value) -> Vec<u8> {
    let p = enc(&Value::Map(prot));
    let pl = if let Value::Bytes(b) = &payload {
        b.clone()
    } else {
        vec![]
    };
    let sig = k.sign(&sigstruct(&p, &pl));
    enc(&Value::Tag(
        18,
        Box::new(Value::Array(vec![
            Value::Bytes(p),
            unprot,
            payload,
            Value::Bytes(sig.to_bytes().to_vec()),
        ])),
    ))
}
fn std_prot(kid: &[u8]) -> Vec<(Value, Value)> {
    vec![(int(1), int(-19)), (int(4), Value::Bytes(kid.to_vec()))]
}
fn pl() -> Value {
    Value::Bytes(b"hello".to_vec())
}
fn ver(k: &SigningKey, m: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CoseError> {
    let vk = k.verifying_key();
    verify1(m, |_| Some(vk))
}

// sanity: the helper builds what verify1 accepts, and matches sign1 byte for byte (Sig_structure correct per RFC 9052 4.4)
#[test]
fn regression_sig_structure_matches_rfc9052() {
    let k = sk(1);
    let m = build(&k, std_prot(b"kid"), Value::Map(vec![]), pl());
    assert_eq!(ver(&k, &m).unwrap().1, b"hello");
    // independent check: signature in sign1() output verifies over ["Signature1", protected, h'', payload]
    let m2 = sign1(&k, b"kid", "t", b"hello");
    let v: Value = ciborium::de::from_reader(&m2[..]).unwrap();
    let Value::Tag(18, inner) = v else { panic!() };
    let Value::Array(a) = *inner else { panic!() };
    let (Value::Bytes(p), Value::Bytes(sig)) = (&a[0], &a[3]) else {
        panic!()
    };
    let s = ed25519_dalek::Signature::from_slice(sig).unwrap();
    k.verifying_key()
        .verify_strict(&sigstruct(p, b"hello"), &s)
        .unwrap();
    // CBOR head of the Sig_structure: 84 6a "Signature1" 58 .. 40 ..
    let ss = sigstruct(p, b"hello");
    assert_eq!(&ss[..12], b"\x84\x6aSignature1");
}

#[test]
fn regression_tag_and_shape() {
    let k = sk(1);
    let good = build(&k, std_prot(b"kid"), Value::Map(vec![]), pl());
    // untagged
    let v: Value = ciborium::de::from_reader(&good[..]).unwrap();
    let Value::Tag(_, inner) = v.clone() else {
        panic!()
    };
    assert_eq!(ver(&k, &enc(&inner)), Err(CoseError::Malformed), "untagged");
    // wrong tag (COSE_Sign = 98, COSE_Mac0 = 17)
    for t in [98u64, 17, 19, 0] {
        assert_eq!(
            ver(&k, &enc(&Value::Tag(t, inner.clone()))),
            Err(CoseError::Malformed),
            "tag {t}"
        );
    }
    // double tag
    assert_eq!(
        ver(&k, &enc(&Value::Tag(18, Box::new(v.clone())))),
        Err(CoseError::Malformed)
    );
    // extra / missing array elements
    let Value::Array(a) = *inner.clone() else {
        panic!()
    };
    let mut five = a.clone();
    five.push(Value::Null);
    assert_eq!(
        ver(&k, &enc(&Value::Tag(18, Box::new(Value::Array(five))))),
        Err(CoseError::Malformed)
    );
    assert_eq!(
        ver(
            &k,
            &enc(&Value::Tag(18, Box::new(Value::Array(a[..3].to_vec()))))
        ),
        Err(CoseError::Malformed)
    );
    // nil (detached) payload
    let det = build(&k, std_prot(b"kid"), Value::Map(vec![]), Value::Null);
    assert_eq!(ver(&k, &det), Err(CoseError::Malformed), "detached payload");
    // protected header is not a bstr (map inline), or a bstr not containing a map
    let mut np = a.clone();
    np[0] = Value::Map(std_prot(b"kid"));
    assert_eq!(
        ver(&k, &enc(&Value::Tag(18, Box::new(Value::Array(np))))),
        Err(CoseError::Malformed)
    );
    let mut np = a.clone();
    np[0] = Value::Bytes(enc(&Value::Array(vec![])));
    assert_eq!(
        ver(&k, &enc(&Value::Tag(18, Box::new(Value::Array(np))))),
        Err(CoseError::Malformed)
    );
    // empty protected bstr
    let mut np = a.clone();
    np[0] = Value::Bytes(vec![]);
    assert!(ver(&k, &enc(&Value::Tag(18, Box::new(Value::Array(np))))).is_err());
    // truncated input and empty input
    for n in 0..good.len() {
        assert!(ver(&k, &good[..n]).is_err(), "truncated at {n}");
    }
}

// alg / kid only in the UNPROTECTED header must not be honoured (unauthenticated).
#[test]
fn regression_alg_kid_only_in_unprotected_rejected() {
    let k = sk(1);
    let m = build(
        &k,
        vec![],
        Value::Map(vec![
            (int(1), int(-19)),
            (int(4), Value::Bytes(b"kid".to_vec())),
        ]),
        pl(),
    );
    assert_eq!(ver(&k, &m), Err(CoseError::Alg));
    let m = build(
        &k,
        vec![(int(1), int(-19))],
        Value::Map(vec![(int(4), Value::Bytes(b"kid".to_vec()))]),
        pl(),
    );
    assert_eq!(ver(&k, &m), Err(CoseError::Kid));
    // wrong alg values / types in protected
    for a in [
        int(-8),
        int(-7),
        int(0),
        Value::Text("-19".into()),
        Value::Null,
    ] {
        let m = build(
            &k,
            vec![(int(1), a), (int(4), Value::Bytes(b"k".to_vec()))],
            Value::Map(vec![]),
            pl(),
        );
        assert_eq!(ver(&k, &m), Err(CoseError::Alg));
    }
    // kid wrong type
    let m = build(
        &k,
        vec![(int(1), int(-19)), (int(4), Value::Text("k".into()))],
        Value::Map(vec![]),
        pl(),
    );
    assert_eq!(ver(&k, &m), Err(CoseError::Kid));
}

// BUG (medium): trailing bytes after the COSE_Sign1 CBOR item are ignored (ciborium::from_reader reads one item).
// Two different byte strings therefore verify as the same statement (envelope malleability); a receipt/leaf hash
// computed over the raw bytes would differ from what was verified.
#[test]
fn bug_trailing_bytes_after_sign1_accepted() {
    let k = sk(1);
    let mut m = build(&k, std_prot(b"kid"), Value::Map(vec![]), pl());
    m.extend_from_slice(b"TRAILING GARBAGE");
    assert!(ver(&k, &m).is_err(), "trailing bytes accepted");
}

// BUG (low): trailing bytes inside the protected header bstr are ignored (the map is decoded then the rest dropped).
#[test]
fn bug_trailing_bytes_in_protected_accepted() {
    let k = sk(1);
    let mut p = enc(&Value::Map(std_prot(b"kid")));
    p.extend_from_slice(&[0xff, 0xff]);
    let sig = k.sign(&sigstruct(&p, b"hello"));
    let m = enc(&Value::Tag(
        18,
        Box::new(Value::Array(vec![
            Value::Bytes(p),
            Value::Map(vec![]),
            pl(),
            Value::Bytes(sig.to_bytes().to_vec()),
        ])),
    ));
    assert!(
        ver(&k, &m).is_err(),
        "protected header with trailing bytes accepted"
    );
}

// BUG (low/medium): RFC 9052 3.1: duplicate labels in a header map MUST NOT occur and receivers must reject. verify1
// takes the first occurrence. Attack: {1:-19, 4:"A", 4:"B"} - another implementation taking the LAST kid selects a different key.
#[test]
fn bug_duplicate_protected_keys_accepted() {
    let k = sk(1);
    let prot = vec![
        (int(1), int(-19)),
        (int(4), Value::Bytes(b"A".to_vec())),
        (int(4), Value::Bytes(b"B".to_vec())),
    ];
    let m = build(&k, prot, Value::Map(vec![]), pl());
    assert!(
        ver(&k, &m).is_err(),
        "duplicate kid label accepted; first wins"
    );
    let prot = vec![
        (int(1), int(-19)),
        (int(1), int(-8)),
        (int(4), Value::Bytes(b"A".to_vec())),
    ];
    assert!(
        ver(&k, &build(&k, prot, Value::Map(vec![]), pl())).is_err(),
        "duplicate alg label accepted; first wins"
    );
}

// BUG (low): unprotected header (element 1) is not type-checked; any CBOR value (integer, text, array) is accepted.
// RFC 9052: it must be a map.
#[test]
fn bug_unprotected_not_a_map_accepted() {
    let k = sk(1);
    for u in [
        int(5),
        Value::Text("x".into()),
        Value::Array(vec![]),
        Value::Null,
    ] {
        let m = build(&k, std_prot(b"kid"), u.clone(), pl());
        assert!(ver(&k, &m).is_err(), "unprotected header {u:?} accepted");
    }
}

// BUG (low): `crit` (label 2) in the protected header is ignored; RFC 9052 3.1: a verifier that does not understand a
// critical parameter MUST reject.
#[test]
fn bug_crit_header_ignored() {
    let k = sk(1);
    let mut prot = std_prot(b"kid");
    prot.push((int(2), Value::Array(vec![int(-999)])));
    prot.push((int(-999), Value::Text("must be understood".into())));
    assert!(
        ver(&k, &build(&k, prot, Value::Map(vec![]), pl())).is_err(),
        "unknown critical header accepted"
    );
}

// BUG (low): indefinite-length (non-deterministic) CBOR for the outer envelope is accepted. Not a forgery but, with
// trailing data, gives many encodings of one signed statement. Tested for outer array and payload bstr.
#[test]
fn bug_indefinite_length_envelope_accepted() {
    let k = sk(1);
    let p = enc(&Value::Map(std_prot(b"kid")));
    let sig = k.sign(&sigstruct(&p, b"hello"));
    let mut m = vec![0xd2, 0x9f]; // tag 18, indefinite array
    m.extend(enc(&Value::Bytes(p)));
    m.extend(enc(&Value::Map(vec![])));
    m.extend(vec![0x5f, 0x43, b'h', b'e', b'l', 0x42, b'l', b'o', 0xff]); // indefinite bstr "hel"+"lo"
    m.extend(enc(&Value::Bytes(sig.to_bytes().to_vec())));
    m.push(0xff);
    assert!(ver(&k, &m).is_err(), "indefinite-length CBOR accepted");
}

// regression: hostile CBOR must not panic, hang or exhaust memory.
#[test]
fn regression_hostile_cbor_no_panic() {
    let k = sk(1);
    // deep nesting in the unprotected slot
    let mut m = vec![0xd2, 0x84, 0x40];
    m.extend(std::iter::repeat_n(0x81, 100_000));
    m.push(0x00);
    assert!(ver(&k, &m).is_err());
    // huge declared lengths
    for head in [
        &[
            0xd2u8, 0x84, 0x5b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ][..],
        &[0xd2, 0x9b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        &[0xd2, 0x84, 0x58, 0xff],
    ] {
        assert!(ver(&k, head).is_err());
    }
}

// signature malleability: S + L (non-reduced scalar) must be rejected.
#[test]
fn regression_signature_scalar_malleability_rejected() {
    let k = sk(1);
    let m = sign1(&k, b"kid", "t", b"hello");
    let v: Value = ciborium::de::from_reader(&m[..]).unwrap();
    let Value::Tag(18, inner) = v else { panic!() };
    let Value::Array(mut a) = *inner else {
        panic!()
    };
    let Value::Bytes(sig) = a[3].clone() else {
        panic!()
    };
    // L = 2^252 + 27742317777372353535851937790883648493 little endian
    let l: [u8; 32] = [
        0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde,
        0x14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10,
    ];
    let mut s = sig[32..].to_vec();
    let mut carry = 0u16;
    for i in 0..32 {
        let t = s[i] as u16 + l[i] as u16 + carry;
        s[i] = t as u8;
        carry = t >> 8;
    }
    let mut bad = sig[..32].to_vec();
    bad.extend(s);
    a[3] = Value::Bytes(bad);
    let m2 = enc(&Value::Tag(18, Box::new(Value::Array(a))));
    assert!(ver(&k, &m2).is_err(), "S+L accepted");
}

// BUG (low): verification uses `verify` not `verify_strict`: with a small-order (identity) public key, the signature
// R = identity, S = 0 verifies for EVERY message. If any such key ever reaches the trust lookup (e.g. from a registry) it is a
// universal forgery key. verify_strict rejects it.
#[test]
fn bug_small_order_key_universal_forgery() {
    let mut ident = [0u8; 32];
    ident[0] = 1; // compressed identity point (0,1)
    let weak = VerifyingKey::from_bytes(&ident).expect("identity decompresses");
    let p = enc(&Value::Map(std_prot(b"kid")));
    let mut sig = [0u8; 64];
    sig[0] = 1; // R = identity, S = 0
    for payload in [&b"anything"[..], b"attacker chosen"] {
        let m = enc(&Value::Tag(
            18,
            Box::new(Value::Array(vec![
                Value::Bytes(p.clone()),
                Value::Map(vec![]),
                Value::Bytes(payload.to_vec()),
                Value::Bytes(sig.to_vec()),
            ])),
        ));
        assert!(
            verify1(&m, |_| Some(weak)).is_err(),
            "forged signature verified under identity key for payload {payload:?}"
        );
    }
}

// regression: kid lookup is exact bytes, the lookup closure receives the PROTECTED kid and its result is what verifies.
#[test]
fn regression_kid_lookup_semantics() {
    let (k1, k2) = (sk(1), sk(2));
    let m = sign1(&k1, b"k1", "t", b"x");
    let (v1, v2) = (k1.verifying_key(), k2.verifying_key());
    let look = |id: &[u8]| match id {
        b"k1" => Some(v1),
        b"k2" => Some(v2),
        _ => None,
    };
    assert!(verify1(&m, look).is_ok());
    assert_eq!(
        verify1(&m, |id| (id == b"K1").then_some(v1)),
        Err(CoseError::Kid)
    );
    assert_eq!(
        verify1(&m, |id| (id == b"k1 ").then_some(v1)),
        Err(CoseError::Kid)
    );
    // attacker swaps kid to k2 in protected header without re-signing -> signature fails
    let mut forged = m.clone();
    let pos = forged.windows(2).position(|w| w == b"k1").unwrap();
    forged[pos + 1] = b'2';
    assert_eq!(verify1(&forged, look), Err(CoseError::Signature));
    // empty kid
    let m0 = sign1(&k1, b"", "t", b"x");
    assert_eq!(
        verify1(&m0, |id| id.is_empty().then_some(v1)).unwrap().0,
        Vec::<u8>::new()
    );
}

// AMBIGUITY: arc42 5.1.1 says the protected header carries CWT claims (iss, sub); spec/README.md and code only define
// {1,3,4}. verify1 never requires/inspects content type (3) or CWT claims (15). Passing test documents current behaviour.
#[test]
fn ambiguity_content_type_and_cwt_claims_not_required() {
    let k = sk(1);
    let m = build(&k, std_prot(b"kid"), Value::Map(vec![]), pl()); // no label 3, no label 15
    assert!(ver(&k, &m).is_ok());
}
