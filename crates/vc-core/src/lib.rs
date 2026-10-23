//! vc-core: canonical JSON (RFC 8785), SHA-256 helpers, statement chain rules.
pub mod canonical;
pub mod chain;
pub mod hash;

pub use hash::Hash;
