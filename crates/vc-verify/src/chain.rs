//! Chain rules over a pack's entries in log order, with consecutive-run semantics (C5).
//!
//! A pack may be a subset of the log. Per agent the provided statements must form a consecutive run:
//! `seq` increases by exactly 1 and `prev` equals the id of the previous statement of that agent. If an agent's
//! first provided statement is `seq 0` it must carry the all-zero `prev` (full coverage). Otherwise the run is
//! reported as `partial_coverage` (never a silent pass). Intent/outcome pairing follows `vc_core::chain`; an
//! outcome whose intent is outside the pack is only tolerated for an agent whose run is partial, and is reported.
//! When no agent is partial the result is cross-checked against `vc_core::chain::validate` (fail-closed on any
//! disagreement).
use crate::limits::MAX_DETAIL_ITEMS;
use crate::report::echo;
use serde_json::Value;
use std::collections::{BTreeSet, HashMap, HashSet};

pub struct ChainInput<'a> {
    pub leaf_index: u64,
    pub value: &'a Value,
    /// Hex SHA-256 of the statement's canonical (JCS) bytes.
    pub id: &'a str,
}

pub struct ChainOutcome {
    pub result: Result<String, String>,
    pub partial_agents: Vec<String>,
    pub open_intents: Vec<String>,
}

const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";

pub fn check_chain(stmts: &[ChainInput<'_>]) -> ChainOutcome {
    let fail = |m: String| ChainOutcome { result: Err(m), partial_agents: vec![], open_intents: vec![] };

    for w in stmts.windows(2) {
        if w[1].leaf_index <= w[0].leaf_index {
            return fail(format!(
                "entries not in strictly increasing log order (leaf_index {} followed by {})",
                w[0].leaf_index, w[1].leaf_index
            ));
        }
    }

    struct Run {
        seq: u64,
        id: String,
        partial: bool,
    }
    let mut runs: HashMap<String, Run> = HashMap::new();
    let mut intents: HashMap<String, String> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    let mut closed: HashSet<String> = HashSet::new();
    let mut unresolved: BTreeSet<String> = BTreeSet::new();

    for (i, s) in stmts.iter().enumerate() {
        let v = s.value;
        let (Some(agent), Some(seq), Some(prev), Some(kind)) = (
            v.pointer("/agent/id").and_then(Value::as_str),
            v.get("seq").and_then(Value::as_u64),
            v.get("prev").and_then(Value::as_str),
            v.get("kind").and_then(Value::as_str),
        ) else {
            return fail(format!("entry {i}: missing agent.id, seq, prev or kind"));
        };
        match runs.get_mut(agent) {
            None => {
                let partial = if seq == 0 {
                    if prev != ZERO {
                        return fail(format!("entry {i}: agent {} seq 0 must have prev of 64 zeros", echo(agent)));
                    }
                    false
                } else {
                    true
                };
                runs.insert(agent.to_string(), Run { seq, id: s.id.to_string(), partial });
            }
            Some(r) => {
                if r.seq.checked_add(1) != Some(seq) {
                    return fail(format!(
                        "entry {i}: agent {} seq {seq} but expected {} (gap, repeat or reorder)",
                        echo(agent),
                        r.seq.saturating_add(1)
                    ));
                }
                if prev != r.id {
                    return fail(format!("entry {i}: agent {} prev does not match previous statement", echo(agent)));
                }
                r.seq = seq;
                r.id = s.id.to_string();
            }
        }
        let agent_partial = runs.get(agent).is_some_and(|r| r.partial);
        match kind {
            "intent" => {
                intents.insert(s.id.to_string(), agent.to_string());
                order.push(s.id.to_string());
            }
            "outcome" => {
                let Some(iid) = v.pointer("/pair/intent_id").and_then(Value::as_str) else {
                    return fail(format!("entry {i}: outcome without pair.intent_id"));
                };
                match intents.get(iid) {
                    None if agent_partial => {
                        unresolved.insert(iid.to_string());
                    }
                    None => return fail(format!("entry {i}: outcome references unknown intent")),
                    Some(owner) if owner != agent => {
                        return fail(format!("entry {i}: outcome from a different agent than its intent"))
                    }
                    Some(_) => {
                        if !closed.insert(iid.to_string()) {
                            return fail(format!("entry {i}: duplicate outcome for intent"));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let open_intents: Vec<String> = order.iter().filter(|id| !closed.contains(*id)).cloned().collect();
    let partial_agents: Vec<String> =
        runs.iter().filter(|(_, r)| r.partial).map(|(a, _)| a.clone()).collect::<BTreeSet<_>>().into_iter().collect();

    if partial_agents.is_empty() {
        let values: Vec<Value> = stmts.iter().map(|s| s.value.clone()).collect();
        match vc_core::chain::validate(&values) {
            Ok(rep) if rep.open_intents == open_intents => {}
            Ok(_) => return fail("differential check: vc-core reports different open intents".into()),
            Err(e) => return fail(format!("differential check: vc-core rejects the chain: {e}")),
        }
    }

    let mut d = format!("{} statements over {} agents form consecutive runs in log order", stmts.len(), runs.len());
    if !partial_agents.is_empty() {
        let names: Vec<String> = partial_agents.iter().take(MAX_DETAIL_ITEMS).map(|a| echo(a)).collect();
        d.push_str(&format!(
            "; partial_coverage: run for agent(s) [{}] does not start at seq 0, earlier statements are not in this pack",
            names.join(", ")
        ));
        if !unresolved.is_empty() {
            d.push_str(&format!("; {} outcome(s) reference intents outside the pack", unresolved.len()));
        }
    }
    ChainOutcome { result: Ok(d), partial_agents, open_intents }
}
