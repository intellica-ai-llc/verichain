use serde_json::Value;

/// Canonical bytes of a JSON value per RFC 8785 (JCS).
pub fn canonicalize(v: &Value) -> Result<Vec<u8>, serde_json::Error> {
    serde_jcs::to_vec(v)
}

/// Parse JSON text and canonicalize it.
pub fn canonicalize_str(s: &str) -> Result<Vec<u8>, serde_json::Error> {
    canonicalize(&serde_json::from_str::<Value>(s)?)
}
