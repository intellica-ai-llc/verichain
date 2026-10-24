//! Minimal COSE_Sign1 (RFC 9052) with Ed25519. Algorithm -19 = Ed25519 (RFC 9864, fully specified).
use ciborium::value::{Integer, Value};
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};

pub const ALG_ED25519: i64 = -19;
const TAG_SIGN1: u64 = 18;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum CoseError {
    #[error("malformed COSE_Sign1")]
    Malformed,
    #[error("unsupported algorithm")]
    Alg,
    #[error("key id mismatch")]
    Kid,
    #[error("signature invalid")]
    Signature,
    #[error("weak (small-order) public key")]
    WeakKey,
    #[error("critical header parameter not understood")]
    Crit,
}

/// Structural scan of one CBOR data item: returns its encoded length. Rejects indefinite-length
/// items, reserved additional-info values and excessive nesting. Does not allocate.
fn scan_item(b: &[u8], depth: usize) -> Result<usize, CoseError> {
    if depth > 32 {
        return Err(CoseError::Malformed);
    }
    let first = *b.first().ok_or(CoseError::Malformed)?;
    let (major, info) = (first >> 5, first & 0x1f);
    let (arg, mut n): (u64, usize) = match info {
        0..=23 => (info as u64, 1),
        24 => (*b.get(1).ok_or(CoseError::Malformed)? as u64, 2),
        25 => (
            u16::from_be_bytes(b.get(1..3).ok_or(CoseError::Malformed)?.try_into().unwrap()) as u64,
            3,
        ),
        26 => (
            u32::from_be_bytes(b.get(1..5).ok_or(CoseError::Malformed)?.try_into().unwrap()) as u64,
            5,
        ),
        27 => (
            u64::from_be_bytes(b.get(1..9).ok_or(CoseError::Malformed)?.try_into().unwrap()),
            9,
        ),
        _ => return Err(CoseError::Malformed), // 28-30 reserved, 31 indefinite/break
    };
    match major {
        0 | 1 | 7 => Ok(n),
        2 | 3 => {
            let len = usize::try_from(arg).map_err(|_| CoseError::Malformed)?;
            let end = n.checked_add(len).ok_or(CoseError::Malformed)?;
            if end > b.len() {
                return Err(CoseError::Malformed);
            }
            Ok(end)
        }
        4 | 5 => {
            let count = if major == 5 {
                arg.checked_mul(2)
            } else {
                Some(arg)
            }
            .ok_or(CoseError::Malformed)?;
            for _ in 0..count {
                n += scan_item(b.get(n..).ok_or(CoseError::Malformed)?, depth + 1)?;
            }
            Ok(n)
        }
        6 => Ok(n + scan_item(b.get(n..).ok_or(CoseError::Malformed)?, depth + 1)?),
        _ => Err(CoseError::Malformed),
    }
}

/// Require exactly one definite-length CBOR item occupying all of `b`.
fn require_single_item(b: &[u8]) -> Result<(), CoseError> {
    if scan_item(b, 0)? == b.len() {
        Ok(())
    } else {
        Err(CoseError::Malformed)
    }
}

fn int(i: i64) -> Value {
    Value::Integer(Integer::from(i))
}

fn protected(kid: &[u8], content_type: &str) -> Vec<u8> {
    // {1: alg, 4: kid, 3: content type}
    let m = Value::Map(vec![
        (int(1), int(ALG_ED25519)),
        (int(3), Value::Text(content_type.into())),
        (int(4), Value::Bytes(kid.to_vec())),
    ]);
    let mut out = vec![];
    ciborium::ser::into_writer(&m, &mut out).expect("cbor");
    out
}

fn sig_structure(protected: &[u8], payload: &[u8]) -> Vec<u8> {
    let v = Value::Array(vec![
        Value::Text("Signature1".into()),
        Value::Bytes(protected.to_vec()),
        Value::Bytes(vec![]),
        Value::Bytes(payload.to_vec()),
    ]);
    let mut out = vec![];
    ciborium::ser::into_writer(&v, &mut out).expect("cbor");
    out
}

/// Build a tagged COSE_Sign1 with an attached payload.
pub fn sign1(sk: &SigningKey, kid: &[u8], content_type: &str, payload: &[u8]) -> Vec<u8> {
    let p = protected(kid, content_type);
    let sig = sk.sign(&sig_structure(&p, payload));
    let v = Value::Tag(
        TAG_SIGN1,
        Box::new(Value::Array(vec![
            Value::Bytes(p),
            Value::Map(vec![]),
            Value::Bytes(payload.to_vec()),
            Value::Bytes(sig.to_bytes().to_vec()),
        ])),
    );
    let mut out = vec![];
    ciborium::ser::into_writer(&v, &mut out).expect("cbor");
    out
}

/// Verify a COSE_Sign1; returns (kid, payload). Caller maps kid to a trusted key via `lookup`.
/// Strictness: exactly one definite-length CBOR item (no trailing bytes), duplicate header labels
/// rejected, `crit` rejected, unprotected header must be a map, signatures checked with
/// `verify_strict` and weak (small-order) keys refused.
pub fn verify1(
    bytes: &[u8],
    lookup: impl Fn(&[u8]) -> Option<VerifyingKey>,
) -> Result<(Vec<u8>, Vec<u8>), CoseError> {
    require_single_item(bytes)?;
    let v: Value = ciborium::de::from_reader(bytes).map_err(|_| CoseError::Malformed)?;
    let Value::Tag(TAG_SIGN1, inner) = v else {
        return Err(CoseError::Malformed);
    };
    let Value::Array(a) = *inner else {
        return Err(CoseError::Malformed);
    };
    let [Value::Bytes(p), Value::Map(_), Value::Bytes(payload), Value::Bytes(sig)] = &a[..] else {
        return Err(CoseError::Malformed);
    };
    require_single_item(p)?;
    let hdr: Value = ciborium::de::from_reader(&p[..]).map_err(|_| CoseError::Malformed)?;
    let Value::Map(m) = hdr else {
        return Err(CoseError::Malformed);
    };
    for (i, (k, _)) in m.iter().enumerate() {
        if m[..i].iter().any(|(k2, _)| k2 == k) {
            return Err(CoseError::Malformed); // RFC 9052 3.1: duplicate labels are forbidden
        }
    }
    let get = |k: i64| {
        m.iter()
            .find(|(kk, _)| *kk == int(k))
            .map(|(_, v)| v.clone())
    };
    if get(2).is_some() {
        return Err(CoseError::Crit);
    } // no critical parameters are understood
    if get(1) != Some(int(ALG_ED25519)) {
        return Err(CoseError::Alg);
    }
    let Some(Value::Bytes(kid)) = get(4) else {
        return Err(CoseError::Kid);
    };
    let key = lookup(&kid).ok_or(CoseError::Kid)?;
    if key.is_weak() {
        return Err(CoseError::WeakKey);
    }
    let sig = Signature::from_slice(sig).map_err(|_| CoseError::Signature)?;
    key.verify_strict(&sig_structure(p, payload), &sig)
        .map_err(|_| CoseError::Signature)?;
    Ok((kid, payload.clone()))
}

/// Read the kid and payload of a COSE_Sign1 **without verifying the signature**. Only for data that this
/// process already verified and stored itself (e.g., replaying its own log); never for untrusted input.
pub fn peek_payload(bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CoseError> {
    require_single_item(bytes)?;
    let v: Value = ciborium::de::from_reader(bytes).map_err(|_| CoseError::Malformed)?;
    let Value::Tag(TAG_SIGN1, inner) = v else {
        return Err(CoseError::Malformed);
    };
    let Value::Array(a) = *inner else {
        return Err(CoseError::Malformed);
    };
    let [Value::Bytes(p), _, Value::Bytes(payload), _] = &a[..] else {
        return Err(CoseError::Malformed);
    };
    let hdr: Value = ciborium::de::from_reader(&p[..]).map_err(|_| CoseError::Malformed)?;
    let Value::Map(m) = hdr else {
        return Err(CoseError::Malformed);
    };
    let kid = m
        .iter()
        .find(|(k, _)| *k == int(4))
        .and_then(|(_, v)| {
            if let Value::Bytes(b) = v {
                Some(b.clone())
            } else {
                None
            }
        })
        .ok_or(CoseError::Kid)?;
    Ok((kid, payload.clone()))
}
