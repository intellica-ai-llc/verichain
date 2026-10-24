//! Differential tests against independently authored vectors. Requirement trace: FR-L0-01 (chain), canonicalization.
use serde_json::Value;
use vc_core::{canonical::canonicalize_str, chain::*};

fn load(name: &str) -> Value {
    let p = format!("{}/../../spec/vectors/{}", env!("CARGO_MANIFEST_DIR"), name);
    serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap()
}

#[test]
fn jcs_vectors() {
    let d = load("jcs_v1.json");
    let mut bad = vec![];
    for v in d["vectors"].as_array().unwrap() {
        let want = hex::decode(v["expected_canonical_hex"].as_str().unwrap()).unwrap();
        match canonicalize_str(v["input_json_text"].as_str().unwrap()) {
            Ok(got) if got == want => {}
            other => bad.push(format!(
                "{} (confidence {}): got {:?}",
                v["name"],
                v["confidence"],
                other.map(|b| String::from_utf8_lossy(&b).to_string())
            )),
        }
    }
    assert!(bad.is_empty(), "JCS vector mismatches:\n{}", bad.join("\n"));
}

#[test]
fn chain_vectors() {
    let d = load("chain_v1.json");
    let mut bad = vec![];
    for c in d["cases"].as_array().unwrap() {
        let stmts: Vec<Value> = c["statements"].as_array().unwrap().clone();
        // ids must agree with the independent implementation
        for (i, s) in stmts.iter().enumerate() {
            let want = c["statement_ids"][i].as_str().unwrap();
            if statement_id(s).unwrap() != want {
                bad.push(format!("{}: id[{i}] differs", c["name"]));
            }
        }
        let r = validate(&stmts);
        match (c["expected"].as_str().unwrap(), r) {
            ("ok", Ok(rep)) => {
                let want: Vec<String> = c["open_intent_ids"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect();
                if rep.open_intents != want {
                    bad.push(format!("{}: open intents differ", c["name"]));
                }
            }
            ("error", Err(e)) => {
                let dbg = format!("{e:?}");
                let want = c["error"].as_str().unwrap();
                let idx = c["index"].as_u64().unwrap();
                if !dbg.starts_with(want) || !dbg.contains(&format!("({idx}")) {
                    bad.push(format!(
                        "{}: expected {want} at {idx}, got {dbg}",
                        c["name"]
                    ));
                }
            }
            (exp, got) => bad.push(format!("{}: expected {exp}, got {got:?}", c["name"])),
        }
    }
    assert!(
        bad.is_empty(),
        "chain vector mismatches:\n{}",
        bad.join("\n")
    );
}
