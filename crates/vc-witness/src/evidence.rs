//! Last-seen records, gossip merging and third-party verifiable equivocation proofs (FR-L0-03, FR-L0-08).
use crate::cosig::split_note;
use crate::{WitnessError, MAX_LOGS, MAX_ORIGIN_BYTES, MAX_PROOF_HASHES};
use std::collections::HashMap;
use vc_core::hash::sha256;
use vc_core::Hash;
use vc_log::checkpoint::{verify_note_for_origin, Checkpoint, CheckpointError, KeyInfo};

/// A verified last-seen checkpoint: the operator-signed note and its parsed contents. Can only be built by
/// verifying the operator signature, so holding one means the operator really signed (size, root).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LastSeen {
    origin: String,
    size: u64,
    root: Hash,
    note: String,
}

impl LastSeen {
    /// Verify `note` against the operator key of `origin` (bounded input, strict signature checks).
    pub fn from_note(note: &str, origin: &str, operator: &KeyInfo) -> Result<Self, WitnessError> {
        if origin.len() > MAX_ORIGIN_BYTES {
            return Err(WitnessError::UnknownOrigin);
        }
        split_note(note)?; // size bound
        let cp = verify_note_for_origin(note, origin, operator, &[], 0).map_err(|e| match e {
            CheckpointError::NoOperator => WitnessError::BadOperatorSignature,
            CheckpointError::WrongOrigin => WitnessError::UnknownOrigin,
            other => WitnessError::Malformed(other),
        })?;
        if usize::try_from(cp.size).is_err() {
            return Err(WitnessError::SizeTooLarge);
        }
        if cp.size == 0 && cp.root != sha256(b"") {
            return Err(WitnessError::BadEmptyRoot);
        }
        Ok(LastSeen {
            origin: cp.origin,
            size: cp.size,
            root: cp.root,
            note: note.to_string(),
        })
    }
    pub fn origin(&self) -> &str {
        &self.origin
    }
    pub fn size(&self) -> u64 {
        self.size
    }
    pub fn root(&self) -> &Hash {
        &self.root
    }
    /// The operator-signed note exactly as it was presented (this is what peers exchange).
    pub fn note(&self) -> &str {
        &self.note
    }
}

/// Two operator-signed notes for the same origin and size with different roots. Verifiable by anyone who
/// knows the operator key, without trusting the reporter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquivocationProof {
    pub origin: String,
    pub note_a: String,
    pub note_b: String,
}

/// Verify an equivocation proof against the operator key. Returns the two conflicting checkpoints.
pub fn verify_equivocation_proof(
    proof: &EquivocationProof,
    operator: &KeyInfo,
) -> Result<(Checkpoint, Checkpoint), WitnessError> {
    let bad = WitnessError::BadEquivocationProof;
    let a = LastSeen::from_note(&proof.note_a, &proof.origin, operator)
        .map_err(|_| bad("first note does not verify under the operator key for the origin"))?;
    let b = LastSeen::from_note(&proof.note_b, &proof.origin, operator)
        .map_err(|_| bad("second note does not verify under the operator key for the origin"))?;
    if a.size != b.size {
        return Err(bad("sizes differ"));
    }
    if a.root == b.root {
        return Err(bad("roots are identical"));
    }
    let mk = |l: &LastSeen| Checkpoint {
        origin: l.origin.clone(),
        size: l.size,
        root: l.root,
    };
    Ok((mk(&a), mk(&b)))
}

/// Relationship of an offered checkpoint to a held one.
pub(crate) enum Rel {
    Same,
    Equivocation(Box<EquivocationProof>),
    OfferedNewerLinked,
    OfferedNewerUnlinked,
    Behind,
}

/// Verify a consistency proof from (first, first_root) to (second, second_root); bounded and total.
pub(crate) fn check_consistency(
    first: u64,
    first_root: &Hash,
    second: u64,
    second_root: &Hash,
    proof: &[Hash],
) -> Result<(), WitnessError> {
    if proof.len() > MAX_PROOF_HASHES {
        return Err(WitnessError::ProofTooLong);
    }
    if first == 0 {
        // The empty tree is consistent with every tree; no proof hashes are meaningful.
        return if proof.is_empty() && *first_root == sha256(b"") {
            Ok(())
        } else {
            Err(WitnessError::BadConsistencyProof)
        };
    }
    let f = usize::try_from(first).map_err(|_| WitnessError::SizeTooLarge)?;
    let s = usize::try_from(second).map_err(|_| WitnessError::SizeTooLarge)?;
    vc_log::verify_consistency(f, s, first_root, second_root, proof)
        .map_err(|_| WitnessError::BadConsistencyProof)
}

pub(crate) fn relate(
    held: &LastSeen,
    offered: &LastSeen,
    proof: Option<&[Hash]>,
) -> Result<Rel, WitnessError> {
    use std::cmp::Ordering::*;
    match offered.size.cmp(&held.size) {
        Equal if offered.root == held.root => Ok(Rel::Same),
        Equal => Ok(Rel::Equivocation(Box::new(EquivocationProof {
            origin: held.origin.clone(),
            note_a: held.note.clone(),
            note_b: offered.note.clone(),
        }))),
        Less => Ok(Rel::Behind),
        Greater => match proof {
            None => Ok(Rel::OfferedNewerUnlinked),
            Some(p) => {
                check_consistency(held.size, &held.root, offered.size, &offered.root, p)?;
                Ok(Rel::OfferedNewerLinked)
            }
        },
    }
}

/// Result of merging a peer's last-seen record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeOutcome {
    /// No prior record for this origin; the offered one was adopted (Gossip only).
    Added,
    /// Same size and root.
    Same,
    /// Offered is larger and a consistency proof linked it to ours. Gossip adopts it; a `Witness` does NOT
    /// (only `add_checkpoint` advances witness state).
    Advanced,
    /// Offered is smaller than what we hold; nothing learned.
    Behind,
    /// Offered is larger but no proof was supplied; not adopted. Fetch a proof and merge again.
    NeedsProof { held: u64, offered: u64 },
    /// Split view: two operator-signed roots for one size. Third-party verifiable.
    Equivocation(Box<EquivocationProof>),
    /// A `Witness` has no state for this origin, so there is nothing to compare.
    NoLocalState,
}

/// In-memory gossip state: the largest verified `LastSeen` per configured origin, plus the first equivocation
/// proof found per origin. Bounded by the number of configured logs (at most [`MAX_LOGS`]).
pub struct Gossip {
    operators: HashMap<String, KeyInfo>,
    seen: HashMap<String, LastSeen>,
    proofs: HashMap<String, EquivocationProof>,
}

impl Gossip {
    pub fn new(logs: Vec<(String, KeyInfo)>) -> Result<Self, WitnessError> {
        if logs.len() > MAX_LOGS {
            return Err(WitnessError::BadConfig("too many logs".into()));
        }
        let mut operators = HashMap::new();
        for (o, k) in logs {
            if o.is_empty() || o.len() > MAX_ORIGIN_BYTES || o.contains('\n') {
                return Err(WitnessError::BadConfig("bad origin".into()));
            }
            if operators.insert(o, k).is_some() {
                return Err(WitnessError::BadConfig("duplicate origin".into()));
            }
        }
        Ok(Self {
            operators,
            seen: HashMap::new(),
            proofs: HashMap::new(),
        })
    }

    /// Merge a peer's record (or a witness's own). The offered note is re-verified against the configured
    /// operator key, never trusted as given. `proof` is an optional consistency proof from our held size to
    /// the offered size. Held state only changes on `Added` and `Advanced`.
    pub fn merge(
        &mut self,
        offered: &LastSeen,
        proof: Option<&[Hash]>,
    ) -> Result<MergeOutcome, WitnessError> {
        let op = self
            .operators
            .get(offered.origin())
            .ok_or(WitnessError::UnknownOrigin)?;
        let fresh = LastSeen::from_note(offered.note(), offered.origin(), op)?;
        let Some(held) = self.seen.get(fresh.origin()) else {
            self.seen.insert(fresh.origin.clone(), fresh);
            return Ok(MergeOutcome::Added);
        };
        match relate(held, &fresh, proof)? {
            Rel::Same => Ok(MergeOutcome::Same),
            Rel::Behind => Ok(MergeOutcome::Behind),
            Rel::OfferedNewerUnlinked => Ok(MergeOutcome::NeedsProof {
                held: held.size,
                offered: fresh.size,
            }),
            Rel::OfferedNewerLinked => {
                self.seen.insert(fresh.origin.clone(), fresh);
                Ok(MergeOutcome::Advanced)
            }
            Rel::Equivocation(p) => {
                self.proofs
                    .entry(p.origin.clone())
                    .or_insert_with(|| (*p).clone());
                Ok(MergeOutcome::Equivocation(p))
            }
        }
    }

    pub fn last_seen(&self, origin: &str) -> Option<&LastSeen> {
        self.seen.get(origin)
    }

    /// Records to send to peers (one per origin, sorted by origin for determinism).
    pub fn export(&self) -> Vec<LastSeen> {
        let mut v: Vec<LastSeen> = self.seen.values().cloned().collect();
        v.sort_by(|a, b| a.origin.cmp(&b.origin));
        v
    }

    /// First equivocation proof found for `origin`, if any.
    pub fn equivocation(&self, origin: &str) -> Option<&EquivocationProof> {
        self.proofs.get(origin)
    }
}
