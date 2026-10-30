//! Persistent record encoding. One record per origin state and one per equivocation evidence; each is a
//! single `TileStore::put` (atomic) and ends in a SHA-256 of its body so corruption fails closed.
use crate::{EquivocationProof, WitnessError, MAX_NOTE_BYTES, MAX_ORIGIN_BYTES};
use vc_core::hash::sha256;
use vc_core::Hash;

const MAGIC_STATE: &[u8] = b"VCWS1\n";
const MAGIC_EVID: &[u8] = b"VCWE1\n";

pub(crate) fn state_path(origin: &str) -> String {
    format!("witness/v1/{}/state", hex::encode(sha256(origin.as_bytes())))
}
pub(crate) fn evidence_path(origin: &str) -> String {
    format!("witness/v1/{}/equivocation", hex::encode(sha256(origin.as_bytes())))
}

fn put_bytes(out: &mut Vec<u8>, b: &[u8]) {
    out.extend_from_slice(&(b.len() as u32).to_be_bytes());
    out.extend_from_slice(b);
}

struct Rd<'a>(&'a [u8]);
impl<'a> Rd<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], WitnessError> {
        if self.0.len() < n {
            return Err(corrupt("truncated"));
        }
        let (a, b) = self.0.split_at(n);
        self.0 = b;
        Ok(a)
    }
    fn u64(&mut self) -> Result<u64, WitnessError> {
        let mut b = [0u8; 8];
        b.copy_from_slice(self.take(8)?);
        Ok(u64::from_be_bytes(b))
    }
    fn bytes(&mut self, max: usize) -> Result<&'a [u8], WitnessError> {
        let mut b = [0u8; 4];
        b.copy_from_slice(self.take(4)?);
        let n = u32::from_be_bytes(b) as usize;
        if n > max {
            return Err(corrupt("field too long"));
        }
        self.take(n)
    }
    fn string(&mut self, max: usize) -> Result<String, WitnessError> {
        String::from_utf8(self.bytes(max)?.to_vec()).map_err(|_| corrupt("not utf-8"))
    }
}

fn corrupt(s: &str) -> WitnessError {
    WitnessError::CorruptState(s.to_string())
}

fn seal(mut body: Vec<u8>) -> Vec<u8> {
    let h = sha256(&body);
    body.extend_from_slice(&h);
    body
}

fn unseal<'a>(magic: &[u8], data: &'a [u8]) -> Result<Rd<'a>, WitnessError> {
    if data.len() < magic.len() + 32 || !data.starts_with(magic) {
        return Err(corrupt("bad header"));
    }
    let (body, sum) = data.split_at(data.len() - 32);
    if sha256(body) != sum {
        return Err(corrupt("checksum mismatch"));
    }
    Ok(Rd(&body[magic.len()..]))
}

pub(crate) struct StateRecord {
    pub origin: String,
    pub size: u64,
    pub root: Hash,
    pub note: String,
}

pub(crate) fn encode_state(origin: &str, size: u64, root: &Hash, note: &str) -> Vec<u8> {
    let mut b = MAGIC_STATE.to_vec();
    put_bytes(&mut b, origin.as_bytes());
    b.extend_from_slice(&size.to_be_bytes());
    b.extend_from_slice(root);
    put_bytes(&mut b, note.as_bytes());
    seal(b)
}

pub(crate) fn decode_state(data: &[u8]) -> Result<StateRecord, WitnessError> {
    let mut r = unseal(MAGIC_STATE, data)?;
    let origin = r.string(MAX_ORIGIN_BYTES)?;
    let size = r.u64()?;
    let mut root = [0u8; 32];
    root.copy_from_slice(r.take(32)?);
    let note = r.string(MAX_NOTE_BYTES)?;
    if !r.0.is_empty() {
        return Err(corrupt("trailing bytes"));
    }
    Ok(StateRecord { origin, size, root, note })
}

pub(crate) fn encode_evidence(p: &EquivocationProof) -> Vec<u8> {
    let mut b = MAGIC_EVID.to_vec();
    put_bytes(&mut b, p.origin.as_bytes());
    put_bytes(&mut b, p.note_a.as_bytes());
    put_bytes(&mut b, p.note_b.as_bytes());
    seal(b)
}

pub(crate) fn decode_evidence(data: &[u8]) -> Result<EquivocationProof, WitnessError> {
    let mut r = unseal(MAGIC_EVID, data)?;
    let origin = r.string(MAX_ORIGIN_BYTES)?;
    let note_a = r.string(MAX_NOTE_BYTES)?;
    let note_b = r.string(MAX_NOTE_BYTES)?;
    if !r.0.is_empty() {
        return Err(corrupt("trailing bytes"));
    }
    Ok(EquivocationProof { origin, note_a, note_b })
}
