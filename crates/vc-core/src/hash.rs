use sha2::{Digest, Sha256};

/// A SHA-256 digest.
pub type Hash = [u8; 32];

pub fn sha256(data: &[u8]) -> Hash {
    Sha256::digest(data).into()
}

pub fn to_hex(h: &Hash) -> String {
    hex::encode(h)
}

pub fn from_hex(s: &str) -> Option<Hash> {
    let v = hex::decode(s).ok()?;
    v.try_into().ok()
}

/// Merkle leaf hash: SHA-256(0x00 || data) (RFC 9162 style domain separation).
pub fn leaf_hash(data: &[u8]) -> Hash {
    let mut h = Sha256::new();
    h.update([0x00]);
    h.update(data);
    h.finalize().into()
}

/// Merkle interior node hash: SHA-256(0x01 || left || right).
pub fn node_hash(left: &Hash, right: &Hash) -> Hash {
    let mut h = Sha256::new();
    h.update([0x01]);
    h.update(left);
    h.update(right);
    h.finalize().into()
}
