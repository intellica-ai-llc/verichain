//! vc-witness: witness cosigning with consistency check and gossip (requirement FR-L0-08, contract C6,
//! FR-L0-03 for equivocation detection).
//!
//! A [`Witness`] verifies a log operator's signed checkpoint, checks that it is consistent with the last
//! checkpoint it saw for that log (RFC 9162 consistency proof), persists the new last-seen state atomically
//! through [`vc_store::TileStore`] and only then returns a cosignature in the C2SP `tlog-cosignature` format.
//! [`Gossip`] and [`verify_equivocation_proof`] let witnesses and monitors exchange last-seen records and
//! produce a third-party verifiable proof when an operator signed two different roots for one size.
//!
//! Spec status:
//! - `tlog-cosignature` v1 (type byte 0x04, `cosignature/v1\ntime <t>\n` message, 72-byte timestamped
//!   signature): checked against https://github.com/C2SP/C2SP/blob/tlog-cosignature/v1.0.0/tlog-cosignature.md.
//! - UNVERIFIED: C2SP `tlog-witness` and `signed-note` pages could not be fetched (not on the session
//!   allowlist). The request/response semantics (old size, consistency proof, 409 with the witness size,
//!   proof at most 63 hashes) and the signed-note rules are implemented from memory of those specs and from
//!   `vc_log::checkpoint`, which already implements signed-note.
//! - Interop note: `vc_log::checkpoint::verify_note*` only recognises plain Ed25519 (type 0x01) signature lines,
//!   not type 0x04 cosignatures. By default a witness therefore emits BOTH a `tlog-cosignature` line and a legacy
//!   type-0x01 line (same name, same key) so notes verify with `vc_log` today; see README and
//!   `docs/reports/contract_requests.md`.
#![forbid(unsafe_code)]

mod cosig;
mod evidence;
mod state;
mod witness;

pub use cosig::{
    cosig_key_id, cosig_message, cosign_line, verify_cosignature, SIG_TYPE_COSIGNATURE_V1,
};
pub use evidence::{
    verify_equivocation_proof, EquivocationProof, Gossip, LastSeen, MergeOutcome,
};
pub use witness::{Clock, CosignedNote, LogConfig, SystemClock, Witness, WitnessConfig};

use vc_log::checkpoint::CheckpointError;

/// Maximum size of a presented note (text plus signature lines), bytes. Larger input is refused before parsing.
pub const MAX_NOTE_BYTES: usize = 8 * 1024;
/// Maximum number of hashes in a consistency proof (a proof between u64 sizes never needs more than 64).
pub const MAX_PROOF_HASHES: usize = 64;
/// Maximum number of configured logs (origins) per witness / gossip instance.
pub const MAX_LOGS: usize = 4096;
/// Maximum length of an origin line in bytes.
pub const MAX_ORIGIN_BYTES: usize = 256;

#[derive(Debug, thiserror::Error)]
pub enum WitnessError {
    #[error("invalid configuration: {0}")]
    BadConfig(String),
    #[error("note larger than {MAX_NOTE_BYTES} bytes")]
    NoteTooLarge,
    #[error("consistency proof longer than {MAX_PROOF_HASHES} hashes")]
    ProofTooLong,
    #[error("malformed checkpoint note: {0}")]
    Malformed(CheckpointError),
    #[error("size 0 checkpoint with a root other than the empty-tree root")]
    BadEmptyRoot,
    #[error("checkpoint size does not fit this platform")]
    SizeTooLarge,
    #[error("unknown log origin")]
    UnknownOrigin,
    #[error("operator signature missing or invalid for this origin")]
    BadOperatorSignature,
    #[error("no state for this origin and trust-on-first-use is disabled (bootstrap explicitly)")]
    TofuDisabled,
    #[error("the witness already has state for this origin")]
    AlreadyInitialised,
    #[error("old size {got} does not match the witness's last-seen size {expected}")]
    OldSizeMismatch { expected: u64, got: u64 },
    #[error("consistency proof does not verify")]
    BadConsistencyProof,
    #[error("checkpoint size {presented} is smaller than last-seen size {last_seen}")]
    SizeRegression { last_seen: u64, presented: u64 },
    /// The operator signed two different roots for the same size. Nothing was cosigned.
    #[error("equivocation: operator signed two roots for one size (evidence persisted: {persisted})")]
    Equivocation {
        evidence: Box<EquivocationProof>,
        /// False if writing the evidence record failed (the evidence is still in this error).
        persisted: bool,
    },
    #[error("store error: {0}")]
    Store(std::io::Error),
    #[error("stored state is corrupt: {0}")]
    CorruptState(String),
    #[error("clock unavailable")]
    Clock,
    #[error("no valid cosignature by this witness on the note")]
    NoCosignature,
    #[error("invalid equivocation proof: {0}")]
    BadEquivocationProof(&'static str),
}
