//! T-L0-09: a validly signed checkpoint of another log must not be accepted. Requirement trace: FR-L0-06.
use ed25519_dalek::SigningKey;
use vc_log::checkpoint::*;

#[test]
fn origin_is_bound() {
    let (op, w) = (
        SigningKey::from_bytes(&[1; 32]),
        SigningKey::from_bytes(&[2; 32]),
    );
    let (opk, wk) = (
        KeyInfo {
            name: "op".into(),
            key: op.verifying_key(),
        },
        KeyInfo {
            name: "w".into(),
            key: w.verifying_key(),
        },
    );
    let cp = Checkpoint {
        origin: "log-a.example".into(),
        size: 3,
        root: [7; 32],
    };
    let t = cp.text();
    let n = note(&t, &[sign_line(&t, "op", &op), sign_line(&t, "w", &w)]);
    assert!(
        verify_note_for_origin(&n, "log-a.example", &opk, std::slice::from_ref(&wk), 1).is_ok()
    );
    assert_eq!(
        verify_note_for_origin(&n, "log-b.example", &opk, std::slice::from_ref(&wk), 1),
        Err(CheckpointError::WrongOrigin)
    );
}
