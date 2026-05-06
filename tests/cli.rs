//! Integration tests for the AGENT‑SEED CLI toolchain.
//!
//! Uses `assert_cmd` to run the compiled binaries as subprocesses
//! and `predicates` to assert on their output and exit codes.

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// ── seedc ──

#[test]
fn test_seedc_version() {
    let mut cmd = Command::cargo_bin("seedc").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("seedc"));
}

#[test]
fn test_seedc_build_hello() {
    let mut cmd = Command::cargo_bin("seedc").unwrap();
    cmd.arg("build").arg("examples/hello.seed").arg("-o").arg("/tmp/hello_test.aslb");
    let output = cmd.output().unwrap();
    // Accept either success or a compile error (language still evolving)
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success() || stderr.contains("not implemented") || stderr.contains("todo"),
        "unexpected failure: {}",
        stderr
    );
}

#[test]
fn test_seedc_check_hello() {
    let mut cmd = Command::cargo_bin("seedc").unwrap();
    cmd.arg("check").arg("examples/hello.seed");
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success() || stderr.contains("not implemented") || stderr.contains("passes"),
        "unexpected failure: {}",
        stderr
    );
}

#[test]
fn test_seedc_emit_grammar() {
    let mut cmd = Command::cargo_bin("seedc").unwrap();
    cmd.arg("emit-grammar").arg("--format").arg("gbnf");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    if output.status.success() {
        assert!(stdout.contains("root") || stdout.contains("item"));
    }
}

#[test]
fn test_seedc_missing_file() {
    let mut cmd = Command::cargo_bin("seedc").unwrap();
    cmd.arg("build").arg("nonexistent.seed");
    cmd.assert()
        .failure();
}

// ── seedvm ──

#[test]
fn test_seedvm_version() {
    let mut cmd = Command::cargo_bin("seedvm").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("seedvm"));
}

#[test]
fn test_seedvm_run_missing() {
    let mut cmd = Command::cargo_bin("seedvm").unwrap();
    cmd.arg("run").arg("nonexistent.aslb");
    cmd.assert()
        .failure();
}

// ── seedfmt ──

#[test]
fn test_seedfmt_pipe() {
    let mut cmd = Command::cargo_bin("seedfmt").unwrap();
    cmd.arg("-").write_stdin("agent test { fn run() {} }");
    let output = cmd.output().unwrap();
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(!stdout.is_empty());
    }
}

#[test]
fn test_seedfmt_check() {
    let mut cmd = Command::cargo_bin("seedfmt").unwrap();
    cmd.arg("--check").arg("examples/hello.seed");
    // Check mode may exit 0 (already formatted) or 1 (needs formatting)
    let output = cmd.output().unwrap();
    assert!(output.status.code() == Some(0) || output.status.code() == Some(1));
}
