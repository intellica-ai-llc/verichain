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
fn cp(size: u64, b: u8) -> Checkpoint {
    Checkpoint {
        origin: "vc.example/log1".into(),
        size,
        root: [b; 32],
    }
}

#[test]
fn text_roundtrip_and_format() {
    let c = cp(7, 9);
    let t = c.text();
    assert!(t.starts_with("vc.example/log1\n7\n"));
    assert_eq!(Checkpoint::parse(&t).unwrap(), c);
    assert!(Checkpoint::parse("a\nx\nAAAA\n").is_err());
}

fn make(c: &Checkpoint, signers: &[(&str, &SigningKey)]) -> String {
    let t = c.text();
    let lines: Vec<String> = signers.iter().map(|(n, k)| sign_line(&t, n, k)).collect();
    note(&t, &lines)
}

#[test]
fn quorum_and_negatives() {
    let (op, w1, w2, w3, evil) = (sk(1), sk(2), sk(3), sk(4), sk(5));
    let opk = ki("op", &op);
    let ws = [ki("w1", &w1), ki("w2", &w2), ki("w3", &w3)];
    let c = cp(10, 1);

    let n = make(&c, &[("op", &op), ("w1", &w1), ("w2", &w2)]);
    assert_eq!(verify_note(&n, &opk, &ws, 2).unwrap(), c);
    assert!(matches!(
        verify_note(&n, &opk, &ws, 3),
        Err(CheckpointError::Quorum { got: 2, need: 3 })
    ));

    // untrusted cosigner does not count; duplicate signature by the same witness counts once
    let n2 = make(
        &c,
        &[("op", &op), ("w1", &w1), ("w1", &w1), ("evil", &evil)],
    );
    assert!(matches!(
        verify_note(&n2, &opk, &ws, 2),
        Err(CheckpointError::Quorum { got: 1, need: 2 })
    ));

    // missing operator
    let n3 = make(&c, &[("w1", &w1), ("w2", &w2)]);
    assert_eq!(
        verify_note(&n3, &opk, &ws, 2),
        Err(CheckpointError::NoOperator)
    );

    // tampered text (root changed) invalidates every signature
    let n4 = n.replacen(&c.text(), &cp(10, 2).text(), 1);
    assert_eq!(
        verify_note(&n4, &opk, &ws, 0),
        Err(CheckpointError::NoOperator)
    );

    // witness signing a different (stale) checkpoint does not count for this one
    let stale = cp(5, 7);
    let mut lines = vec![
        sign_line(&c.text(), "op", &op),
        sign_line(&stale.text(), "w1", &w1),
        sign_line(&stale.text(), "w2", &w2),
    ];
    let n5 = note(&c.text(), &lines);
    assert!(matches!(
        verify_note(&n5, &opk, &ws, 1),
        Err(CheckpointError::Quorum { got: 0, need: 1 })
    ));
    lines.clear();
}

#[test]
fn split_view_detected() {
    assert!(is_split_view(&cp(10, 1), &cp(10, 2)));
    assert!(!is_split_view(&cp(10, 1), &cp(11, 2)));
    assert!(!is_split_view(&cp(10, 1), &cp(10, 1)));
}
