//! The witness: verify, persist, then cosign (FR-L0-08, C6).
use crate::cosig::{cosign_line, split_note};
use crate::evidence::{check_consistency, relate, Rel};
use crate::state::{
    decode_evidence, decode_state, encode_evidence, encode_state, evidence_path, state_path,
};
use crate::{
    EquivocationProof, LastSeen, MergeOutcome, WitnessError, MAX_LOGS, MAX_NOTE_BYTES,
    MAX_ORIGIN_BYTES, MAX_PROOF_HASHES,
};
use ed25519_dalek::SigningKey;
use std::collections::HashMap;
use std::sync::Mutex;
use vc_core::Hash;
use vc_log::checkpoint::{sign_line, valid_name, Checkpoint, KeyInfo};
use vc_store::TileStore;

/// Source of cosignature timestamps (Unix seconds). `None` means the clock is unavailable.
pub trait Clock: Send + Sync {
    fn now_unix(&self) -> Option<u64>;
}

pub struct SystemClock;
impl Clock for SystemClock {
    fn now_unix(&self) -> Option<u64> {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()
            .map(|d| d.as_secs())
    }
}

/// A log this witness serves.
#[derive(Clone)]
pub struct LogConfig {
    pub origin: String,
    pub operator: KeyInfo,
    /// Trust-on-first-use: accept the first checkpoint seen for this origin without a consistency proof.
    /// Explicit opt-in; `LogConfig::new` sets it to false.
    pub tofu: bool,
}

impl LogConfig {
    pub fn new(origin: impl Into<String>, operator: KeyInfo) -> Self {
        Self { origin: origin.into(), operator, tofu: false }
    }
    pub fn with_tofu(mut self, tofu: bool) -> Self {
        self.tofu = tofu;
        self
    }
}

pub struct WitnessConfig {
    /// Cosigner key name (signed-note rules: non-empty, no whitespace, no '+').
    pub name: String,
    pub key: SigningKey,
    pub logs: Vec<LogConfig>,
    /// Also emit a plain type-0x01 Ed25519 line (same name/key) so `vc_log::checkpoint::verify_note*` counts
    /// this witness. Default true. See crate docs.
    pub emit_legacy_line: bool,
}

impl WitnessConfig {
    pub fn new(name: impl Into<String>, key: SigningKey, logs: Vec<LogConfig>) -> Self {
        Self { name: name.into(), key, logs, emit_legacy_line: true }
    }
}

/// Result of a successful `add_checkpoint`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CosignedNote {
    pub checkpoint: Checkpoint,
    /// Unix-seconds timestamp inside the tlog-cosignature line.
    pub timestamp: u64,
    /// The signature line(s) this witness adds (what a tlog-witness HTTP binding returns).
    pub signature_lines: String,
    /// The presented note with `signature_lines` appended.
    pub note: String,
}

struct Log {
    operator: KeyInfo,
    tofu: bool,
    /// Serialises every read-modify-write transition for this origin.
    lock: Mutex<()>,
}

pub struct Witness<S: TileStore> {
    store: S,
    name: String,
    key: SigningKey,
    emit_legacy_line: bool,
    logs: HashMap<String, Log>,
    clock: Box<dyn Clock>,
}

fn io(e: std::io::Error) -> WitnessError {
    WitnessError::Store(e)
}

impl<S: TileStore> Witness<S> {
    pub fn open(store: S, config: WitnessConfig) -> Result<Self, WitnessError> {
        Self::open_with_clock(store, config, Box::new(SystemClock))
    }

    /// Validates the configuration and takes the store's writer lock (a second witness process on the same
    /// `FsStore` fails here instead of racing on state).
    pub fn open_with_clock(
        store: S,
        config: WitnessConfig,
        clock: Box<dyn Clock>,
    ) -> Result<Self, WitnessError> {
        let bad = |s: &str| WitnessError::BadConfig(s.to_string());
        if !valid_name(&config.name) {
            return Err(bad("invalid witness key name"));
        }
        if config.logs.len() > MAX_LOGS {
            return Err(bad("too many logs"));
        }
        let wk = config.key.verifying_key();
        let mut logs = HashMap::new();
        for l in config.logs {
            if l.origin.is_empty() || l.origin.len() > MAX_ORIGIN_BYTES || l.origin.contains('\n') {
                return Err(bad("invalid origin"));
            }
            if l.operator.key.is_weak() || !valid_name(&l.operator.name) {
                return Err(bad("invalid operator key"));
            }
            if l.operator.key == wk || l.operator.name == config.name {
                return Err(bad("witness key/name must differ from the operator's"));
            }
            let log = Log { operator: l.operator, tofu: l.tofu, lock: Mutex::new(()) };
            if logs.insert(l.origin, log).is_some() {
                return Err(bad("duplicate origin"));
            }
        }
        store.lock_writer().map_err(io)?;
        Ok(Self {
            store,
            name: config.name,
            key: config.key,
            emit_legacy_line: config.emit_legacy_line,
            logs,
            clock,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn verifying_key(&self) -> ed25519_dalek::VerifyingKey {
        self.key.verifying_key()
    }

    fn log(&self, origin: &str) -> Result<&Log, WitnessError> {
        self.logs.get(origin).ok_or(WitnessError::UnknownOrigin)
    }

    fn load(&self, origin: &str, log: &Log) -> Result<Option<LastSeen>, WitnessError> {
        let Some(bytes) = self.store.get(&state_path(origin)).map_err(io)? else {
            return Ok(None);
        };
        let rec = decode_state(&bytes)?;
        let ls = LastSeen::from_note(&rec.note, origin, &log.operator)
            .map_err(|e| WitnessError::CorruptState(format!("stored note invalid: {e}")))?;
        if rec.origin != origin || rec.size != ls.size() || rec.root != *ls.root() {
            return Err(WitnessError::CorruptState("record disagrees with its note".into()));
        }
        Ok(Some(ls))
    }

    fn persist_state(&self, ls: &LastSeen) -> Result<(), WitnessError> {
        let rec = encode_state(ls.origin(), ls.size(), ls.root(), ls.note());
        self.store.put(&state_path(ls.origin()), &rec).map_err(io)
    }

    /// Record evidence once per origin (first wins). Returns whether it is durably recorded.
    fn record_evidence(&self, p: &EquivocationProof) -> bool {
        let path = evidence_path(&p.origin);
        match self.store.exists(&path) {
            Ok(true) => true,
            Ok(false) => self.store.put(&path, &encode_evidence(p)).is_ok(),
            Err(_) => false,
        }
    }

    fn equivocation_error(&self, p: Box<EquivocationProof>) -> WitnessError {
        let persisted = self.record_evidence(&p);
        WitnessError::Equivocation { evidence: p, persisted }
    }

    /// The last checkpoint this witness cosigned or bootstrapped for `origin`.
    pub fn last_seen(&self, origin: &str) -> Result<Option<LastSeen>, WitnessError> {
        let log = self.log(origin)?;
        let _g = log.lock.lock().unwrap_or_else(|p| p.into_inner());
        self.load(origin, log)
    }

    /// The persisted, reportable equivocation evidence for `origin`, if any was recorded.
    pub fn equivocation_evidence(&self, origin: &str) -> Result<Option<EquivocationProof>, WitnessError> {
        let log = self.log(origin)?;
        let _g = log.lock.lock().unwrap_or_else(|p| p.into_inner());
        match self.store.get(&evidence_path(origin)).map_err(io)? {
            None => Ok(None),
            Some(b) => {
                let p = decode_evidence(&b)?;
                if p.origin != origin {
                    return Err(WitnessError::CorruptState("evidence origin mismatch".into()));
                }
                crate::verify_equivocation_proof(&p, &log.operator)
                    .map_err(|e| WitnessError::CorruptState(format!("stored evidence invalid: {e}")))?;
                Ok(Some(p))
            }
        }
    }

    /// Explicit administrative bootstrap: adopt an operator-signed checkpoint as the initial last-seen state
    /// without cosigning it. Only when no state exists. This is how a witness with TOFU disabled starts.
    pub fn bootstrap(&self, note: &str) -> Result<Checkpoint, WitnessError> {
        let origin = parse_origin(note)?;
        let log = self.log(&origin)?;
        let seen = LastSeen::from_note(note, &origin, &log.operator)?;
        let _g = log.lock.lock().unwrap_or_else(|p| p.into_inner());
        if self.load(&origin, log)?.is_some() {
            return Err(WitnessError::AlreadyInitialised);
        }
        self.persist_state(&seen)?;
        Ok(Checkpoint { origin, size: seen.size(), root: *seen.root() })
    }

    /// C6 `add-checkpoint`. State is persisted (one atomic store put) BEFORE the cosignature is produced;
    /// any failure returns an error and no cosignature.
    pub fn add_checkpoint(
        &self,
        note: &str,
        old_size: u64,
        proof: &[Hash],
    ) -> Result<CosignedNote, WitnessError> {
        if note.len() > MAX_NOTE_BYTES {
            return Err(WitnessError::NoteTooLarge);
        }
        if proof.len() > MAX_PROOF_HASHES {
            return Err(WitnessError::ProofTooLong);
        }
        let origin = parse_origin(note)?;
        let log = self.log(&origin)?;
        let seen = LastSeen::from_note(note, &origin, &log.operator)?;
        let ts = self.clock.now_unix().ok_or(WitnessError::Clock)?;

        let _g = log.lock.lock().unwrap_or_else(|p| p.into_inner());
        match self.load(&origin, log)? {
            None => {
                if !log.tofu {
                    return Err(WitnessError::TofuDisabled);
                }
                if old_size != 0 {
                    return Err(WitnessError::OldSizeMismatch { expected: 0, got: old_size });
                }
                if !proof.is_empty() {
                    return Err(WitnessError::BadConsistencyProof);
                }
                self.persist_state(&seen)?;
            }
            Some(held) => match relate(&held, &seen, None)? {
                Rel::Behind => {
                    return Err(WitnessError::SizeRegression {
                        last_seen: held.size(),
                        presented: seen.size(),
                    })
                }
                Rel::Equivocation(p) => return Err(self.equivocation_error(p)),
                Rel::Same => {
                    if old_size != held.size() {
                        return Err(WitnessError::OldSizeMismatch {
                            expected: held.size(),
                            got: old_size,
                        });
                    }
                    if !proof.is_empty() {
                        return Err(WitnessError::BadConsistencyProof);
                    }
                    // idempotent: nothing to persist
                }
                Rel::OfferedNewerLinked | Rel::OfferedNewerUnlinked => {
                    if old_size != held.size() {
                        return Err(WitnessError::OldSizeMismatch {
                            expected: held.size(),
                            got: old_size,
                        });
                    }
                    check_consistency(held.size(), held.root(), seen.size(), seen.root(), proof)?;
                    self.persist_state(&seen)?;
                }
            },
        }

        // State is durable; now (and only now) sign.
        let (text, _) = split_note(note)?;
        let mut lines = cosign_line(text, &self.name, &self.key, ts)?;
        if self.emit_legacy_line {
            lines.push_str(&sign_line(text, &self.name, &self.key));
        }
        let mut full = note.to_string();
        if !full.ends_with('\n') {
            full.push('\n');
        }
        full.push_str(&lines);
        Ok(CosignedNote {
            checkpoint: Checkpoint { origin, size: seen.size(), root: *seen.root() },
            timestamp: ts,
            signature_lines: lines,
            note: full,
        })
    }

    /// Gossip input: compare a peer's last-seen record with ours. Detects split views and persists the
    /// evidence; never advances witness state and never cosigns.
    pub fn observe_peer(
        &self,
        offered: &LastSeen,
        proof: Option<&[Hash]>,
    ) -> Result<MergeOutcome, WitnessError> {
        let log = self.log(offered.origin())?;
        let fresh = LastSeen::from_note(offered.note(), offered.origin(), &log.operator)?;
        let _g = log.lock.lock().unwrap_or_else(|p| p.into_inner());
        let Some(held) = self.load(fresh.origin(), log)? else {
            return Ok(MergeOutcome::NoLocalState);
        };
        match relate(&held, &fresh, proof)? {
            Rel::Same => Ok(MergeOutcome::Same),
            Rel::Behind => Ok(MergeOutcome::Behind),
            Rel::OfferedNewerLinked => Ok(MergeOutcome::Advanced),
            Rel::OfferedNewerUnlinked => Ok(MergeOutcome::NeedsProof {
                held: held.size(),
                offered: fresh.size(),
            }),
            Rel::Equivocation(p) => {
                self.record_evidence(&p);
                Ok(MergeOutcome::Equivocation(p))
            }
        }
    }
}

fn parse_origin(note: &str) -> Result<String, WitnessError> {
    let (text, _) = split_note(note)?;
    let cp = Checkpoint::parse(text).map_err(WitnessError::Malformed)?;
    Ok(cp.origin)
}
