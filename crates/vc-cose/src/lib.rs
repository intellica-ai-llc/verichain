//! Minimal COSE_Sign1 (RFC 9052) with Ed25519. Algorithm -19 = Ed25519 (RFC 9864, fully specified).
use ciborium::value::{Integer, Value};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

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
}

fn int(i: i64) -> Value { Value::Integer(Integer::from(i)) }

fn protected(kid: &[u8], content_type: &str) -> Vec<u8> {
    // {1: alg, 4: kid, 3: content type}
    let m = Value::Map(vec![(int(1), int(ALG_ED25519)), (int(3), Value::Text(content_type.into())), (int(4), Value::Bytes(kid.to_vec()))]);
    let mut out = vec![];
    ciborium::ser::into_writer(&m, &mut out).expect("cbor");
    out
}

fn sig_structure(protected: &[u8], payload: &[u8]) -> Vec<u8> {
    let v = Value::Array(vec![Value::Text("Signature1".into()), Value::Bytes(protected.to_vec()), Value::Bytes(vec![]), Value::Bytes(payload.to_vec())]);
    let mut out = vec![];
    ciborium::ser::into_writer(&v, &mut out).expect("cbor");
    out
}

/// Build a tagged COSE_Sign1 with an attached payload.
pub fn sign1(sk: &SigningKey, kid: &[u8], content_type: &str, payload: &[u8]) -> Vec<u8> {
    let p = protected(kid, content_type);
    let sig = sk.sign(&sig_structure(&p, payload));
    let v = Value::Tag(TAG_SIGN1, Box::new(Value::Array(vec![Value::Bytes(p), Value::Map(vec![]), Value::Bytes(payload.to_vec()), Value::Bytes(sig.to_bytes().to_vec())])));
    let mut out = vec![];
    ciborium::ser::into_writer(&v, &mut out).expect("cbor");
    out
}

/// Verify a COSE_Sign1; returns (kid, payload). Caller maps kid to a trusted key via `lookup`.
pub fn verify1(bytes: &[u8], lookup: impl Fn(&[u8]) -> Option<VerifyingKey>) -> Result<(Vec<u8>, Vec<u8>), CoseError> {
    let v: Value = ciborium::de::from_reader(bytes).map_err(|_| CoseError::Malformed)?;
    let Value::Tag(TAG_SIGN1, inner) = v else { return Err(CoseError::Malformed) };
    let Value::Array(a) = *inner else { return Err(CoseError::Malformed) };
    let [Value::Bytes(p), _, Value::Bytes(payload), Value::Bytes(sig)] = &a[..] else { return Err(CoseError::Malformed) };
    let hdr: Value = ciborium::de::from_reader(&p[..]).map_err(|_| CoseError::Malformed)?;
    let Value::Map(m) = hdr else { return Err(CoseError::Malformed) };
    let get = |k: i64| m.iter().find(|(kk, _)| *kk == int(k)).map(|(_, v)| v.clone());
    if get(1) != Some(int(ALG_ED25519)) { return Err(CoseError::Alg) }
    let Some(Value::Bytes(kid)) = get(4) else { return Err(CoseError::Kid) };
    let key = lookup(&kid).ok_or(CoseError::Kid)?;
    let sig = Signature::from_slice(sig).map_err(|_| CoseError::Signature)?;
    key.verify(&sig_structure(p, payload), &sig).map_err(|_| CoseError::Signature)?;
    Ok((kid, payload.clone()))
}
