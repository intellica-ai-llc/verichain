//! Every bound the verifier enforces on untrusted input (FR-L0-11). All are checked before expensive work.

/// Maximum size of a pack / receipt-bundle JSON document in bytes (32 MiB). Larger input is refused, not truncated.
pub const MAX_PACK_BYTES: usize = 32 * 1024 * 1024;
/// Maximum number of entries in a pack.
pub const MAX_ENTRIES: usize = 10_000;
/// Maximum size of one COSE_Sign1 statement in bytes (hex field is twice this).
pub const MAX_STATEMENT_BYTES: usize = 64 * 1024;
/// Maximum number of sibling hashes in an inclusion proof (a tree of 2^64 leaves needs 64).
pub const MAX_PROOF_HASHES: usize = 64;
/// Maximum size of a signed checkpoint note in bytes.
pub const MAX_CHECKPOINT_BYTES: usize = 8 * 1024;
/// Maximum number of distinct checkpoint notes in one pack (each costs signature verifications).
pub const MAX_DISTINCT_CHECKPOINTS: usize = 64;
/// Maximum number of trusted issuers.
pub const MAX_ISSUERS: usize = 1_024;
/// Maximum number of trusted witnesses.
pub const MAX_WITNESSES: usize = 64;
/// Maximum length in bytes of an origin or key name.
pub const MAX_NAME_BYTES: usize = 256;
/// Maximum COSE key id length in bytes.
pub const MAX_KID_BYTES: usize = 64;
/// Maximum number of failing entries listed in a check's `detail`.
pub const MAX_DETAIL_ITEMS: usize = 8;
/// Maximum length of any single message echoed in a report (hostile strings are truncated).
pub const MAX_ECHO_CHARS: usize = 120;
