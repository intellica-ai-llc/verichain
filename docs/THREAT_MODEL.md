# VeriChain threat model (G1 deliverable, v1)

Scope: all layers per ARC42 v9; depth is greatest for L0 (built) and decreases upward. Method: assets → trust boundaries → adversaries → threats (STRIDE) → control → test. Each control names the requirement/test that must prove it. Status: draft for human security review; update with every verifier report.

## 1. Assets
A1 integrity and completeness of the evidence log · A2 signing keys (operator, witness, agent, venue, KMS roots) · A3 tenant payloads and salts (off-log) · A4 agent sandboxes and host · A5 funds and venue credentials · A6 policy and delegation authority · A7 reputation/identity registry · A8 the build pipeline and releases.

## 2. Adversaries
Malicious or compromised agent code · malicious tool/MCP server/model provider · curious or hostile tenant · compromised operator (insider or intruder) · single compromised witness or TSA · network attacker · malicious counterparty/venue · supply-chain attacker (dependency, build) · colluding builder and verifier agents in our own factory · cryptanalytic adversary (future quantum).

## 3. Boundaries
B1 agent ⟷ host (runtime) · B2 tenant ⟷ tenant · B3 operator ⟷ verifier (operator not trusted for integrity) · B4 platform ⟷ external effects · B5 factory ⟷ production (code provenance).

## 4. L0 threats and controls (implemented or specified)
| ID | Threat | Control | Proof (test / vector) |
|---|---|---|---|
| T-L0-01 | Operator shows different histories to different parties (split view) | witnessed checkpoints; same origin+size with different roots flagged | `checkpoint.rs#split_view_detected`; consistency negatives in `merkle_v1.json` |
| T-L0-02 | Witness quorum inflation: one key under many names, operator counted as its own witness, ambiguous names | quorum counted per distinct key and name; operator key and name excluded | `verifier_g05_checkpoint.rs#bug_quorum_counts_names_not_keys`, `#bug_operator_counts_as_witness`, `#bug_operator_alias_as_witness` |
| T-L0-03 | Forged proofs, truncated/extended proofs, wrong index or size | RFC 9162 verification; sizes taken from the signed checkpoint | 3,179 inclusion + 2,862 consistency negative vectors |
| T-L0-04 | Canonicalization differences between implementations (UTF-16 key order, >2^53 integers, duplicate keys) → different statement ids, signature confusion | own strict JCS (I-JSON): UTF-16 sort, ES6 numbers, duplicates and lone surrogates rejected | `jcs_v1.json` (50 vectors), `verifier_g05_jcs.rs` |
| T-L0-05 | Parser differential in COSE (trailing bytes, duplicate labels, indefinite lengths, `crit`) | single definite CBOR item, duplicate labels rejected, crit rejected | `verifier_g05_cose.rs` |
| T-L0-06 | Small-order public key makes every signature forgeable | `verify_strict` plus weak-key rejection | `verifier_g05_cose.rs#bug_small_order_key_universal_forgery` |
| T-L0-07 | Omission or reordering of an agent's statements | per-agent seq/prev chain; orphan intents reported | `chain_v1.json` (27 cases) |
| T-L0-08 | Signature-line injection through hostile key names | control characters neutralized; strict name validator available | `verifier_g05_checkpoint.rs#bug_sign_line_name_with_newline_injects_line` |
| T-L0-09 | Checkpoint replayed from a different log | `verify_note_for_origin` binds the expected origin | `origin_binding.rs#origin_is_bound` |
| T-L0-10 | Algorithm break of classical signatures | hybrid COSE_Sign (Ed25519 + ML-DSA-65), renewal by re-anchoring | pending (FR-L1-02) |
| T-L0-11 | Time backdating | ≥2 independent RFC 3161 TSAs plus OpenTimestamps | pending (FR-L0-09) |

## 5. Upper layers (control summaries; tests specified in requirements)
- **L1** forged/over-broad delegation → attenuation checks, status lists, short expiry (FR-L1-01); key theft → HSM/KMS, rotation drills (FR-L1-02); registry tampering → registry derived from the log (FR-L1-03).
- **L2** sandbox escape → WASM capability model, microVM for hostile code, no route except host calls, seccomp; egress without intent → fail-closed host ABI (FR-L2-01); cross-tenant access → per-tenant keys/prefixes/quotas (FR-L2-02); credential theft → vault references only; model/tool supply chain → `artifact`/`attestation` statements, pinned digests, unsigned tools flagged; prompt injection leading to a harmful action → policy gate and capability bounds outside the model (OWASP agentic risks mapping to be completed).
- **L3** policy bypass or drift → deterministic fail-closed PDP, policy hash in every verdict; false assurance claims → verifier enforces assurance minimums, tolerances recorded; zk under-constrained circuits → audits, fuzzing, pinned versions.
- **L4** ordering attacks → witnesses plus (later) BFT with ≥3f+1; experimental consensus never in production before R1.
- **L5** payment replay/double spend → idempotency keys, mandates bound to statement hashes; escrow logic bugs → audits before funds; sybil reputation → derived only from verified evidence, R6.
- **L6** governance capture → separation of powers, timelocks, public tally; console compromise → no secrets in browser, step-up approvals for irreversible actions.

## 6. Factory threats (B5)
Builder and verifier collusion or shared blind spots → different prompts and model families; vector authors independent of builders (done for G0.5); malicious dependency → `cargo deny`/`cargo vet`, pinned lockfile hashed in acceptance bundles; AI-written code defects → traceability gate, adversarial verification, human sign-off on security-critical modules, external audit before assurance claims; secrets leakage to agents → vault references and scoped credentials.

## 7. Residual risks and open items
Public witness ecosystem immaturity; post-quantum libraries maturity; legal gates; formal specification of ordering and key rotation (Quint/TLA+) not yet written; this model has not had independent human review.
