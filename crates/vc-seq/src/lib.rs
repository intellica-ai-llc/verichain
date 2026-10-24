//! vc-seq: the sequencer core (ARC42 v9 §5.1, FR-L0-07).
//!
//! A submission is a COSE_Sign1 statement. It is accepted only if: the signature verifies under a registered
//! issuer key (strict COSE rules), the payload is exactly canonical RFC 8785 JSON, the statement is bound to the
//! issuer's agent id and does not claim more completeness than the issuer class allows, and the per-agent chain
//! rules hold. Accepted statements are appended durably (entry bundle first) before a registration is returned.
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde_json::Value;
use std::collections::HashMap;
use vc_core::canonical::{canonicalize_str, CanonError};
use vc_core::chain::{ChainError, ChainState};
use vc_core::hash::{leaf_hash, to_hex, Hash};
use vc_cose::CoseError;
use vc_log::checkpoint::{note, sign_line, Checkpoint};
use vc_store::{StoreError, TileLog, TileStore};

/// Maximum accepted statement size (COSE bytes).
pub const MAX_STATEMENT_BYTES: usize = 64 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum SeqError {
    #[error("statement too large")]
    TooLarge,
    #[error("cose: {0}")]
    Cose(#[from] CoseError),
    #[error("payload is not canonical RFC 8785 JSON")]
    NotCanonical,
    #[error("payload is not valid I-JSON: {0}")]
    Json(#[from] CanonError),
    #[error("chain rule violated: {0}")]
    Chain(#[from] ChainError),
    #[error("statement agent id does not match the issuer's registered agent")]
    AgentBinding,
    #[error("statement claims completeness {claimed} above issuer maximum {max}")]
    Completeness { claimed: String, max: String },
    #[error("storage: {0}")]
    Store(#[from] StoreError),
}

/// A registered issuer: key, the single agent id it may speak for, and its completeness ceiling (0..=3).
#[derive(Clone)]
pub struct Issuer {
    pub key: VerifyingKey,
    pub agent_id: String,
    pub max_completeness: u8,
}

#[derive(Default, Clone)]
pub struct Registry {
    by_kid: HashMap<Vec<u8>, Issuer>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(&mut self, kid: &[u8], issuer: Issuer) {
        self.by_kid.insert(kid.to_vec(), issuer);
    }
    fn lookup(&self, kid: &[u8]) -> Option<&Issuer> {
        self.by_kid.get(kid)
    }
}

/// What a client receives when a statement is registered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Registration {
    pub index: u64,
    pub leaf_hash: Hash,
    pub statement_id: String,
    /// True if this exact statement had already been registered (idempotent retry).
    pub duplicate: bool,
}

pub struct Sequencer<S: TileStore> {
    log: TileLog<S>,
    chain: ChainState,
    registry: Registry,
    by_id: HashMap<String, u64>,
    operator: SigningKey,
    operator_name: String,
    origin: String,
}

fn completeness_rank(s: &str) -> Option<u8> {
    match s {
        "C0" => Some(0),
        "C1" => Some(1),
        "C2" => Some(2),
        "C3" => Some(3),
        _ => None,
    }
}

impl<S: TileStore> Sequencer<S> {
    /// Open a sequencer over a store. Replays all entries to rebuild chain state (trusts its own log).
    pub fn open(
        store: S,
        registry: Registry,
        operator: SigningKey,
        operator_name: &str,
        origin: &str,
    ) -> Result<Self, SeqError> {
        let log = TileLog::open(store)?;
        let mut s = Sequencer {
            log,
            chain: ChainState::new(),
            registry,
            by_id: HashMap::new(),
            operator,
            operator_name: operator_name.to_string(),
            origin: origin.to_string(),
        };
        s.replay()?;
        Ok(s)
    }

    fn replay(&mut self) -> Result<(), SeqError> {
        self.chain = ChainState::new();
        self.by_id.clear();
        for i in 0..self.log.size() {
            let e = self.log.entry(i)?;
            let (_, payload) = vc_cose::peek_payload(&e)?;
            let v: Value = serde_json::from_slice(&payload).map_err(|_| SeqError::NotCanonical)?;
            let c = self.chain.check(i as usize, &v)?;
            self.by_id.insert(c.id().to_string(), i);
            self.chain.commit(c);
        }
        Ok(())
    }

    pub fn size(&self) -> u64 {
        self.log.size()
    }

    pub fn log(&self) -> &TileLog<S> {
        &self.log
    }

    pub fn open_intents(&self) -> Vec<String> {
        self.chain.open_intents()
    }

    /// Validate one submission against the current state without changing anything.
    fn validate(
        &self,
        cose: &[u8],
        index_hint: usize,
    ) -> Result<(vc_core::chain::Checked, Value), SeqError> {
        if cose.len() > MAX_STATEMENT_BYTES {
            return Err(SeqError::TooLarge);
        }
        let reg = &self.registry;
        let (kid, payload) = vc_cose::verify1(cose, |kid| reg.lookup(kid).map(|i| i.key))?;
        let issuer = reg.lookup(&kid).ok_or(CoseError::Kid)?.clone();
        let text = std::str::from_utf8(&payload).map_err(|_| SeqError::NotCanonical)?;
        let canon = canonicalize_str(text)?;
        if canon != payload {
            return Err(SeqError::NotCanonical);
        }
        let v: Value = serde_json::from_slice(&payload).map_err(|_| SeqError::NotCanonical)?;
        if v.pointer("/agent/id").and_then(Value::as_str) != Some(issuer.agent_id.as_str()) {
            return Err(SeqError::AgentBinding);
        }
        if let Some(c) = v.get("completeness").and_then(Value::as_str) {
            let claimed = completeness_rank(c).unwrap_or(u8::MAX);
            if claimed > issuer.max_completeness {
                return Err(SeqError::Completeness {
                    claimed: c.to_string(),
                    max: format!("C{}", issuer.max_completeness),
                });
            }
        }
        let checked = self.chain.check(index_hint, &v)?;
        Ok((checked, v))
    }

    /// Register one statement (durable before return).
    pub fn submit(&mut self, cose: &[u8]) -> Result<Registration, SeqError> {
        self.submit_batch(&[cose]).pop().expect("one result")
    }

    /// Register a batch with group commit: each statement is validated in order against the state left by the
    /// accepted ones before it; accepted statements are appended in one durable write. Results are per statement.
    pub fn submit_batch(&mut self, batch: &[&[u8]]) -> Vec<Result<Registration, SeqError>> {
        let mut results: Vec<Option<Result<Registration, SeqError>>> =
            (0..batch.len()).map(|_| None).collect();
        let mut accepted: Vec<(usize, &[u8], vc_core::chain::Checked)> = Vec::new();
        let base = self.log.size();
        for (i, cose) in batch.iter().enumerate() {
            let next_index = base + accepted.len() as u64;
            // idempotent retry of an already-registered statement
            if let Some(dup) = self.duplicate_of(cose) {
                results[i] = Some(Ok(dup));
                continue;
            }
            match self.validate(cose, next_index as usize) {
                Ok((checked, _)) => {
                    // make the accepted statement visible to later statements in this batch
                    self.chain.commit(checked.clone());
                    self.by_id.insert(checked.id().to_string(), next_index);
                    accepted.push((i, cose, checked));
                }
                Err(e) => results[i] = Some(Err(e)),
            }
        }
        if !accepted.is_empty() {
            let entries: Vec<&[u8]> = accepted.iter().map(|(_, c, _)| *c).collect();
            match self.log.append_batch(&entries) {
                Ok((first, _)) => {
                    for (n, (i, cose, checked)) in accepted.iter().enumerate() {
                        results[*i] = Some(Ok(Registration {
                            index: first + n as u64,
                            leaf_hash: leaf_hash(cose),
                            statement_id: checked.id().to_string(),
                            duplicate: false,
                        }));
                    }
                }
                Err(e) => {
                    // durable write failed: roll in-memory state back to what the log actually holds
                    let msg = e.to_string();
                    let _ = self.replay();
                    for (i, _, _) in &accepted {
                        results[*i] = Some(Err(SeqError::Store(StoreError::Io(
                            std::io::Error::other(msg.clone()),
                        ))));
                    }
                }
            }
        }
        results
            .into_iter()
            .map(|r| r.expect("every statement has a result"))
            .collect()
    }

    /// If `cose` is byte-identical to an already-registered statement, return its registration.
    fn duplicate_of(&self, cose: &[u8]) -> Option<Registration> {
        let (_, payload) = vc_cose::peek_payload(cose).ok()?;
        let text = std::str::from_utf8(&payload).ok()?;
        let canon = canonicalize_str(text).ok()?;
        let id = to_hex(&vc_core::hash::sha256(&canon));
        let idx = *self.by_id.get(&id)?;
        let stored = self.log.entry(idx).ok()?;
        (stored == cose).then(|| Registration {
            index: idx,
            leaf_hash: leaf_hash(cose),
            statement_id: id,
            duplicate: true,
        })
    }

    /// Sign a checkpoint for the current tree (operator signature only; witnesses cosign separately).
    pub fn checkpoint(&self) -> Result<(Checkpoint, String), SeqError> {
        let cp = Checkpoint {
            origin: self.origin.clone(),
            size: self.log.size(),
            root: self.log.root()?,
        };
        let text = cp.text();
        let n = note(
            &text,
            &[sign_line(&text, &self.operator_name, &self.operator)],
        );
        Ok((cp, n))
    }
}
