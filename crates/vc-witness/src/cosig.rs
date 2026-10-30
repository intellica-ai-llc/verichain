//! C2SP `tlog-cosignature` v1: key ID, signed message and signature line.
use crate::{WitnessError, MAX_NOTE_BYTES};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use sha2::{Digest, Sha256};
use vc_log::checkpoint::{valid_name, Checkpoint};

/// signed-note signature type byte for `cosignature/v1` (timestamped Ed25519).
pub const SIG_TYPE_COSIGNATURE_V1: u8 = 0x04;

/// Key ID: SHA-256(name || 0x0A || 0x04 || pubkey)[:4].
pub fn cosig_key_id(name: &str, key: &VerifyingKey) -> [u8; 4] {
    let mut h = Sha256::new();
    h.update(name.as_bytes());
    h.update([0x0a, SIG_TYPE_COSIGNATURE_V1]);
    h.update(key.as_bytes());
    let d = h.finalize();
    [d[0], d[1], d[2], d[3]]
}

/// Signed message: "cosignature/v1\n" "time <decimal>\n" then the checkpoint body (with final newline,
/// without signature lines).
pub fn cosig_message(timestamp: u64, text: &str) -> Vec<u8> {
    format!("cosignature/v1\ntime {timestamp}\n{text}").into_bytes()
}

/// One signature line: "— name base64(keyid || u64be timestamp || ed25519 signature)\n".
/// Fails for names that violate signed-note rules.
pub fn cosign_line(
    text: &str,
    name: &str,
    sk: &SigningKey,
    timestamp: u64,
) -> Result<String, WitnessError> {
    if !valid_name(name) {
        return Err(WitnessError::BadConfig("invalid witness key name".into()));
    }
    let sig = sk.sign(&cosig_message(timestamp, text));
    let mut raw = cosig_key_id(name, &sk.verifying_key()).to_vec();
    raw.extend_from_slice(&timestamp.to_be_bytes());
    raw.extend_from_slice(&sig.to_bytes());
    Ok(format!("\u{2014} {} {}\n", name, B64.encode(raw)))
}

/// Same split as `vc_log::checkpoint` (first blank line ends the text); must stay byte-identical so the
/// text we cosign is exactly the text the operator signed.
pub(crate) fn split_note(n: &str) -> Result<(&str, &str), WitnessError> {
    if n.len() > MAX_NOTE_BYTES {
        return Err(WitnessError::NoteTooLarge);
    }
    let i = n
        .find("\n\n")
        .ok_or(WitnessError::Malformed(vc_log::checkpoint::CheckpointError::Malformed))?;
    Ok((&n[..=i], &n[i + 2..]))
}

/// Verify that `note` carries a valid `cosignature/v1` line by `name`/`key`. Returns the checkpoint and
/// the cosignature timestamp. Does not check the operator signature (use `vc_log::checkpoint`).
pub fn verify_cosignature(
    note: &str,
    name: &str,
    key: &VerifyingKey,
) -> Result<(Checkpoint, u64), WitnessError> {
    let (text, sigs) = split_note(note)?;
    let cp = Checkpoint::parse(text).map_err(WitnessError::Malformed)?;
    if key.is_weak() {
        return Err(WitnessError::NoCosignature);
    }
    let want = cosig_key_id(name, key);
    for line in sigs.lines() {
        let Some(rest) = line.strip_prefix("\u{2014} ") else {
            continue;
        };
        let Some((n, b64)) = rest.rsplit_once(' ') else {
            continue;
        };
        if n != name {
            continue;
        }
        let Ok(raw) = B64.decode(b64) else { continue };
        if raw.len() != 4 + 8 + 64 || raw[..4] != want {
            continue;
        }
        let mut ts = [0u8; 8];
        ts.copy_from_slice(&raw[4..12]);
        let ts = u64::from_be_bytes(ts);
        let Ok(sig) = Signature::from_slice(&raw[12..]) else {
            continue;
        };
        if key.verify_strict(&cosig_message(ts, text), &sig).is_ok() {
            return Ok((cp, ts));
        }
    }
    Err(WitnessError::NoCosignature)
}
