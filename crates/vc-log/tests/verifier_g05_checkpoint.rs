//! Verifier G05: checkpoint / signed-note probes. Tests named `bug_*` assert the CORRECT behaviour and
//! FAIL today (confirmed bug). `regression_*` / `ambiguity_*` pass and document current behaviour.
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use ed25519_dalek::SigningKey;
use vc_log::checkpoint::*;

fn sk(n: u8) -> SigningKey {
    SigningKey::from_bytes(&[n; 32])
}
fn ki(name: &str, s: &SigningKey) -> KeyInfo {
    KeyInfo {
        name: name.into(),
        key: s.verifying_key(),
    }
}
fn cp(size: u64) -> Checkpoint {
    Checkpoint {
        origin: "vc.example/log1".into(),
        size,
        root: [7; 32],
    }
}
fn make(text: &str, signers: &[(&str, &SigningKey)]) -> String {
    note(
        text,
        &signers
            .iter()
            .map(|(n, k)| sign_line(text, n, k))
            .collect::<Vec<_>>(),
    )
}

// BUG (medium): quorum counts distinct witness NAMES, not distinct KEYS. One physical key configured under
// two names (or a misconfigured/duplicated trust list) satisfies a 2-of-N quorum with a single signer.
#[test]
fn bug_quorum_counts_names_not_keys() {
    let (op, w) = (sk(1), sk(2));
    let ws = [ki("w-a", &w), ki("w-b", &w)]; // same key, two names
    let t = cp(1).text();
    let n = make(&t, &[("op", &op), ("w-a", &w), ("w-b", &w)]);
    let r = verify_note(&n, &ki("op", &op), &ws, 2);
    assert!(
        r.is_err(),
        "one key signing under two names satisfied a 2-witness quorum"
    );
}

// BUG (medium): operator key listed in the witness set is counted as a witness, so an operator can
// self-satisfy the quorum ("operator counted as a witness").
#[test]
fn bug_operator_counts_as_witness() {
    let op = sk(1);
    let opk = ki("op", &op);
    let t = cp(1).text();
    let n = make(&t, &[("op", &op)]);
    let r = verify_note(&n, &opk, std::slice::from_ref(&opk), 1);
    assert!(r.is_err(), "operator alone satisfied min_witnesses=1");
}

// BUG (medium): alias form of the operator-as-witness bug: operator key registered under a second name.
#[test]
fn bug_operator_alias_as_witness() {
    let op = sk(1);
    let t = cp(1).text();
    let n = make(&t, &[("op", &op), ("op-alias", &op)]);
    let ws = [ki("op-alias", &op)];
    assert!(
        verify_note(&n, &ki("op", &op), &ws, 1).is_err(),
        "operator key under an alias name counted as an independent witness"
    );
}

// BUG (low): size is parsed with u64::from_str, accepting "+5" and "0005". The canonical text() never emits these, so two
// distinct signed byte strings denote the same checkpoint; tlog-checkpoint says decimal integer, no sign, no leading zeros.
#[test]
fn bug_size_noncanonical_accepted() {
    let root = B64.encode([7u8; 32]);
    for s in ["+5", "0005", "00", "-0"] {
        let t = format!("o\n{s}\n{root}\n");
        assert!(Checkpoint::parse(&t).is_err(), "size {s:?} accepted");
    }
}

// REGRESSION/AMBIGUITY: extra lines after the root are silently dropped by parse(). tlog-checkpoint allows non-empty
// extension lines (the signature still covers them), but nothing exposes them to the caller.
#[test]
fn regression_extension_lines_ignored_not_exposed() {
    let root = B64.encode([7u8; 32]);
    let t = format!("o\n5\n{root}\nextension-line\n");
    // passes today: extension lines are accepted and discarded
    assert_eq!(Checkpoint::parse(&t).unwrap().size, 5);
}

#[test]
fn regression_parse_rejects_malformed() {
    let root = B64.encode([7u8; 32]);
    assert!(
        Checkpoint::parse(&format!("o\n5\n{root}")).is_err(),
        "no trailing newline"
    );
    assert!(
        Checkpoint::parse(&format!("o\r\n5\r\n{root}\r\n")).is_err(),
        "CRLF"
    );
    assert!(
        Checkpoint::parse(&format!("\n5\n{root}\n")).is_err(),
        "empty origin"
    );
    assert!(Checkpoint::parse("o\n5\n").is_err());
    assert!(
        Checkpoint::parse("o\n18446744073709551616\nAAAA\n").is_err(),
        "u64 overflow"
    );
    assert!(
        Checkpoint::parse(&format!("o\n 5\n{root}\n")).is_err(),
        "leading space"
    );
    assert!(
        Checkpoint::parse(&format!("o\n5 \n{root}\n")).is_err(),
        "trailing space"
    );
    // non canonical base64: trailing bits set / missing padding / url-safe alphabet / whitespace
    let canon = B64.encode([0u8; 32]); // 43 chars + '='
    assert!(canon.ends_with("A="));
    let noncanon = format!("{}B=", &canon[..42]); // last sextet has nonzero trailing bits
    assert!(
        Checkpoint::parse(&format!("o\n5\n{noncanon}\n")).is_err(),
        "non-canonical trailing bits accepted"
    );
    assert!(
        Checkpoint::parse(&format!("o\n5\n{}\n", canon.trim_end_matches('='))).is_err(),
        "unpadded"
    );
    assert!(Checkpoint::parse(&format!("o\n5\n{canon} \n")).is_err());
    assert!(
        Checkpoint::parse(&format!("o\n5\n{}\n", B64.encode([0u8; 31]))).is_err(),
        "31-byte root"
    );
    assert!(
        Checkpoint::parse(&format!("o\n5\n{}\n", B64.encode([0u8; 33]))).is_err(),
        "33-byte root"
    );
    let url = B64.encode([0xfbu8; 32]).replace('+', "-").replace('/', "_");
    assert!(
        Checkpoint::parse(&format!("o\n5\n{url}\n")).is_err(),
        "url-safe alphabet"
    );
}

// regression: blank line inside note text, CRLF notes, and sig-line framing
#[test]
fn regression_note_framing() {
    let (op, w) = (sk(1), sk(2));
    let ws = [ki("w", &w)];
    let t = cp(3).text();
    let good = make(&t, &[("op", &op), ("w", &w)]);
    assert_eq!(verify_note(&good, &ki("op", &op), &ws, 1).unwrap(), cp(3));
    // no blank separator line
    assert!(verify_note(&good.replacen("\n\n", "\n", 1), &ki("op", &op), &ws, 1).is_err());
    // CRLF translation of the whole note must not verify
    assert!(verify_note(&good.replace('\n', "\r\n"), &ki("op", &op), &ws, 1).is_err());
    // signature over text that excludes the trailing newline does not verify
    let lines = vec![sign_line(t.trim_end(), "op", &op)];
    assert!(verify_note(&note(&t, &lines), &ki("op", &op), &ws, 0).is_err());
    // reordered sig lines are fine
    let mut l = vec![sign_line(&t, "w", &w), sign_line(&t, "op", &op)];
    assert!(verify_note(&note(&t, &l), &ki("op", &op), &ws, 1).is_ok());
    l.reverse();
    assert!(verify_note(&note(&t, &l), &ki("op", &op), &ws, 1).is_ok());
    // garbage / unknown / malformed lines among valid ones are skipped, not fatal
    let mut n = good.clone();
    n.push_str("\u{2014} nobody !!!notbase64\n\u{2014} \nplain text\n\u{2014} w\n");
    assert!(verify_note(&n, &ki("op", &op), &ws, 1).is_ok());
}

// regression: signature bound to exact text (size/root/origin); a witness cosignature for another size does not transfer.
#[test]
fn regression_signature_bound_to_text() {
    let (op, w) = (sk(1), sk(2));
    let ws = [ki("w", &w)];
    let a = cp(3).text();
    let b = cp(4).text();
    let n = note(&a, &[sign_line(&a, "op", &op), sign_line(&b, "w", &w)]);
    assert_eq!(
        verify_note(&n, &ki("op", &op), &ws, 1),
        Err(CheckpointError::Quorum { got: 0, need: 1 })
    );
}

// regression: key names containing spaces round-trip (rsplit on the last space) and a name cannot be spoofed by
// shifting the space boundary between name and base64.
#[test]
fn regression_key_name_with_spaces() {
    let (op, w) = (sk(1), sk(2));
    let t = cp(3).text();
    let n = make(&t, &[("op", &op), ("my witness", &w)]);
    assert!(verify_note(&n, &ki("op", &op), &[ki("my witness", &w)], 1).is_ok());
    assert!(verify_note(&n, &ki("op", &op), &[ki("witness", &w)], 1).is_err());
    assert!(verify_note(&n, &ki("op", &op), &[ki("my", &w)], 1).is_err());
}

// regression: one name, two keys: only the key matching key-id and signature counts; counts at most once.
#[test]
fn regression_one_name_two_keys_counts_once() {
    let (op, w1, w2) = (sk(1), sk(2), sk(3));
    let t = cp(3).text();
    let n = make(&t, &[("op", &op), ("w", &w1), ("w", &w2)]);
    let r = verify_note(&n, &ki("op", &op), &[ki("w", &w1), ki("w", &w2)], 2);
    assert_eq!(r, Err(CheckpointError::Quorum { got: 1, need: 2 }));
}

// regression: wrong key id prefix, wrong-length payload, and flipped signature bit are ignored.
#[test]
fn regression_bad_sig_payloads() {
    let (op, w) = (sk(1), sk(2));
    let t = cp(3).text();
    let ws = [ki("w", &w)];
    let line = sign_line(&t, "w", &w);
    let b64 = line.trim_end().rsplit(' ').next().unwrap();
    let raw = B64.decode(b64).unwrap();
    let opl = sign_line(&t, "op", &op);
    for mutate in 0..raw.len() {
        let mut r = raw.clone();
        r[mutate] ^= 1;
        let n = format!("{}\n{}\u{2014} w {}\n", t, opl, B64.encode(&r));
        assert!(
            verify_note(&n, &ki("op", &op), &ws, 1).is_err(),
            "byte {mutate}"
        );
    }
    let mut r = raw.clone();
    r.push(0);
    assert!(verify_note(
        &format!("{}\n{}\u{2014} w {}\n", t, opl, B64.encode(&r)),
        &ki("op", &op),
        &ws,
        1
    )
    .is_err());
    assert!(verify_note(
        &format!("{}\n{}\u{2014} w {}\n", t, opl, B64.encode(&raw[..67])),
        &ki("op", &op),
        &ws,
        1
    )
    .is_err());
}

// regression: key id matches signed-note definition, SHA-256(name || 0x0A || 0x01 || pubkey)[:4].
#[test]
fn regression_key_id_definition() {
    use sha2::{Digest, Sha256};
    let k = sk(9).verifying_key();
    let mut h = Sha256::new();
    h.update(b"name\n\x01");
    h.update(k.as_bytes());
    assert_eq!(key_id("name", &k)[..], h.finalize()[..4]);
}

// AMBIGUITY (medium): verify_note returns the checkpoint without binding it to an expected origin. A note validly signed
// by the same operator/witnesses for ANOTHER log origin is accepted. Callers must compare cp.origin themselves;
// API gives no way to pass the expected origin. Test passes (documents behaviour).
#[test]
fn ambiguity_origin_not_checked() {
    let (op, w) = (sk(1), sk(2));
    let other = Checkpoint {
        origin: "other.example/log".into(),
        size: 1,
        root: [1; 32],
    };
    let t = other.text();
    let n = make(&t, &[("op", &op), ("w", &w)]);
    assert_eq!(
        verify_note(&n, &ki("op", &op), &[ki("w", &w)], 1)
            .unwrap()
            .origin,
        "other.example/log"
    );
}

// AMBIGUITY (low): trailing newline after the last signature line is not required, and "\r" before it is tolerated
// (str::lines strips \r\n). signed-note requires the note to end with "\n".
#[test]
fn ambiguity_note_without_final_newline_accepted() {
    let (op, w) = (sk(1), sk(2));
    let t = cp(3).text();
    let n = make(&t, &[("op", &op), ("w", &w)]);
    let n = n.trim_end_matches('\n').to_string();
    assert!(verify_note(&n, &ki("op", &op), &[ki("w", &w)], 1).is_ok());
}

// BUG (low): note with key name containing '+' or whitespace chars (forbidden by signed-note) is accepted as KeyInfo
// without validation; e.g. a name with a leading/trailing space or a newline.  A name containing '\n' can never verify
// (cannot appear on a line) but is not rejected at construction either; this documents that sign_line happily emits a
// line-injecting name.
#[test]
fn bug_sign_line_name_with_newline_injects_line() {
    let w = sk(2);
    let line = sign_line("t\n", "evil\n\u{2014} forged AAAA", &w);
    assert_eq!(
        line.matches('\n').count(),
        1,
        "sign_line emitted a multi-line signature (line injection) for name containing newline"
    );
}

// regression: split view
#[test]
fn regression_split_view_origin_differs() {
    let a = cp(3);
    let mut b = cp(3);
    b.origin = "x".into();
    b.root = [8; 32];
    assert!(!is_split_view(&a, &b));
}
