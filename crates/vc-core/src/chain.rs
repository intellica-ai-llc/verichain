//! Statement chain rules (ARC42 v7 §5.2): per-agent seq/prev, intent/outcome pairing, orphans.
use crate::{canonical::canonicalize, hash::{sha256, to_hex}};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum ChainError {
    #[error("statement {0}: missing or invalid field `{1}`")]
    Field(usize, &'static str),
    #[error("statement {0}: seq {1} but expected {2}")]
    SeqGap(usize, u64, u64),
    #[error("statement {0}: prev hash does not match previous statement of this agent")]
    BadPrev(usize),
    #[error("statement {0}: outcome references unknown intent")]
    UnknownIntent(usize),
    #[error("statement {0}: duplicate outcome for intent")]
    DuplicateOutcome(usize),
    #[error("statement {0}: outcome from a different agent than its intent")]
    AgentMismatch(usize),
}

/// Identifier of a statement: hex SHA-256 of its canonical JSON bytes.
pub fn statement_id(v: &Value) -> Result<String, serde_json::Error> {
    Ok(to_hex(&sha256(&canonicalize(v)?)))
}

/// Result of validating a whole log's worth of statements.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// Intent ids with no outcome (orphans). Caller applies the timeout policy.
    pub open_intents: Vec<String>,
}

/// Validate statements in log order. The first statement of an agent must have
/// seq 0 and prev = 64 zeros; later ones seq = last+1 and prev = id of the last.
pub fn validate(stmts: &[Value]) -> Result<Report, ChainError> {
    let zero = "0".repeat(64);
    let mut last: HashMap<String, (u64, String)> = HashMap::new();
    let mut intents: HashMap<String, String> = HashMap::new(); // id -> agent
    let mut order: Vec<String> = Vec::new();
    let mut closed: HashSet<String> = HashSet::new();

    for (i, s) in stmts.iter().enumerate() {
        let agent = s.pointer("/agent/id").and_then(Value::as_str).ok_or(ChainError::Field(i, "agent.id"))?.to_string();
        let seq = s.get("seq").and_then(Value::as_u64).ok_or(ChainError::Field(i, "seq"))?;
        let prev = s.get("prev").and_then(Value::as_str).ok_or(ChainError::Field(i, "prev"))?;
        let kind = s.get("kind").and_then(Value::as_str).ok_or(ChainError::Field(i, "kind"))?;

        let (want_seq, want_prev) = match last.get(&agent) {
            None => (0, zero.clone()),
            Some((n, id)) => (n + 1, id.clone()),
        };
        if seq != want_seq {
            return Err(ChainError::SeqGap(i, seq, want_seq));
        }
        if prev != want_prev {
            return Err(ChainError::BadPrev(i));
        }
        let id = statement_id(s).map_err(|_| ChainError::Field(i, "canonicalization"))?;

        match kind {
            "intent" => {
                intents.insert(id.clone(), agent.clone());
                order.push(id.clone());
            }
            "outcome" => {
                let iid = s.pointer("/pair/intent_id").and_then(Value::as_str).ok_or(ChainError::Field(i, "pair.intent_id"))?;
                let owner = intents.get(iid).ok_or(ChainError::UnknownIntent(i))?;
                if *owner != agent {
                    return Err(ChainError::AgentMismatch(i));
                }
                if !closed.insert(iid.to_string()) {
                    return Err(ChainError::DuplicateOutcome(i));
                }
            }
            _ => {}
        }
        last.insert(agent, (seq, id));
    }
    let open_intents = order.into_iter().filter(|id| !closed.contains(id)).collect();
    Ok(Report { open_intents })
}
