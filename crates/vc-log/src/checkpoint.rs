//! Signed checkpoints in the C2SP tlog-checkpoint / signed-note text format.
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use vc_core::Hash;

const SIG_TYPE_ED25519: u8 = 0x01;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum CheckpointError {
    #[error("malformed checkpoint")]
    Malformed,
    #[error("quorum not met: {got} of {need} required cosigners valid")]
    Quorum { got: usize, need: usize },
    #[error("operator signature missing or invalid")]
    NoOperator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checkpoint {
    pub origin: String,
    pub size: u64,
    pub root: Hash,
}

impl Checkpoint {
    /// Note text: origin, decimal size, base64 root; each line ends in "\n".
    pub fn text(&self) -> String {
        format!("{}\n{}\n{}\n", self.origin, self.size, B64.encode(self.root))
    }

    pub fn parse(text: &str) -> Result<Self, CheckpointError> {
        let mut l = text.strip_suffix('\n').ok_or(CheckpointError::Malformed)?.split('\n');
        let origin = l.next().filter(|s| !s.is_empty()).ok_or(CheckpointError::Malformed)?.to_string();
        let size = l.next().and_then(|s| s.parse::<u64>().ok()).ok_or(CheckpointError::Malformed)?;
        let root: Hash = B64.decode(l.next().ok_or(CheckpointError::Malformed)?).map_err(|_| CheckpointError::Malformed)?
            .try_into().map_err(|_| CheckpointError::Malformed)?;
        Ok(Checkpoint { origin, size, root })
    }
}

/// A trusted verifier key: name + Ed25519 public key.
#[derive(Clone)]
pub struct KeyInfo {
    pub name: String,
    pub key: VerifyingKey,
}

/// Key ID per signed-note: SHA-256(name || 0x0A || type || pubkey)[:4].
pub fn key_id(name: &str, key: &VerifyingKey) -> [u8; 4] {
    let mut h = Sha256::new();
    h.update(name.as_bytes());
    h.update([0x0a, SIG_TYPE_ED25519]);
    h.update(key.as_bytes());
    let d = h.finalize();
    [d[0], d[1], d[2], d[3]]
}

/// Produce one signature line ("— name base64(keyid||sig)\n") over `text`.
pub fn sign_line(text: &str, name: &str, sk: &SigningKey) -> String {
    let sig = sk.sign(text.as_bytes());
    let mut raw = key_id(name, &sk.verifying_key()).to_vec();
    raw.extend_from_slice(&sig.to_bytes());
    format!("\u{2014} {} {}\n", name, B64.encode(raw))
}

/// Full signed note: text, blank line, signature lines.
pub fn note(text: &str, sig_lines: &[String]) -> String {
    format!("{}\n{}", text, sig_lines.concat())
}

/// Split a note into (text, signature lines).
fn split_note(n: &str) -> Result<(&str, Vec<&str>), CheckpointError> {
    let i = n.find("\n\n").ok_or(CheckpointError::Malformed)?;
    let (text, rest) = (&n[..=i], &n[i + 2..]);
    Ok((text, rest.lines().collect()))
}

/// Names of trusted keys whose signature on `text` verifies.
fn valid_signers<'a>(text: &str, lines: &[&str], keys: &'a [KeyInfo]) -> HashSet<&'a str> {
    let mut ok = HashSet::new();
    for line in lines {
        let Some(rest) = line.strip_prefix("\u{2014} ") else { continue };
        let Some((name, b64)) = rest.rsplit_once(' ') else { continue };
        let Ok(raw) = B64.decode(b64) else { continue };
        if raw.len() != 4 + 64 { continue }
        for k in keys.iter().filter(|k| k.name == name) {
            if raw[..4] != key_id(&k.name, &k.key) { continue }
            let Ok(sig) = Signature::from_slice(&raw[4..]) else { continue };
            if k.key.verify(text.as_bytes(), &sig).is_ok() {
                ok.insert(k.name.as_str());
            }
        }
    }
    ok
}

/// Verify a checkpoint note: the operator must have signed, and at least
/// `min_witnesses` distinct trusted witnesses must have cosigned this exact text.
pub fn verify_note(n: &str, operator: &KeyInfo, witnesses: &[KeyInfo], min_witnesses: usize) -> Result<Checkpoint, CheckpointError> {
    let (text, lines) = split_note(n)?;
    let cp = Checkpoint::parse(text)?;
    let op = valid_signers(text, &lines, std::slice::from_ref(operator));
    if !op.contains(operator.name.as_str()) { return Err(CheckpointError::NoOperator) }
    let got = valid_signers(text, &lines, witnesses).len();
    if got < min_witnesses { return Err(CheckpointError::Quorum { got, need: min_witnesses }) }
    Ok(cp)
}

/// Split-view detection: same origin and size but different roots.
pub fn is_split_view(a: &Checkpoint, b: &Checkpoint) -> bool {
    a.origin == b.origin && a.size == b.size && a.root != b.root
}
