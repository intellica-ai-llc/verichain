//! vc-verify: offline, deterministic verifier for VeriChain receipts (C4) and evidence packs (C5).
//! Requirements: FR-L0-11 (verifier library, CLI `vcv`, WASM-ready), verification side of FR-L0-10.
//!
//! The library performs no file, clock, environment, network or thread access (the `cli` feature adds the `vcv`
//! binary, which is the only code using `std::fs`/`std::io`/`std::env`). Contracts: `spec/contracts/L0.md` C4/C5.
//! Threats addressed: T-L0-01 (split view), 02 (quorum inflation), 03 (forged proofs), 04/05 (parser
//! differentials, via vc-core JCS and vc-cose strictness), 06 (weak keys), 07 (omission/reorder), 09 (origin).
#![forbid(unsafe_code)]

pub mod chain;
pub mod limits;
pub mod report;
pub mod schema;
pub mod strict;
pub mod types;
pub mod verify;
#[cfg(feature = "wasm")]
pub mod wasm;

pub use report::{Check, Report};
pub use types::{
    Assurance, Completeness, Entry, EvidencePack, Hex, Hex32, Issuer, NamedKey, PackError, Policy, Receipt,
    ReceiptBundle, Trust,
};
pub use verify::{verify_pack, verify_pack_bytes, verify_pack_pinned, verify_receipt, verify_receipt_bytes};
