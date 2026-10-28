# Builder brief (wave 1, shared by all builder agents)

You are a BUILDER agent in the VeriChain factory (ARC42 v9 Appendix G). You own exactly one crate. Another agent will
independently write vectors and a third will attack your code adversarially, so write code you would defend.

## Read first (in this order)
1. `HANDOFF.md`, `spec/contracts/L0.md` (C-contracts; the `Store`/`TileLog` parts are STALE: the real storage API is
   `vc-store` `TileStore`/`TileLog`, the real log/checkpoint API is `vc-log`), `spec/requirements.md`, `docs/THREAT_MODEL.md`.
2. `docs/arc42/VERICHAIN_ARC42_v9.md` section 5.1 and anything your crate-specific brief points to.
3. The public APIs you build on: `crates/vc-core`, `crates/vc-cose`, `crates/vc-log` (checkpoint.rs, merkle), `crates/vc-store`,
   `crates/vc-seq` (read `src/lib.rs` for style: strict validation, typed errors, transactional state).
4. `docs/reports/verifier_w1_store_seq.md` and `verifier_g05.md`: these are the classes of defect verifiers find
   (partial failure leaving state inconsistent, quorum/identity confusion, parser differentials, unbounded work on
   unauthenticated input, swallowed errors, trusting unverified data). Design against them from the start.

## Rules
- Work ONLY inside your crate directory (`crates/<your-crate>/`, tests and docs under it) plus, if essential, `spec/vectors/` files
  you create with a unique name. Do not edit other crates, root `Cargo.toml` members (the orchestrator pre-registered
  them), `spec/requirements.md`, or ARC42. If you need a change elsewhere, write it to `docs/reports/contract_requests.md`
  (append) and work around it.
- Do NOT run any git command that changes state (no add/commit/checkout/stash/reset). The orchestrator commits.
- Use `CARGO_TARGET_DIR=/home/claude/targets/<your-crate>` for every cargo invocation (other agents build concurrently; the machine has 2 cores, 8 GB).
  Always scope to your package: `cargo test -p <crate>`, `cargo clippy -p <crate> --all-targets -- -D warnings`, `cargo fmt -p <crate>`.
- Network: only the package registry works via the proxy (see /root/.ccr/README.md if TLS/proxy errors). No other site from the shell;
  WebFetch/WebSearch tools can read public specs. Prefer small, well-known, pure-Rust dependencies, pin sensibly, justify each in the crate README.
  Never disable TLS verification.
- Quality bar: library APIs only (no HTTP servers; HTTP clients only behind a trait with an in-memory fake). Deterministic, offline,
  `#![forbid(unsafe_code)]`, typed errors (`thiserror`), no `unwrap`/`expect`/panics on external input, bounded work and memory on
  untrusted input (document limits as constants), no swallowed errors, all-or-nothing state changes (compute new state, persist, then commit).
- Tests: your own unit/integration tests in `crates/<crate>/tests/` named `builder_*.rs`. Each test that proves a requirement must be named in
  your final report so the orchestrator can trace it. Keep the default `cargo test -p <crate>` under ~30 s in debug; mark long ones
  `#[ignore = "reason; nightly"]`.
- Traceability: put the requirement id (e.g. FR-L0-08) in the crate-level doc comment and in test names/doc comments.
- Write `crates/<crate>/README.md` (purpose, API summary, limits, dependencies and why, what is NOT done / unverified, how to run tests).
- If a spec you must follow is unreachable (fetch blocked), say so, implement from the best available source, and mark the module
  doc with `UNVERIFIED: <what>` so the independent vector author and verifier know.

## Final report (your last message, <= 60 lines)
Files created, public API summary, requirement ids covered with test names, commands run and their results (tests passed counts, clippy, fmt),
limits and known gaps, unverified assumptions, contract requests filed.
