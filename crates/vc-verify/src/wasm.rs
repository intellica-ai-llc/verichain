//! WASM entry point (feature `wasm`): a pure function from JSON text to JSON report text. A wrapper crate can
//! export it with wasm-bindgen; this crate stays free of any binding dependency.

/// Verify a pack given as JSON text; returns the JSON report. Never panics; malformed input yields `ok:false`.
pub fn verify_pack_json(input: &str) -> String {
    crate::verify::verify_pack_bytes(input.as_bytes(), None).to_json()
}
