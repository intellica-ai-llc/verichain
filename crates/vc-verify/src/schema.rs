//! Validation of a statement payload against `spec/statement.schema.json` (Decision Primitive v2).
//! Unknown properties are allowed (the schema has no `additionalProperties: false`); every known property is
//! type-checked. Returns the first violation as a JSON-pointer-prefixed message.
use crate::strict::{is_hex64, is_rfc3339};
use serde_json::{Map, Value};

const KINDS: [&str; 10] = [
    "intent",
    "outcome",
    "policy",
    "delegation",
    "artifact",
    "attestation",
    "lifecycle",
    "approval",
    "settlement",
    "governance",
];

fn obj<'a>(v: &'a Value, p: &str) -> Result<&'a Map<String, Value>, String> {
    v.as_object().ok_or(format!("{p}: must be an object"))
}
fn string<'a>(v: &'a Value, p: &str) -> Result<&'a str, String> {
    v.as_str().ok_or(format!("{p}: must be a string"))
}
fn hex64(v: &Value, p: &str) -> Result<(), String> {
    if is_hex64(string(v, p)?) {
        Ok(())
    } else {
        Err(format!("{p}: must be 64 lowercase hex characters"))
    }
}
fn strings(v: &Value, p: &str) -> Result<(), String> {
    let a = v.as_array().ok_or(format!("{p}: must be an array"))?;
    for (i, s) in a.iter().enumerate() {
        string(s, &format!("{p}/{i}"))?;
    }
    Ok(())
}
fn enum_of(v: &Value, p: &str, allowed: &[&str]) -> Result<(), String> {
    let s = string(v, p)?;
    if allowed.contains(&s) {
        Ok(())
    } else {
        Err(format!("{p}: {s:.40?} is not one of {allowed:?}"))
    }
}

/// Validate a parsed statement payload. See module docs.
pub fn validate_statement(v: &Value) -> Result<(), String> {
    let o = obj(v, "")?;
    for r in ["v", "kind", "agent", "seq", "prev", "ts"] {
        if !o.contains_key(r) {
            return Err(format!("/{r}: required property missing"));
        }
    }
    if o["v"].as_u64() != Some(2) {
        return Err("/v: must be 2".into());
    }
    enum_of(&o["kind"], "/kind", &KINDS)?;
    let agent = obj(&o["agent"], "/agent")?;
    match agent.get("id").and_then(Value::as_str) {
        Some(s) if !s.is_empty() => {}
        _ => return Err("/agent/id: must be a non-empty string".into()),
    }
    if let Some(rt) = agent.get("runtime") {
        let rt = obj(rt, "/agent/runtime")?;
        if let Some(k) = rt.get("kind") {
            enum_of(k, "/agent/runtime/kind", &["asl", "gateway", "signer", "sdk"])?;
        }
        if let Some(k) = rt.get("version") {
            string(k, "/agent/runtime/version")?;
        }
    }
    if o["seq"].as_u64().is_none() {
        return Err("/seq: must be an integer >= 0 (representable as u64)".into());
    }
    hex64(&o["prev"], "/prev")?;
    if !is_rfc3339(string(&o["ts"], "/ts")?) {
        return Err("/ts: not an RFC 3339 date-time".into());
    }
    if let Some(p) = o.get("principal") {
        let p = obj(p, "/principal")?;
        if let Some(x) = p.get("delegator") {
            string(x, "/principal/delegator")?;
        }
        if let Some(x) = p.get("capability_chain") {
            strings(x, "/principal/capability_chain")?;
        }
        if let Some(x) = p.get("policy_hash") {
            hex64(x, "/principal/policy_hash")?;
        }
    }
    if let Some(a) = o.get("action") {
        let a = obj(a, "/action")?;
        for k in ["type", "target", "params_commitment"] {
            if let Some(x) = a.get(k) {
                string(x, &format!("/action/{k}"))?;
            }
        }
    }
    if let Some(p) = o.get("pair") {
        let p = obj(p, "/pair")?;
        hex64(p.get("intent_id").ok_or("/pair/intent_id: required property missing")?, "/pair/intent_id")?;
    }
    if let Some(vd) = o.get("verdict") {
        let vd = obj(vd, "/verdict")?;
        if let Some(x) = vd.get("decision") {
            enum_of(x, "/verdict/decision", &["allow", "deny", "escalate"])?;
        }
        if let Some(x) = vd.get("reasons") {
            strings(x, "/verdict/reasons")?;
        }
    }
    if let Some(x) = o.get("io") {
        obj(x, "/io")?;
    }
    if let Some(x) = o.get("completeness") {
        enum_of(x, "/completeness", &["C0", "C1", "C2", "C3"])?;
    }
    if let Some(x) = o.get("assurance") {
        let a = x.as_array().ok_or("/assurance: must be an array")?;
        for (i, s) in a.iter().enumerate() {
            enum_of(s, &format!("/assurance/{i}"), &["A0", "A1", "A2", "A3"])?;
        }
    }
    if let Some(x) = o.get("ext") {
        obj(x, "/ext")?;
    }
    if o["kind"] == "outcome" && !o.contains_key("pair") {
        return Err("/pair: required when kind is outcome".into());
    }
    Ok(())
}
