use ed25519_dalek::SigningKey;
use vc_cose::*;

fn sk(n: u8) -> SigningKey { SigningKey::from_bytes(&[n; 32]) }

#[test]
fn roundtrip() {
    let k = sk(1);
    let msg = sign1(&k, b"agent-1", "application/vc-statement+json", b"{\"v\":2}");
    let vk = k.verifying_key();
    let (kid, payload) = verify1(&msg, |id| (id == b"agent-1").then_some(vk)).unwrap();
    assert_eq!(kid, b"agent-1");
    assert_eq!(payload, b"{\"v\":2}");
    assert_eq!(msg[0], 0xd2); // CBOR tag 18
}

#[test]
fn negatives() {
    let k = sk(1);
    let vk = k.verifying_key();
    let msg = sign1(&k, b"a", "t", b"payload");
    // wrong kid / untrusted key
    assert_eq!(verify1(&msg, |_| None), Err(CoseError::Kid));
    // wrong key for kid
    let other = sk(2).verifying_key();
    assert_eq!(verify1(&msg, |_| Some(other)), Err(CoseError::Signature));
    // tampered payload: flip a byte of "payload" in the encoding
    let mut bad = msg.clone();
    let pos = bad.windows(7).position(|w| w == b"payload").unwrap();
    bad[pos] ^= 1;
    assert_eq!(verify1(&bad, |_| Some(vk)), Err(CoseError::Signature));
    // garbage
    assert_eq!(verify1(b"\x00", |_| Some(vk)), Err(CoseError::Malformed));
}
