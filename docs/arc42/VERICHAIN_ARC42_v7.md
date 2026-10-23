# VeriChain — ARC42 v7 (design baseline for coding)

Status: DRAFT for approval at gate G1 · Date: 2026-10-23 · Supersedes `docs/archive/VERICHAIN_ARC42_v6.md`
Evidence base: `docs/addenda/A–G`; claim status in `docs/SOURCES.md`. Anything not marked **[verified]** in SOURCES is an assumption to be checked at the gate that first depends on it.

---
## 1. Introduction and goals
> Financial objectives and revenue plan: `docs/addenda/H_financial_objectives_and_innovation.md` (evidence/assurance revenue; trading is not the revenue plan).

### 1.1 Purpose
VeriChain is **verifiable evidence for autonomous agents**: a witnessed, append-only, post-quantum-ready log of what an agent was authorized to do and what it did, with offline verification, evidence packs for auditors/insurers/regulators, and a neutral verifier other vendors' receipts can be checked in.

### 1.2 Requirements (functional)
| ID | Requirement |
|---|---|
| F1 | Accept signed **statements** (intent, outcome, policy, delegation) from agents/runtimes/gateways and return **receipts** proving registration in a tile-based Merkle log |
| F2 | Publish signed **checkpoints**, inclusion and consistency proofs; accept **witness cosignatures**; serve all of it as static tiles |
| F3 | Provide a **completeness** mechanism: intent/outcome pairing with per-agent sequence numbers so omitted or orphaned actions are detectable |
| F4 | Offline **verifier** (CLI + WASM): verify signature chain, inclusion, consistency, witness quorum, anchor, and completeness for a log slice |
| F5 | **Gateway tier** (MCP/A2A/HTTP proxy) so non-ASL agents get C1 evidence; **policy-enforcing signer** and **ASL hook** for C2 |
| F6 | **Evidence-pack exporter** (per agent, per period) with commitments, proofs, orphan list, assurance/completeness levels |
| F7 | **Privacy**: payloads off-log, salted commitments on-log, crypto-shredding for erasure |
| F8 | **Time anchoring** (RFC 3161 and/or OpenTimestamps) of checkpoints |
| F9 | **PQ-ready signing**: hybrid checkpoint cosignatures, documented key rotation and evidence renewal |

### 1.3 Quality goals (priority order)
1. **Verifiability by outsiders** (anyone can check without trusting the operator).
2. **Honest claims** (every record declares completeness C0–C3 and assurance A0–A3).
3. **Interoperability** (SCITT-conformant receipts; C2SP tile-log formats where applicable).
4. **Low operating cost / small footprint** (runs on a 4 GB machine; static-file serving).
5. **Long-term evidentiary durability** (key rotation, renewal, retention).

### 1.4 Stakeholders
Agent developers/deployers; compliance and audit teams; insurers/underwriters; regulators and examiners; counterparties/marketplaces; witnesses/monitors; Intellica products (Verity, VeriCrypt, ASL) as first deployers; the founder (single maintainer).

---
## 2. Constraints
| Constraint | Consequence |
|---|---|
| Solo founder; dev machine is a 4 GB-RAM Chromebook (Linux) | small dependency set; heavy builds in GitHub Actions; incremental builds with `-j2`; no heavyweight local services |
| No code before G1 approval | this document + test vectors come first |
| ASL v0.2.0 charters/speculation not implemented (separate repo) | MVP cannot depend on ASL; gateway tier (C1) and signer (C2 for venue actions) ship first |
| Standards in flux (SCITT agent-receipt drafts expire Dec 2026; PQ transparency drafts) | isolate profile choices behind a versioned `profile` field; track drafts in SOURCES |
| Legal: no compliance guarantees may be claimed | marketing/docs say "supports evidence for…", never "compliant with" |
| Open-core | spec, verifier, log server, SDKs under Apache-2.0; hosted operations and packs commercial |
| Python/Rust constraints from dev environment | pip needs `--break-system-packages`; use `python3 -m` entry points; never `pip freeze` for requirements |

---
## 3. Context and scope

### 3.1 Business context
```
 Agent runtime (ASL / any agent) ──┐
 Gateway (MCP/A2A/HTTP proxy)  ────┼─► VeriChain ──► Verifier (CLI/WASM) ─► Auditor / Insurer / Regulator
 Policy-enforcing signer ──────────┘      │                               └► Counterparty / Marketplace
                                          ├─► Witnesses (independent cosigners)
                                          └─► Time anchors (RFC 3161 TSA, OpenTimestamps)
```
### 3.2 Technical interfaces
| Interface | Protocol | Notes |
|---|---|---|
| Submit statement | HTTPS, SCITT SCRAPI-style (draft) | `POST /v1/statements` → registration id; receipt via `GET /v1/receipts/{id}` (async) |
| Read log | static HTTPS tiles + `GET /checkpoint` | C2SP tlog-tiles / tlog-checkpoint style (to be confirmed at G1) |
| Witness | cosignature over checkpoint | tlog-cosignature style; PQ cosignature algorithm is an extension to define |
| Anchor | RFC 3161 request; OpenTimestamps calendar | on checkpoint hash |
| Export | evidence pack (zip: statements, receipts, tiles slice, checkpoints, anchors, manifest) | verifier consumes it offline |

### 3.3 Out of scope for v1
Own consensus/blockchain, tokens (UBE), World ID, agent exchange, Lightning settlement, reputation scores, multi-operator ordering. (Deferred; see ADR-V12.)

---
## 4. Solution strategy
| Goal | Approach |
|---|---|
| Outsider verifiability | Merkle log + signed checkpoints + independent witness cosignatures + open offline verifier |
| Interoperability | SCITT (RFC 9943) statements/receipts; COSE_Sign1; RFC 8785 JSON canonicalization; tile format for static hosting |
| Completeness | two-phase **intent → outcome** statements, per-agent monotonic `seq` and `prev` hash; a chokepoint (signer, gateway, VM discharge) emits intent **before** the effect; orphans are reported |
| Honest assurance | explicit levels C0–C3 and A0–A3 on every statement |
| PQ durability | sign/cosign **checkpoints** (hybrid Ed25519 + ML-DSA), give each receipt a hash path; renewal by re-anchoring, never re-signing history |
| Low cost | log is files; sequencer is a single small binary; SQLite index |
| Privacy | commitments only on-log; payloads in tenant storage |

Levels
- **Completeness:** C0 self-reported · C1 gateway-observed (bypassable) · C2 chokepoint-enforced (signer/VM: no effect without prior intent receipt) · C3 C2 + attested runtime (TEE/measured boot).
- **Assurance:** A0 signed · A1 deterministic replay of non-model logic with recorded model outputs as oracle inputs · A2 TEE attestation of the runtime · A3 zkVM proof that the *gate* applied policy P to committed inputs and returned verdict V (never a proof of the LLM, never "100% proven").

---
## 5. Building block view

### 5.1 Level 1 (Cargo workspace; names are working names)
| Crate / binary | Responsibility | Depends on |
|---|---|---|
| `vc-core` | types, canonical JSON (RFC 8785), hashing (SHA-256), ids, errors | — |
| `vc-cose` | COSE_Sign1 build/verify; algorithm registry (Ed25519 `-19`, ML-DSA candidate) | vc-core |
| `vc-log` | tile log: append, checkpoint, inclusion/consistency proofs, tile read/write (FS and S3-compatible) | vc-core |
| `vc-seq` (bin `vcd`) | sequencer service: validates statements, enforces registration policy, appends, signs checkpoints, SCRAPI HTTP API | vc-log, vc-cose |
| `vc-witness` (bin) | fetches checkpoint, verifies consistency vs last-seen, cosigns | vc-log, vc-cose |
| `vc-anchor` | RFC 3161 client; OpenTimestamps client | vc-core |
| `vc-verify` (lib, CLI, WASM) | verifies statements, receipts, checkpoints, cosignature quorum, anchors, completeness | vc-core, vc-cose, vc-log (proof logic only) |
| `vc-gateway` (bin) | MCP/A2A/HTTP proxy emitting intent/outcome statements (C1) | vc-sdk |
| `vc-signer` (bin) | policy-checked signing service; emits intent before signing (C2 for venue actions) | vc-sdk |
| `vc-sdk` | client library for emitters (Rust first; Python/TS bindings later) | vc-core, vc-cose |
| `vc-pack` | evidence-pack builder and manifest | vc-verify |
| `vc-asl-hook` (later) | ASL `prov!`/discharge adapter (C2/C3) | vc-sdk |

### 5.2 Level 2 — statement model (**Decision Primitive v2**)
COSE_Sign1; protected header: `alg`, `kid`, content type, CWT claims `iss` (issuer/agent key id), `sub` (agent id). Payload = RFC 8785 canonical JSON:
```json
{
  "v": 2,
  "kind": "intent | outcome | policy | delegation | artifact | attestation",
  "agent": {"id": "<did or key thumbprint>", "runtime": {"kind": "asl|gateway|signer|sdk", "version": "..."}},
  "seq": 41, "prev": "<sha256 of previous statement of this agent>",
  "ts": "2026-10-23T02:23:00Z",
  "principal": {"delegator": "<id>", "capability_chain": ["<hash>", "..."], "policy_hash": "<sha256>"},
  "action": {"type": "venue.order|tool.call|payment|...", "target": "<uri>", "params_commitment": "<hmac>"},
  "pair": {"intent_id": "<id>"},                       // outcome only
  "verdict": {"decision": "allow|deny|escalate", "reasons": ["..."]},
  "io": {"input_commitment": "<hmac>", "output_commitment": "<hmac>",
         "model": {"id": "...", "version_hash": "..."}, "oracle_inputs": ["<commitment>", "..."]},
  "completeness": "C0|C1|C2|C3", "assurance": ["A0"],
  "ext": {}
}
```
Rules: (1) `seq` strictly increases per agent; (2) an `outcome` references its `intent`; (3) an intent with no outcome after `timeout` is an **orphan** and reported; (4) commitments = HMAC-SHA256(tenant_key, canonical payload bytes) with per-record salt kept off-log; (5) unknown `ext` keys are ignored by verifiers but covered by the signature.

### 5.3 Level 2 — log
- Leaf = `SHA-256(0x00 || statement_bytes)`; node = `SHA-256(0x01 || left || right)` (RFC 9162 style).
- Tiles of 256 hashes; entry bundles; checkpoint = origin line, tree size, root hash, signature lines (operator + witnesses).
- Operator key signs checkpoints every ≤ N seconds or M entries; witnesses cosign; anchors timestamp the checkpoint hash.

### 5.4 Registration policy (published on the log, per RFC 9943)
Accept if: valid signature by a registered issuer key; schema valid; `seq` = last+1 for that agent (or an explicit reset statement); size limits; rate limits. The most recently committed policy applies.

---
## 6. Runtime view
**S1 — Gateway (C1):** agent → gateway → (1) gateway submits `intent` statement, waits for registration id (not full receipt) → (2) forwards tool call → (3) submits `outcome` with output commitment. Bypass = no evidence; C1 says so.
**S2 — Signer (C2 for venue actions):** agent proposes order → signer checks policy/limits → submits `intent` (must be registered) → signs order with the venue key (agent never holds the key) → submits `outcome` on venue ack/fill. Venue accepts only signer-signed orders, so omission is structurally prevented for those actions.
**S3 — ASL (C2/C3, later):** VM discharge gate emits `intent` before any effect; effects without registered intent are rejected by the VM.
**S4 — Witnessing:** witness fetches checkpoint, validates consistency against last-cosigned, cosigns; verifier requires N-of-M cosignatures in its policy.
**S5 — Offline verification:** pack → verify signatures → inclusion proofs → checkpoint signatures + witness quorum → anchor tokens → completeness (pairs, seq gaps, orphans) → report with levels.
**S6 — Erasure:** tenant deletes payload+salt+per-record key; log keeps the commitment; verifier reports "payload unavailable (erased)" without breaking the chain.
**S7 — Key rotation/renewal:** new operator and witness keys announced by a policy statement; old checkpoints stay valid under old keys; evidence renewal = new anchor + new hybrid cosignature over a recent checkpoint that commits to the old tree.

---
## 7. Deployment view
- **v1 topology:** one `vcd` sequencer, object-storage (or local FS) tile store behind a CDN/static host, SQLite index, 2+ independent `vc-witness` instances (different operators/providers), TSA client. No consensus protocol required.
- **Environments:** local (docker-compose: vcd + witness + mock TSA), staging, production-hosted (commercial).
- **Cost model (assumption):** receipt ≈ 1 KB → 1M receipts ≈ 1 GB; verify at G2.
- **Build/CI:** GitHub Actions does full builds, `cargo deny`, `cargo audit`, fuzz targets, vector tests; releases with SLSA provenance and SBOM.

---
## 8. Cross-cutting concepts
- **Crypto suite (v1):** SHA-256; Ed25519 (COSE `-19` fully specified); hybrid checkpoint cosignature adds ML-DSA (FIPS 204; parameter set chosen at G6, candidate ML-DSA-65); SLH-DSA evaluated for the long-lived root. PQ cosignature encoding in tile-log formats is not standardized: define as an extension with a version tag and track IETF PLANTS/Merkle Tree Certificate work.
- **Canonicalization:** RFC 8785; reject duplicate keys; numbers as strings for amounts.
- **Privacy/GDPR:** commitments, payload locality, crypto-shredding; no personal data on-log by schema.
- **Security:** `#![forbid(unsafe_code)]` in all crates except audited bindings; fuzzing for parsers; constant-time comparisons; dependency policy via `cargo deny`; no secrets in logs.
- **Observability:** structured logs, metrics (submit latency, checkpoint age, witness lag, orphan count).
- **Compliance posture:** mappings (EU AI Act Art. 12, NIST agent logging themes, SEC/FINRA, DORA) are *evidence-support documents*, reviewed by a practitioner before publication.
- **Testing:** golden test vectors, property tests (Merkle proofs), differential tests against an independent tlog implementation, fuzzing, chaos tests for witness failure/split view.

---
## 9. Architecture decisions
| ID | Decision | Why | Consequence |
|---|---|---|---|
| V01 | Follow SCITT (RFC 9943) for statements/receipts | standard; interoperability; credibility | track agent-receipt drafts; profile versioned |
| V02 | Tile-based Merkle log, static serving | cheap, cacheable, proven (Tessera/Rekor v2 direction) | write own minimal implementation or reuse; decide at G2 |
| V03 | Single sequencer + independent witnesses; no BFT | transparency logs need non-equivocation, not distributed ordering | simpler, cheaper; sequencer is a trusted-but-checked component |
| V04 | COSE_Sign1 over RFC 8785 JSON | aligns with agent-receipt drafts; human-debuggable | canonicalization bugs are a risk → vectors + fuzz |
| V05 | Two-phase intent/outcome with per-agent seq | makes omission/orphans detectable | extra latency and one more round trip |
| V06 | Commitments on-log, payloads off-log | privacy, erasure | verifier needs payload+salt for full checks |
| V07 | PQ at checkpoint level (hybrid), hash paths per receipt | avoids per-receipt PQ signature cost | PQ cosignature format is an extension; renewal procedure required |
| V08 | Assurance ladder A0–A3, completeness C0–C3 | honest claims | needs careful docs; no "100% proven" |
| V09 | Open-core: Apache-2.0 spec/verifier/server/SDK; commercial hosting, witnessing, retention, packs | trust + revenue (Addenda D, G) | competitors may self-host; moat = independence, packs, standards |
| V10 | Rust workspace; WASM verifier; SQLite index | footprint, safety, portability | build size/time on 4 GB machine managed via CI |
| V11 | Ship gateway (C1) and signer (C2) before ASL hook | ASL v0.2.0 incomplete; faster adoption | ASL becomes the premium C2/C3 path |
| V12 | Defer: ORCHID consensus, UBE/World ID, exchange, Lightning-first settlement, own trading P&L as revenue | unverified/unfit/legal-heavy (Addenda A, B, F) | revisit only with evidence |
| V13 | Tool/artifact provenance profile: `artifact` (tool, MCP server, skill, model, config build digest) and `attestation` (vetting by a named party) statement kinds; gateway can verify or flag unsigned tools | no major MCP client verifies tool signatures natively; supply-chain provenance is a proven paying category (Addendum H P2) | validate with 2 design partners at M0 |

---
## 10. Quality requirements (measurable; targets to be validated at the stated gate)
| ID | Scenario | Target | Gate |
|---|---|---|---|
| Q1 | Third party verifies an evidence pack offline with only the verifier binary | passes with no network | G4 |
| Q2 | Split-view attempt (two different checkpoints for same size) is detected by verifier with 2 witnesses | detection 100% in test | G3 |
| Q3 | Submit→registration id latency, 1 vCPU, batching on | p99 ≤ 1 s at 200 stmts/s (to measure) | G2 |
| Q4 | Memory footprint of `vcd` | < 1 GB RSS at 1M entries | G2 |
| Q5 | Omitted action (C2 path) cannot execute without intent | 0 escapes in 10k fuzzed runs | G5 |
| Q6 | Orphan intent reported within `timeout` + 1 checkpoint interval | 100% in tests | G5 |
| Q7 | Key rotation without invalidating old proofs | old packs still verify | G6 |
| Q8 | Verifier WASM size | ≤ 2 MB (target) | G4 |
| Q9 | Independent implementation verifies a VeriChain receipt | at least one | M1 |

---
## 11. Risks and technical debt
| Risk | Severity | Mitigation |
|---|---|---|
| Standards churn / draft expiry | M | profile versioning; track in SOURCES; contribute test vectors |
| Cryptographic implementation bugs | H | vectors, differential testing, fuzzing, independent review before selling assurance |
| Witness ecosystem access (public witness network onboarding unconfirmed) | M | run own witnesses with distinct operators; confirm onboarding path at G3 |
| PQ format immaturity | M | extension with version tag; renewal procedure |
| Market: crowded receipt space; incumbents bundle logging | H | differentiate on completeness, independence, packs, neutral verifier |
| Demand timing (EU high-risk Dec 2027; banking rules in flux) | M | developer-first adoption; paid pilots; kill criteria in ROADMAP |
| Solo-founder capacity | H | narrow v1; automation; open-source contributors |
| Legal claims (compliance, performance verification for allocators) | H | evidence-support language; counsel review |
| ASL dependency | M | C1/C2-signer first |

---
## 12. Glossary
**Statement** signed claim (intent/outcome/policy/delegation). **Receipt** proof of registration in the log (inclusion proof + checkpoint). **Checkpoint** signed log head. **Witness** independent party cosigning a checkpoint after a consistency check. **Orphan** intent with no outcome. **C0–C3** completeness levels. **A0–A3** assurance levels. **Evidence pack** offline-verifiable export. **SCITT** Supply Chain Integrity, Transparency and Trust (RFC 9943). **Tile log** Merkle log served as static tiles.

---
## Appendix A — Test vector plan (G1 deliverable)
1. JCS canonicalization (20 vectors incl. unicode, number edge cases, key ordering).
2. COSE_Sign1 encode/verify (Ed25519; negative: wrong kid/alg/payload).
3. Merkle: leaf/node hashing, inclusion/consistency proofs for sizes 1–1,000 plus powers of two ±1.
4. Checkpoint signature + witness cosignature quorum (N-of-M, negative: split view, stale witness).
5. Statement chain: seq gaps, wrong `prev`, outcome without intent, duplicate outcome, orphan timeout.
6. Commitments: HMAC with salt; erasure path.
7. Evidence pack manifest + tamper cases.
8. Time anchor tokens (mock TSA) and rotation/renewal scenario.

## Appendix B — Repository layout (target)
```
verichain/
  README.md ARCHITECTURE.md DEVLOG.md ROADMAP.md
  docs/ arc42/ addenda/ archive/ SOURCES.md
  spec/            # profile, schemas, vectors (language-neutral)
  crates/ vc-core vc-cose vc-log vc-seq vc-witness vc-anchor vc-verify vc-gateway vc-signer vc-sdk vc-pack
  verifier-wasm/  examples/  deploy/ (docker-compose, CI)
```
