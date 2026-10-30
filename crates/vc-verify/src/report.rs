//! Verification report: `{ok, checks:[{name, ok, detail}], partial_coverage}`.
use crate::limits::{MAX_DETAIL_ITEMS, MAX_ECHO_CHARS};
use serde::Serialize;

pub type Res = Result<String, String>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Check {
    pub name: String,
    pub ok: bool,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Report {
    /// True only if every check passed.
    pub ok: bool,
    pub checks: Vec<Check>,
    /// True if some agent's chain was only verified as a consecutive run that does not start at seq 0.
    /// Never a silent pass: the chain_rules detail names the agents.
    pub partial_coverage: bool,
}

impl Report {
    pub fn new(checks: Vec<Check>, partial_coverage: bool) -> Self {
        Report { ok: checks.iter().all(|c| c.ok), checks, partial_coverage }
    }
    pub fn check(&self, name: &str) -> Option<&Check> {
        self.checks.iter().find(|c| c.name == name)
    }
    pub fn to_json(&self) -> String {
        // Serialization of plain strings/bools cannot fail; fall back to a fixed failing report regardless.
        serde_json::to_string(self).unwrap_or_else(|_| {
            r#"{"ok":false,"checks":[{"name":"report","ok":false,"detail":"serialization failed"}],"partial_coverage":false}"#.into()
        })
    }
}

/// Truncate hostile text echoed into a report.
pub fn echo(s: &str) -> String {
    let mut out: String = s.chars().take(MAX_ECHO_CHARS).map(|c| if c.is_control() { '?' } else { c }).collect();
    if s.chars().count() > MAX_ECHO_CHARS {
        out.push_str("...");
    }
    out
}

pub fn check_of(name: &str, r: Res) -> Check {
    match r {
        Ok(d) => Check { name: name.into(), ok: true, detail: d },
        Err(d) => Check { name: name.into(), ok: false, detail: d },
    }
}

/// Aggregate per-entry results: ok iff all ok; failing detail lists the first few failing entries.
pub fn aggregate(name: &str, results: &[Res]) -> Check {
    let bad: Vec<(usize, &String)> =
        results.iter().enumerate().filter_map(|(i, r)| r.as_ref().err().map(|e| (i, e))).collect();
    if bad.is_empty() {
        return Check { name: name.into(), ok: true, detail: format!("{} of {} entries ok", results.len(), results.len()) };
    }
    let mut d = format!("{} of {} entries failed: ", bad.len(), results.len());
    let parts: Vec<String> = bad.iter().take(MAX_DETAIL_ITEMS).map(|(i, e)| format!("entry {i}: {}", echo(e))).collect();
    d.push_str(&parts.join("; "));
    if bad.len() > MAX_DETAIL_ITEMS {
        d.push_str(&format!("; (+{} more)", bad.len() - MAX_DETAIL_ITEMS));
    }
    Check { name: name.into(), ok: false, detail: d }
}
