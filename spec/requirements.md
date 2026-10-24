# Requirements and traceability (maintained by the spec steward)

Status values: `implemented` (all listed tests pass), `partial` (some behaviour done, rest pending), `pending` (no code yet).
Rule (Q19): every `implemented` or `partial` row must reference at least one existing test; `tools/factory/trace_check.py` enforces this in CI.
Test reference format: `path#test_fn`, comma separated.

| ID | Requirement | Component | Status | Tests |
|---|---|---|---|---|
| FR-L0-01 | Validate statement chains: per-agent seq/prev, intent/outcome pairing, schema constants; report open intents | vc-core | partial | crates/vc-core/tests/jcs_chain.rs#negatives, crates/vc-core/tests/vectors_core.rs#chain_vectors, crates/vc-core/tests/verifier_g05_chain.rs#bug_schema_constants_not_enforced |
| FR-L0-02 | Offline verification of Merkle inclusion and consistency proofs (RFC 9162) | vc-log | implemented | crates/vc-log/tests/vectors_merkle.rs#inclusion_vectors, crates/vc-log/tests/vectors_merkle.rs#consistency_vectors, crates/vc-log/tests/merkle.rs#inclusion_all_sizes, crates/vc-log/tests/merkle.rs#consistency_all_pairs |
| FR-L0-03 | Detect equivocation (split view) | vc-log | implemented | crates/vc-log/tests/checkpoint.rs#split_view_detected |
| FR-L0-04 | RFC 8785 canonicalization, I-JSON strict (UTF-16 key order, ES6 numbers, duplicate keys rejected) | vc-core | implemented | crates/vc-core/tests/vectors_core.rs#jcs_vectors, crates/vc-core/tests/jcs_chain.rs#rfc8785_example, crates/vc-core/tests/verifier_g05_jcs.rs#bug_duplicate_keys_rejected |
| FR-L0-05 | Strict COSE_Sign1 verification (single definite item, no duplicate labels, no crit, strict Ed25519, weak keys refused) | vc-cose | implemented | crates/vc-cose/tests/sign1.rs#roundtrip, crates/vc-cose/tests/verifier_g05_cose.rs#bug_small_order_key_universal_forgery, crates/vc-cose/tests/verifier_g05_cose.rs#bug_trailing_bytes_after_sign1_accepted |
| FR-L0-06 | Checkpoint note (C2SP) verification with operator signature and independent witness quorum counted by distinct key and name | vc-log | implemented | crates/vc-log/tests/checkpoint.rs#quorum_and_negatives, crates/vc-log/tests/verifier_g05_checkpoint.rs#bug_quorum_counts_names_not_keys, crates/vc-log/tests/verifier_g05_checkpoint.rs#bug_operator_counts_as_witness, crates/vc-log/tests/origin_binding.rs#origin_is_bound |
| FR-L0-07 | Sequencer accepts only valid in-policy statements and returns a registration id before any dependent effect | vc-seq | pending | |
| FR-L0-08 | Witness cosigning with consistency check and gossip | vc-witness | pending | |
| FR-L0-09 | Trusted-time anchoring (RFC 3161 x2 + OpenTimestamps) | vc-anchor | pending | |
| FR-L0-10 | Evidence packs with manifest and tamper detection | vc-pack | pending | |
| FR-L0-11 | Verifier CLI and WASM build verify all of the above offline | vc-verify | pending | |
| FR-L0-12 | AAT import/export adapter | vc-aat | pending | |
| FR-L1-01 | Verify attenuating delegation chains offline | vc-cap | pending | |
| FR-L1-02 | Rotate keys without invalidating old evidence; hybrid COSE_Sign (Ed25519 + ML-DSA-65) | vc-kms, vc-cose | pending | |
| FR-L1-03 | Registry state derived from the log | vc-id | pending | |
| FR-L2-01 | No external effect without durable intent (hosted) | vc-hostio | pending | |
| FR-L2-02 | Per-tenant isolation of keys, storage, quotas | vc-run, vc-vault | pending | |
| FR-L2-03 | Policy-enforcing signer for venues and payments | vc-signer | pending | |
| FR-L2-04 | Gateway records MCP/A2A/HTTP calls and checks tool provenance | vc-gateway | pending | |
| FR-L3-01 | Deterministic policy evaluation; AuthZEN PDP API | vc-fge | pending | |
| FR-L3-02 | Replay verification (A1) | vc-replay | pending | |
| FR-L3-03 | TEE attestation and verified inference (A2) | vc-attest | pending | |
| FR-L3-04 | ZK proof of policy-gate evaluation (A3) | vc-prove | pending | |
| FR-L4-01 | Orderer swap without verifier change | vc-seq, vc-bft | pending | |
| FR-L4-02 | Federation and cross-log witnessing | vc-federate | pending | |
| FR-L5-01 | Agent payments (x402, AP2 mandates) recorded as statements | vc-pay | pending | |
| FR-L5-02 | Evidence-linked job escrow | vc-jobs | pending | |
| FR-L5-03 | Reputation derived from evidence | vc-rep | pending | |
| FR-L6-01 | Verifiable governance outcomes | vc-gov | pending | |
| FR-L6-02 | Compliance evidence packs and mappings | vc-comply | pending | |
| FR-L6-03 | Operator console with approvals and kill switch | vc-console | pending | |
